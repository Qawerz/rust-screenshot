#![warn(clippy::all, clippy::pedantic)]

use std::{env, fs, path};
mod colors;
use chrono::{DateTime, Utc};
use colors::{blue_text_and_msg, red_text_and_msg};
use rdev::{Event, EventType, Key, grab};
use screenshots::Screen;

const TARGET_DIR: &str = "screens";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path = env::current_dir()?;
    path.push(&screens_dir);

    fs::create_dir_all(path)?;

    if let Err(error) = grab(move |e| callback(e, &screens_dir)) {
        let err = format!("{:?}", error);
        println!("{}", red_text_and_msg("ERROR", &err));
    };

    Ok(())
}

fn callback(event: Event, screens_dir: &String) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::AltGr) => {
            make_screen(screens_dir);
            None
        }
        _ => Some(event),
    }
}

fn make_screen(screens_dir: &String) {
    let screens = Screen::all().unwrap();

    for screen in screens {
        let image = screen.capture().unwrap();

        let now: DateTime<Utc> = Utc::now();

        image
            .save(format!("{}/{}.png", screens_dir, now.format("%d-%m-%Y_%H_%M_%S_%f")))
            .unwrap();
        let message = format!("Скриншот сохранен в {}\\{}\\{}.png", env::current_dir().unwrap().into_os_string().into_string().unwrap(), screens_dir, now.format("%d-%m-%Y_%H_%M_%S_%f"));
        println!("{}", blue_text_and_msg("INFO", &message))
    }
    
}
