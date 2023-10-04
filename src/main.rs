use iced::widget::{container, scrollable, Button, Column, Container, Rule, Text, TextInput};
use iced::{window, Element, Sandbox, Settings};
use native_dialog::FileDialog;

mod json_writer;

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            min_size: Some((700, 400)),
            ..Default::default()
        },
        ..Default::default()
    };
    Creator::run(settings)
}

#[derive(Debug, Clone)]
enum Message {
    PCBFilenameInputChanged(String),
    PCBNameInputChanged(String),
    SaveClicked,
    AddComponent,

    CountInputChanged { value: String, id: usize },
    ThresholdInputChanged { value: String, id: usize },
    FilenameInputChanged { value: String, id: usize },
}

struct Creator {
    pcb_filename: String,
    pcb_name: String,
    save_button_text: String,

    component_num: i32,            // Keeps how many components are on this board.
    threshold_inputs: Vec<String>, // All the values of the inputs.
    count_inputs: Vec<String>,     // All the values of the inputs.
    filename_inputs: Vec<String>,  // All the values of the inputs.
}

impl Sandbox for Creator {
    type Message = Message;

    fn new() -> Self {
        Self {
            component_num: 0,
            pcb_name: String::new(),
            pcb_filename: String::new(),
            save_button_text: String::from("Save"),
            threshold_inputs: Vec::new(),
            count_inputs: Vec::new(),
            filename_inputs: Vec::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Configuration Creator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PCBFilenameInputChanged(x) => self.pcb_filename = x,
            Message::PCBNameInputChanged(x) => self.pcb_name = x,
            Message::SaveClicked => {
                let path = FileDialog::new()
                    .set_location("~/Documents")
                    .add_filter("JSON File", &["json"])
                    .show_save_single_file()
                    .unwrap_or(Option::None);

                let path = match path {
                    Some(path) => path,
                    None => return,
                };

                let path = path.to_string_lossy();

                json_writer::write_to_json(
                    self.threshold_inputs.clone(),
                    self.count_inputs.clone(),
                    self.filename_inputs.clone(),
                    self.pcb_name.clone(),
                    String::from(path),
                )
                .expect("Coudn't save JSON file.");

                self.save_button_text = "Saved".to_string();
            }
            Message::AddComponent => add_component(self),
            Message::CountInputChanged { value, id } => {
                self.count_inputs[id] = value;
            }
            Message::ThresholdInputChanged { value, id } => {
                self.threshold_inputs[id] = value;
            }
            Message::FilenameInputChanged { value, id } => {
                self.filename_inputs[id] = value;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut content: Column<Message> = Column::new()
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .padding(5);

        // View title
        let title: Text = Text::new("Enter Configuration File Values")
            .width(iced::Length::Fill)
            .size(25)
            .horizontal_alignment(iced::alignment::Horizontal::Center);
        content = content.push(title);

        // PCB Name column.
        let pcb_name_help: Text = Text::new("Name of the PCB")
            .horizontal_alignment(iced::alignment::Horizontal::Left)
            .vertical_alignment(iced::alignment::Vertical::Top);

        let pcb_name_input: TextInput<Message> = TextInput::new(
            "Enter the name of the printed circuit board",
            &self.pcb_name,
        )
        .on_input(Message::PCBNameInputChanged);

        let pcb_name_column: Column<Message> = Column::new()
            .push(pcb_name_help)
            .push(pcb_name_input)
            .padding(5);
        content = content.push(pcb_name_column);

        // PCB Filename column.
        let pcb_name_help: Text = Text::new("Filename of the PCB")
            .horizontal_alignment(iced::alignment::Horizontal::Left)
            .vertical_alignment(iced::alignment::Vertical::Top);

        let pcb_name_input: TextInput<Message> = TextInput::new(
            "Enter filename to golden sample of the whole PCB",
            &self.pcb_filename,
        )
        .on_input(Message::PCBFilenameInputChanged);

        let pcb_name_column: Column<Message> = Column::new()
            .push(pcb_name_help)
            .push(pcb_name_input)
            .padding(5);
        content = content.push(pcb_name_column);

        // The dynamic list of components.
        let mut components: Column<Message> = Column::new();
        for current_num in 0..self.component_num {
            // Loop through the IDs
            let mut this_component: Column<Message> = Column::new();
            // The IDs are the same as 0 thru component_num. Random IDs would be much harder to implement.
            let id = current_num as usize;

            // Divider. The padding is for extra space on top.
            this_component =
                this_component.push(Container::new(Rule::horizontal(5)).padding([15, 0, 15, 0]));

            let title: Text = Text::new(format!("Component {}", current_num))
                .width(iced::Length::Fill)
                .size(20)
                .horizontal_alignment(iced::alignment::Horizontal::Center);
            this_component = this_component.push(title);

            // Filename
            let component_image_help: Text = Text::new("Filename")
                .horizontal_alignment(iced::alignment::Horizontal::Left)
                .vertical_alignment(iced::alignment::Vertical::Top);
            this_component = this_component.push(component_image_help);

            // Create a text input for the filename. Use a closure to pass the id of the the text input when stuff is typed in.
            let component_image_path: TextInput<'_, Message> = TextInput::new(
                "Enter the path to the sample image of the component",
                &self.filename_inputs[id],
            ) // Now that its created, deal with the text updates.
            .on_input(move |new_value| Message::FilenameInputChanged {
                value: new_value.clone(),
                id: id,
            });
            this_component = this_component.push(component_image_path);

            // Threshold
            let component_threshold_help: Text = Text::new("Sensitivity Threshold")
                .horizontal_alignment(iced::alignment::Horizontal::Left)
                .vertical_alignment(iced::alignment::Vertical::Top);
            this_component = this_component.push(component_threshold_help);

            // Create a text input. Use a closure to pass the id of the the text input when stuff is typed in.
            let component_threshold: TextInput<'_, Message> = TextInput::new(
                "Enter how similar the board components need to be with the source component images",
                &self.threshold_inputs[id],
            ) // Now that its created, deal with the text updates.
            .on_input(move |new_value| Message::ThresholdInputChanged {
                value: new_value.clone(),
                id: id,
            });
            this_component = this_component.push(component_threshold);

            // Count
            let component_count_help: Text = Text::new("Component Count")
                .horizontal_alignment(iced::alignment::Horizontal::Left)
                .vertical_alignment(iced::alignment::Vertical::Top);
            this_component = this_component.push(component_count_help);

            // Create a text input. Use a closure to pass the id of the the text input when stuff is typed in.
            let component_count: TextInput<'_, Message> = TextInput::new(
                "Enter the number of components of this type should be on the board",
                &self.count_inputs[id],
            ) // Now that its created, deal with the text updates.
            .on_input(move |new_value| Message::CountInputChanged {
                value: new_value.clone(),
                id: id,
            });
            this_component = this_component.push(component_count);

            // Add some padding to indicate a new type of column.
            this_component = this_component.padding([0, 10, 0, 10]);

            // Everything has been stacked into a column. Put in the main column for components.
            components = components.push(this_component);
        }
        content = content.push(components);

        // The add new component button column
        let add_button: Button<Message> = Button::new("Add New Component")
            .on_press(Message::AddComponent)
            .padding([5, 10, 5, 10]);

        let add_container = Container::new(add_button)
            .center_x()
            .width(iced::Length::Fill)
            .padding(10);
        content = content.push(add_container);

        // Save button
        let save_button: Button<Message> = Button::new(self.save_button_text.as_str())
            .on_press(Message::SaveClicked)
            .padding([10, 40, 10, 40]);
        let button_container = Container::new(save_button)
            .height(iced::Length::Fill)
            .center_x()
            .width(iced::Length::Fill);
        content = content.push(button_container);

        // Max width.
        content = content.max_width(800);

        let scrollable = scrollable(container(content).width(iced::Length::Fill).center_x());

        container(scrollable)
            .height(iced::Length::Fill)
            .center_y()
            .into()
    }
}

fn add_component(app: &mut Creator) {
    app.component_num += 1;

    app.threshold_inputs.push(String::from(""));
    app.count_inputs.push(String::from(""));
    app.filename_inputs.push(String::from(""));
}
