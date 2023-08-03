use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Main {
    pub(crate) colors: Colors,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Colors {
    pub(crate) primary: Primary,
    pub(crate) cursor: Cursor,
    pub(crate) normal: Palette,
    pub(crate) bright: Palette,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Primary {
    pub(crate) background: Color,
    pub(crate) foreground: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Cursor {
    pub(crate) text: Color,
    pub(crate) cursor: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Palette {
    pub(crate) black: Color,
    pub(crate) red: Color,
    pub(crate) green: Color,
    pub(crate) yellow: Color,
    pub(crate) blue: Color,
    pub(crate) magenta: Color,
    pub(crate) cyan: Color,
    pub(crate) white: Color,
}

pub(crate) type Color = String;
