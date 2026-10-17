#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod fonts;
mod generate;

fn main() -> iced::Result {
    let mut builder = iced::application(app::SuimiApp::new, app::update, app::view)
        .title("随密")
        .theme(iced::Theme::Dark)
        .window(iced::window::Settings {
            size: iced::Size::new(460.0, 580.0),
            min_size: Some(iced::Size::new(420.0, 480.0)),
            ..Default::default()
        });

    if let Some(bytes) = fonts::load_system_cjk_font() {
        builder = builder.font(bytes);
    }

    builder.run()
}
