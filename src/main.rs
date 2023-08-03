use kde_color::connections::{alacritty, starship};

fn main() {
    // Set Starship Prompt shades
    starship::paint();

    // Set Alacritty color scheme
    alacritty::paint();
}
