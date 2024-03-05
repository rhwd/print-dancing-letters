use std::collections::HashMap;
use std::fs;
use std::process::Command;

pub fn run(input: &String, file_name: &String) {
    let (chars, size) = parse_input(&input);
    let ffmpeg_command = generate_ffmpeg_command(chars, size, file_name);

    let output = Command::new("ffmpeg")
        .args(&ffmpeg_command)
        .output()
        .expect("Failed to execute ffmpeg");

    if output.status.success() {
        println!("ffmpeg command executed successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error running ffmpeg: {}", stderr);
    }
}

fn parse_input(input: &String) -> (Vec<String>, usize) {
    let char_paths = parse_chars_json();
    let mut chars: Vec<String> = Vec::new();
    for byte in input.bytes() {
        chars.push("-i".to_string());
        chars.push(char_paths[&byte].clone())
    }
    (chars, input.bytes().len())
}

fn parse_chars_json() -> HashMap<u8, String> {
    let contents = fs::read_to_string("src/chars.json").expect("failed to read file.");
    let json: HashMap<u8, String> = serde_json::from_str(&contents).expect("Failed to parse JSON");
    json
}

fn generate_ffmpeg_command(chars: Vec<String>, size: usize, file_name: &String) -> Vec<String> {
    let filters: String = format!(
        "hstack=inputs={},split[x][y];[x]palettegen[pal];[y][pal]paletteuse",
        size
    );
    let file_name_with_extension = format!("{}.gif", file_name);
    let ffmpeg_command: Vec<String> = vec![
        chars,
        vec![
            "-filter_complex".to_string(),
            filters,
            file_name_with_extension,
        ],
    ]
    .concat();
    ffmpeg_command
}
