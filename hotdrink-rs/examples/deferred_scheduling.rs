use std::{
    error::Error,
    fmt::Debug,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Clone)]
struct Task {
    name: String,
    outputs: Vec<usize>,
    deps_left: Arc<AtomicUsize>,
    body: Arc<Mutex<dyn Fn() + Send>>,
    #[allow(clippy::type_complexity)]
    on_completion: Arc<Mutex<Option<Box<dyn FnOnce() + Send>>>>,
}

impl Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Task {
    fn new(
        name: impl Into<String>,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
        body: impl Fn() + Send + 'static,
    ) -> Self {
        let deps_left = inputs.len();
        Self {
            name: name.into(),
            outputs,
            deps_left: Arc::new(AtomicUsize::new(deps_left)),
            body: Arc::new(Mutex::new(body)),
            on_completion: Arc::new(Mutex::new(None)),
        }
    }
    fn execute(&self) {
        (self.body.lock().unwrap())();
        if let Some(f) = self.on_completion.lock().unwrap().take() {
            f()
        }
    }
    fn on_completion(&self, f: impl Fn() + Send + 'static) {
        *self.on_completion.lock().unwrap() = Some(Box::new(f));
    }
}

struct ThreadPoolInner {
    threads: Vec<JoinHandle<()>>,
}

impl ThreadPoolInner {
    fn new(num_threads: usize) -> (Self, Sender<Task>) {
        let (sender, tasks) = std::sync::mpsc::channel::<Task>();
        let tasks = Arc::new(Mutex::new(tasks));
        let threads = (0..num_threads)
            .map(|tid| {
                let tasks = tasks.clone();
                thread::spawn(move || loop {
                    log::trace!("Thread {}: Awaiting task", tid);
                    let task = { tasks.lock().unwrap().recv() };
                    match task {
                        Ok(task) => {
                            log::trace!("Thread {}: Received task {}", tid, task.name);
                            task.execute()
                        }

                        Err(_) => {
                            log::trace!("Thread {}: No more tasks", tid);
                            break;
                        }
                    }
                })
            })
            .collect();
        let tpi = Self { threads };
        (tpi, sender)
    }
}

#[derive(Clone)]
struct ScopedThreadPool {
    sender: Option<Sender<Task>>,
    inner: Arc<Mutex<ThreadPoolInner>>,
}

impl ScopedThreadPool {
    fn new(num_threads: usize) -> Self {
        let (tpi, sender) = ThreadPoolInner::new(num_threads);
        Self {
            sender: Some(sender),
            inner: Arc::new(Mutex::new(tpi)),
        }
    }
    fn schedule(&self, task: Task) {
        if let Some(sender) = &self.sender {
            sender.send(task).unwrap();
        }
    }
}

impl Drop for ScopedThreadPool {
    fn drop(&mut self) {
        let threads: Vec<_> = {
            let mut inner = self.inner.lock().unwrap();
            self.sender = None;
            inner.threads.drain(..).collect()
        };
        for t in threads.into_iter() {
            t.join().unwrap();
        }
    }
}

fn topologically_sort(tasks: Vec<Task>) -> Vec<Task> {
    let mut stack: Vec<_> = (0..tasks.len()).collect();
    let mut permastack = Vec::new();
    let mut visited = vec![false; tasks.len()];
    let mut pushed = vec![false; tasks.len()];
    while let Some(i) = stack.pop() {
        if visited[i] && !pushed[i] {
            permastack.push(i);
            pushed[i] = true;
        } else if !visited[i] {
            visited[i] = true;
            let task = &tasks[i];
            stack.push(i);
            for &d in &task.outputs {
                if !visited[d] {
                    stack.push(d);
                }
            }
        }
    }

    let mut tasks: Vec<_> = tasks.into_iter().map(Some).collect();
    permastack
        .into_iter()
        .rev()
        .map(|i| tasks[i].take().unwrap())
        .collect()
}

#[test]
fn topologically_sort_test() {
    let mut tasks = Vec::new();
    tasks.push(Task::new("a", vec![1], vec![], || {}));
    tasks.push(Task::new("b", vec![2], vec![0], || {}));
    tasks.push(Task::new("c", vec![], vec![1], || {}));
    dbg!(&tasks);
    dbg!(topologically_sort(tasks));
    assert!(false)
}

/// Schedule
fn schedule(mut tasks: Vec<Task>, thread_pool: ScopedThreadPool) {
    for t in tasks.drain(..) {
        thread_pool.schedule(t);
    }
}

/// The deferred scheduling algorithm
fn schedule_deferred(tasks: Vec<Task>, thread_pool: ScopedThreadPool) {
    for t in tasks.clone() {
        let tasks = tasks.clone();
        let tp = thread_pool.sender.clone().unwrap();
        let t_clone = t.clone();
        t.on_completion(move || {
            for d_idx in &t_clone.outputs {
                let d = &tasks[*d_idx];
                if d.deps_left.fetch_sub(1, Ordering::SeqCst) == 1 {
                    log::info!("{} scheduled {}", t_clone.name, d.name);
                    tp.send(d.clone()).unwrap();
                }
            }
        });
    }

    for t in tasks {
        if t.deps_left.load(Ordering::SeqCst) == 0 {
            log::info!("Root {}", t.name);
            thread_pool.schedule(t.clone());
        }
    }
}

#[allow(dead_code)]
fn dual_chain_example() -> Vec<Task> {
    // Create tasks in topological order
    let mut tasks = Vec::new();

    let (zero, one) = mpsc::channel::<()>();
    tasks.push(Task::new("0", vec![], vec![1], move || {
        thread::sleep(Duration::from_millis(1000));
        zero.send(()).unwrap();
        println!("Task 0: Hello!");
    }));
    tasks.push(Task::new("1", vec![0], vec![], move || {
        one.recv().unwrap();
        thread::sleep(Duration::from_millis(1000));
        println!("Task 1: Hello!");
    }));

    let (two, three) = mpsc::channel::<()>();
    tasks.push(Task::new("2", vec![], vec![3], move || {
        thread::sleep(Duration::from_millis(2000));
        two.send(()).unwrap();
        println!("Task 2: Hello!");
    }));
    tasks.push(Task::new("3", vec![2], vec![], move || {
        three.recv().unwrap();
        thread::sleep(Duration::from_millis(1000));
        println!("Task 3: Hello!");
    }));

    tasks
}

fn split_merge_example() -> Vec<Task> {
    let mut tasks = Vec::new();

    tasks.push(Task::new("m1", vec![], vec![1, 3, 5], move || {
        thread::sleep(Duration::from_millis(500))
    }));

    tasks.push(Task::new("m2", vec![0], vec![2], move || {
        thread::sleep(Duration::from_millis(500))
    }));
    tasks.push(Task::new("m3", vec![1], vec![7], move || {
        thread::sleep(Duration::from_millis(500))
    }));

    tasks.push(Task::new("m4", vec![0], vec![4], move || {
        thread::sleep(Duration::from_millis(500))
    }));
    tasks.push(Task::new("m5", vec![3], vec![7], move || {
        thread::sleep(Duration::from_millis(500))
    }));

    tasks.push(Task::new("m6", vec![0], vec![6], move || {
        thread::sleep(Duration::from_millis(500))
    }));
    tasks.push(Task::new("m7", vec![5], vec![7], move || {
        thread::sleep(Duration::from_millis(500))
    }));

    tasks.push(Task::new("m8", vec![2, 4, 6], vec![], move || {
        thread::sleep(Duration::from_millis(500))
    }));

    tasks
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Get program arguments
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let num_threads: usize = args.next().unwrap().parse().unwrap();
    let mode = args.next().unwrap_or_else(|| "deferred".to_string());

    let tasks = split_merge_example();
    dbg!(&tasks);
    let tasks = topologically_sort(tasks);
    dbg!(&tasks);

    // Create thread pool
    let thread_pool = ScopedThreadPool::new(num_threads);
    match mode.as_str() {
        "deferred" => {
            schedule_deferred(tasks, thread_pool);
        }
        "pre" => {
            schedule(tasks, thread_pool);
        }
        _ => {}
    }

    Ok(())
}
