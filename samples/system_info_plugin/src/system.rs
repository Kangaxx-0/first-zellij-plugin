pub(crate) mod system_event;

use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt};
use zellij_tile::prelude::*;

use self::system_event::SystemEvent;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SystemInternalState {
    mem_usage: u64,
}

impl<'de> ZellijWorker<'de> for SystemInternalState {
    fn on_message(&mut self, message: String, payload: String) {
        match message.as_ref() {
            "search ram usage" => {
                let sys = get_sys();
                let total_ram = sys.total_memory();
                let used_ram = sys.used_memory();
                self.mem_usage = (used_ram as f64 / total_ram as f64 * 100.0) as u64;
                post_message_to_plugin(PluginMessage {
                    name: serde_json::to_string(&SystemEvent::MemoryUsage).unwrap(),
                    payload,
                    worker_name: None,
                });
            }
            _ => {}
        }
    }
}

fn get_sys() -> System {
    let mut sys = System::new();
    sys.refresh_all();
    sys
}
