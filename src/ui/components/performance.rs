use crate::config::{AudioCodec, AudioSource, VideoSource};
use crate::ui::Message;
use crate::{d_column, d_row, d_sub_title, d_text_input, define_component, t};
use iced::widget::{checkbox, text};

define_component!(performance, |config, _| {
    let have_audio = config.default.audio_source != AudioSource::No;
    let have_video = config.default.video_source != VideoSource::No;

    let sub_title = d_sub_title!(t! {
        en: "Performance",
        zh: "性能"
    }
    .to_string(),);

    let mut column = d_column![sub_title];

    if !have_audio && !have_video {
        return column
            .push(
                text(
                    t! {
                        en: "No audio and video source",
                        zh: "没有音频和视频源"
                    }
                    .to_string(),
                )
                .color([0.5, 0.5, 0.5]),
            )
            .into();
    }

    let mut bit_rate = d_row![text(
        t! {
            en: "Bit rate: ",
            zh: "码率："
        }
        .to_string()
    )];

    if have_video {
        bit_rate = bit_rate
            .push(text(
                t! {
                    en: "video",
                    zh: "视频"
                }
                .to_string(),
            ))
            .push(
                d_text_input!("8M", &config.default.video_bit_rate.to_string())
                    .on_input(Message::VideoBitRateChanged)
                    .width(100),
            );
    }

    if have_audio && config.default.audio_codec != AudioCodec::Raw {
        bit_rate = bit_rate
            .push(text(
                t! {
                    en: "audio",
                    zh: "音频"
                }
                .to_string(),
            ))
            .push(
                d_text_input!("128K", &config.default.audio_bit_rate.to_string())
                    .on_input(Message::AudioBitRateChanged)
                    .width(100),
            );
    }

    if have_video || (have_audio && config.default.audio_codec != AudioCodec::Raw) {
        column = column.push(bit_rate);
    }

    if have_video {
        let fps = d_row![
            if config.default.video_source == VideoSource::Camera {
                text(
                    t! {
                        en: "FPS: ",
                        zh: "帧率："
                    }
                    .to_string(),
                )
            } else {
                text(
                    t! {
                        en: "Max FPS: ",
                        zh: "最大帧率："
                    }
                    .to_string(),
                )
            },
            d_text_input!(
                "",
                &match &config.default.fps {
                    None => "".to_string(),
                    Some(fps) => fps.to_string(),
                }
            )
            .on_input(Message::FpsChanged)
            .width(60)
        ];
        column = column.push(fps);
    }

    let mut buffer = d_row![text(
        t! {
            en: "Buffer: ",
            zh: "缓冲："
        }
        .to_string(),
    )];

    if have_video {
        buffer = buffer
            .push(text(
                t! {
                    en: "video",
                    zh: "视频"
                }
                .to_string(),
            ))
            .push(
                d_text_input!(
                    "50",
                    &match &config.default.video_buffer {
                        None => "".to_string(),
                        Some(buffer) => buffer.to_string(),
                    },
                )
                .on_input(Message::VideoBufferChanged)
                .width(52),
            )
            .push(text("ms"));
    }
    if have_audio {
        buffer = buffer
            .push(text(
                t! {
                    en: "audio",
                    zh: "音频"
                }
                .to_string(),
            ))
            .push(
                d_text_input!(
                    "50",
                    &match &config.default.audio_buffer {
                        None => "".to_string(),
                        Some(buffer) => buffer.to_string(),
                    },
                )
                .on_input(Message::AudioBufferChanged)
                .width(52),
            )
            .push(text("ms"));
    }
    if have_video && have_audio {
        buffer = buffer.push(
            checkbox(
                t! {
                    en: "Sync",
                    zh: "同步"
                }
                .to_string(),
                config.default.buffer_sync,
            )
            .on_toggle(Message::BufferSyncChanged),
        );
    }

    column.push(buffer).into()
});
