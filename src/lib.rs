use std::process::Command;

pub fn run() {
    let output = Command::new("ffmpeg")
        .args(&[
            "-i",
            "characters/a.gif",
            "-i",
            "characters/a.gif",
            "-i",
            "characters/a.gif",
            "-filter_complex",
            "[0][1][2]hstack=inputs=3,split[x][y];[x]palettegen[pal];[y][pal]paletteuse",
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
