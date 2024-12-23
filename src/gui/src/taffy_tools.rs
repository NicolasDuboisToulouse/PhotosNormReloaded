use egui_taffy::taffy;

pub trait FromHPercent {
    fn from_h_percent<Input: Into<f32> + Copy>(percent: Input) -> Self;
}

impl<T: taffy::prelude::FromPercent + taffy::prelude::FromLength> FromHPercent for taffy::Rect<T> {
    fn from_h_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        taffy::Rect {
            left: T::from_percent(percent.into()),
            right: T::from_percent(percent.into()),
            top: T::from_length(0.),
            bottom: T::from_length(0.),
        }
    }
}

pub fn h_percent<Input: Into<f32> + Copy, T: FromHPercent>(percent: Input) -> T {
    T::from_h_percent(percent)
}

pub fn rect_lp(
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
) -> taffy::Rect<taffy::LengthPercentage> {
    taffy::Rect {
        top: taffy::LengthPercentage::Length(top),
        bottom: taffy::LengthPercentage::Length(bottom),
        left: taffy::LengthPercentage::Length(left),
        right: taffy::LengthPercentage::Length(right),
    }
}
