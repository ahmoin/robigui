pub mod style;
pub mod utils;

use iced::{
    Length, Task, Theme,
    alignment::{Horizontal, Vertical},
    widget::{Column, button, column, horizontal_rule, pick_list, row, text, text_input},
};

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(Theme),
    UsernameChanged(String),
    SearchClicked,
    ApiResponse(Result<String, String>),
}

#[derive(Debug)]
pub struct Robigui {
    username: String,
    selected_theme: Theme,
}

impl Default for Robigui {
    fn default() -> Self {
        Self {
            username: Default::default(),
            selected_theme: Theme::CatppuccinMacchiato,
        }
    }
}

impl Robigui {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn selected_theme(&self) -> Theme {
        self.selected_theme.clone()
    }
}

fn search_user(username: String) -> Message {
    let url = format!(
        "https://apis.roblox.com/search-api/omni-search?verticalType=user&searchQuery={}&sessionId=36b8f84d-df4e-4d49-b662-bcde71a8764f",
        username
    );

    match ureq::get(&url).call() {
        Ok(response) => match response.into_string() {
            Ok(text) => {
                println!("API Response: {}", text);
                Message::ApiResponse(Ok(text))
            }
            Err(e) => {
                println!("Error reading response: {}", e);
                Message::ApiResponse(Err(format!("Error reading response: {}", e)))
            }
        },
        Err(e) => {
            println!("Error making request: {}", e);
            Message::ApiResponse(Err(format!("Error making request: {}", e)))
        }
    }
}

impl Robigui {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::ThemeChanged(x) => self.selected_theme = x,
            Message::UsernameChanged(username) => self.username = username,
            Message::SearchClicked => {
                let username = self.username.clone();
                return Task::perform(async move { search_user(username) }, |msg| msg);
            }
            Message::ApiResponse(result) => match result {
                Ok(response) => println!("Search completed successfully: {}", response),
                Err(error) => println!("Search failed: {}", error),
            },
        }

        Task::none()
    }

    pub fn view(&self) -> Column<Message> {
        let padding = 20;
        let spacing = 20;

        let username = text_input("Username", &self.username).on_input(Message::UsernameChanged);

        let version = text(format!("Robigui  v{}", env!("CARGO_PKG_VERSION")))
            .color(self.theme().extended_palette().primary.base.color);

        let theme_list = pick_list(
            Theme::ALL,
            Some(self.selected_theme.clone()),
            Message::ThemeChanged,
        );

        let search = button(text("Search ")).on_press(Message::SearchClicked);

        let setting_panel = row![username, search]
            .spacing(spacing)
            .align_y(Vertical::Center);

        let footer = column![
            horizontal_rule(0),
            row![
                column![version]
                    .width(Length::Fill)
                    .align_x(Horizontal::Left),
                column![theme_list]
                    .width(Length::Fill)
                    .align_x(Horizontal::Right)
            ]
            .padding([0, padding])
            .align_y(Vertical::Center)
        ]
        .align_x(Horizontal::Center);

        let home = column![
            column![setting_panel]
                .padding(padding)
                .align_x(Horizontal::Center)
                .spacing(spacing),
            footer
        ]
        .align_x(Horizontal::Center);

        home
    }

    pub fn theme(&self) -> Theme {
        self.selected_theme()
    }

    pub fn title(&self) -> String {
        "Robigui".to_string()
    }
}
