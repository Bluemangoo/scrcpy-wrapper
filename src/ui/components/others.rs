use crate::ui::{Message, StateButton};
use crate::{d_button, d_column, d_row, d_sub_title, d_text_input, define_component, t};
use iced::widget::{checkbox, text};

define_component!(others, |config, _| {
    let sub_title = d_sub_title!(t! {
        en: "Others",
        zh: "其他"
    }
    .to_string(),);

    let start_app = d_row![
        text(
            t! {
                en: "Start app: ",
                zh: "启动应用："
            }
            .to_string()
        ),
        StateButton::button(config.default.app_name_type, Message::AppNameTypeChanged),
        d_text_input!("", &config.default.start_app)
            .width(200)
            .on_input(Message::StartAppChanged),
        checkbox(
            t! {en: "Restart if running", zh: "如果正在运行则重启"}.to_string(),
            config.default.restart_app,
        )
        .on_toggle(Message::RestartAppChanged),
    ];

    let time_limit = d_row![
        text(
            t! {
                en: "Time limit: ",
                zh: "时间限制："
            }
            .to_string()
        ),
        d_text_input!(
            "",
            &match &config.default.time_limit {
                Some(time_limit) => time_limit.to_string(),
                None => "".to_string(),
            },
        )
        .width(100)
        .on_input(Message::TimeLimitChanged),
        text("s")
    ];

    let stay_awake = checkbox(
        t! {en: "Stay awake (device)", zh: "保持唤醒（设备）"}.to_string(),
        config.default.stay_awake,
    )
    .on_toggle(Message::StayAwakeChanged);

    let disable_window = checkbox(
        t! {en: "Disable window", zh: "禁用窗口"}.to_string(),
        config.default.disable_window,
    )
    .on_toggle(Message::DisableWindowChanged);

    let borderless = checkbox(
        t! {en: "Borderless", zh: "无边框"}.to_string(),
        config.default.borderless,
    )
    .on_toggle(Message::BorderlessChanged);

    let always_on_top = checkbox(
        t! {en: "Always on top", zh: "窗口置顶"}.to_string(),
        config.default.always_on_top,
    )
    .on_toggle(Message::AlwaysOnTopChanged);

    let fullscreen = checkbox(
        t! {en: "Fullscreen", zh: "全屏"}.to_string(),
        config.default.fullscreen,
    )
    .on_toggle(Message::FullscreenChanged);

    let disable_screensaver = checkbox(
        t! {en: "Disable screensaver (PC)", zh: "禁用屏保（电脑）"}.to_string(),
        config.default.disable_screensaver,
    )
    .on_toggle(Message::DisableScreensaverChanged);

    let additional_args = d_row![
        text(
            t! {
                en: "Additional arguments",
                zh: "附加参数"
            }
            .to_string()
        ),
        d_text_input!("", &config.default.additional_args)
            .width(700)
            .on_input(Message::AdditionalArgsChanged)
    ];

    let reset = d_button!(t! {
        en: "Reset",
        zh: "重置"
    }
    .to_string())
    .on_press(Message::Reset);

    d_column![
        sub_title,
        start_app,
        time_limit,
        d_row![stay_awake],
        d_row![disable_window, borderless],
        d_row![always_on_top, fullscreen],
        disable_screensaver,
        additional_args,
        reset
    ]
    .into()
});
