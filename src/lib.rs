use dirs::home_dir;
use std::{error::Error, process::Command};

use palette::{FromColor, GetHue, Hsl, Srgb};

pub mod connections;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn get_accent_color() -> Result<Srgb<u8>> {
    let home = home_dir()
        .expect("failed to get user home directory")
        .to_str()
        .expect("failed to parse home directory")
        .to_owned();

    let kde_globals_file_path = format!("{}{}", home, "/.config/kdeglobals");

    let color_str_vec = Command::new("kreadconfig5")
        .args(["--file", &kde_globals_file_path])
        .args(["--group", "General"])
        .args(["--key", "AccentColor"])
        .output()
        .expect("failed to get accent color")
        .stdout;

    let mut color_str = std::str::from_utf8(color_str_vec.as_slice())
        .expect("unsupported value received from kreadconfig5 util")
        .to_owned();

    color_str.retain(|c| !c.is_whitespace());

    let mut color_split = color_str.split(',');
    let (r, g, b): (u8, u8, u8);
    r = color_split
        .next()
        .expect("failed to extract red value")
        .parse::<u8>()
        .expect("failed to parse red value");
    g = color_split
        .next()
        .expect("failed to extract green value")
        .parse::<u8>()
        .expect("failed to parse green value");
    b = color_split
        .next()
        .expect("failed to extract blue value")
        .parse::<u8>()
        .expect("failed to parse blue value");

    let color = Srgb::new(r, g, b);

    Ok(color)
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
