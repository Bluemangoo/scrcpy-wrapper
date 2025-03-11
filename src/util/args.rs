use crate::config::{
    AppNameType, AudioCodec, AudioSource, Camera, ConnectMethod, Gamepad, Keyboard, Mouse,
    OrientationAngle, OrientationType, VideoCodec, VideoSource,
};
use crate::CONFIG;

pub fn build_args() -> String {
    let mut args = String::new();
    let config = CONFIG.try_read().unwrap().default.clone();
    let have_audio = config.audio_source != AudioSource::No;
    let have_video = config.video_source != VideoSource::No;

    if let ConnectMethod::Otg = config.connect_method {
        args.push_str(" --otg");
    }

    match config.video_source {
        VideoSource::No => {
            args.push_str(" --no-video");
        }
        VideoSource::Display => {}
        VideoSource::Camera => {
            args.push_str(" --video-source=camera");
            match config.camera {
                Camera::Default => {}
                Camera::Front => {
                    args.push_str(" --camera-facing=front");
                }
                Camera::Back => {
                    args.push_str(" --camera-facing=back");
                }
                Camera::External => {
                    args.push_str(" --camera-facing=external");
                }
            }
        }
    }

    if have_video {
        if let Some(size) = config.video_size {
            args.push_str(" -m ");
            args.push_str(&size.to_string());
        }

        match &config.video_codec {
            VideoCodec::H264 => {}
            VideoCodec::H265 => {
                args.push_str(" --video-codec=h265");
            }
            VideoCodec::Av1 => {
                args.push_str(" --video-codec=av1");
            }
        }

        if !config.video_codec_options.trim().is_empty() {
            for opt in config.video_codec_options.split_whitespace() {
                args.push_str(" --audio-codec-options=");
                args.push_str(opt);
            }
        }

        if (config.orientation_angle != OrientationAngle::Default)
            || (config.orientation_type == OrientationType::Capture && config.orientation_lock)
        {
            args.push_str(match config.orientation_type {
                OrientationType::Client => " --capture-orientation=",
                OrientationType::Capture => "--orientation=",
            });
            if config.orientation_type == OrientationType::Capture && config.orientation_lock {
                args.push('@');
            }
            if config.orientation_angle != OrientationAngle::Default && config.orientation_flip {
                args.push_str("flip");
            }
            if config.orientation_angle != OrientationAngle::Default {
                args.push_str(match config.orientation_angle {
                    OrientationAngle::Default => "0",
                    OrientationAngle::_0 => "0",
                    OrientationAngle::_90 => "90",
                    OrientationAngle::_180 => "180",
                    OrientationAngle::_270 => "270",
                });
            }
        }
    }

    match config.audio_source {
        AudioSource::No => {
            args.push_str(" --no-audio");
        }
        AudioSource::Output => {
            if config.video_source == VideoSource::Camera {
                args.push_str(" --audio-source=output");
            }
        }
        AudioSource::Playback => {
            args.push_str(" --audio-source=playback");
            if config.audio_dup {
                args.push_str(" --audio-dup");
            }
        }
        AudioSource::Mic => {
            if config.video_source != VideoSource::Camera {
                args.push_str(" --audio-source=mic");
            }
        }
    }

    if have_audio {
        match &config.audio_codec {
            AudioCodec::Opus => {}
            AudioCodec::Aac => {
                args.push_str(" --audio-codec=aac");
            }
            AudioCodec::Flac => {
                args.push_str(" --audio-codec=flac");
            }
            AudioCodec::Raw => {
                args.push_str(" --audio-codec=raw");
            }
        }

        if !config.audio_codec_options.trim().is_empty() {
            for opt in config.audio_codec_options.split_whitespace() {
                args.push_str(" --audio-codec-options=");
                args.push_str(opt);
            }
        }

        if have_audio && have_video && !config.audio_playback && !config.video_playback {
            args.push_str(" --no-playback");
        } else {
            if have_audio && !config.audio_playback {
                args.push_str(" --no-audio-playback");
            }
            if have_video && !config.video_playback {
                args.push_str(" --no-video-playback");
            }
        }
    }

    if have_video && !config.video_bit_rate.trim().is_empty() {
        args.push_str(" --video-bit-rate=");
        args.push_str(config.video_bit_rate.trim());
    }
    if have_audio
        && config.audio_codec != AudioCodec::Raw
        && !config.audio_bit_rate.trim().is_empty()
    {
        args.push_str(" --audio-bit-rate=");
        args.push_str(config.audio_bit_rate.trim());
    }

    if let Some(fps) = config.fps {
        match config.video_source {
            VideoSource::No => {}
            VideoSource::Display => {
                args.push_str(" --max-fps=");
                args.push_str(&fps.to_string());
            }
            VideoSource::Camera => {
                args.push_str(" --camera-fps=");
                args.push_str(&fps.to_string());
            }
        }
    }

    if have_video {
        if let Some(buffer) = config.video_buffer {
            args.push_str(" --video-buffer=");
            args.push_str(&buffer.to_string());
        }
    }
    if have_audio {
        if let Some(buffer) = config.audio_buffer {
            args.push_str(" --audio-buffer=");
            args.push_str(&buffer.to_string());
        }
    }

    match config.keyboard {
        Keyboard::Sdk => {}
        Keyboard::Uhid => {
            args.push_str(" --keyboard=uhid");
        }
        Keyboard::Aoa => {
            args.push_str(" --keyboard=aoa");
        }
        Keyboard::Disabled => {
            args.push_str(" --keyboard=disabled");
        }
    }
    match config.mouse {
        Mouse::Sdk => {}
        Mouse::Uhid => {
            args.push_str(" --mouse=uhid");
        }
        Mouse::Aoa => {
            args.push_str(" --mouse=aoa");
        }
        Mouse::Disabled => {
            args.push_str(" --mouse=disabled");
        }
    }
    match config.gamepad {
        Gamepad::Disabled => {}
        Gamepad::Uhid => {
            args.push_str(" --gamepad=uhid");
        }
        Gamepad::Aoa => {
            args.push_str(" --gamepad=aoa");
        }
    }

    if !config.record.trim().is_empty() {
        args.push_str(" --record=");
        args.push_str(config.record.trim());
    }

    if !config.v4l2.trim().is_empty() {
        args.push_str(" --v4l2=");
        args.push_str(config.v4l2.trim());
    }

    if config.virtual_display {
        args.push_str(" --new-display=");
        args.push_str(&config.display_width.to_string());
        args.push('x');
        args.push_str(&config.display_height.to_string());
        if !config.destroy_app_on_close {
            args.push_str(" --no-vd-destroy-content");
        }
    }

    if !config.start_app.trim().is_empty() {
        args.push_str(" --start-app=");
        if config.restart_app {
            args.push('+');
        }
        if config.app_name_type == AppNameType::AppName {
            args.push('?');
        }
        args.push_str(config.start_app.trim());
    }

    if let Some(time_limit) = config.time_limit {
        args.push_str(" --time-limit=");
        args.push_str(&time_limit.to_string());
    }

    if config.stay_awake {
        args.push_str(" --stay-awake");
    }

    if config.disable_window {
        args.push_str(" --no-window");
    }

    if config.borderless {
        args.push_str(" --window-borderless");
    }

    if config.always_on_top {
        args.push_str(" --always-on-top");
    }

    if config.fullscreen {
        args.push_str(" --fullscreen");
    }

    if config.disable_screensaver {
        args.push_str(" --no-screensaver");
    }

    if !config.additional_args.trim().is_empty() {
        args.push(' ');
        args.push_str(config.additional_args.trim());
    }

    args.trim().to_string()
}
