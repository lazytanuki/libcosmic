// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Application API example

use cosmic::app::{Command, Core, Settings};
use cosmic::iced_core::Size;
use cosmic::{executor, iced, ApplicationExt, Element, Theme};

/// Runs application with these settings
#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let _ = tracing_log::LogTracer::init();


    let settings = Settings::default()
        .size(Size::new(1024., 768.));

    cosmic::app::run::<App>(settings, ())?;

    Ok(())
}

/// Messages that are used specifically by our [`App`].
#[derive(Clone, Debug)]
pub enum Message {
    UpdateAnimationMultiplier(u16),
    BtnPressed,
}

/// The [`App`] stores application-specific state.
pub struct App {
    core: Core,
    animation_multiplier: u16,
}

/// Implement [`cosmic::Application`] to integrate with COSMIC.
impl cosmic::Application for App {
    /// Default async executor to use with the app.
    type Executor = executor::Default;

    /// Argument received [`cosmic::Application::new`].
    type Flags = ();

    /// Message type specific to our [`App`].
    type Message = Message;

    /// The unique application ID to supply to the window manager.
    const APP_ID: &'static str = "org.cosmic.AppDemo";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Creates the application, and optionally emits command on initialize.
    fn init(core: Core, _input: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = App {
            core,
            animation_multiplier: 10,
        };

        let command = app.update_title();

        (app, command)
    }
    /// Handle application events here.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::UpdateAnimationMultiplier(value) => {
                self.animation_multiplier = value;
                let mut theme = self.core.system_theme().cosmic().clone();
                let layer = self.core.system_theme().layer;
                theme.animation_multiplier = value as f32 / 10.0;
                let theme = std::sync::Arc::new(theme);
                return cosmic::app::command::set_theme(Theme {
                    theme_type: cosmic::theme::ThemeType::Custom(theme),
                    layer,
                });
            }
            Message::BtnPressed => {
                println!("Button pressed");
            }
        };
        Command::none()
    }

    /// Creates a view after each update.
    fn view(&self) -> Element<Self::Message> {
        let btns_row = cosmic::widget::row()
            .push(cosmic::widget::button::suggested("Push me").on_press(Message::BtnPressed))
            .push(cosmic::widget::button::destructive("Push me").on_press(Message::BtnPressed))
            .push(cosmic::widget::button::standard("Push me").on_press(Message::BtnPressed));
        let slider = cosmic::widget::slider(
            0..=100,
            self.animation_multiplier,
            Message::UpdateAnimationMultiplier,
        );
        let col = cosmic::widget::column()
            .push(btns_row)
            .push(cosmic::widget::text("coucou"))
            .push(slider);

        let centered = cosmic::widget::container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center);

        Element::from(centered)
    }
}

impl App
where
    Self: cosmic::Application,
{
    fn update_title(&mut self) -> Command<Message> {
        self.set_window_title("Widget animations".into())
    }
}
