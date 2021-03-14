use hotdrink_rs::{component, data::traits::ComponentLike, ret, Component};
use iced::{text_input::State, Align, Column, Element, Length, Sandbox, Settings, TextInput};

pub fn main() -> iced::Result {
    GuiState::run(Settings::default())
}

struct GuiState {
    field_a: State,
    field_b: State,
    field_c: State,
    field_d: State,
    component: Component<i32>,
}

#[derive(Debug, Clone)]
enum Message {
    SetVariable(String, Option<i32>),
}

impl Sandbox for GuiState {
    type Message = Message;

    fn new() -> Self {
        // Create constraint system
        let component = component! {
            component Sum {
                let a: i32 = 0, b: i32 = 0, c: i32 = 0, d: i32 = 0;
                constraint Sum {
                    s1(a: &i32, b: &i32) -> [c] = ret![a + b];
                    s2(a: &i32, c: &i32) -> [b] = ret![c - a];
                    s3(b: &i32, c: &i32) -> [a] = ret![c - b];
                }
                constraint Product {
                    p1(a: &i32, b: &i32) -> [d] = ret![a * b];
                    p2(a: &i32, d: &i32) -> [b] = ret![d / a];
                    p3(b: &i32, d: &i32) -> [a] = ret![d / b];
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
            .width(Length::Units(200))
            .align_items(Align::Center)
            .push(bind(&mut self.field_a, "a", &self.component))
            .push(bind(&mut self.field_b, "b", &self.component))
            .push(bind(&mut self.field_c, "c", &self.component))
            .push(bind(&mut self.field_d, "d", &self.component))
            .into()
    }
}

fn bind<'a>(state: &'a mut State, name: &str, cs: &Component<i32>) -> TextInput<'a, Message> {
    let value = cs.get_variable(&name);
    let name_clone = name.to_string();
    TextInput::new(state, name, &value.unwrap().to_string(), move |v| {
        Message::SetVariable(name_clone.clone(), v.parse().ok())
    })
}
