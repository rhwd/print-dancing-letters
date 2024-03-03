use std::collections::HashMap;
use std::fs;
use std::process::Command;

pub fn run(arg: &String) {
    let char_commands = parse_arg(&arg);
    let ffmpeg_command = vec![
        char_commands,
        vec![
            "-filter_complex".to_string(),
            "[0][1][2][3]hstack=inputs=4,split[x][y];[x]palettegen[pal];[y][pal]paletteuse"
                .to_string(),
            "outputs/output.gif".to_string(),
        ],
    ]
    .concat();
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

fn parse_arg(arg: &String) -> Vec<String> {
    let char_paths = parse_chars_json();
    let mut chars: Vec<String> = Vec::new();
    for byte in arg.bytes() {
        chars.push("-i".to_string());
        chars.push(char_paths[&byte].clone())
    }
    chars
}

fn parse_chars_json() -> HashMap<u8, String> {
    let contents = fs::read_to_string("src/chars.json").expect("failed to read file.");
    let json: HashMap<u8, String> = serde_json::from_str(&contents).expect("Failed to parse JSON");
    json
}
