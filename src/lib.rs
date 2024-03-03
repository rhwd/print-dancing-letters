use std::collections::HashMap;
use std::fs;
use std::process::Command;

pub fn run(arg: &String) {
    parse_arg(&arg);
    let output = Command::new("ffmpeg")
        .args(&[
            "-i",
            "characters/a.gif",
            "-i",
            "characters/b.gif",
            "-i",
            "characters/c.gif",
            "-i",
            "characters/d.gif",
            "-filter_complex",
            "[0][1][2][3]xstack=inputs=4:layout=0_0|w0_0|0_h0|w0_h0",
            "outputs/output.gif",
        ])
        .output()
        .expect("Failed to execute ffmpeg");

    if output.status.success() {
        println!("ffmpeg command executed successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error running ffmpeg: {}", stderr);
    }
}

fn parse_arg(arg: &String) {
    let char_paths = parse_chars_json();
    let mut chars: Vec<u8> = Vec::new();
    for byte in arg.bytes() {
        println!("{}", byte);
        chars.push(byte)
    }
}

fn parse_chars_json() -> HashMap<u8, String> {
    let contents = fs::read_to_string("src/chars.json").expect("failed to read file.");
    let json: HashMap<u8, String> = serde_json::from_str(&contents).expect("Failed to parse JSON");
    json
}
