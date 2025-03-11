use crate::ui::{Message, StateButton};
use crate::{d_column, d_row, d_sub_title, define_component, t};
use iced::widget::text;

define_component!(control, |config, _| {
    let sub_title = d_sub_title!(t! {
        en: "Control",
        zh: "控制"
    }
    .to_string(),);

    let keyboard = d_row![
        text(
            t! {
                en: "Keyboard input modes: ",
                zh: "键盘输入模式："
            }
            .to_string()
        ),
        StateButton::pick_list(config.default.keyboard, Message::KeyboardChanged)
    ];
    let mouse = d_row![
        text(
            t! {
                en: "Mouse input modes: ",
                zh: "鼠标输入模式："
            }
            .to_string()
        ),
        StateButton::pick_list(config.default.mouse, Message::MouseChanged)
    ];
    let gamepad = d_row![
        text(
            t! {
                en: "Gamepad input modes: ",
                zh: "游戏手柄输入模式："
            }
            .to_string()
        ),
        StateButton::pick_list(config.default.gamepad, Message::GamepadChanged)
    ];

    d_column![sub_title, keyboard, mouse, gamepad].into()
});
