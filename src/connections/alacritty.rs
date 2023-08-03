use std::{fs, io::Write};

use crate::{
    get_color,
    helpers::{format_hex, get_home_path},
};

mod schema;

fn get_config_path() -> String {
    get_home_path() + "/.config/alacritty/palette.yml"
}

pub fn paint() {
    let current_palette = fs::read_to_string(get_config_path()).unwrap();

    let mut current_palette: schema::Main = serde_yaml::from_str(&current_palette).unwrap();

    current_palette.colors.primary.background =
        format_hex(get_color("Colors:Window".to_owned(), "BackgroundNormal".to_owned()).unwrap());

    current_palette.colors.primary.foreground =
        format_hex(get_color("Colors:Window".to_owned(), "ForegroundNormal".to_owned()).unwrap());

    current_palette.colors.cursor.cursor =
        format_hex(get_color("Colors:Window".to_owned(), "ForegroundVisited".to_owned()).unwrap());

    current_palette.colors.cursor.text =
        format_hex(get_color("Colors:Window".to_owned(), "BackgroundNormal".to_owned()).unwrap());

    let yaml = serde_yaml::to_string(&current_palette).expect("error serializing palette to yaml");
    fs::File::create(get_config_path())
        .expect("cannot open palette.yaml for writing")
        .write_all(yaml.as_bytes())
        .expect("error writing to palette.yaml");
}
