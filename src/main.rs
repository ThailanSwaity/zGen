use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};
use std::io::{stdout, Stdout, Write};
use std::{thread, time};

use json::JsonValue;

fn main() {
    let mut stdout = stdout();

    let mut width = terminal::window_size().unwrap().columns;
    let mut height = terminal::window_size().unwrap().rows;

    let mut box_width;
    let mut box_height = 10;

    terminal::enable_raw_mode().unwrap();

    let quotes = get_quote_cache();
    let padding = 5;

    for index in 0..quotes.len() {
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        box_width = std::cmp::min((quotes[index]["q"].to_string().len()) as u16, width - 25);

        queue_text_with_wrap(
            &mut stdout,
            &quotes[index]["q"].to_string(),
            (width / 2) - (box_width / 2) + padding,
            height / 2 - 1,
            box_width - (2 * padding),
        );

        // Print quote author
        stdout
            .queue(cursor::MoveTo(
                width / 2,
                (height / 2) + (box_height / 2) - 2,
            ))
            .unwrap()
            .queue(style::Print(&quotes[index]["a"]))
            .unwrap();

        queue_box(
            &mut stdout,
            (width / 2) - (box_width / 2),
            (height / 2) - (box_height / 2),
            box_width,
            box_height,
        );

        stdout.queue(cursor::MoveTo(0, height - 1)).unwrap();

        stdout.flush().unwrap();

        thread::sleep(time::Duration::from_secs(10));
    }

    terminal::disable_raw_mode().unwrap();
}

fn get_quote_cache() -> JsonValue {
    let resp = reqwest::blocking::get("https://zenquotes.io/api/quotes/")
        .unwrap()
        .text()
        .unwrap();
    json::parse(&resp).unwrap()
}

fn queue_box(stdout: &mut Stdout, x: u16, y: u16, width: u16, height: u16) {
    for dy in 0..height {
        for dx in 0..width {
            if dy == 0 && dx == 0 {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))
                    .unwrap()
                    .queue(style::Print('╔'))
                    .unwrap();
            } else if (dy == 0 || dy == height - 1) && (dx != 0 && dx != width - 1) {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))
                    .unwrap()
                    .queue(style::Print('═'))
                    .unwrap();
            } else if dy == 0 && dx == width - 1 {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))
                    .unwrap()
                    .queue(style::Print('╗'))
                    .unwrap();
            } else if (dx == 0 || dx == width - 1) && (dy != 0 && dy != height - 1) {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))
                    .unwrap()
                    .queue(style::Print('║'))
                    .unwrap();
            } else if dy == height - 1 && dx == 0 {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))
                    .unwrap()
                    .queue(style::Print('╚'))
                    .unwrap();
            } else if dy == height - 1 && dx == width - 1 {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))
                    .unwrap()
                    .queue(style::Print('╝'))
                    .unwrap();
            }
        }
    }
}

fn queue_text_with_wrap(stdout: &mut Stdout, text: &str, x: u16, y: u16, width: u16) {
    let mut dy = 0;
    let mut dx = 0;
    for word in text.split_whitespace() {
        if dx + word.len() as u16 >= width {
            dy += 1;
            dx = 0;
        }
        stdout
            .queue(cursor::MoveTo(x + dx, y + dy))
            .unwrap()
            .queue(style::Print(&word))
            .unwrap();
        dx += word.len() as u16 + 1;
    }
}
