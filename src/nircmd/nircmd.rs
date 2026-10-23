use std::process::Command;

pub fn set_mic_volume(volume: u32) {
    // let command = format!("setsysvolume {} \"default_record\"", &volume.to_string());
    Command::new("C:\\Windows\\nircmd.exe")
        .args(["setsysvolume", &volume.to_string(), "default_record"])
        .output()
        .expect("Failed to execute command");
    // println!("MIC {}", volume);
    // println!("{}", command.split_whitespace().collect::<Vec<_>>().join(" "));
}