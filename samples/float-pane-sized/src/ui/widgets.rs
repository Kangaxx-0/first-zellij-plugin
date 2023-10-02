use std::io::{self, Write};

use super::color::Colors;
use super::panes::{DrawPaneLine, PaneUi};

pub fn compose_ui(
    rows: usize,
    cols: usize,
    colors: Colors,
    panes: Vec<PaneUi>,
    selected_pane: Option<&PaneUi>,
    current_pane_index: Option<usize>,
    new_width: u8,
    new_height: u8,
) {
    clear_screen();
    if let Some(pane) = selected_pane {
        header_resize(rows, cols, colors, pane.pane_id);
        selected_pane_size(&pane, colors);
        set_pane_size(new_width, new_height, colors);
        resize_control(rows, cols, colors);
    } else {
        header_man(rows, cols, colors);
        listing_panes(rows, cols, colors, panes, selected_pane, current_pane_index);
        pane_control(rows, cols, colors);
    }
    io::stdout().flush().unwrap();
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

pub fn header_resize(rows: usize, cols: usize, color: Colors, pane_id: u32) {
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

fn selected_pane_size(pane: &PaneUi, colors: Colors) {
    let width = colors.orange(&pane.pane_rows.to_string());
    let height = colors.orange(&pane.pane_columns.to_string());

    let cell_horizontal_border = "".repeat(32); // Adjust the length as needed
    let cell_vertical_border = "|";

    println!("{}", cell_horizontal_border);
    println!(
        "{} Width: {} | Length: {} {}",
        cell_vertical_border, width, height, cell_vertical_border
    );
    println!("{}", cell_horizontal_border);
}

fn set_pane_size(new_width: u8, new_height: u8, colors: Colors) {
    let new_width = colors.magenta(&new_width.to_string());
    let new_height = colors.magenta(&new_height.to_string());
    let width_text = colors.bold("Enter new width");
    let height_text = colors.bold("Enter new length");
    println!("- {width_text}  -> [{new_width}] percent");
    println!("- {height_text} -> [{new_height}] percent");
}

pub fn pane_control(row: usize, max_cols: usize, colors: Colors) {
    let arrows = colors.magenta("<↓↑>");
    let navigate = colors.bold("Navigate");
    let enter = colors.magenta("<ENTER>");
    let select = colors.bold("Select a pane");
    let esc = colors.magenta("<ESC>");
    let hide = colors.bold("Hide this plugin");

    let split = "─".repeat(max_cols);
    let split_ln_no = row - 1;
    // ANSI escape code to draw a line at the second last line, \x1b[{}H sets the cursor to second last line
    println!("\x1b[{}H{}", split_ln_no, split);
    print!("\u{1b}[m\u{1b}[{row}H{arrows} : {navigate}; {enter} : {select}; {esc} : {hide}");
}

pub fn resize_control(row: usize, max_cols: usize, colors: Colors) {
    let numbers = colors.magenta("<0-99>");
    let size = colors.bold("Set size");
    let enter = colors.magenta("<ENTER>");
    let confirm = colors.bold("Confirm a size");
    let select = colors.magenta("<Ctrl+S>");
    let submit = colors.bold("Submit");
    let reset = colors.magenta("<Ctrl+R>");
    let reset_size = colors.bold("Reset size");
    let esc = colors.magenta("<ESC>");
    let cancel = colors.bold("Cancel");

    let split = "─".repeat(max_cols);
    let split_ln_no = row - 1;
    // ANSI escape code to draw a line at the second last line, \x1b[{}H sets the cursor to second last line
    println!("\x1b[{}H{}", split_ln_no, split);
    print!("\u{1b}[m\u{1b}[{row}H{numbers} : {size}; {enter} : {confirm}; {select} : {submit}; {reset} : {reset_size};  {esc} : {cancel}");
}

pub fn listing_panes(
    row: usize,
    max_cols: usize,
    colors: Colors,
    panes: Vec<PaneUi>,
    selected_pane: Option<&PaneUi>,
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
