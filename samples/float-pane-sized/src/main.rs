mod ui;

use ui::color::Colors;
use ui::tabs::TabUi;
use ui::widgets::{header, listing_panes, navigation};

use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    loaded: bool,
    tabs: Vec<TabUi>,
    selected_pane: Option<usize>,
    colors: Colors,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ReadApplicationState]);
        subscribe(&[
            EventType::SessionUpdate,
            EventType::Key,
            EventType::ModeUpdate,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                self.colors = Colors::new(mode_info.style.colors);
                render = true;
            }

            Event::Key(key) => match key {
                Key::Ctrl(c) => match c {
                    's' => {
                        self.selected_pane = Some(0);
                        render = true;
                    }
                    _ => {}
                },
                _ => {}
            },
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

    fn render(&mut self, rows: usize, cols: usize) {
        header(rows, cols, self.colors);
        listing_panes(rows, cols, self.colors, &self.tabs, self.selected_pane);
        navigation(rows, cols, self.colors);
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

    fn handle_key(&mut self, e: Event) {
        match e {
            Event::Key(key) => match key {
                Key::Ctrl(c) => match c {
                    's' => {
                        self.selected_pane = Some(0);
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}
