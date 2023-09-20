mod ui;
use ui::tabs::TabUi;
use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    loaded: bool,
    tabs: Vec<TabUi>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ReadApplicationState]);
        subscribe(&[EventType::SessionUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut render = false;
        match event {
            Event::SessionUpdate(session_info) => {
                self.get_tabs(session_info);
                render = true;
                self.loaded = false;
            }
            Event::PermissionRequestResult(_result) => {
                render = true;
            }
            _ => {
                self.loaded = true;
            }
        }
        render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        if !self.loaded {
            println!("float-pane-plugin is loading");
        } else {
            for tab in &self.tabs {
                println!("Tab > {}:", tab.name);
                for pane in &tab.panes {
                    println!("     panes > {}", pane.name);
                }
            }
        }
    }
}

impl State {
    fn get_tabs(&mut self, session: Vec<SessionInfo>) {
        let current_session = session
            .into_iter()
            .find(|session| session.is_current_session)
            .unwrap();
        self.tabs = current_session
            .tabs
            .iter()
            .map(|tab| TabUi::new(tab, &current_session))
            .collect();
    }
}
