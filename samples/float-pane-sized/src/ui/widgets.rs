use std::io::{self, Write};

use super::color::Colors;
use super::tabs::TabUi;

const MAX_PATH_LEN: usize = 20;

pub fn header(rows: usize, cols: usize, color: Colors) {
    clear_screen();
    let text = color.cyan("Floating Pane Resize Manager");
    let text_length = text.len();

    let padding_each_side = (cols - text_length) / 2;
    let repeated = " ".repeat(padding_each_side);

    print!("{}", repeated);
    println!("{}", text);

    if rows % 2 != text_length % 2 {
        print!(" ");
    }

    let split = "─".repeat(cols);
    println!("{}", split);
    println!();
}

pub fn navigation(row: usize, max_cols: usize, colors: Colors) {
    let is_searching = false;

    let (arrows, navigate) = if is_searching {
        (colors.magenta("<↓↑>"), colors.bold("Navigate"))
    } else {
        (colors.magenta("<←↓↑→>"), colors.bold("Navigate and Expand"))
    };
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
    tabs: &[TabUi],
    selected_pane: Option<usize>,
) {
    let mut index = 1;
    for tab in tabs {
        for pane in &tab.panes {
            let focused_text = if pane.is_focused {
                colors.blue("Yes")
            } else {
                colors.orange("No")
            };

            let selected_indicator = if Some(pane.pane_id as usize) == selected_pane {
                colors.bold(">")
            } else {
                " ".into()
            };

            let index_color = colors.magenta(&(index.to_string()));
            let pane_id = colors.magenta(&(pane.pane_id.to_string()));
            let focus = colors.magenta("Focus");
            println!(
                "{}: [{}] {:<16} (ID: {}, {}: {})",
                index_color,
                tab.name,
                middle_truncate(&pane.name),
                pane_id,
                focus,
                focused_text
            );
            index += 1;
        }
    }
}

pub fn selected_pane(panes: &[&str], selected_pane: usize, width: usize, height: usize) {
    clear_screen();
    for (i, pane) in panes.iter().enumerate() {
        let marker = if i == selected_pane { ">" } else { " " };
        println!("| {} {}: {}                  |", marker, i + 1, pane);
    }
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

fn middle_truncate(s: &str) -> String {
    if s.len() > MAX_PATH_LEN {
        let part_len = (MAX_PATH_LEN - 1) / 2;
        format!("{}~{}", &s[0..part_len], &s[s.len() - part_len..])
    } else {
        s.to_string()
    }
}
