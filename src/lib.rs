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
    let mut chars: Vec<u8> = Vec::new();
    for byte in arg.bytes() {
        println!("{}", byte);
        chars.push(byte)
    }
}
