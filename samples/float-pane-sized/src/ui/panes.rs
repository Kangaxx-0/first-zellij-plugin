use zellij_tile::prelude::PaneInfo;

#[derive(Default, Debug, Clone)]
pub struct PaneUi {
    pub name: String,
    pub pane_id: u32,
    pub is_plugin: bool,
    pub is_focused: bool,
}

impl PaneUi {
    pub fn new(pane: &PaneInfo) -> Self {
        Self {
            name: pane.title.clone(),
            pane_id: pane.id,
            is_plugin: pane.is_plugin,
            is_focused: pane.is_focused,
        }
    }
}
