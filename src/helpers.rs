use dirs::home_dir;
use palette::Srgb;

pub fn get_home_path() -> String {
    home_dir()
        .expect("failed to get user home directory")
        .to_str()
        .expect("failed to parse home directory")
        .to_owned()
}

pub fn format_hash(color: Srgb<u8>) -> String {
    format!("#{:02X}{:02X}{:02X}", color.red, color.green, color.blue)
}

pub fn format_hex(color: Srgb<u8>) -> String {
    format!("0x{:02X}{:02X}{:02X}", color.red, color.green, color.blue)
}

pub fn into_color(kde_out: Vec<u8>) -> Srgb<u8> {
    let mut kde_string = std::str::from_utf8(kde_out.as_slice())
        .expect("unsupported value received from kreadconfig5 util")
        .to_owned();

    kde_string.retain(|c| !c.is_whitespace());

    let mut color_split = kde_string.split(',');
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

    color
}
