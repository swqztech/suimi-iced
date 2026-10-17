use crate::generate::{self, Mode, PasswordOptions, Strength};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, column, container, progress_bar, row, slider, text, toggler, Space,
};
use iced::{clipboard, Color, Element, Length, Task, Theme};

pub struct SuimiApp {
    mode: Mode,
    password_opts: PasswordOptions,
    username_length: u32,
    output: String,
    strength: Strength,
}

#[derive(Clone, Debug)]
pub enum Message {
    Mode(Mode),
    Roll,
    RollSix,
    Copy,
    PasswordLength(f32),
    UsernameLength(f32),
    ToggleLowercase(bool),
    ToggleUppercase(bool),
    ToggleDigits(bool),
    ToggleSymbols(bool),
    ToggleExcludeAmbiguous(bool),
}

impl SuimiApp {
    pub fn new() -> (Self, Task<Message>) {
        let password_opts = PasswordOptions::default();
        let strength = generate::password_strength(&password_opts);
        let output = generate::generate_password(&password_opts);
        (
            Self {
                mode: Mode::Password,
                password_opts,
                username_length: 12,
                output,
                strength,
            },
            Task::none(),
        )
    }

    fn roll_once(&mut self) {
        self.output = match self.mode {
            Mode::Password => generate::generate_password(&self.password_opts),
            Mode::Username => generate::generate_username(self.username_length),
            Mode::Nickname => generate::generate_nickname(),
        };
        if self.mode == Mode::Password {
            self.strength = generate::password_strength(&self.password_opts);
        }
    }

    fn roll_six(&mut self) {
        if self.mode == Mode::Password {
            self.output = generate::generate_passwords(&self.password_opts, 6);
            self.strength = generate::password_strength(&self.password_opts);
        } else {
            let lines: Vec<String> = (0..6)
                .map(|_| match self.mode {
                    Mode::Username => generate::generate_username(self.username_length),
                    Mode::Nickname => generate::generate_nickname(),
                    Mode::Password => unreachable!(),
                })
                .collect();
            self.output = lines.join("\n");
        }
    }
}

pub fn update(app: &mut SuimiApp, message: Message) -> Task<Message> {
    match message {
        Message::Mode(m) if m != app.mode => {
            app.mode = m;
            app.roll_once();
            Task::none()
        }
        Message::Roll => {
            app.roll_once();
            Task::none()
        }
        Message::RollSix => {
            app.roll_six();
            Task::none()
        }
        Message::Copy => {
            if app.output.is_empty() {
                Task::none()
            } else {
                clipboard::write(app.output.clone())
            }
        }
        Message::PasswordLength(v) => {
            app.password_opts.length = v.round().clamp(6.0, 64.0) as u32;
            app.strength = generate::password_strength(&app.password_opts);
            Task::none()
        }
        Message::UsernameLength(v) => {
            app.username_length = v.round().clamp(4.0, 32.0) as u32;
            Task::none()
        }
        Message::ToggleLowercase(v) => {
            app.password_opts.lowercase = v;
            app.strength = generate::password_strength(&app.password_opts);
            Task::none()
        }
        Message::ToggleUppercase(v) => {
            app.password_opts.uppercase = v;
            app.strength = generate::password_strength(&app.password_opts);
            Task::none()
        }
        Message::ToggleDigits(v) => {
            app.password_opts.digits = v;
            app.strength = generate::password_strength(&app.password_opts);
            Task::none()
        }
        Message::ToggleSymbols(v) => {
            app.password_opts.symbols = v;
            app.strength = generate::password_strength(&app.password_opts);
            Task::none()
        }
        Message::ToggleExcludeAmbiguous(v) => {
            app.password_opts.exclude_ambiguous = v;
            app.strength = generate::password_strength(&app.password_opts);
            Task::none()
        }
        _ => Task::none(),
    }
}

pub fn view(app: &SuimiApp) -> Element<'_, Message> {
    let header = column![
        text("随密").size(32).color(theme::GOLD),
        text("掷一次，拿一把新钥匙").size(14).color(theme::TEXT),
        text("本地生成 · 不上传 · 系统安全随机源")
            .size(11)
            .color(theme::MUTED),
    ]
    .spacing(4)
    .align_x(Horizontal::Center);

    let tabs = row![
        mode_tab("密码", app.mode == Mode::Password, Message::Mode(Mode::Password)),
        mode_tab("用户名", app.mode == Mode::Username, Message::Mode(Mode::Username)),
        mode_tab("网名", app.mode == Mode::Nickname, Message::Mode(Mode::Nickname)),
    ]
    .spacing(8);

    let out_display = if app.output.is_empty() {
        "—"
    } else {
        app.output.as_str()
    };

    let output_panel = container(
        text(out_display)
            .size(15)
            .color(Color::from_rgb(0.9, 0.93, 0.94))
            .font(iced::Font::MONOSPACE),
    )
    .width(Length::Fill)
    .padding([14, 16])
    .style(|_| panel_style());

    let mut body = column![
        header,
        vspace(14.0),
        tabs,
        vspace(16.0),
        output_panel,
    ]
    .spacing(0)
    .width(Length::Fill);

    if app.mode == Mode::Password {
        let strength_label = format!(
            "{} · 约 {:.0} bit",
            app.strength.label,
            app.strength.bits
        );
        body = body.push(vspace(8.0)).push(
            row![
                progress_bar(0.0..=1.0, app.strength.ratio).girth(6.0),
                text(strength_label).size(12).color(theme::MUTED),
            ]
            .spacing(8)
            .align_y(Vertical::Center),
        );
    }

    body = body.push(vspace(18.0)).push(mode_controls(app));

    body = body
        .push(vspace(20.0))
        .push(action_row())
        .push(vspace(16.0))
        .push(
            text("Passroll · 字符从勾选集合中均匀抽取")
                .size(11)
                .color(theme::MUTED),
        );

    container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([18, 22])
        .style(|_| container::Style {
            background: Some(theme::BG.into()),
            ..Default::default()
        })
        .into()
}

fn vspace(h: f32) -> Space {
    Space::new().height(h)
}

fn mode_controls(app: &SuimiApp) -> Element<'_, Message> {
    match app.mode {
        Mode::Password => column![
            row![
                text("长度").size(13).color(theme::MUTED),
                slider(
                    6.0..=64.0,
                    app.password_opts.length as f32,
                    Message::PasswordLength,
                )
                    .step(1.0_f32),
                text(format!("{}", app.password_opts.length))
                    .size(14)
                    .color(theme::GOLD),
            ]
            .spacing(10)
            .align_y(Vertical::Center),
            vspace(12.0),
            row![
                pill_toggler("小写 a-z", app.password_opts.lowercase, Message::ToggleLowercase),
                pill_toggler("大写 A-Z", app.password_opts.uppercase, Message::ToggleUppercase),
                pill_toggler("数字 0-9", app.password_opts.digits, Message::ToggleDigits),
                pill_toggler("符号 !@#", app.password_opts.symbols, Message::ToggleSymbols),
                pill_toggler(
                    "排除易混 0OIl1",
                    app.password_opts.exclude_ambiguous,
                    Message::ToggleExcludeAmbiguous,
                ),
            ]
            .spacing(8),
        ]
        .spacing(0)
        .into(),
        Mode::Username => column![
            row![
                text("长度").size(13).color(theme::MUTED),
                slider(4.0..=32.0, app.username_length as f32, Message::UsernameLength)
                    .step(1.0_f32),
                text(format!("{}", app.username_length))
                    .size(14)
                    .color(theme::GOLD),
            ]
            .spacing(10)
            .align_y(Vertical::Center),
            vspace(8.0),
            text("以小写字母开头，可含数字与下划线")
                .size(11)
                .color(theme::MUTED),
        ]
        .spacing(0)
        .into(),
        Mode::Nickname => text("从内置词库随机组合中文网名，可带数字后缀")
            .size(11)
            .color(theme::MUTED)
            .into(),
    }
}

fn mode_tab(label: &'static str, selected: bool, msg: Message) -> Element<'static, Message> {
    let style = if selected {
        tab_selected_style
    } else {
        tab_idle_style
    };
    button(text(label).size(13).color(if selected {
        theme::GOLD
    } else {
        theme::MUTED
    }))
    .padding([6, 14])
    .on_press(msg)
    .style(style)
    .into()
}

fn pill_toggler(
    label: &'static str,
    value: bool,
    on_toggle: fn(bool) -> Message,
) -> Element<'static, Message> {
    toggler(value)
        .label(label)
        .on_toggle(on_toggle)
        .size(16)
        .text_size(12)
        .into()
}

fn action_row() -> Element<'static, Message> {
    row![
        button(text("再掷一次").size(14))
            .padding([8, 16])
            .on_press(Message::Roll)
            .style(primary_btn_style),
        button(text("复制").size(13))
            .padding([8, 14])
            .on_press(Message::Copy)
            .style(secondary_btn_style),
        button(text("一次六个").size(13))
            .padding([8, 14])
            .on_press(Message::RollSix)
            .style(secondary_btn_style),
    ]
    .spacing(10)
    .into()
}

mod theme {
    use iced::Color;

    pub const BG: Color = Color::from_rgb(0.102, 0.149, 0.165);
    pub const PANEL: Color = Color::from_rgb(0.071, 0.11, 0.125);
    pub const GOLD: Color = Color::from_rgb(0.831, 0.686, 0.373);
    pub const TEXT: Color = Color::from_rgb(0.824, 0.855, 0.871);
    pub const MUTED: Color = Color::from_rgb(0.471, 0.529, 0.557);
    pub const BORDER: Color = Color::from_rgb(0.216, 0.282, 0.306);
    pub const PILL_ON: Color = Color::from_rgb(0.176, 0.243, 0.267);
    pub const PILL_OFF: Color = Color::from_rgb(0.125, 0.173, 0.188);
}

fn panel_style() -> container::Style {
    container::Style {
        background: Some(theme::PANEL.into()),
        border: iced::Border {
            color: theme::BORDER,
            width: 1.0,
            radius: 10.0.into(),
        },
        ..Default::default()
    }
}

fn tab_selected_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(theme::PILL_ON.into()),
        border: iced::Border {
            color: theme::GOLD,
            width: 1.5,
            radius: 6.0.into(),
        },
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(theme::PILL_ON.into()),
            ..base
        },
        _ => base,
    }
}

fn tab_idle_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(theme::PILL_OFF.into()),
        border: iced::Border {
            color: theme::BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    };
    if status == button::Status::Hovered {
        button::Style {
            background: Some(theme::PILL_ON.into()),
            ..base
        }
    } else {
        base
    }
}

fn primary_btn_style(_theme: &Theme, status: button::Status) -> button::Style {
    let green = Color::from_rgb(0.227, 0.698, 0.424);
    let hover = Color::from_rgb(0.27, 0.78, 0.48);
    button::Style {
        background: Some(
            (if status == button::Status::Hovered {
                hover
            } else {
                green
            })
            .into(),
        ),
        text_color: Color::WHITE,
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn secondary_btn_style(_theme: &Theme, status: button::Status) -> button::Style {
    let bg = if status == button::Status::Hovered {
        theme::PILL_ON
    } else {
        theme::PILL_OFF
    };
    button::Style {
        background: Some(bg.into()),
        text_color: theme::TEXT,
        border: iced::Border {
            color: theme::BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}
