use crate::config::{AppNameType, AudioCodec, AudioSource, Camera, ConfigItemRaw, ConnectMethod, DisplayImePolicy, Gamepad, Keyboard, Mouse, OrientationAngle, OrientationType, VideoCodec, VideoSource};
use crate::i18n::{Language, LANGUAGE};
use crate::ui::{components, style_default};
use crate::util::{build_args, select_config_valid, ConfigStatus};
use crate::{d_hr, t, ARGS, CONFIG};
use iced::widget::container::Id;
use iced::widget::{column, container, scrollable};
use iced::window::close;
use iced::{window, Element, Size, Subscription, Task};

pub struct WinMain {
    pub(crate) args: String,
    size: Size,
    pub(crate) config_status: ConfigStatus,
}

impl Default for WinMain {
    fn default() -> Self {
        Self {
            args: build_args(),
            size: Size {
                width: 800.0,
                height: 600.0,
            },
            config_status: ConfigStatus::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ExecutablePathChanged(String),
    ConnectMethodChanged(ConnectMethod),
    VideoSourceChanged(VideoSource),
    CameraChanged(Camera),
    VideoSizeChanged(String),
    VideoPlaybackChanged(bool),
    VideoCodecChanged(VideoCodec),
    VideoCodecOptionsChanged(String),
    OrientationTypeChanged(OrientationType),
    OrientationAngleChanged(OrientationAngle),
    OrientationLockChanged(bool),
    OrientationFlipChanged(bool),
    AudioSourceChanged(AudioSource),
    AudioDupChanged(bool),
    AudioPlaybackChanged(bool),
    AudioCodecChanged(AudioCodec),
    AudioCodecOptionsChanged(String),
    VideoBitRateChanged(String),
    AudioBitRateChanged(String),
    FpsChanged(String),
    VideoBufferChanged(String),
    AudioBufferChanged(String),
    BufferSyncChanged(bool),
    KeyboardChanged(Keyboard),
    MouseChanged(Mouse),
    GamepadChanged(Gamepad),
    RecordChanged(String),
    V4l2Changed(String),
    VirtualDisplayChanged(bool),
    DisplayHeightChanged(String),
    DisplayWidthChanged(String),
    DisplayImePolicyChanged(DisplayImePolicy),
    DestroyAppOnCloseChanged(bool),
    StartAppChanged(String),
    RestartAppChanged(bool),
    AppNameTypeChanged(AppNameType),
    TimeLimitChanged(String),
    StayAwakeChanged(bool),
    DisableWindowChanged(bool),
    BorderlessChanged(bool),
    AlwaysOnTopChanged(bool),
    FullscreenChanged(bool),
    DisableScreensaverChanged(bool),
    AdditionalArgsChanged(String),

    ConfigSelectChanged(String),
    ConfigSelectSave,
    ConfigSelectDelete,
    ConfigSelectLoad,
    ConfigInputChanged(String),
    ConfigSave,

    LanguageChanged(Language),

    ArgsChanged(String),
    Reset,
    Run,
    Resize(Size),
}
impl WinMain {
    pub fn title(&self) -> String {
        String::from(&t! {
            en: "Scrcpy Config",
            zh: "Scrcpy 配置"
        })
    }

    pub fn update(&mut self, message: Message) -> impl Into<Task<Message>> {
        match message {
            Message::ExecutablePathChanged(path) => {
                CONFIG.write().unwrap().default.executable = Some(path);
                self.args = build_args();
            }
            Message::ConnectMethodChanged(method) => {
                CONFIG.write().unwrap().default.connect_method = method;
                self.args = build_args();
            }
            Message::VideoSourceChanged(source) => {
                CONFIG.write().unwrap().default.video_source = source;
                self.args = build_args();
            }
            Message::CameraChanged(camera) => {
                CONFIG.write().unwrap().default.camera = camera;
                self.args = build_args();
            }
            Message::VideoSizeChanged(size) => {
                if size.trim().is_empty() {
                    CONFIG.write().unwrap().default.video_size = None
                } else if let Ok(size) = size.parse::<u32>() {
                    CONFIG.write().unwrap().default.video_size = Some(size)
                };
                self.args = build_args();
            }
            Message::VideoPlaybackChanged(display) => {
                CONFIG.write().unwrap().default.video_playback = display;
                self.args = build_args();
            }
            Message::VideoCodecChanged(codec) => {
                CONFIG.write().unwrap().default.video_codec = codec;
                self.args = build_args();
            }
            Message::VideoCodecOptionsChanged(options) => {
                CONFIG.write().unwrap().default.video_codec_options = options;
                self.args = build_args();
            }
            Message::OrientationTypeChanged(orientation) => {
                CONFIG.write().unwrap().default.orientation_type = orientation;
                self.args = build_args();
            }
            Message::OrientationAngleChanged(angle) => {
                CONFIG.write().unwrap().default.orientation_angle = angle;
                self.args = build_args();
            }
            Message::OrientationLockChanged(lock) => {
                CONFIG.write().unwrap().default.orientation_lock = lock;
                self.args = build_args();
            }
            Message::OrientationFlipChanged(flip) => {
                CONFIG.write().unwrap().default.orientation_flip = flip;
                self.args = build_args();
            }
            Message::AudioSourceChanged(source) => {
                CONFIG.write().unwrap().default.audio_source = source;
                self.args = build_args();
            }
            Message::AudioDupChanged(dup) => {
                CONFIG.write().unwrap().default.audio_dup = dup;
                self.args = build_args();
            }
            Message::AudioPlaybackChanged(play) => {
                CONFIG.write().unwrap().default.audio_playback = play;
                self.args = build_args();
            }
            Message::AudioCodecChanged(codec) => {
                CONFIG.write().unwrap().default.audio_codec = codec;
                self.args = build_args();
            }
            Message::AudioCodecOptionsChanged(options) => {
                CONFIG.write().unwrap().default.audio_codec_options = options;
                self.args = build_args();
            }
            Message::VideoBitRateChanged(rate) => {
                CONFIG.write().unwrap().default.video_bit_rate = rate.trim().to_string();
                self.args = build_args();
            }
            Message::AudioBitRateChanged(rate) => {
                CONFIG.write().unwrap().default.audio_bit_rate = rate.trim().to_string();
                self.args = build_args();
            }
            Message::FpsChanged(fps) => {
                if fps.trim().is_empty() {
                    CONFIG.write().unwrap().default.fps = None
                } else if let Ok(size) = fps.parse::<u32>() {
                    CONFIG.write().unwrap().default.fps = Some(size)
                };
                self.args = build_args();
            }
            Message::VideoBufferChanged(buffer) => {
                if buffer.trim().is_empty() {
                    CONFIG.write().unwrap().default.video_buffer = None;
                    if CONFIG.read().unwrap().default.buffer_sync {
                        CONFIG.write().unwrap().default.audio_buffer = None;
                    }
                } else if let Ok(size) = buffer.parse::<u32>() {
                    CONFIG.write().unwrap().default.video_buffer = Some(size);
                    if CONFIG.read().unwrap().default.buffer_sync {
                        CONFIG.write().unwrap().default.audio_buffer = Some(size);
                    }
                };
                self.args = build_args();
            }
            Message::AudioBufferChanged(buffer) => {
                if buffer.trim().is_empty() {
                    CONFIG.write().unwrap().default.audio_buffer = None;
                    if CONFIG.read().unwrap().default.buffer_sync {
                        CONFIG.write().unwrap().default.video_buffer = None;
                    }
                } else if let Ok(size) = buffer.parse::<u32>() {
                    CONFIG.write().unwrap().default.audio_buffer = Some(size);
                    if CONFIG.read().unwrap().default.buffer_sync {
                        CONFIG.write().unwrap().default.video_buffer = Some(size);
                    }
                };
                self.args = build_args();
            }
            Message::BufferSyncChanged(sync) => {
                CONFIG.write().unwrap().default.buffer_sync = sync;
                self.args = build_args();
            }
            Message::KeyboardChanged(keyboard) => {
                CONFIG.write().unwrap().default.keyboard = keyboard;
                self.args = build_args();
            }
            Message::MouseChanged(mouse) => {
                CONFIG.write().unwrap().default.mouse = mouse;
                self.args = build_args();
            }
            Message::GamepadChanged(gamepad) => {
                CONFIG.write().unwrap().default.gamepad = gamepad;
                self.args = build_args();
            }
            Message::RecordChanged(record) => {
                CONFIG.write().unwrap().default.record = record;
                self.args = build_args();
            }
            Message::V4l2Changed(v4l2) => {
                CONFIG.write().unwrap().default.v4l2 = v4l2;
                self.args = build_args();
            }
            Message::VirtualDisplayChanged(virtual_display) => {
                CONFIG.write().unwrap().default.virtual_display = virtual_display;
                self.args = build_args();
            }
            Message::DisplayHeightChanged(height) => {
                if height.trim().is_empty() {
                    CONFIG.write().unwrap().default.display_height = 0
                } else if let Ok(size) = height.parse::<u32>() {
                    CONFIG.write().unwrap().default.display_height = size
                };
                self.args = build_args();
            }
            Message::DisplayWidthChanged(width) => {
                if width.trim().is_empty() {
                    CONFIG.write().unwrap().default.display_width = 0
                } else if let Ok(size) = width.parse::<u32>() {
                    CONFIG.write().unwrap().default.display_width = size
                };
                self.args = build_args();
            }
            Message::DisplayImePolicyChanged(policy) => {
                CONFIG.write().unwrap().default.display_ime_policy = policy;
                self.args = build_args();
            }
            Message::DestroyAppOnCloseChanged(destroy_app_on_close) => {
                CONFIG.write().unwrap().default.destroy_app_on_close = destroy_app_on_close;
                self.args = build_args();
            }
            Message::StartAppChanged(start_app) => {
                CONFIG.write().unwrap().default.start_app = start_app;
                self.args = build_args();
            }
            Message::RestartAppChanged(restart_app) => {
                CONFIG.write().unwrap().default.restart_app = restart_app;
                self.args = build_args();
            }
            Message::AppNameTypeChanged(app_name_type) => {
                CONFIG.write().unwrap().default.app_name_type = app_name_type;
                self.args = build_args();
            }
            Message::TimeLimitChanged(time_limit) => {
                if time_limit.trim().is_empty() {
                    CONFIG.write().unwrap().default.time_limit = None
                } else if let Ok(size) = time_limit.parse::<u32>() {
                    CONFIG.write().unwrap().default.time_limit = Some(size)
                };
                self.args = build_args();
            }
            Message::StayAwakeChanged(stay_awake) => {
                CONFIG.write().unwrap().default.stay_awake = stay_awake;
                self.args = build_args();
            }
            Message::DisableWindowChanged(disable_window) => {
                CONFIG.write().unwrap().default.disable_window = disable_window;
                self.args = build_args();
            }
            Message::BorderlessChanged(borderless) => {
                CONFIG.write().unwrap().default.borderless = borderless;
                self.args = build_args();
            }
            Message::AlwaysOnTopChanged(always_on_top) => {
                CONFIG.write().unwrap().default.always_on_top = always_on_top;
                self.args = build_args();
            }
            Message::FullscreenChanged(fullscreen) => {
                CONFIG.write().unwrap().default.fullscreen = fullscreen;
                self.args = build_args();
            }
            Message::DisableScreensaverChanged(disable_screensaver) => {
                CONFIG.write().unwrap().default.disable_screensaver = disable_screensaver;
                self.args = build_args();
            }
            Message::AdditionalArgsChanged(args) => {
                CONFIG.write().unwrap().default.additional_args = args;
                self.args = build_args();
            }

            Message::ConfigSelectChanged(name) => {
                self.config_status.chosen = name;
            }
            Message::ConfigSelectSave => {
                if self.config_status.chosen.trim().is_empty() {
                    return Task::none();
                }
                let mut config = CONFIG.write().unwrap();
                let config_item = config.default.clone();
                config
                    .saved
                    .insert(self.config_status.chosen.trim().into(), config_item);
                config.to_raw().dump().unwrap();
            }
            Message::ConfigSelectDelete => {
                if self.config_status.chosen.trim().is_empty() {
                    return Task::none();
                }
                let mut config = CONFIG.write().unwrap();
                config.saved.remove(&self.config_status.chosen);
                config.to_raw().dump().unwrap();
            }
            Message::ConfigSelectLoad => {
                let mut config = CONFIG.write().unwrap();
                if !select_config_valid(&self.config_status.chosen, &config) {
                    return Task::none();
                }
                if let Some(c) = config.saved.get(&self.config_status.chosen) {
                    config.default = c.clone();
                }
                drop(config);
                self.args = build_args();
            }
            Message::ConfigInputChanged(name) => {
                self.config_status.input = name;
            }
            Message::ConfigSave => {
                if self.config_status.input.trim().is_empty() {
                    return Task::none();
                }
                let mut config = CONFIG.write().unwrap();
                let config_item = config.default.clone();
                config
                    .saved
                    .insert(self.config_status.input.trim().into(), config_item);
                config.to_raw().dump().unwrap();
            }

            Message::LanguageChanged(language) => {
                CONFIG.write().unwrap().default.language = language;
                *LANGUAGE.write().unwrap() = language;
                self.args = build_args();
            }

            Message::ArgsChanged(command) => {
                self.args = command;
            }
            Message::Reset => {
                let mut c = ConfigItemRaw::default().to_config(false).unwrap();
                c.language = *LANGUAGE.read().unwrap();
                CONFIG.write().unwrap().default = c;
                CONFIG.read().unwrap().to_raw().dump().unwrap();
                self.args = build_args();
            }
            Message::Resize(size) => {
                self.size = size;
            }
            Message::Run => {
                *ARGS.write().unwrap() = Some(self.args.clone());
                return window::get_latest().then(|id| close(id.unwrap()));
            }
        };
        ().into()
    }

    pub fn view(&self) -> Element<Message> {
        let config = CONFIG.try_read().unwrap();

        let config_section = {
            column![
                components::exe_info(&config, self),
                components::connect_method(&config, self),
                d_hr!(),
                components::video(&config, self),
                d_hr!(),
                components::audio(&config, self),
                d_hr!(),
                components::performance(&config, self),
                d_hr!(),
                components::control(&config, self),
                d_hr!(),
                components::output(&config, self),
                d_hr!(),
                components::virtual_display(&config, self),
                d_hr!(),
                components::others(&config, self),
                d_hr!(),
                components::config(&config, self),
            ]
            .padding(style_default::Padding::page())
            .spacing(style_default::Spacing::general())
        };

        container(
            column![
                scrollable(config_section).height(self.size.height - 100.0),
                components::action_section(&config, self)
            ]
            .padding(style_default::Padding::container()),
        )
        .id(Id::new("page"))
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(|size| Message::Resize(size.1))
    }
}
