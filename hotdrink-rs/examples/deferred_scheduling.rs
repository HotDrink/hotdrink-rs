use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::Sender,
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Clone)]
struct Task {
    name: String,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    deps_left: Arc<AtomicUsize>,
    body: Arc<dyn Fn() + Send + Sync>,
    #[allow(clippy::clippy::type_complexity)]
    on_completion: Arc<Mutex<Option<Box<dyn FnOnce() + Send>>>>,
}

impl Task {
    fn new(
        name: impl Into<String>,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
        body: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        let deps_left = inputs.len();
        Self {
            name: name.into(),
            inputs,
            outputs,
            deps_left: Arc::new(AtomicUsize::new(deps_left)),
            body: Arc::new(body),
            on_completion: Arc::new(Mutex::new(None)),
        }
    }
    fn execute(&self) {
        (self.body)();
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
                thread::spawn(move || {
                    loop {
                        let task = { tasks.lock().unwrap().recv() };
                        match task {
                            Ok(task) => {
                                println!("Thread {}: Starting task {}", tid, task.name);
                                task.execute();
                                println!("Thread {}: Completed task {}", tid, task.name);
                            }
                            Err(_) => break,
                        }
                    }
                    // NOTE: Locks for entire loop body :(
                    // while let Ok(task) = tasks.lock().unwrap().recv() {
                    //     println!("Thread {}: Executing task {}", tid, task.name);
                    //     task.execute();
                    // }
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
        let name = task.name.clone();
        println!("Scheduling task {}", name);
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

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let num_threads: usize = args.next().unwrap().parse().unwrap();

    let mut tasks = Vec::new();
    tasks.push(Task::new("0", vec![], vec![1], || {
        thread::sleep(Duration::from_millis(1000));
    }));
    tasks.push(Task::new("1", vec![0], vec![], || {
        thread::sleep(Duration::from_millis(1000));
    }));
    tasks.push(Task::new("2", vec![], vec![3], || {
        thread::sleep(Duration::from_millis(2000));
    }));
    tasks.push(Task::new("3", vec![2], vec![], || {
        thread::sleep(Duration::from_millis(1000));
    }));

    let tp = ScopedThreadPool::new(num_threads);

    // Deferred scheduling algorithm
    for t in tasks.clone() {
        let tasks = tasks.clone();
        let tp = tp.sender.clone().unwrap();
        let t_clone = t.clone();
        t.on_completion(move || {
            for d_idx in &t_clone.outputs {
                let d = &tasks[*d_idx];
                if d.deps_left.fetch_sub(1, Ordering::SeqCst) == 1 {
                    println!("Task {}: Scheduling task {}", t_clone.name, d.name);
                    tp.send(d.clone()).unwrap();
                }
            }
        });
    }

    for t in tasks {
        if t.deps_left.load(Ordering::SeqCst) == 0 {
            tp.schedule(t.clone());
        }
    }
}
