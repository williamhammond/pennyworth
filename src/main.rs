use iced::{executor, text_input, Application, Clipboard, Command, Element, Settings, TextInput};

pub fn main() -> iced::Result {
    Pennyworth::run(Settings::default())
}

#[derive(Debug)]
struct Pennyworth {
    state: State,
}

#[derive(Debug, Default)]
struct State {
    input: text_input::State,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Log,
}

impl Application for Pennyworth {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Pennyworth, Command<Self::Message>) {
        (
            Pennyworth {
                state: State {
                    input: Default::default(),
                    input_value: "".to_string(),
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => self.state.input_value = value,
            Message::Log => {
                println!("hi")
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        TextInput::new(
            &mut self.state.input,
            "What needs to be done?",
            &*self.state.input_value,
            Message::InputChanged,
        )
        .padding(15)
        .size(30)
        .on_submit(Message::Log)
        .into()
    }
}
