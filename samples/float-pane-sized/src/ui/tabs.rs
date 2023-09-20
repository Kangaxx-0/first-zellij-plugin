use super::panes::PaneUi;
use zellij_tile::prelude::{SessionInfo, TabInfo};

#[derive(Default, Debug, Clone)]
pub struct TabUi {
    pub name: String,
    pub tab_id: usize,
    pub is_active: bool,
    pub panes: Vec<PaneUi>,
}

impl TabUi {
    pub fn new(tab: &TabInfo, current_session: &SessionInfo) -> Self {
        let panes = current_session
            .panes
            .panes
            .get(&tab.position)
            .map(|panes| {
                panes
                    .iter()
                    .filter_map(|pane| {
                        if pane.is_floating {
                            Some(PaneUi::new(pane))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        Self {
            name: tab.name.clone(),
            tab_id: tab.position,
            is_active: tab.active,
            panes,
        }
    }
}
