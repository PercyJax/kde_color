use std::{
    fs::{self, File},
    io::Write,
};

use crate::Result;
use dirs::home_dir;
use palette::Srgb;
use toml_edit::{value, Document};

pub fn write_colors(shades: Vec<Srgb<u8>>) -> Result<()> {
    let home = home_dir()
        .expect("failed to get user home directory")
        .to_str()
        .expect("failed to parse home directory")
        .to_owned();

    let color_strings: Vec<String> = shades
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

    Ok(())
}
