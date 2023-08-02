use kde_color::connections::starship;
use kde_color::{get_accent_color, get_shades};

fn main() {
    let color =
        get_accent_color().unwrap_or_else(|e| panic!("error getting KDE accent color: {e:?}"));

    let shades = get_shades(color, 6);

    starship::write_colors(shades);
}
