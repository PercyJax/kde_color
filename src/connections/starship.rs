use std::{
    fs::{self, File},
    io::Write,
};
use toml_edit::{value, Document};

use crate::{
    get_color, get_shades,
    helpers::{format_hash, get_home_path},
};

fn get_config_path() -> String {
    get_home_path() + "/.config/starship.toml"
}

pub fn paint() {
    let shades = get_shades(
        get_color("Colors:Window".to_owned(), "ForegroundActive".to_owned()).unwrap(),
        8,
    );

    let color_strings: Vec<String> = shades.into_iter().map(|color| format_hash(color)).collect();

    let toml = fs::read_to_string(get_config_path()).expect("starship.toml not found");

    let mut doc = toml.parse::<Document>().expect("starship.toml is invalid");

    doc["palettes"]["primary"]["shade1"] = value(&color_strings[7]);
    doc["palettes"]["primary"]["shade2"] = value(&color_strings[6]);
    doc["palettes"]["primary"]["shade3"] = value(&color_strings[5]);
    doc["palettes"]["primary"]["shade4"] = value(&color_strings[2]);
    doc["palettes"]["primary"]["shade5"] = value(&color_strings[1]);
    doc["palettes"]["primary"]["text1"] = value(&color_strings[1]);
    doc["palettes"]["primary"]["text2"] = value(&color_strings[1]);
    doc["palettes"]["primary"]["text3"] = value(&color_strings[1]);
    doc["palettes"]["primary"]["text4"] = value(&color_strings[7]);

    File::create(get_config_path())
        .expect("failed to open starship.toml for writing")
        .write_all(doc.to_string().as_bytes())
        .expect("failed writing to starship.toml");
}
