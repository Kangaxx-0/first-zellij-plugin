use zellij_tile::prelude::{PaletteColor, PaneInfo, TabInfo};

use super::color::Colors;
use super::tabs::TabUi;
const MAX_PATH_LEN: usize = 20;

#[derive(Default, Debug, Clone)]
pub struct PaneUi {
    pub name: String,
    pub pane_id: u32,
    pub is_plugin: bool,
    pub is_focused: bool,
    pub pane_x: usize,
    pub pane_content_x: usize,
    pub pane_y: usize,
    pub pane_content_y: usize,
    pub pane_rows: usize,
    pub pane_content_rows: usize,
    pub pane_columns: usize,
    pub pane_content_columns: usize,
    pub parent_tab: TabUi,
}

impl PaneUi {
    pub fn new(pane: &PaneInfo, tab: &TabInfo) -> Self {
        Self {
            name: pane.title.clone(),
            pane_id: pane.id,
            is_plugin: pane.is_plugin,
            is_focused: pane.is_focused,
            pane_x: pane.pane_x,
            pane_content_x: pane.pane_content_x,
            pane_y: pane.pane_y,
            pane_content_y: pane.pane_content_y,
            pane_rows: pane.pane_rows,
            pane_content_rows: pane.pane_content_rows,
            pane_columns: pane.pane_columns,
            pane_content_columns: pane.pane_content_columns,
            parent_tab: TabUi::new(tab),
        }
    }
}

pub struct DrawPaneLine<'p> {
    pub pane: PaneUi,
    pub selected_resize: &'p Option<PaneUi>,
    pub is_current: Option<usize>,
    pub colors: Colors,
    pub line: String,
}

impl<'p> DrawPaneLine<'p> {
    pub fn new(
        pane: PaneUi,
        selected_resize: &'p Option<PaneUi>,
        is_current: Option<usize>,
        colors: Colors,
    ) -> Self {
        Self {
            pane,
            selected_resize,
            is_current,
            colors,
            line: "".into(),
        }
    }

    pub fn draw(&mut self, index: usize) {
        let focused_text = if self.pane.is_focused {
            self.colors.blue("Yes")
        } else {
            self.colors.orange("No")
        };
        let selected_indicator = if let Some(selected) = self.is_current {
            if selected == index {
                self.colors.green(">")
            } else {
                self.colors.green(" ")
            }
        } else {
            self.colors.green(" ")
        };
        let index_color = self.colors.magenta(&(index.to_string()));
        let pane_id = self.colors.magenta(&(self.pane.pane_id.to_string()));
        let focus = self.colors.magenta("Focus");
        let line = format!(
            "{}{}: [{}] {:<20} (ID: {}, {}: {})",
            selected_indicator,
            index_color,
            self.pane.parent_tab.name,
            middle_truncate(&self.pane.name),
            pane_id,
            focus,
            focused_text
        );

        self.line.push_str(&line);

        if let Some(selected) = self.is_current {
            if selected == index {
                self.make_highlight();
            } else {
                self.make_normal();
            }
        }
    }

    // set the background color to green
    fn make_highlight(&mut self) {
        if self.is_current.is_some() {
            match self.colors.palette.bg {
                PaletteColor::EightBit(byte) => {
                    self.line = format!("\x1b[48;5;{byte}m\x1b[K\r\x1b[48;5;{byte}m{}", self.line);
                }
                PaletteColor::Rgb((r, g, b)) => {
                    self.line = format!(
                        "\x1b[48;2;{};{};{}m\x1b[K\r\x1b[48;2;{};{};{}m{}",
                        r, g, b, r, g, b, self.line
                    );
                }
            }
        }
    }

    // reset the background color to default
    fn make_normal(&mut self) {
        self.line = format!("\x1b[49m{}", self.line);
    }
}
fn middle_truncate(s: &str) -> String {
    if s.len() > MAX_PATH_LEN {
        let part_len = (MAX_PATH_LEN - 1) / 2;
        format!("{}~{}", &s[0..part_len], &s[s.len() - part_len..])
    } else {
        s.to_string()
    }
}
