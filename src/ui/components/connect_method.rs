use crate::ui::{Message, StateButton};
use crate::{d_row, define_component, t};
use iced::widget::{horizontal_space, text};

define_component!(connect_method, |config, _| {
    d_row![
        text(&t! {r
            zh: "连接方式：",
            en: "Connecting method: "
        }),
        StateButton::button(config.default.connect_method, Message::ConnectMethodChanged),
        horizontal_space(),
        StateButton::button(config.default.language, Message::LanguageChanged)
    ]
    .into()
});
