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
    ComponentInputChanged { value: String, id: usize },
}

struct Creator {
    input: String,
    //components: Vec<component::Component>,
    component_num: i32, // Keeps how many components are on this board. 
    component_inputs: Vec<String>, // All the values of the inputs. 
}

impl Sandbox for Creator {
    type Message = Message;

    fn new() -> Self {
        Self {
            component_num: 0,
            input: String::new(),
            component_inputs: Vec::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Configuration Creator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TextInputChanged(x) => self.input = x,
            Message::SaveClicked => println!("Clicked"),
            Message::AddComponent => add_component(self),
            Message::ComponentInputChanged { value, id } => {
                self.component_inputs[id] = value;
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

        let pcb_name_input: TextInput<Message> =
            TextInput::new("Enter the name of the printed circuit board", &self.input)
                .on_input(Message::TextInputChanged);

        let pcb_name_column: Column<Message> = Column::new()
            .push(pcb_name_help)
            .push(pcb_name_input)
            .padding(5);
        content = content.push(pcb_name_column);

        // The dynamic list of components.

        let mut components: Column<Message> = Column::new();
        for current_num in 0..self.component_num {
            let id = current_num as usize;

            println!("{}", id);
            // Create a text input. Use a closure to pass the id of the the text input when stuff is typed in.
            let res: TextInput<'_, Message> = TextInput::new(
                "Enter the name of the printed circuit board",
                &self.component_inputs[id],
            ) // Now that its created, deal with the text updates. 
            .on_input(move |new_value| Message::ComponentInputChanged {
                value: new_value.clone(),
                id: id,
            });
            components = components.push(res);
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
        let save_button: Button<Message> = Button::new("Save")
            .on_press(Message::SaveClicked)
            .padding([10, 40, 10, 40]);
        let button_container = Container::new(save_button)
            .height(iced::Length::Fill)
            .align_y(iced::alignment::Vertical::Bottom)
            .center_x()
            .width(iced::Length::Fill);
        content = content.push(button_container);

        content.into()
    }
}

fn add_component(app: &mut Creator) {
    app.component_num += 1;
    app.component_inputs.push(String::from(""));
}
