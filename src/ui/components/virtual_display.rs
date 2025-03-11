use crate::config::VideoSource;
use crate::ui::Message;
use crate::{d_column, d_row, d_sub_title, d_text_input, define_component, t};
use iced::widget::{checkbox, text};

define_component!(virtual_display, |config, _| {
    let sub_title = d_sub_title!(t! {
        en: "Virtual display",
        zh: "虚拟显示器"
    }
    .to_string(),);

    let mut column = d_column![sub_title];

    if config.default.video_source != VideoSource::Display {
        return column
            .push(
                text(
                    t! {
                        en: "Not using display",
                        zh: "未使用显示器"
                    }
                    .to_string(),
                )
                .color([0.5, 0.5, 0.5]),
            )
            .into();
    }

    let enable_virtual_display = checkbox(
        t! {
            en: "Enable virtual display",
            zh: "启用虚拟显示器"
        }
        .to_string(),
        config.default.virtual_display,
    )
    .on_toggle(Message::VirtualDisplayChanged);

    let display_orientation = if config.default.display_width >= config.default.display_height {
        String::from(&t! {
            en: "landscape",
            zh: "横屏"
        })
    } else {
        String::from(&t! {
            en: "portrait",
            zh: "竖屏"
        })
    };

    let display_size = d_row![
        text(
            t! {
                en: "Display size: ",
                zh: "显示器尺寸："
            }
            .to_string(),
        ),
        d_text_input!("", &config.default.display_width.to_string())
            .width(80)
            .on_input(Message::DisplayWidthChanged),
        text("x"),
        d_text_input!("", &config.default.display_height.to_string())
            .width(80)
            .on_input(Message::DisplayHeightChanged),
        text(display_orientation.to_string())
    ];

    let destroy_app_on_close = checkbox(
        t! {
            en: "Destroy app on close",
            zh: "关闭时销毁应用"
        }
        .to_string(),
        config.default.destroy_app_on_close,
    )
    .on_toggle(Message::DestroyAppOnCloseChanged);

    column = column.push(enable_virtual_display);
    if config.default.virtual_display {
        column = column.push(display_size).push(destroy_app_on_close);
    }
    column.into()
});
