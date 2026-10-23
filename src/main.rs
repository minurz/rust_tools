use rdev::{Event, EventType, Key, listen};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

const TIME_SLEEP: u64 = 200;

#[derive(Default)]
struct KeyState {
    alt: bool,
    ctrl: bool,
    last_press: Option<Instant>,
    //double_press_detected: bool,
}

fn callback(event: Event, state: Arc<Mutex<KeyState>>) {
    match event.event_type {
        EventType::KeyPress(key) => {
            match key {
                Key::Alt | Key::AltGr => state.lock().unwrap().alt = true,
                Key::ControlLeft | Key::ControlRight => state.lock().unwrap().ctrl = true,
                Key::Slash => {
                    if state.lock().unwrap().alt && state.lock().unwrap().ctrl {
                        //println!("{:?}, {:?} + slash!", state.alt, state.ctrl)
                        let now = Instant::now();
                        let threshold = Duration::from_millis(TIME_SLEEP);
                        let ready = state.lock().unwrap().last_press
                            .is_none_or(|last| now.duration_since(last) >= threshold); // true if time since last press >= 200ms (or None), otherwise false
                        let state_clone = Arc::clone(&state);
                        if !ready {
                            println!("Double press detected!");
                            state.lock().unwrap().last_press = None;
                        }
                        else {
                            std::thread::spawn(move || {
                                std::thread::sleep(threshold);
                                if state_clone.lock().unwrap().last_press == Some(now) {
                                    println!("Single press detected!");
                                }
                            });
                            state.lock().unwrap().last_press = Some(now);
                        }
                        
                        // println!("lock detected! {:?}", state.lock().unwrap().last_press);
                    }
                }
                _ => {}
            }
        }
        EventType::KeyRelease(key) => {
            match key {
                Key::Alt | Key::AltGr => state.lock().unwrap().alt = false,
                Key::ControlLeft | Key::ControlRight => state.lock().unwrap().ctrl = false,
                _ => {}
            }
        }
        _ => {}
    }
}

fn main() {
    let state = Arc::new(Mutex::new(KeyState::default()));
    listen(move |event| callback(event, Arc::clone(&state)))
    .unwrap();
}