use crate::config::{AudioSource, VideoSource};
use crate::ui::Message;
use crate::{d_column, d_row, d_sub_title, d_text_input, define_component, t};
use iced::widget::text;

define_component!(output, |config, _| {
    let sub_title = d_sub_title!(t! {
        en: "Output",
        zh: "输出"
    }
    .to_string(),);

    let column = d_column![sub_title];

    if config.default.video_source == VideoSource::No && config.default.audio_source == AudioSource::No {
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

    let record = d_row![
        text(
            t! {
                en: "Record file: ",
                zh: "录制文件："
            }
            .to_string(),
        ),
        d_text_input!("", &config.default.record)
            .width(500)
            .on_input(Message::RecordChanged),
    ];

    let v4l2 = d_row![
        text(
            t! {
                en: "V4L2 device: ",
                zh: "V4L2 设备："
            }
            .to_string(),
        ),
        d_text_input!("", &config.default.v4l2)
            .width(500)
            .on_input(Message::V4l2Changed),
    ];

    column.push(record).push(v4l2).into()
});
