use rdev::{listen, Event, EventType, Key};
use std::time::{Duration, Instant};

const TIME_SLEEP: u64 = 200;

#[derive(Default)]
struct KeyState {
    alt: bool,
    ctrl: bool,
    last_press: Option<Instant>,
}

fn callback(event: Event, state: &mut KeyState) {
    match event.event_type {
        EventType::KeyPress(key) => {
            match key {
                Key::Alt | Key::AltGr => state.alt = true,
                Key::ControlLeft | Key::ControlRight => state.ctrl = true,
                Key::Slash => {
                    if state.alt && state.ctrl {
                        //println!("{:?}, {:?} + slash!", state.alt, state.ctrl)
                        let now = Instant::now();
                        let threshold = Duration::from_millis(TIME_SLEEP);
                        let ready = state.last_press
                            .is_none_or(|last| now.duration_since(last) >= threshold); // true if time since last press >= 200ms (or None), otherwise false

                        if ready {
                            println!("Regular!!");
                        }
                        else {
                            println!("Quiet!");
                        }
                        state.last_press = Some(now);
                    }
                }
                _ => {}
            }
        }
        EventType::KeyRelease(key) => {
            match key {
                Key::Alt | Key::AltGr => state.alt = false,
                Key::ControlLeft | Key::ControlRight => state.ctrl = false,
                _ => {}
            }
        }
        _ => {}
    }
}

fn main() {
    let mut state = KeyState::default();
    listen(move |event| callback(event, &mut state))
    .unwrap();
}