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
                    let (command_name, _) = get_command_and_input(&*self.state.input_value);

                    let command_match = self
                        .modules
                        .iter()
                        .any(|module| module.name() == command_name);
                    if !command_match {
                        println!("Command unmatched");
                        self.state.mode = Mode::DetermineCommand;
                    }
                }
            },
            Message::Submit => match self.state.mode {
                Mode::DetermineCommand => {
                    std::process::exit(0);
                }
                Mode::Input => {
                    let (command_name, input) = get_command_and_input(&*self.state.input_value);

                    let command = self
                        .modules
                        .iter()
                        .find(|module| module.name() == command_name)
                        .unwrap();

                    let result = command.execute(input);
                    match result {
                        Ok(result) => {
                            self.state.input_value = result;
                        }
                        Err(err) => {
                            println!("Error running command {:?}", command.name());
                            println!("{:?}", err);
                        }
                    }
                }
            },
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

fn get_command_and_input(input_value: &str) -> (String, String) {
    let mut split = input_value.split(' ');

    let command_name = split.next().unwrap().to_string();
    let input: String = split.collect();

    (command_name, input)
}
