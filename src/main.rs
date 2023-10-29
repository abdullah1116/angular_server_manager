use inline_colorization::*;
use std::env;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let error_message = "ERROR in TypeError: Cannot read property 'length' of undefined"; // Customize this error message as needed

    let args = {
        let args_vec: Vec<String> = env::args().skip(1).collect();
        args_vec.join(" ")
    };

    loop {
        let mut error_detected = false;

        let mut cmd = Command::new("bash");
        let mut child = cmd
            .arg("-c")
            .arg(args.clone())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut reader = BufReader::new(child.stderr.take().unwrap());
        let mut line = String::new();

        while let Ok(n) = reader.read_line(&mut line).await {
            if n == 0 {
                break;
            }

            print!("{}", line);

            if line.contains(error_message) {
                error_detected = true;

                println!();
                println!("{color_yellow}Error detected{color_reset}");
                println!();

                child.kill().await?;

                break;
            }

            line.clear();
        }

        let cmd_status = child.wait().await?;

        if !error_detected {
            std::process::exit(cmd_status.code().ok_or(1).unwrap());
        }

        time::sleep(Duration::from_secs(5)).await;
    }
}
