use iced::widget::{text, text_input, Column, Container, Row, Text, TextInput};
use iced::{Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Progress::run(Settings::default())
}

#[derive(Default)]
struct Progress {
    input: String,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
}

impl Sandbox for Progress {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Configuration Creator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TextInputChanged(x) => self.input = x,
        }
    }

    fn view(&self) -> Element<Message> {
        let title: Text = Text::new("Enter Configuration File Values")
            .width(iced::Length::Fill)
            .size(25)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        // Component filename row.
        let component_filename_help: Text = Text::new("Path to component image")
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Right)
            .vertical_alignment(iced::alignment::Vertical::Top);

        let component_filename_input: TextInput<Message> =
            TextInput::new("Set filename to component image", &self.input)
                .on_input(Message::TextInputChanged);

        // Component filename row.
        let threshold_help: Text = Text::new("Path to component image")
            .width(iced::Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Right)
            .vertical_alignment(iced::alignment::Vertical::Center);

        let threshold_input: TextInput<Message> =
            TextInput::new("Set filename to component image", &self.input)
                .on_input(Message::TextInputChanged);

        let component_row: Row<Message> = Row::new()
            .push(component_filename_help)
            .push(component_filename_input);

        let threshold_row: Row<Message> = Row::new().push(threshold_help).push(threshold_input);

        let window_box = Container::new(
            Column::new()
                .width(iced::Length::Fill)
                .push(title)
                .push(component_row)
                .push(threshold_row),
        )
        .padding(5)
        .into();

        window_box
    }
}
