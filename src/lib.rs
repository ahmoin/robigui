pub mod style;
pub mod utils;

use iced::{
    Length, Task, Theme,
    alignment::{Horizontal, Vertical},
    widget::{Column, button, column, horizontal_rule, pick_list, row, text},
};

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(Theme),
}

#[derive(Debug)]
pub struct Robigui {
    selected_theme: Theme,
}

impl Default for Robigui {
    fn default() -> Self {
        Self {
            selected_theme: Theme::CatppuccinMacchiato,
        }
    }
}

impl Robigui {
    pub fn selected_theme(&self) -> Theme {
        self.selected_theme.clone()
    }
}

impl Robigui {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::ThemeChanged(x) => self.selected_theme = x,
        }

        Task::none()
    }

    pub fn view(&self) -> Column<Message> {
        let padding = 20;
        let spacing = 20;

        let version = text(format!("Robigui  v{}", env!("CARGO_PKG_VERSION")))
            .color(self.theme().extended_palette().primary.base.color);

        let theme_list = pick_list(
            Theme::ALL,
            Some(self.selected_theme.clone()),
            Message::ThemeChanged,
        );

        let search = button(row![text!("Search ")].spacing(10));

        let setting_panel = row![search].spacing(spacing).align_y(Vertical::Center);

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
