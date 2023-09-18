mod system;

use std::collections::BTreeMap;
use zellij_tile::prelude::*;

use system::system_event::SystemEvent;
use system::SystemInternalState;
use zellij_tile::shim::plugin_api::plugin_permission::ProtobufPermissionType;

#[derive(Default)]
struct State {
    mem: u64,
}

register_plugin!(State);
register_worker!(SystemInternalState, system_state_worker, SYSTEM_STATE);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        // request_permission(&[PermissionType::read])
        set_selectable(false);
        post_message_to(PluginMessage {
            name: "search ram usage".into(),
            payload: String::new(),
            worker_name: Some("system_state".into()),
        });
        subscribe(&[EventType::SessionUpdate, EventType::CustomMessage]);
    }
    fn update(&mut self, event: Event) -> bool {
        let mut render = false;
        match event {
            Event::SessionUpdate(_) => {
                println!("active tab changed");
                render = true;
            }
            Event::CustomMessage(message, payload) => {
                match serde_json::from_str::<SystemEvent>(&message) {
                    Ok(SystemEvent::MemoryUsage) => {
                        self.mem = payload.parse::<u64>().unwrap();
                        println!("memory usage: {}", payload);
                        render = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        render
    } // return true if it should render
    fn render(&mut self, _rows: usize, _cols: usize) {
        println!("hello world! the memory usage is {}", self.mem);
    }
}
