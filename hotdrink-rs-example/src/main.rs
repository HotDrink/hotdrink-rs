use hotdrink_rs::{
    component,
    model::{Component, DoneState},
    ret,
};
use iced::{text_input::State, Align, Column, Element, Length, Sandbox, Settings, TextInput};

pub fn main() -> iced::Result {
    GuiState::run(Settings::default())
}

struct GuiState {
    field_a: State,
    field_b: State,
    field_c: State,
    field_d: State,
    component: Component<i64>,
}

#[derive(Debug, Clone)]
enum Message {
    SetVariable(String, Option<i64>),
}

impl Sandbox for GuiState {
    type Message = Message;

    fn new() -> Self {
        // Create constraint system
        let component = component! {
            component Sum {
                let a: i64 = 0, b: i64 = 0, c: i64 = 0, d: i64 = 0;
                constraint Sum {
                    s1(a: &i64, b: &i64) -> [c] = ret![a.saturating_add(*b)];
                    s2(a: &i64, c: &i64) -> [b] = ret![c - a];
                    s3(b: &i64, c: &i64) -> [a] = ret![c - b];
                }
                constraint Product {
                    p1(a: &i64, b: &i64) -> [d] = ret![a.saturating_mul(*b)];
                    p2(a: &i64, d: &i64) -> [b] = ret![d / a];
                    p3(b: &i64, d: &i64) -> [a] = ret![d / b];
                }
            }
        };

        Self {
            field_a: State::new(),
            field_b: State::new(),
            field_c: State::new(),
            field_d: State::new(),
            component,
        }
    }

    fn title(&self) -> String {
        String::from("Sum/Product - Iced")
    }

    fn update(&mut self, message: Message) {
        println!("{:?}", message);
        match message {
            Message::SetVariable(name, value) => {
                let value = match value {
                    Some(v) => v,
                    None => {
                        self.field_a.move_cursor_to_end();
                        self.field_b.move_cursor_to_end();
                        self.field_c.move_cursor_to_end();
                        self.field_d.move_cursor_to_end();
                        0
                    }
                };
                self.component.set_variable(&name, value).unwrap();
                self.component.update().unwrap();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .width(Length::Units(300))
            .align_items(Align::Center)
            .push(bind(&mut self.field_a, "a", &self.component))
            .push(bind(&mut self.field_b, "b", &self.component))
            .push(bind(&mut self.field_c, "c", &self.component))
            .push(bind(&mut self.field_d, "d", &self.component))
            .into()
    }
}

fn bind<'a>(state: &'a mut State, name: &str, cs: &Component<i64>) -> TextInput<'a, Message> {
    let value = futures::executor::block_on(cs.variable(&name).unwrap());
    let value = match value {
        DoneState::Ready(value) => *value,
        DoneState::Error(errors) => panic!("{:?}", errors),
    };
    let name_clone = name.to_string();
    TextInput::new(state, name, &value.to_string(), move |v| {
        Message::SetVariable(name_clone.clone(), v.parse().ok())
    })
}
