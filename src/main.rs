use iced::application;
use robigui::{
    Robigui,
    utils::{self, error_dialog},
};

fn main() {
    if let Err(e) = try_main() {
        error_dialog(e);
    }
}

fn try_main() -> anyhow::Result<()> {
    application(Robigui::default, Robigui::update, Robigui::view)
        .settings(utils::settings())
        .window(utils::window_settings())
        .theme(Robigui::theme)
        .title(Robigui::title)
        .run()?;

    Ok(())
}
