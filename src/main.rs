use std::{thread, time::Duration};

const LOGO : &[&str] = &[
    " /$$    /$$ /$$$$$$$$ /$$$$$$   /$$$$$$         /$$$$$$  /$$     /$$ /$$$$$$$  /$$$$$$$$ /$$$$$$$        /$$$$$$$$ /$$$$$$$$  /$$$$$$  /$$      /$$",
    "| $$   | $$|__  $$__//$$__  $$ /$$__  $$       /$$__  $$|  $$   /$$/| $$__  $$| $$_____/| $$__  $$      |__  $$__/| $$_____/ /$$__  $$| $$$    /$$$",
    "| $$   | $$   | $$  | $$  \\__/| $$  \\__/      | $$  \\__/ \\  $$ /$$/ | $$  \\ $$| $$      | $$  \\ $$         | $$   | $$      | $$  \\ $$| $$$$  /$$$$",
    "|  $$ / $$/   | $$  | $$      | $$            | $$        \\  $$$$/  | $$$$$$$ | $$$$$   | $$$$$$$/         | $$   | $$$$$   | $$$$$$$$| $$ $$/$$ $$",
    " \\  $$ $$/    | $$  | $$      | $$            | $$         \\  $$/   | $$__  $$| $$__/   | $$__  $$         | $$   | $$__/   | $$__  $$| $$  $$$| $$",
    "  \\  $$$/     | $$  | $$    $$| $$    $$      | $$    $$    | $$    | $$  \\ $$| $$      | $$  \\ $$         | $$   | $$      | $$  | $$| $$\\  $ | $$",
    "   \\  $/      | $$  |  $$$$$$/|  $$$$$$/      |  $$$$$$/    | $$    | $$$$$$$/| $$$$$$$$| $$  | $$         | $$   | $$$$$$$$| $$  | $$| $$ \\/  | $$",
    "    \\_/       |__/   \\______/  \\______/        \\______/     |__/    |_______/ |________/|__/  |__/         |__/   |________/|__/  |__/|__/     |__/",
];

use termion::color;

enum State {
    Display,
    Glitch,
}

fn display_logo() {
    let height_lines = LOGO.len() as u16;
    let term_size = termion::terminal_size().unwrap();

    for (i, col) in
        ((term_size.1 / 2 - height_lines / 2)..(term_size.1 / 2 + height_lines / 2)).enumerate()
    {
        let line = LOGO[i];

        let row = term_size.0 / 2 - (line.len() / 2) as u16;

        println!(
            "{}{}{}",
            termion::cursor::Goto(row, col),
            color::Fg(color::Green),
            line
        );
    }
}

fn rnd_byte() -> u8 {
    fastrand::u8(..)
}

fn rnd_screen_pos(size: u16) -> u16 {
    fastrand::u16(1..size)
}

fn glitch_logo() {
    let term_size = termion::terminal_size().unwrap();

    for line in LOGO.iter().chain(LOGO.iter()) {
        println!(
            "{}{}{}",
            termion::cursor::Goto(
                rnd_screen_pos(term_size.0),
                rnd_screen_pos(term_size.1)
            ),
            color::Fg(color::Rgb(
                rnd_byte(),
                rnd_byte(),
                rnd_byte(),
            )),
            line
        );
    }
}
fn main() {
    let mut state = State::Glitch;
    let mut tick = 0;

    let tick_time = Duration::from_millis(50);
    let disiplay_ticks = 3000 / 50;
    let glitch_ticks = 500 / 50;

    loop {
        println!("{}", termion::clear::All);

        match state {
            State::Display => display_logo(),
            State::Glitch => glitch_logo(),
        }

        tick += 1;

        match state {
            State::Display => {
                if tick % disiplay_ticks == 0 {
                    state = State::Glitch;
                    tick = 0;
                }
            }
            State::Glitch => {
                if tick % glitch_ticks == 0 {
                    state = State::Display;
                    tick = 0;
                }
            }
        }

        println!("{}", termion::cursor::Goto(1, 1));
        thread::sleep(tick_time);
    }
}
