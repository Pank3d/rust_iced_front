pub mod themes;
use iced::{Alignment, Element, Sandbox, Settings,  widget::{button, Button, Column, Space, text, self, scrollable, column, row, horizontal_space, TextInput}, BorderRadius, application::Application, Command, event::Status};
use iced::widget::Row;
pub struct Appearance {
    pub border_radius: BorderRadius
}

struct MessageContent{
    sender: String,
    content: String,
    timestape:String,
}

struct Model{
    contacts:Vec<Contact>,
    active_chat:Option<Contact>
}


struct ChatWindow{
    messages:Vec<MessageContent>,
    input:String,
}









struct Chat{
    contact: Contact,
    messages:Vec<MessageContent>,
    active_chat:Option<Contact>,


}




pub struct Contact {
    pub name: String,
    pub last_message: String,
    pub last_message_time: String,
}

impl Contact{
    fn new(name: String, last_message: String, last_message_time:String) -> Self{
        Contact { name, last_message, last_message_time }
    }
}






enum ContactStatus{
    Online,
    Offline,
    recently,
}


#[derive(Debug, Clone)]
enum Message {
    ImportButtonClicked,
    NewDatabaseButtonClicked,
    ContactOpen(usize),
    NewMessage(String),
    SendMessage,
}

pub struct Chat_posle_knopki{
    pub input:String,
    pub messages:Vec<String>,
    pub send_button:button::State,
    
}





enum Page{
    MainPage,
    AfterImportPage,
}

struct MyApp{
    current_page: Page,
    contacts: Vec<Contact>,
    active_chat:Option<ChatWindow>,

}

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = themes::Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {
            current_page: Page::MainPage,
            contacts: vec![
                Contact {
                    name: "vlad".to_string(),
                    last_message: "Hi".to_string(),
                    last_message_time: "18:42".to_string(),
                },Contact{ name: "Maksim".to_string(),
                     last_message: "when you go home".to_string(),
                      last_message_time: "17:15".to_string() },
                      Contact{ name: "Alesha".to_string(), last_message: "Zavtra k kakoy?".to_string(), last_message_time: "17:47".to_string()},Contact{ name: "Evkalavrat".to_string(), last_message: "Vladik umer".to_string(), last_message_time: "18:15".to_string() },
                      Contact{ name: "Roman".to_string(), last_message: "Ya Doma".to_string(), last_message_time: "10:15".to_string() }
            ],
            active_chat: None,


            
            
        }, Command::none())
    }



    


    fn title(&self) -> String {
        String::from("Bivo Chat")
    }

    

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message
         {Message::ImportButtonClicked => {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                println!("User selected file: {:?}", path);
                self.current_page = Page::AfterImportPage;
            } else {
                println!("User did not select a file.");
            }
        },
        Message::NewDatabaseButtonClicked => {
            // Пустое тело: кнопка "New database" ничего не делает при нажатии.
        },
        Message::ContactOpen(i)=>{
            if let Some(contact) = self.contacts.get(i){
                let chat_window = ChatWindow{
                    messages:vec![],
                    input:String::new(),
                };
                self.active_chat = Some(chat_window);
            }

        },
            Message::NewMessage(_) => {},
            Message::SendMessage => {},
}

    Command::none()

}

    
    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        
        match &self.current_page {
            Page::MainPage => {
            let button1 = Button::new(iced::widget::text("Import your file!").size(25))
                .on_press(Message::ImportButtonClicked);
            
        
            let button2 = Button::new(iced::widget::text("New database").size(25))
                .on_press(Message::NewDatabaseButtonClicked);
        
            let alignment = Alignment::Center; // Выберите желаемое выравнивание
            let button_width = 200.0; // Устанавливаем ширину кнопки
            let button_height = 60.0;
            let gap_height = 50.0;
        
            let centered_button = Column::new()
                .push(Space::with_height(iced::Length::Fill))
                .push(
                    Column::new()
                        .align_items(alignment)
                        .push(Space::with_width(iced::Length::Fill))
                        .push(button1.width(button_width).height(button_height))
                        .push(Space::with_height(gap_height))
                        .push(button2.width(button_width).height(button_height))
                        .push(Space::with_width(iced::Length::Fill))
                )
                .push(
                    Space::with_height(iced::Length::Fill)
                );
        
            centered_button.into()
        },
        Page::AfterImportPage => {
            let mut contacts = Vec::<Element<'_, Self::Message, iced::Renderer::<Self::Theme>>>::new();

            if let Some(Chat) = self.active_chat{
                let chat_view = Column::new()
                .push(
                    // Отображение истории сообщений
                    Column::with_children(
                        chat.messages.iter().map(|message| {
                            // Создайте виджет для отдельного сообщения (например, с именем отправителя, текстом сообщения и временем)
                            let message_widget = Column::new()
                                .push(text(message.sender))
                                .push(text(message.content))
                                .push(text(message.timestamp));
            
                            message_widget = message_widget.spacing(iced::Length::Units(10));
            
                            message_widget.into()
                        }),
                    ),
                )
                .push(
                    TextInput::new(
                        &mut self.active_chat.input,
                        "Введите ваше сообщение"

                    )
                );
            
            

            for (i, contact) in self.contacts.iter().enumerate() {
                contacts
                    .push(
                        Button::new(column![
                            text(contact.name.clone()),
                            horizontal_space(2),
                            row![
                                text(contact.last_message.clone()),
                                horizontal_space(2),
                                text(contact.last_message_time.clone()),
                            ],
                        ])
                        .style(themes::ButtonStyle::Contact).height(200).width(200)
                        .on_press(Self::Message::ContactOpen(i))

                        .into()
                    );
            }

            scrollable::Scrollable::new(Column::with_children(contacts)).into()
        }
    }

        


            
    }   
}



fn main() -> iced::Result {
    MyApp::run(Settings::default())
}