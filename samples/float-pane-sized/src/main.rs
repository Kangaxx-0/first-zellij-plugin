use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    panes: Panes,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[EventType::SessionUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut render = false;
        match event {
            Event::SessionUpdate(session_info) => {
                self.panes = get_panes(session_info);
                render = true;
            }
            _ => {}
        }
        render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {}
}

fn get_panes(session: Vec<SessionInfo>) -> Panes {
    let current_session = session
        .into_iter()
        .find(|session| session.is_current_session)
        .unwrap();
    todo!();
}

#[derive(Default)]
struct Panes {}
