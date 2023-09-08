use zellij_tile::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

const INIT_PATH: &str = "/host";

#[derive(Deserialize, Serialize, Debug, Default)]
struct FilesWorker {
    files: Vec<String>,
}

impl FilesWorker {
    fn search(&mut self, path: String) {
        let mut files = Vec::new();
        let paths = std::fs::read_dir(path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            files.push(file_name.to_string());
        }
        self.files = files;

        // Simulation of a long running task for 10 seconds
        let start_time = Instant::now();
        let two_seconds = Duration::from_secs(10);
        loop {
            if Instant::now() - start_time >= two_seconds {
                break;
            }
        }

        post_message_to_plugin(PluginMessage {
            name: String::from("done"),
            payload: serde_json::to_string(&self.files).unwrap(),
            ..Default::default()
        });
    }
}

impl<'de> ZellijWorker<'de> for FilesWorker {
    fn on_message(&mut self, message: String, payload: String) {
        if message == *"file_search" {
            self.search(payload);
        } else {
            println!("Unknown message: {}", message);
        }
    }
}

#[derive(Default)]
struct ModuleState {
    files: Vec<String>,
    render_counter: usize,
    last_render_event: String,
}

register_plugin!(ModuleState);
register_worker!(FilesWorker, file_search_worker, FILE_SEARCH);

impl ZellijPlugin for ModuleState {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        self.render_counter = 0;
        subscribe(&[
            EventType::CustomMessage,
            EventType::Key,
            EventType::SessionUpdate,
        ]);
        request_permission(&[PermissionType::ReadApplicationState]);
        post_message_to(PluginMessage {
            name: String::from("file_search"),
            payload: String::from(INIT_PATH),
            worker_name: Some("file_search".into()),
        })
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::CustomMessage(name, payload) => {
                if name == *"done" {
                    let files: Vec<String> = serde_json::from_str(&payload).unwrap();
                    self.files = files;
                    should_render = true;
                }
            }
            Event::Key(_) => {
                self.last_render_event = "key".into();
                should_render = true;
            }
            Event::SessionUpdate(_) => {
                self.last_render_event = "session".into();
                should_render = true;
            }
            _ => {}
        }
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        self.render_counter += 1;

        println!("Render counter: {}", self.render_counter);
        println!("Last render event: {}", self.last_render_event);
        for (idx, file) in self.files.iter().enumerate() {
            println!("# {} : file name{}", idx, file);
        }
    }
}
