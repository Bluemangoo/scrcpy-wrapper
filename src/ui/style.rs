pub mod style_default {
    use iced::{Length, Pixels};

    pub struct Padding;

    impl Padding {
        pub fn input() -> iced::Padding {
            iced::Padding {
                top: 4.0,
                bottom: 4.0,
                left: 8.0,
                right: 8.0,
            }
        }

        pub fn page() -> iced::Padding {
            iced::Padding {
                top: 10.0,
                bottom: 10.0,
                left: 20.0,
                right: 20.0,
            }
        }

        pub fn container() -> iced::Padding {
            iced::Padding {
                top: 5.0,
                bottom: 0.0,
                left: 0.0,
                right: 0.0,
            }
        }
    }

    pub struct Spacing;

    impl Spacing {
        pub fn general() -> Pixels {
            Pixels(10.0)
        }
    }

    pub struct Size;

    impl Size {
        pub fn input() -> Pixels {
            Pixels(15.0)
        }

        pub fn text_in_button() -> Pixels {
            Pixels(15.0)
        }

        pub fn text_sub_title() -> Pixels {
            Pixels(20.0)
        }
    }

    pub struct Height;

    impl Height {
        pub fn button() -> Length {
            Length::Fixed(28.0)
        }

        pub fn hr() -> Pixels {
            Pixels(10.0)
        }
    }
}

#[macro_export]
macro_rules! d_row {
    ($($x:expr),+ $(,)?) => {
        iced::widget::row![$($x,)+]
            .spacing($crate::ui::style_default::Spacing::general())
            .align_y(iced::Center)
    };
}

#[macro_export]
macro_rules! d_column {
    ($($x:expr),+ $(,)?) => {
        iced::widget::column![$($x,)+]
            .spacing($crate::ui::style_default::Spacing::general())
    };
}

#[macro_export]
macro_rules! d_text_input {
    ($($args:tt)*) => {
        iced::widget::text_input($($args)*)
            .padding($crate::ui::style_default::Padding::input())
            .size($crate::ui::style_default::Size::input())
    };
}

#[macro_export]
macro_rules! d_hr {
    () => {
        iced::widget::horizontal_rule($crate::ui::style_default::Height::hr())
    };
}

#[macro_export]
macro_rules! d_sub_title {
    ($($args:tt)*) => {
        iced::widget::text($($args)*)
            .size($crate::ui::style_default::Size::text_sub_title())
    };
}

#[macro_export]
macro_rules! d_button {
    ($text:expr) => {
        iced::widget::button(
            iced::widget::text($text)
                .align_x(iced::Center)
                .align_y(iced::Center)
                .size($crate::ui::style_default::Size::text_in_button()),
        )
        .style(iced::widget::button::secondary)
        .height($crate::ui::style_default::Height::button())
        .width(iced::Length::Shrink)
    };
}

#[macro_export]
macro_rules! d_pick_list {
    ($($args:tt)*) => {
        iced::widget::pick_list($($args)*)
            .text_size($crate::ui::style_default::Size::text_in_button())
            .padding($crate::ui::style_default::Padding::input())
            .width(iced::Length::Shrink)
    };
}
