use crate::config::AudioSource;
use crate::ui::{Message, StateButton};
use crate::{d_column, d_row, d_sub_title, d_text_input, define_component, t};
use iced::widget::{checkbox, text};

define_component!(audio, |config, _| {
    let sub_title = d_sub_title!(t! {
        en: "Audio",
        zh: "音频"
    }
    .to_string(),);

    let column = d_column![sub_title];

    let mut source = d_row![
        text(
            t! {
                en: "Audio source: ",
                zh: "音频源："
            }
            .to_string()
        ),
        StateButton::pick_list(config.default.audio_source, Message::AudioSourceChanged)
    ];

    if config.default.audio_source == AudioSource::No {
        return column.push(source).into();
    }

    if config.default.audio_source == AudioSource::Playback {
        source = source.push(
            checkbox(
                t! {
                    en: "duplicate audio",
                    zh: "复制音频"
                }
                .to_string(),
                config.default.audio_dup,
            )
            .on_toggle(Message::AudioDupChanged),
        );
    }

    source = source.push(
        checkbox(
            t! {
                en: "playback",
                zh: "播放"
            }
            .to_string(),
            config.default.audio_playback,
        )
        .on_toggle(Message::AudioPlaybackChanged),
    );

    let codec = d_row![
        text(
            t! {
                en: "Audio codec: ",
                zh: "音频编解码器："
            }
            .to_string()
        ),
        StateButton::pick_list(config.default.audio_codec, Message::AudioCodecChanged),
        text(
            t! {
                en: "Options: ",
                zh: "参数："
            }
            .to_string()
        ),
        d_text_input!("", &config.default.audio_codec_options)
            .width(300)
            .on_input(Message::AudioCodecOptionsChanged)
    ];

    column.push(source).push(codec).into()
});
