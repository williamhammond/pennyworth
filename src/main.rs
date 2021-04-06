mod errors;
mod module;

use crate::module::Module;
use iced::{
    executor, text_input, window, Application, Clipboard, Command, Element, Settings, TextInput,
};

pub fn main() -> iced::Result {
    Pennyworth::run(Settings {
        window: window::Settings {
            size: (800, 60),
            min_size: None,
            max_size: None,
            resizable: false,
            decorations: false,
            transparent: true,
            always_on_top: false,
            icon: None,
        },
        flags: (),
        default_font: None,
        default_text_size: 0,
        exit_on_close_request: true,
        antialiasing: false,
    })
}

#[derive(Debug)]
struct Pennyworth {
    state: State,
    modules: Vec<Box<dyn Module>>,
}

#[derive(Debug)]
enum Mode {
    DetermineCommand,
    Input,
}

#[derive(Debug)]
struct State {
    input: text_input::State,
    input_value: String,
    mode: Mode,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Submit,
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
                    mode: Mode::DetermineCommand,
                },
                modules: vec![Box::new(module::TimestampModule {})],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => match self.state.mode {
                Mode::DetermineCommand => {
                    self.state.input_value = value;
                    for module in self.modules.iter() {
                        if self.state.input_value == module.name() {
                            println!("Command {:?} matched", self.state.input_value);
                            self.state.mode = Mode::Input;
                        }
                    }
                }
                Mode::Input => {
                    self.state.input_value = value;
                    let mut split = self.state.input_value.split(' ');

                    let command: String = String::from(split.next().unwrap());
                    let command_match = self.modules.iter().any(|module| module.name() == command);
                    if !command_match {
                        println!("Command unmatched");
                        self.state.mode = Mode::DetermineCommand;
                    } else {
                        let input: String = split.skip(1).collect();
                    }
                }
            },
            Message::Submit => {
                std::process::exit(0);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        TextInput::new(
            &mut self.state.input,
            "Enter command",
            &*self.state.input_value,
            Message::InputChanged,
        )
        .padding(15)
        .size(30)
        .on_submit(Message::Submit)
        .into()
    }
}
