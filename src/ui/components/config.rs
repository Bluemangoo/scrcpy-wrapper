use crate::ui::Message;
use crate::util::select_config_valid;
use crate::{
    d_button, d_column, d_pick_list, d_row, d_sub_title, d_text_input, define_component, t,
};
use iced::widget::text;

define_component!(config, |config, win_main| {
    let keys: Vec<String> = config.saved.keys().cloned().collect();
    let valid;
    let chosed_config = if select_config_valid(&win_main.config_status.chosen, config) {
        valid = true;
        Some(win_main.config_status.chosen.clone())
    } else {
        valid = false;
        None
    };
    let saved_config = d_row![
        text(
            t! {
                en: "Saved Config: ",
                zh: "保存的配置："
            }
            .to_string()                    
        ),
        d_pick_list!(keys, chosed_config, Message::ConfigSelectChanged)
    ];
    let saved_config_actions = d_row![
        d_button!(t! {
            en: "Save",
            zh: "保存"
        }
        .to_string())
        .on_press_maybe(if valid {
            Some(Message::ConfigSelectSave)
        } else {
            None
        }),
        d_button!(t! {
            en: "Load",
            zh: "加载"
        }
        .to_string())
        .on_press_maybe(if valid {
            Some(Message::ConfigSelectLoad)
        } else {
            None
        }),
        d_button!(t! {
            en: "Delete",
            zh: "删除"
        }
        .to_string())
        .on_press_maybe(if valid {
            Some(Message::ConfigSelectDelete)
        } else {
            None
        })
    ];
    let save_actions = d_row![
        text(
            t! {
                en: "Save as",
                zh: "另存为"
            }
            .to_string()
        ),
        d_text_input!("", &win_main.config_status.input).on_input(Message::ConfigInputChanged),
        d_button!(t! {
            en: "Save",
            zh: "保存"
        }
        .to_string())
        .on_press(Message::ConfigSave),
    ];
    d_column![
        d_sub_title!(t! {
            en: "Config",
            zh: "配置"
        }
        .to_string()),
        saved_config,
        saved_config_actions,
        save_actions
    ]
    .into()
});
