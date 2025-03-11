use crate::config::{OrientationAngle, OrientationType, VideoSource};
use crate::ui::{Message, StateButton};
use crate::{d_column, d_row, d_sub_title, d_text_input, define_component, t};
use iced::widget::{checkbox, text};

define_component!(video, |config, _| {
    let sub_title = d_sub_title!(t! {
        en: "Video",
        zh: "视频"
    }
    .to_string());

    let column = d_column![sub_title];

    let mut source = d_row![
        text(
            t! {
                en: "Video source: ",
                zh: "视频源："
            }
            .to_string()
        ),
        StateButton::pick_list(config.default.video_source, Message::VideoSourceChanged)
    ];

    if config.default.video_source == VideoSource::No {
        return column.push(source).into();
    }

    let camera_size = match &config.default.video_size {
        None => "".to_string(),
        Some(size) => size.to_string(),
    };
    if config.default.video_source == VideoSource::Camera {
        source = source.push(StateButton::pick_list(
            config.default.camera,
            Message::CameraChanged,
        ))
    }

    source = source
        .push(text(
            t! {
                en: "Size: ",
                zh: "分辨率："
            }
            .to_string(),
        ))
        .push(
            d_text_input!(
                &t! {
                    en: "longest side, e.g. 1920",
                    zh: "最长边，例如 1920"
                },
                &camera_size,
            )
            .on_input(Message::VideoSizeChanged)
            .width(180),
        )
        .push(
            checkbox(
                t! {
                    en: "playback",
                    zh: "播放"
                }
                .to_string(),
                config.default.video_playback,
            )
            .on_toggle(Message::VideoPlaybackChanged),
        );

    let codec = d_row![
        text(
            t! {
                en: "Video codec: ",
                zh: "视频编解码器："
            }
            .to_string()
        ),
        StateButton::pick_list(config.default.video_codec, Message::VideoCodecChanged),
        text(
            t! {
                en: "Options: ",
                zh: "参数："
            }
            .to_string()
        ),
        d_text_input!("", &config.default.video_codec_options)
            .width(300)
            .on_input(Message::VideoCodecOptionsChanged)
    ];

    let mut orientation = d_row![
        text(
            t! {
                en: "Orientation (clockwise): ",
                zh: "方向 (顺时针)："
            }
            .to_string(),
        ),
        StateButton::button(config.default.orientation_type, Message::OrientationTypeChanged),
        StateButton::pick_list(config.default.orientation_angle, Message::OrientationAngleChanged)
    ];

    if config.default.orientation_type == OrientationType::Capture {
        orientation = orientation.push(
            checkbox(
                t! {
                    en: "Lock orientation",
                    zh: "锁定方向"
                }
                .to_string(),
                config.default.orientation_lock,
            )
            .on_toggle(Message::OrientationLockChanged),
        );
    }

    if config.default.orientation_angle != OrientationAngle::Default {
        orientation = orientation.push(
            checkbox(
                t! {
                    en: "Flip",
                    zh: "翻转"
                }
                .to_string(),
                config.default.orientation_flip,
            )
            .on_toggle(Message::OrientationFlipChanged),
        );
    }

    column.push(source).push(codec).push(orientation).into()
});
