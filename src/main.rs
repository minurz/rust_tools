#![windows_subsystem = "windows"]

use rdev::{grab, Event, EventType, Key};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

mod nircmd;
use nircmd::tooltip::show_tooltip;
use nircmd::nircmd::set_mic_volume;

const TIME_SLEEP: u64 = 200;

#[derive(Default)]
struct KeyState {
    lalt: bool,
    ralt: bool,
    ctrl: bool,
    last_press: Option<Instant>,
}

impl KeyState {
    fn alt(&self) -> bool {
        self.lalt || self.ralt
    }
}

fn callback(event: Event, state: Arc<Mutex<KeyState>>) -> Option<Event> { // Callback function to handle key events
    match event.event_type {
        EventType::KeyPress(key) => {
            match key {
                // Bind Keys
                Key::Alt => state.lock().unwrap().lalt = true,
                Key::AltGr => state.lock().unwrap().ralt = true,
                Key::ControlLeft | Key::ControlRight => state.lock().unwrap().ctrl = true,
                // Trigger Keys
                Key::Tab => {
                    let s = state.lock().unwrap();
                    if s.lalt {
                        // println!("LAlt + Tab detected!");
                        return None // prevent LAlt + Tab from being sent to the OS
                    }
                }
                Key::Slash => {
                    let mut s = state.lock().unwrap();
                    if s.alt() && s.ctrl {
                        //println!("{:?}, {:?} + slash!", state.alt, state.ctrl)
                        let now = Instant::now();
                        let threshold = Duration::from_millis(TIME_SLEEP);

                        let state_clone = Arc::clone(&state);
                        let is_double = !s.last_press.is_none_or(|last| now.duration_since(last) >= threshold); // Check if the last press was within the time threshold
                        s.last_press = Some(now);

                        if is_double {
                            show_tooltip("MIC 15"); // println!("MIC 15!");
                            set_mic_volume(9830);
                            s.last_press = None;
                            //drop(s);
                        }
                        else {
                            drop(s);
                            std::thread::spawn(move || {
                                std::thread::sleep(threshold);
                                if state_clone.lock().unwrap().last_press == Some(now) {
                                    show_tooltip("MIC 71"); // println!("MIC 71!");
                                    set_mic_volume(46530);
                                }
                            });
                        }
                        
                        
                        // println!("lock detected! {:?}", state.lock().unwrap().last_press);
                    }
                }
                _ => {}
            }
        }
        EventType::KeyRelease(key) => {
            match key {
                Key::Alt => state.lock().unwrap().lalt = false,
                Key::AltGr => state.lock().unwrap().ralt = false,
                Key::ControlLeft | Key::ControlRight => state.lock().unwrap().ctrl = false,
                _ => {}
            }
        }
        _ => {}
    }
    Some(event)
}

fn main() {
    let state = Arc::new(Mutex::new(KeyState::default())); // Create a shared state for key states
    if let Err(error) = grab(move |event| callback(event, Arc::clone(&state))) {
        eprintln!("Grab failed: {:?}", error);
    }
}