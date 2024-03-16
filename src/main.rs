use iced::widget::{
    pick_list, text_editor, Button, Column, PickList, Row, Text, TextEditor, TextInput,
};
use iced::{executor, Alignment, Application, Command, Element, Settings};
use reqwest;
use serde_json::json;
use std::fmt;
use tokio::runtime::Runtime;

fn main() {
    // Uruchom aplikację Iced z użyciem tokio runtime
    let runtime = Runtime::new().unwrap();
    App::run(Settings::default());
}

struct Response {
    status_code: i8,
    jsonString: String,
}

struct Request {}

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
pub enum RequestType {
    #[default]
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl RequestType {
    const ALL: [RequestType; 5] = [
        RequestType::GET,
        RequestType::PATCH,
        RequestType::POST,
        RequestType::DELETE,
        RequestType::PUT,
    ];
}

impl std::fmt::Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
struct App {
    url_input: String,
    response_body: text_editor::Content,
    request_type: Option<RequestType>,
}

#[derive(Debug, Clone)]
enum Message {
    ContentLoaded(String),
    UrlInputChanged(String),
    FetchButtonPressed,
    RequestTypeSelected(RequestType),
}

impl Application for App {
    type Executor = executor::Default;
    type Theme = iced::Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (App, Command<Message>) {
        (
            App {
                url_input: String::from("https://jsonplaceholder.typicode.com/todos/1"),
                response_body: text_editor::Content::new(),
                request_type: Some(RequestType::GET),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("App - HttpClient")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ContentLoaded(content) => {
                self.response_body = text_editor::Content::with_text(content.as_str());
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
            Message::RequestTypeSelected(request_type) => {
                self.request_type = Some(request_type);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let url_input = TextInput::new("Wpisz URL...", &self.url_input)
            .padding(10)
            .on_input(Message::UrlInputChanged);

        let fetch_button = Button::new(Text::new("Pobierz")).on_press(Message::FetchButtonPressed);

        let text_editor = TextEditor::new(&self.response_body).padding(40);

        let pick_list1: PickList<RequestType, _, _, _> = pick_list(
            &RequestType::ALL[..],
            self.request_type,
            Message::RequestTypeSelected,
        )
        .placeholder("Choose a req type...");
        let top_row = Row::new()
            .push(url_input)
            .push(fetch_button)
            .spacing(20)
            .align_items(Alignment::Center)
            .padding(20);

        let editor_row = Row::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(text_editor)
            .push(pick_list1)
            .padding(20);

        Column::new().push(top_row).push(editor_row).into()
    }
}

async fn fetch_content(url: String) -> String {
    let response = reqwest::get(&url).await.unwrap();
    let json = response.text().await.unwrap();
    json
}
