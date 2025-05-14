use crate::config::{Config, ConfigRaw};
use iced::window::Settings;
use iced::{Font, Size};
use once_cell::sync::Lazy;
use std::error::Error;
use std::process::Stdio;
use std::sync::RwLock;
use sys_locale::get_locale;

mod config;
pub mod i18n;
mod ui;
mod util;

pub static CONFIG: Lazy<RwLock<Box<Config>>> = Lazy::new(|| {
    RwLock::new(Box::new(
        ConfigRaw::load()
            .unwrap_or_default()
            .to_config(true)
            .unwrap(),
    ))
});

pub static ARGS: RwLock<Option<String>> = RwLock::new(None);
fn main() -> Result<(), Box<dyn Error>> {
    if let Some(locale) = get_locale() {
        if locale.starts_with("zh") {
            *i18n::LANGUAGE.write().unwrap() = i18n::Language::Zh;
        }
    }

    drop(CONFIG.read().unwrap());

    let font = Font::with_name(if cfg!(target_os = "windows") {
        "Microsoft YaHei"
    } else if cfg!(target_os = "macos") {
        "PingFang SC"
    } else {
        "Noto Sans CJK SC"
    });

    iced::application(ui::WinMain::default, ui::WinMain::update, ui::WinMain::view)
        .window(Settings {
            size: Size {
                width: 800.0,
                height: 600.0,
            },
            resizable: true,
            decorations: true,
            min_size: Some(Size {
                width: 600.0,
                height: 450.0,
            }),
            ..Default::default()
        })
        .title(ui::WinMain::title)
        .subscription(ui::WinMain::subscription)
        .default_font(font)
        .run()
        .unwrap();

    let exe = CONFIG
        .read()
        .unwrap()
        .default
        .executable
        .clone()
        .unwrap_or_default();
    let command = ARGS.read().unwrap().clone();

    if let Some(args) = command {
        let _ = CONFIG.read().unwrap().to_raw().dump();

        let mut p = std::process::Command::new(exe);

        if !args.trim().is_empty() {
            let args = shell_words::split(&args).unwrap();
            p.args(args);
        }

        p.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    Ok(())
}
