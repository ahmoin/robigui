use iced::{
    Font, Pixels, Settings, Size, Task,
    window::{self, Level, Position},
};
use rfd::{AsyncMessageDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

pub const GEIST_MONO_FONT: Font = Font::with_name("GeistMono NFM");

pub fn error_dialog(error: anyhow::Error) {
    MessageDialog::new()
        .set_buttons(MessageButtons::Ok)
        .set_description(error.to_string())
        .set_level(MessageLevel::Error)
        .set_title("Robigui")
        .show();
}

pub fn message_dialog(message: String, level: MessageLevel) -> Task<MessageDialogResult> {
    let dialog = AsyncMessageDialog::new()
        .set_buttons(MessageButtons::Ok)
        .set_description(message)
        .set_level(level)
        .set_title("Robigui")
        .show();
    Task::future(dialog)
}

pub fn settings() -> Settings {
    Settings {
        fonts: vec![include_bytes!("../assets/GeistMonoNerdFontMono-Regular.otf").into()],
        default_font: GEIST_MONO_FONT,
        default_text_size: Pixels(13.0),
        antialiasing: true,
        ..Default::default()
    }
}

pub fn window_settings() -> window::Settings {
    let size: Size = Size::new(1000.0, 600.0);

    window::Settings {
        size,
        position: Position::Centered,
        min_size: Some(size),
        visible: true,
        resizable: true,
        decorations: true,
        transparent: false,
        level: Level::Normal,
        icon: None,
        // icon: window::icon::from_file_data(
        //     include_bytes!("../logo/icon.png"),
        //     Some(ImageFormat::Png),
        // )
        // .ok(),
        exit_on_close_request: true,
        ..Default::default()
    }
}
