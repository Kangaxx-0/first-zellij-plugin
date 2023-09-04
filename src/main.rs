use std::collections::BTreeMap;

use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    key_strokes: Vec<char>,
    counter: usize,
    render_event: String,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ChangeApplicationState]);
        subscribe(&[EventType::Key, EventType::ModeUpdate]);
        self.counter = 0;
    }

    fn update(&mut self, event: Event) -> bool {
        let mut render = false;
        match event {
            Event::Key(Key::Char(c)) => {
                self.key_strokes.push(c);
                self.render_event = "key char".into();
                render = true;
            }
            Event::ModeUpdate(_) => {
                self.render_event = "Mode update".into();
                render = true;
            }
            _ => {
                self.render_event = "other event".into();
            }
        }
        render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        self.counter += 1;
        println!(
            "\x1b[1;31m last key stroke is \x1b[0m => \x1b[1;32m {:?}\x1b[0m",
            self.key_strokes
        );

        println!(
            "render at {} times due to event {}",
            self.counter, self.render_event
        );
    }
}
