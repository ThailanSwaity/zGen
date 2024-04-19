use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    style, terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Stdout, Write};
use std::time;

use json::JsonValue;

fn main() {
    terminal::enable_raw_mode().unwrap();

    loop {
        let quotes = get_quote_cache();

        for index in 0..quotes.len() {
            draw(
                &quotes[index]["q"].to_string(),
                &quotes[index]["a"].to_string(),
            );

            for _ in 0..6000 {
                if blocking_poll_for_terminal_resize(time::Duration::from_millis(10)) {
                    draw(
                        &quotes[index]["q"].to_string(),
                        &quotes[index]["a"].to_string(),
                    );
                }
                if blocking_poll_for_cntrlc(time::Duration::from_millis(10)) {
                    terminal::disable_raw_mode().unwrap();
                    return;
                }
            }
        }
    }
}

fn draw(quote: &str, author: &str) {
    let mut stdout = stdout();

    let width = terminal::window_size().unwrap().columns;
    let height = terminal::window_size().unwrap().rows;

    let box_height = 10;

    let padding = 5;

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    let box_width = std::cmp::min((quote.len()) as u16, width - 25);

    queue_text_with_wrap(
        &mut stdout,
        quote,
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
        .queue(style::Print(&author))
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

fn blocking_poll_for_terminal_resize(delay: time::Duration) -> bool {
    if poll(delay).unwrap() {
        return match read().unwrap() {
            Event::Resize(width, height) => true,
            _ => false,
        };
    }
    false
}

fn blocking_poll_for_cntrlc(delay: time::Duration) -> bool {
    if poll(delay).unwrap() {
        return match read().unwrap() {
            Event::Key(event) => event.code == KeyCode::Char('c') && event.modifiers.bits() == 2,
            _ => false,
        };
    }
    false
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
