use serde::de::value::Error;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::io::{BufReader, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            service,
            username,
            password,
        }
    }
    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }
    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        println!("Enter Service Entry");
        let mut service = String::new();
        io::stdin()
            .read_line(&mut service)
            .expect("failed to read line");
        println!("Enter Username Entry");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("failed to read line");
        println!("Enter Password Entry");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("failed to read line");

        ServiceInfo::new(
            service.trim().to_string(),
            username.to_string(),
            password.to_string(),
        )
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("failed to stringify json")
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("password.json")
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Error writing to file : {}", e);
                } else {
                    println!("successfully written to file")
                }
            }
            Err(e) => eprintln!("Error opening file {}", e),
        }
    }
}
pub fn prompt(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
pub fn read_password_from_file() -> Result<Vec<ServiceInfo>, io::Error> {
    let file = File::open("password.json")?;
    let reader = BufReader::new(file);
    let mut services = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                services.push(service_info);
            }
        }
    }
    Ok(services)
}
