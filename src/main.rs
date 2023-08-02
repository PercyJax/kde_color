use dirs::home_dir;
use std::{
    fs::{self, File},
    io::Write,
    process::Command,
};
use toml_edit::{value, Document};

use palette::{FromColor, GetHue, Hsl, Srgb};

fn main() {
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

    let color_linear = color.into_linear::<f64>();

    let color_hue = color_linear.get_hue();

    let saturation = 0.75;

    let brightnesses = [0.15, 0.30, 0.45, 0.60, 0.75, 0.90];

    let mut values = Vec::new();

    for b in brightnesses {
        values.push(Srgb::from_color(Hsl::new_srgb(color_hue, saturation, b)).into_format::<u8>());
    }

    let color_strings: Vec<String> = values
        .into_iter()
        .map(|shade| format!("#{:02X}{:02X}{:02X}", shade.red, shade.green, shade.blue))
        .collect();

    let toml = fs::read_to_string(home.clone() + "/.config/starship.toml")
        .expect("starship.toml not found");

    let mut doc = toml.parse::<Document>().expect("starship.toml is invalid");

    doc["palettes"]["primary"]["shade1"] = value(&color_strings[5]);
    doc["palettes"]["primary"]["shade2"] = value(&color_strings[4]);
    doc["palettes"]["primary"]["shade3"] = value(&color_strings[3]);
    doc["palettes"]["primary"]["shade4"] = value(&color_strings[2]);
    doc["palettes"]["primary"]["shade5"] = value(&color_strings[1]);
    doc["palettes"]["primary"]["text1"] = value(&color_strings[0]);
    doc["palettes"]["primary"]["text2"] = value(&color_strings[0]);
    doc["palettes"]["primary"]["text3"] = value(&color_strings[5]);
    doc["palettes"]["primary"]["text4"] = value(&color_strings[5]);

    let starship_config_path = home.clone() + "/.config/starship.toml";

    println!("{}", starship_config_path);

    File::create(starship_config_path)
        .expect("failed to open starship.toml for writing")
        .write_all(doc.to_string().as_bytes())
        .expect("failed writing to starship.toml");
}
