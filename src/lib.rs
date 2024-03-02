use std::process::Command;

pub fn run() {
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
