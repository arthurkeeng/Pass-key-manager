mod pass_entry;

use pass_entry::ServiceInfo;

use crate::pass_entry::prompt;
use crate::pass_entry::read_password_from_file;

fn clear() {
    print!("{}[23", 27 as char);
}
fn main() {
    clear();
    println!("Password vault");
    loop {
        println!("password manager menu");
        println!("1.  Add Entry");
        println!("2.  List Entries");
        println!("3.  Search Entries");
        println!("4.  Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clear();
                let entry = ServiceInfo::new(
                    prompt("Service : "),
                    prompt("Username : "),
                    prompt("Password : "),
                );
                println!("Entry given is {:?}", entry);
                entry.write_to_file();
            }
            "2" => {
                clear();
                let services = read_password_from_file().unwrap_or_else(|err| {
                    eprintln!("error reading password : {}", err);
                    Vec::new()
                });

                for item in &services {
                    println!(
                        "Service = {}
                        - Username : {}
                        - Password : {}",
                        item.service, item.username, item.password
                    )
                }
            }
            "3" => {
                clear();
                let services = read_password_from_file().unwrap_or_else(|err| {
                    eprintln!("error reading password : {}", err);
                    Vec::new()
                });
                let search = prompt("search : ");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "
                                Service = {}
                                - Username : {}
                                - Password : {}
                            ",
                            item.service, item.username, item.password
                        )
                    }
                }
            }
            "4" => {
                clear();
                println!("goodbye");
                break;
            }
            _ => println!("invalid choice"),
        }
        println!("\n\n");
    }
}
