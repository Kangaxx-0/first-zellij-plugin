use std::io::{self, Write};

use super::color::Colors;
use super::panes::{DrawPaneLine, PaneUi};

pub fn compose_ui(
    rows: usize,
    cols: usize,
    colors: Colors,
    panes: Vec<PaneUi>,
    selected_pane: Option<usize>,
    current_pane_index: Option<usize>,
) {
    clear_screen();
    if let Some(pane_id) = selected_pane {
        header_resize(rows, cols, colors, pane_id);
        println!("TODO: ");
    } else {
        header_man(rows, cols, colors);
        listing_panes(rows, cols, colors, panes, selected_pane, current_pane_index);
        pane_control(rows, cols, colors);
    }
}

pub fn header_man(rows: usize, cols: usize, color: Colors) {
    let text = color.cyan("Floating Pane Manager");
    let text_length = text.len();

    let padding_each_side = (cols - text_length) / 2;
    let repeated = " ".repeat(padding_each_side);

    print!("{}", repeated);
    println!("{}", text);

    if rows % 2 != text_length % 2 {
        print!(" ");
    }

    let split = "-".repeat(cols - 1);
    println!("{}", split);
}

pub fn header_resize(rows: usize, cols: usize, color: Colors, pane_id: usize) {
    let header = format!("Resize: Pane by index - {}", pane_id);
    let head = color.cyan(&header);
    let text_length = head.len();

    let padding_each_side = (cols - text_length) / 2;
    let repeated = " ".repeat(padding_each_side);

    print!("{}", repeated);
    println!("{}", head);

    if rows % 2 != text_length % 2 {
        print!(" ");
    }

    let split = "-".repeat(cols - 1);
    println!("{}", split);
}

pub fn pane_control(row: usize, max_cols: usize, colors: Colors) {
    let arrows = colors.magenta("<↓↑>");
    let enter = colors.magenta("<ENTER>");
    let esc = colors.magenta("<ESC>");

    let split = "─".repeat(max_cols);
    let split_ln_no = row - 1;
    // ANSI escape code to draw a line at the second last line, \x1b[{}H sets the cursor to second last line
    println!("\x1b[{}H{}", split_ln_no, split);
    print!("\u{1b}[m\u{1b}[{row}H{arrows}/{enter}/{esc}");

    io::stdout().flush().unwrap();
}

pub fn listing_panes(
    row: usize,
    max_cols: usize,
    colors: Colors,
    panes: Vec<PaneUi>,
    selected_pane: Option<usize>,
    current_pane_index: Option<usize>,
) {
    let mut index = 1;
    for pane in panes {
        let mut new_line = DrawPaneLine::new(pane, selected_pane, current_pane_index, colors);
        new_line.draw(index);
        println!("{}", new_line.line);
        index += 1;
    }
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}
