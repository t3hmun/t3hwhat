use std::process::Command;

fn main() {
    exe("lsb_release -a", "Linux disro os version information.");
}

fn exe(cmd: &str, explanation: &str) {
    println!("> {explanation}");
    println!("$ {cmd}");
    let output = Command::new("sh").arg("-c").arg(cmd).output();
    let out_vec = match output {
        Ok(output) => output.stdout,
        Err(err) => {
            println!("Execute command failed:\n{err}");
            return;
        }
    };

    let text = String::from_utf8(out_vec);
    match text {
        Ok(text) => {
            println!("{}", text);
        }
        Err(err) => {
            println!("Parse output failed:\n{err}");
            return;
        }
    };
}
