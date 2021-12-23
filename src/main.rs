mod info;
use info::Info;
use std::{env, fs};

fn get_distro() -> Option<String> {
    let distro = fs::read_to_string("/etc/lsb-release")
        .expect("Couldn't read /etc/lsb-release!");
    for i in distro.lines() {
        if &i[..10] == "DISTRIB_DE" {
            return Some(i[21..i.len()-1].to_string())
        }
    }
    return None
}

fn main() {
    let host = Info {
        name: String::from("Host"),
        data: fs::read_to_string("/etc/hostname") .expect("/etc/hostname seems to not exist :/"),
    };
    let user = Info {
        name: String::from("User"),
        data: match env::var("USER"){
            Ok(val) => val,
            Err(oof) => format!("Couldn't find user; {}", oof) 
        },
    };
    let distro = Info {
        name: String::from("Distro"),
        data: match get_distro(){
            Some(val) => val,
            None => String::from("Distro not found :/"),
        },
    };
    host.print();
    user.print();
    distro.print();
}
