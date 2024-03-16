use iced::widget::{button, column, text, Button, Column, Text, TextInput};
use iced::{executor, Alignment, Application, Command, Element, Settings};
use reqwest;
use std::io::Read;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

fn main() {
    // Uruchom aplikację Iced z użyciem tokio runtime
    let runtime = Runtime::new().unwrap();
    App::run(Settings::default());
}

struct App {
    content: Arc<Mutex<String>>,
    url_input: String,
    fetch_button_state: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ContentLoaded(String),
    UrlInputChanged(String),
    FetchButtonPressed,
}

impl Application for App {
    type Executor = executor::Default;
    type Theme = iced::Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (App, Command<Message>) {
        let content = Arc::new(Mutex::new(String::new()));
        let content_clone = Arc::clone(&content);

        (
            App {
                content: Arc::new(Mutex::new(String::new())),
                url_input: String::new(),
                fetch_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("App - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ContentLoaded(content) => {
                let mut locked_content = self.content.lock();

                //*locked_content = content;
                Command::none()
            }
            Message::UrlInputChanged(url) => {
                self.url_input = url;
                Command::none()
            }
            Message::FetchButtonPressed => {
                let url = self.url_input.clone();
                let future = fetch_content(url);
                Command::perform(future, Message::ContentLoaded)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let url_input = TextInput::new(&mut self.url_input, "Wpisz URL...").padding(10);

        let fetch_button = Button::new(&mut self.fetch_button_state, Text::new("Pobierz"))
            .on_press(Message::FetchButtonPressed)
            .padding(10);

        let content = Text::new(&*self.content.lock());

        let column = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(url_input)
            .push(fetch_button)
            .push(content)
            .padding(20);

        column.into()
    }
}

async fn fetch_content(url: String) -> String {
    let response = reqwest::get(&url).await.unwrap();
    response.text().await.unwrap()
}
