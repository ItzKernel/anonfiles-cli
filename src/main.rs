use reqwest::blocking::Client;
use serde_json::{from_str, Value};
use std::path::{Path};

pub mod util;

fn pad_cmdname(cmd: &str) -> String {
    return format!("{:<10}", cmd);
}

#[rustfmt::skip]
fn h(command: String) {
    println!("\n- Anonfiles CLI -");
    println!("{} | {} -h, --help", pad_cmdname("Help"), command);
    println!("{} | {} -u, --upload <path>", pad_cmdname("Upload"), command);
    println!("");
}

fn u(path: String) {
    let working_dir = &util::dir::get_working_dir();

    #[cfg(debug_assertions)]
    println!("File: {}", working_dir.join(Path::new(&path)).display());

    let form = reqwest::blocking::multipart::Form::new()
        .file("file", working_dir.join(Path::new(&path)))
        .unwrap();

    let client = Client::new();
    let res = client
        .post("https://api.anonfiles.com/upload")
        .multipart(form)
        .send()
        .expect("Failed to send request.");

    let json: Value = from_str(res.text().unwrap().as_str()).unwrap();
    let status: bool = json["status"].as_bool().unwrap();

    if status == true {
        println!("URL: {}", json["data"]["file"]["url"]["short"].as_str().unwrap());
    } else {
        println!(
            "Anonfiles said: \"{}\"",
            &json["error"]["message"].as_str().unwrap()
        );

        #[cfg(debug_assertions)]
        println!(
            "({}) {}",
            &json["error"]["code"].as_u64().unwrap(),
            &json["error"]["type"].as_str().unwrap()
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return h(args[0].clone());
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            h(args[0].clone());
        }
        "-u" | "--upload" => {
            if args.len() < 3 {
                return h(args[0].clone());
            }

            u(args[2].clone());
        }
        _ => {
            h(args[0].clone());
        }
    };

    println!("");
}
