use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        println!("hello world!");
    }
    fn update(&mut self, event: Event) -> bool {
        true
    } // return true if it should render
    fn render(&mut self, rows: usize, cols: usize) {
        println!("hello world!");
    }
}
