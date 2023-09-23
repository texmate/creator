use iced::widget::{Button, Column, Container, Text, TextInput};
use iced::{Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Creator::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
    SaveClicked,
    AddComponent,
}

#[derive(Default)]
struct Creator {
    input: String,
    //components: Vec<>,
}


impl Sandbox for Creator {
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
            Message::SaveClicked => println!("Clicked"),
            Message::AddComponent => add_component(self),
        }
    }

    fn view(&self) -> Element<Message> {
        // View title
        let title: Text = Text::new("Enter Configuration File Values")
            .width(iced::Length::Fill)
            .size(25)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        // PCB Name column.
        let pcb_name_help: Text = Text::new("Name of the PCB")
            .horizontal_alignment(iced::alignment::Horizontal::Left)
            .vertical_alignment(iced::alignment::Vertical::Top);

        let pcb_name_input: TextInput<Message> =
            TextInput::new("Enter the name of the printed circuit board", &self.input)
                .on_input(Message::TextInputChanged);

        // Save button
        let save_button: Button<Message> = Button::new("Save")
            .on_press(Message::SaveClicked)
            .padding([10, 40, 10, 40]);
        let button_container = Container::new(save_button)
            .height(iced::Length::Fill)
            .align_y(iced::alignment::Vertical::Bottom)
            .center_x()
            .width(iced::Length::Fill);

        // Add new component column
        let add_button: Button<Message> = Button::new("Add New Component")
            .on_press(Message::SaveClicked)
            .padding([5, 10, 5, 10]);

        let add_container = Container::new(add_button)
            .center_x()
            .width(iced::Length::Fill)
            .padding(10);

        // Bring the info text and the placeholder text into a column
        let pcb_name_column: Column<Message> = Column::new()
            .push(pcb_name_help)
            .push(pcb_name_input)
            .padding(5);

        let window_box = Container::new(
            Column::new()
                .height(iced::Length::Fill)
                .width(iced::Length::Fill)
                .push(title)
                .push(pcb_name_column)
                .push(add_container)
                .push(button_container),
        )
        .padding(5)
        .into();

        window_box
    }
}

fn add_component(app: &Creator){
    // Component filename column.
    let component_filename_help: Text = Text::new("Filename for the master component image")
    .horizontal_alignment(iced::alignment::Horizontal::Left)
    .vertical_alignment(iced::alignment::Vertical::Top);

    let component_filename_input: TextInput<Message> =
    TextInput::new("Enter filename for master component image", &app.input)
        .on_input(Message::TextInputChanged);

    // Component threshold row.
    let threshold_help: Text =
    Text::new("Matching threshold for master component image vs. test board component")
        .width(iced::Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Left)
        .vertical_alignment(iced::alignment::Vertical::Center);

    let threshold_input: TextInput<Message> =
    TextInput::new("Set component comparison threshold number", &app.input)
        .on_input(Message::TextInputChanged);


    let component_column: Column<Message> = Column::new()
        .push(component_filename_help)
        .push(component_filename_input)
        .padding(5);

    let threshold_column: Column<Message> = Column::new()
        .push(threshold_help)
        .push(threshold_input)
        .padding(5);

    app.window_box
}