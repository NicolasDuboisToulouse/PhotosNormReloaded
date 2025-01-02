use eframe::egui;
use egui_taffy::taffy;

///
/// Create Self from an horizontal percentage
///
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

impl<T: taffy::prelude::FromPercent + taffy::prelude::FromLength> FromHPercent for taffy::Size<T> {
    fn from_h_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        taffy::Size {
            width: T::from_percent(percent.into()),
            height: T::from_length(0.),
        }
    }
}

pub fn h_percent<Input: Into<f32> + Copy, T: FromHPercent>(percent: Input) -> T {
    T::from_h_percent(percent)
}

///
/// Create Self from a taffy::Size
///
pub trait FromTaffySize {
    fn from_size<T: taffy::ResolveOrZero<Option<f32>, f32>>(size: taffy::Size<T>) -> Self;
}

impl FromTaffySize for egui::Vec2 {
    fn from_size<T: taffy::ResolveOrZero<Option<f32>, f32>>(size: taffy::Size<T>) -> Self {
        egui::Vec2 {
            x: size.width.resolve_or_zero(Some(0.0)),
            y: size.height.resolve_or_zero(Some(0.0)),
        }
    }
}

///
/// Create a Rect<LengthPercentage>
///
pub fn rect_lp(top: f32, bottom: f32, left: f32, right: f32) -> taffy::Rect<taffy::LengthPercentage> {
    taffy::Rect {
        top: taffy::LengthPercentage::Length(top),
        bottom: taffy::LengthPercentage::Length(bottom),
        left: taffy::LengthPercentage::Length(left),
        right: taffy::LengthPercentage::Length(right),
    }
}

///
/// Create a Size<LengthPercentage>
///
pub fn size_lp(width: f32, height: f32) -> taffy::Size<taffy::LengthPercentage> {
    taffy::Size {
        width: taffy::LengthPercentage::Length(width),
        height: taffy::LengthPercentage::Length(height),
    }
}
