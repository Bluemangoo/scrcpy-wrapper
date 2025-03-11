use crate::ui::Message;
use crate::{d_row, d_text_input, define_component, t};
use iced::widget::text;

define_component!(exe_info, |config, _| {
    let executable_path = {
        let exe_str = match &config.default.executable {
            None => String::from(""),
            Some(p) => p.clone(),
        };
        d_text_input!(
            &t! {
                en: "Executable path",
                zh: "可执行文件路径"
            },
            &exe_str,
        )
        .on_input(Message::ExecutablePathChanged)
    };
    let executable_path_row = d_row![
        text(&t! {r
            en: "scrcpy executable path: ",
            zh: "scrcpy 可执行文件路径："
        }),
        executable_path,
    ];

    executable_path_row.into()
});
