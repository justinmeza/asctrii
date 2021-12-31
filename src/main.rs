// Draws an ASCII art conifer tree using ANSI colors.
// Accepts one argument for the name to include in the message.
// Best run as something like `watch -n 0.5 -p -c -t asctrii $USER`

use ansi_term::Color;
use chrono::{Timelike, Utc};
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify a name!");
        return ();
    }
    let name = &args[1];

    let max = 3;
    let scale = 3;

    // Use `N..=M` to yield all values in the interval [N, M], inclusive!
    for half in 1..=max {
        draw_segment(half * scale, max * scale, half - 1, half)
    }
    draw_base(max, scale);
    draw_message(name);
}

fn draw_segment(half: i8, center: i8, offset: i8, gap: i8) {
    let width = half * 2;

    // Draw segment from top to bottom.
    for y in offset..half {
        // Add any indentation to the beginning of the line.
        let indent = center - half;
        for _ in 0..indent {
            print!(" ");
        }

        // Draw across each line.
        for x in 0..width {
            if x == half - y - 1 {
                print!("{}", Color::Green.paint("╱"));
            } else if x == half + y {
                print!("{}", Color::Green.paint("╲"));
            } else if y == half - 1 {
                if x > half - 1 - gap && x < half + gap {
                    print!(" ");
                } else {
                    print!("{}", Color::Green.paint("_"));
                }
            } else {
                // Within the interior, randomly draw ornaments.
                let mut rng = rand::thread_rng();
                let r: u8 = rng.gen();
                if x > half - y && x < half + y - 1 && r % 8 == 0 {
                    let c: u8 = rng.gen();
                    match c % 4 {
                        0 => print!("{}", Color::Red.paint("◉")),
                        1 => print!("{}", Color::Blue.paint("◉")),
                        2 => print!("{}", Color::Yellow.paint("◉")),
                        3 => print!("{}", Color::Purple.paint("◉")),
                        _ => unreachable!(),
                    }
                } else {
                    print!(" ");
                }
            }
        }
        println!("");
    }
}

fn draw_base(max: i8, scale: i8) {
    // Indent base so that it is centered.
    for _ in 0..max * scale - max + 1 {
        print!(" ");
    }
    print!("{}", Color::Red.paint("┗"));

    for _ in 0..max {
        print!("{}", Color::Red.paint("━"));
    }
    print!("{}", Color::Red.paint("┛"));

    println!("");
}

fn draw_message(name: &str) {
    let text: &str = &format!("Happy Holidays, {}!!!", name);
    print!(" ");

    // Draw a message in alternating colors.
    let now = Utc::now();

    // Starting color is based on second.
    let mut color_flag = now.second() % 2 == 0;

    for c in text.chars() {
        if color_flag {
            print!("{}", Color::Red.paint(c.to_string()));
        } else {
            print!("{}", Color::Green.paint(c.to_string()));
        }

        // Toggle the color flag.
        color_flag ^= true;
    }
}
