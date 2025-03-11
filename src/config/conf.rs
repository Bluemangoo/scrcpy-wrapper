use crate::i18n::{Language, LANGUAGE};
use crate::t;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::{current_dir, current_exe};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

#[inline]
fn config_path() -> PathBuf {
    let xdg_config_home = env::var("XDG_CONFIG_HOME").unwrap_or_default();
    if xdg_config_home.trim().is_empty() {
        home::home_dir()
            .map(|home| home.join(".config"))
            .unwrap_or(current_dir().unwrap())
    } else {
        PathBuf::from(xdg_config_home)
    }
    .join("scrcpy-wrapper.toml")
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigItemRaw {
    pub language: Option<String>,
    pub executable: Option<String>,
    pub connect_method: Option<String>,
    pub video_source: Option<String>,
    pub camera: Option<String>,
    pub video_size: Option<u32>,
    pub video_playback: Option<bool>,
    pub video_codec: Option<String>,
    pub video_codec_options: Option<String>,
    pub orientation_type: Option<String>,
    pub orientation_angle: Option<String>,
    pub orientation_lock: Option<bool>,
    pub orientation_flip: Option<bool>,
    pub audio_source: Option<String>,
    pub audio_dup: Option<bool>,
    pub audio_playback: Option<bool>,
    pub audio_codec: Option<String>,
    pub audio_codec_options: Option<String>,
    pub video_bit_rate: Option<String>,
    pub audio_bit_rate: Option<String>,
    pub fps: Option<u32>,
    pub video_buffer: Option<u32>,
    pub audio_buffer: Option<u32>,
    pub buffer_sync: Option<bool>,
    pub keyboard: Option<String>,
    pub mouse: Option<String>,
    pub gamepad: Option<String>,
    pub record: Option<String>,
    pub v4l2: Option<String>,
    pub virtual_display: Option<bool>,
    pub display_height: Option<u32>,
    pub display_width: Option<u32>,
    pub destroy_app_on_close: Option<bool>,
    pub start_app: Option<String>,
    pub restart_app: Option<bool>,
    pub app_name_type: Option<String>,
    pub time_limit: Option<u32>,
    pub stay_awake: Option<bool>,
    pub disable_window: Option<bool>,
    pub borderless: Option<bool>,
    pub always_on_top: Option<bool>,
    pub fullscreen: Option<bool>,
    pub disable_screensaver: Option<bool>,
    pub additional_args: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConfigItem {
    pub language: Language,
    pub executable: Option<String>,
    pub connect_method: ConnectMethod,
    pub video_source: VideoSource,
    pub camera: Camera,
    pub video_size: Option<u32>,
    pub video_playback: bool,
    pub video_codec: VideoCodec,
    pub video_codec_options: String,
    pub orientation_type: OrientationType,
    pub orientation_angle: OrientationAngle,
    pub orientation_lock: bool,
    pub orientation_flip: bool,
    pub audio_source: AudioSource,
    pub audio_dup: bool,
    pub audio_playback: bool,
    pub audio_codec: AudioCodec,
    pub audio_codec_options: String,
    pub video_bit_rate: String,
    pub audio_bit_rate: String,
    pub fps: Option<u32>,
    pub video_buffer: Option<u32>,
    pub audio_buffer: Option<u32>,
    pub buffer_sync: bool,
    pub keyboard: Keyboard,
    pub mouse: Mouse,
    pub gamepad: Gamepad,
    pub record: String,
    pub v4l2: String,
    pub virtual_display: bool,
    pub display_height: u32,
    pub display_width: u32,
    pub destroy_app_on_close: bool,
    pub start_app: String,
    pub restart_app: bool,
    pub app_name_type: AppNameType,
    pub time_limit: Option<u32>,
    pub stay_awake: bool,
    pub disable_window: bool,
    pub borderless: bool,
    pub always_on_top: bool,
    pub fullscreen: bool,
    pub disable_screensaver: bool,
    pub additional_args: String,
}
impl ConfigItemRaw {
    pub fn to_config(&self, apply: bool) -> Result<ConfigItem, Box<dyn Error>> {
        let base = current_exe()?
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let language = match &self.language {
            Some(language) => match language.as_str() {
                "zh" => {
                    if apply {
                        *LANGUAGE.write().unwrap() = Language::Zh;
                    };
                    Language::Zh
                }
                "en" => {
                    if apply {
                        *LANGUAGE.write().unwrap() = Language::En;
                    };
                    Language::En
                }
                _ => *LANGUAGE.read().unwrap(),
            },
            None => *LANGUAGE.read().unwrap(),
        };
        let executable = match &self.executable {
            None => {
                let mut executable: Option<String> = None;
                let mut search_list = vec![base];
                if let Ok(path) = env::var("PATH") {
                    #[cfg(target_os = "windows")]
                    let path_sep = ";";
                    #[cfg(not(target_os = "windows"))]
                    let path_sep = ":";
                    search_list.extend(path.split(path_sep).map(|s| s.to_string()));
                }
                let mut flag = false;
                #[cfg(target_os = "windows")]
                let search_filename_list = vec!["scrcpy.exe", "scrcpy.cmd", "scrcpy.bat"];
                #[cfg(not(target_os = "windows"))]
                let search_filename_list = vec!["scrcpy", "scrcpy.sh"];
                for file in search_filename_list {
                    for dir in &search_list {
                        let full_path = Path::new(dir).join(file);

                        if fs::metadata(&full_path).is_ok() && full_path.is_file() {
                            executable = Some(full_path.to_string_lossy().to_string());
                            flag = true;
                            break;
                        }
                    }
                    if flag {
                        break;
                    }
                }
                executable
            }
            Some(v) => Some(v.clone()),
        };

        Ok(ConfigItem {
            language,
            executable,
            connect_method: ConnectMethod::from_config_str(&self.connect_method),
            video_source: VideoSource::from_config_str(&self.video_source),
            camera: Camera::from_config_str(&self.camera),
            video_size: self.video_size,
            video_playback: self.video_playback.unwrap_or(true),
            video_codec: VideoCodec::from_config_str(&self.video_codec),
            video_codec_options: self.video_codec_options.clone().unwrap_or_default(),
            orientation_type: OrientationType::from_config_str(&self.orientation_type),
            orientation_angle: OrientationAngle::from_config_str(&self.orientation_angle),
            orientation_lock: self.orientation_lock.unwrap_or_default(),
            orientation_flip: self.orientation_flip.unwrap_or_default(),
            audio_source: AudioSource::from_config_str(&self.audio_source),
            audio_dup: self.audio_dup.unwrap_or_default(),
            audio_playback: self.audio_playback.unwrap_or(true),
            audio_codec: AudioCodec::from_config_str(&self.audio_codec),
            audio_codec_options: self.audio_codec_options.clone().unwrap_or_default(),
            video_bit_rate: self.video_bit_rate.clone().unwrap_or_default(),
            audio_bit_rate: self.audio_bit_rate.clone().unwrap_or_default(),
            fps: self.fps,
            video_buffer: self.video_buffer,
            audio_buffer: self.audio_buffer,
            buffer_sync: self.buffer_sync.unwrap_or(true),
            keyboard: Keyboard::from_config_str(&self.keyboard),
            mouse: Mouse::from_config_str(&self.mouse),
            gamepad: Gamepad::from_config_str(&self.gamepad),
            record: self.record.clone().unwrap_or_default(),
            v4l2: self.v4l2.clone().unwrap_or_default(),
            virtual_display: self.virtual_display.unwrap_or_default(),
            display_height: self.display_height.unwrap_or_default(),
            display_width: self.display_width.unwrap_or_default(),
            destroy_app_on_close: self.destroy_app_on_close.unwrap_or(true),
            start_app: self.start_app.clone().unwrap_or_default(),
            restart_app: self.restart_app.unwrap_or_default(),
            app_name_type: AppNameType::from_config_str(&self.app_name_type),
            time_limit: self.time_limit,
            stay_awake: self.stay_awake.unwrap_or_default(),
            disable_window: self.disable_window.unwrap_or_default(),
            borderless: self.borderless.unwrap_or_default(),
            always_on_top: self.always_on_top.unwrap_or_default(),
            fullscreen: self.fullscreen.unwrap_or_default(),
            disable_screensaver: self.disable_screensaver.unwrap_or_default(),
            additional_args: self.additional_args.clone().unwrap_or_default(),
        })
    }
}

impl ConfigItem {
    pub fn to_raw(&self) -> ConfigItemRaw {
        ConfigItemRaw {
            language: Some(self.language.to_config_string()),
            executable: self.executable.clone(),
            connect_method: Some(self.connect_method.to_config_string()),
            video_source: Some(self.video_source.to_config_string()),
            camera: Some(self.camera.to_config_string()),
            video_size: self.video_size,
            video_playback: Some(self.video_playback),
            video_codec: Some(self.video_codec.to_config_string()),
            video_codec_options: Some(self.video_codec_options.clone()),
            orientation_type: Some(self.orientation_type.to_config_string()),
            orientation_angle: Some(self.orientation_angle.to_config_string()),
            orientation_lock: Some(self.orientation_lock),
            orientation_flip: Some(self.orientation_flip),
            audio_source: Some(self.audio_source.to_config_string()),
            audio_dup: Some(self.audio_dup),
            audio_playback: Some(self.audio_playback),
            audio_codec: Some(self.audio_codec.to_config_string()),
            audio_codec_options: Some(self.audio_codec_options.clone()),
            video_bit_rate: Some(self.video_bit_rate.clone()),
            audio_bit_rate: Some(self.audio_bit_rate.clone()),
            fps: self.fps,
            video_buffer: self.video_buffer,
            audio_buffer: self.audio_buffer,
            buffer_sync: Some(self.buffer_sync),
            keyboard: Some(self.keyboard.to_config_string()),
            mouse: Some(self.mouse.to_config_string()),
            gamepad: Some(self.gamepad.to_config_string()),
            record: Some(self.record.clone()),
            v4l2: Some(self.v4l2.clone()),
            virtual_display: Some(self.virtual_display),
            display_height: Some(self.display_height),
            display_width: Some(self.display_width),
            destroy_app_on_close: Some(self.destroy_app_on_close),
            start_app: Some(self.start_app.clone()),
            restart_app: Some(self.restart_app),
            app_name_type: Some(self.app_name_type.to_config_string()),
            time_limit: self.time_limit,
            stay_awake: Some(self.stay_awake),
            disable_window: Some(self.disable_window),
            borderless: Some(self.borderless),
            always_on_top: Some(self.always_on_top),
            fullscreen: Some(self.fullscreen),
            disable_screensaver: Some(self.disable_screensaver),
            additional_args: Some(self.additional_args.clone()),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigRaw {
    pub version: Option<u32>,
    pub default: Option<ConfigItemRaw>,
    pub saved: Option<HashMap<String, ConfigItemRaw>>,
}

impl ConfigRaw {
    pub fn to_config(&self, apply: bool) -> Result<Config, Box<dyn Error>> {
        Ok(Config {
            default: self.default.clone().unwrap_or_default().to_config(apply)?,
            saved: self
                .saved
                .clone()
                .unwrap_or_default()
                .iter()
                .map(|(k, v)| (k.clone(), v.to_config(false).unwrap()))
                .collect(),
        })
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        if Path::new(&config_path()).exists() {
            let mut file = File::open(config_path())?;
            let mut toml_str = String::new();
            file.read_to_string(&mut toml_str)?;
            let t: ConfigRaw = toml::from_str(toml_str.as_str())?;
            if t.version.is_none() {
                let t: ConfigItemRaw = toml::from_str(toml_str.as_str())?;
                return Ok(Self {
                    version: Some(1),
                    default: Some(t),
                    saved: None,
                });
            }
            Ok(t)
        } else {
            Err("Config file not found".into())
        }
    }

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = config_path().parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(config_path())?;
        file.write_all(toml::to_string(self)?.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub default: ConfigItem,
    pub saved: HashMap<String, ConfigItem>,
}

impl Config {
    pub fn to_raw(&self) -> ConfigRaw {
        ConfigRaw {
            version: Some(1),
            default: Some(self.default.to_raw()),
            saved: Some(
                self.saved
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_raw()))
                    .collect(),
            ),
        }
    }
}

pub trait ConfigEnum {
    fn from_config_str(s: &Option<String>) -> Self;
    fn to_config_string(&self) -> String;
}

#[macro_export]
macro_rules! config_enum {
    (
        pub enum $name:ident {
            $(
                $(#[$meta:meta])?
                $variant:ident: $label:expr, $display:expr,
            )*
        }
    ) => {
        #[derive(Copy, Clone, PartialEq, Debug, Default)]
        pub enum $name {
            $(
                $(#[$meta])?
                $variant,
            )*
        }

        impl $crate::ui::ButtonState for $name {
            fn states() -> Vec<Self>
            where
                Self: Sized,
            {
                vec![$(
                    $name::$variant,
                )*]
            }
        }

        impl $crate::config::ConfigEnum for $name {
            fn from_config_str(s: &Option<String>) -> Self {
                match s {
                    Some(s) => match s.as_str() {
                        $(
                            $label => $name::$variant,
                        )*
                        _ => $name::default(),
                    },
                    None => $name::default(),
                }
            }

            fn to_config_string(&self) -> String {
                match self {
                    $(
                        $name::$variant => $label.to_string(),
                    )*
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        $(
                            $name::$variant => $display,
                        )*
                    }
                )
            }
        }
    };
}

config_enum! {
    pub enum ConnectMethod {
        #[default]
        Adb: "adb", "ADB",
        Otg: "otg", "OTG",
    }
}

config_enum! {
    pub enum VideoSource {
        No: "no", t! {zh: "无",en: "no video"}.to_string(),
        #[default]
        Display: "display", t! {zh: "屏幕",en: "display"}.to_string(),
        Camera:"camera",  t! {zh: "摄像头 (Android 12+)",en: "camera (Android 12+)"}.to_string(),
    }
}

config_enum! {
    pub enum Camera {
        #[default]
        Default: "default", t! {zh: "默认",en: "default"}.to_string(),
        Front:"front", t! {zh: "前置",en: "front"}.to_string(),
        Back:"back", t! {zh: "后置",en: "back"}.to_string(),
        External:"external", t! {zh: "外置",en: "external"}.to_string(),
    }
}

config_enum! {
    pub enum VideoCodec{
        #[default]
        H264: "h264", "h264",
        H265: "h265", "h265",
        Av1: "av1", "av1",
    }
}

config_enum! {
    pub enum OrientationType{
        #[default]
        Client: "client", t! {zh: "客户端",en: "client"}.to_string(),
        Capture: "capture", t! {zh: "捕获",en: "capture"}.to_string(),
    }
}

config_enum! {
    pub enum OrientationAngle {
        #[default]
        Default:"default", t! {zh: "默认",en: "default"}.to_string(),
        _0: "0", "0°".to_string(),
        _90:"90", "90°".to_string(),
        _180: "180", "180°".to_string(),
        _270: "270", "270°".to_string(),
    }
}

config_enum! {
    pub enum AudioSource {
        No: "no", t! {zh: "无音频",en: "No audio"}.to_string(),
        #[default]
        Output: "output", t! {zh: "音频输出 (Android 11+)",en: "Audio output (Android 11+)"}.to_string(),
        Playback: "playback", "Playback (Android 13+)".to_string(),
        Mic: "mic", t!{zh: "麦克风",en: "Microphone"}.to_string(),
    }
}

config_enum! {
    pub enum AudioCodec {
        #[default]
        Opus: "opus", "opus",
        Aac: "aac", "aac",
        Flac: "flac", "flac",
        Raw: "raw", "raw",
    }
}

config_enum! {
    pub enum Keyboard{
        #[default]
        Sdk: "sdk", "SDK".to_string(),
        Uhid: "uhid", "UHID".to_string(),
        Aoa: "aoa", "AOA".to_string(),
        Disabled: "disabled", t! {zh: "禁用",en: "Disabled"}.to_string(),
    }
}

config_enum! {
    pub enum Mouse{
        #[default]
        Sdk: "sdk", "SDK".to_string(),
        Uhid: "uhid", "UHID".to_string(),
        Aoa: "aoa", "AOA".to_string(),
        Disabled: "disabled", t! {zh: "禁用",en: "Disabled"}.to_string(),
    }
}

config_enum! {
    pub enum Gamepad{
        #[default]
        Disabled: "disabled", t! {zh: "禁用",en: "Disabled"}.to_string(),
        Uhid: "uhid", "UHID".to_string(),
        Aoa: "aoa", "AOA".to_string(),
    }
}

config_enum! {
    pub enum AppNameType{
        #[default]
        PackageName: "package_name", t! {zh: "包名",en: "Package name"}.to_string(),
        AppName: "app_name", t! {zh: "应用名",en: "App name"}.to_string(),
    }
}
