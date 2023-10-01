use zellij_tile::prelude::{Palette, PaletteColor};

#[derive(Debug, Default, Clone, Copy)]
pub struct Colors {
    pub palette: Palette,
}
impl Colors {
    pub fn new(palette: Palette) -> Self {
        Colors { palette }
    }
    pub fn bold(&self, text: &str) -> String {
        format!("\u{1b}[1m{}\u{1b}[22m", text)
    }

    fn color(&self, color: &PaletteColor, text: &str) -> String {
        match color {
            PaletteColor::EightBit(byte) => {
                format!("\u{1b}[38;5;{};1m{}\u{1b}[39;22m", byte, text)
            }
            PaletteColor::Rgb((r, g, b)) => {
                format!("\u{1b}[38;2;{};{};{};1m{}\u{1b}[39;22m", r, g, b, text)
            }
        }
    }
    pub fn orange(&self, text: &str) -> String {
        self.color(&self.palette.orange, text)
    }

    pub fn green(&self, text: &str) -> String {
        self.color(&self.palette.green, text)
    }

    pub fn red(&self, text: &str) -> String {
        self.color(&self.palette.red, text)
    }

    pub fn cyan(&self, text: &str) -> String {
        self.color(&self.palette.cyan, text)
    }

    pub fn magenta(&self, text: &str) -> String {
        self.color(&self.palette.magenta, text)
    }

    pub fn blue(&self, text: &str) -> String {
        self.color(&self.palette.blue, text)
    }

    pub fn pink(&self, text: &str) -> String {
        self.color(&self.palette.pink, text)
    }
}
