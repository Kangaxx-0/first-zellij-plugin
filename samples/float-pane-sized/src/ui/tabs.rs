use zellij_tile::prelude::TabInfo;

#[derive(Default, Debug, Clone)]
pub struct TabUi {
    pub name: String,
    pub tab_id: usize,
    pub is_active: bool,
}

impl TabUi {
    pub fn new(tab: &TabInfo) -> Self {
        Self {
            name: tab.name.clone(),
            tab_id: tab.position,
            is_active: tab.active,
        }
    }
}
