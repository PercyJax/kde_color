use helpers::get_home_path;
use std::process::Command;
use thiserror::Error;

use palette::{FromColor, GetHue, Hsl, Srgb};

pub mod connections;
pub mod helpers;

type Result<T> = std::result::Result<T, KdeColorError>;

#[derive(Error, Debug, Clone)]
pub enum KdeColorError {
    #[error("kde config `{0}` not found")]
    ValueNotFound(String),
    #[error("unknown error")]
    Unknown,
}

fn get_config_path() -> String {
    get_home_path() + "/.config/kdeglobals"
}

fn get_color(group: String, key: String) -> Option<Srgb<u8>> {
    let color_str_vec = Command::new("kreadconfig5")
        .args(["--file", &get_config_path()])
        .args(["--group", &group])
        .args(["--key", &key])
        .output()
        .expect("failed to get accent color")
        .stdout;
    Some(helpers::into_color(color_str_vec))
}

pub fn get_header_foreground_color() -> Result<Srgb<u8>> {
    match get_color("Colors:Header".to_owned(), "ForegroundActive".to_owned()) {
        Some(color) => Ok(color),
        None => Err(KdeColorError::ValueNotFound(
            "Colors:Header -> ForegroundActive".to_owned(),
        )),
    }
}

pub fn get_shades(color: Srgb<u8>, num: usize) -> Vec<Srgb<u8>> {
    let color_linear = color.into_linear::<f64>();
    let color_hue = color_linear.get_hue();
    let saturation = 0.75;
    let mut brightnesses = Vec::new();
    for i in 0..num {
        brightnesses.push(0.1 + (i as f64 * 0.8 / (num - 1) as f64));
    }
    let mut values = Vec::new();
    for b in brightnesses {
        values.push(Srgb::from_color(Hsl::new_srgb(color_hue, saturation, b)).into_format::<u8>());
    }
    values
}
