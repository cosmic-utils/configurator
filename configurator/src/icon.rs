use cosmic::iced_core::Length;

pub static ICON_LENGTH: Length = Length::Fixed(25.0);

#[macro_export]
macro_rules! icon_handle {
    ($name:literal) => {{
        let bytes = include_bytes!(concat!("../res/icons/", $name, ".svg"));
        cosmic::widget::icon::from_svg_bytes(bytes)
    }};
}

#[macro_export]
macro_rules! icon {
    ($name:literal) => {{
        use $crate::icon::ICON_LENGTH;
        use $crate::icon_handle;

        cosmic::widget::icon::icon(icon_handle!($name))
            .height(ICON_LENGTH)
            .width(ICON_LENGTH)
    }};
}
#[macro_export]
macro_rules! icon_button {
    ($name:literal) => {{
        use $crate::icon_handle;
        cosmic::widget::button::icon(icon_handle!($name))
    }};
}
