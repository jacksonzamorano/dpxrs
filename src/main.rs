use std::{env, process::exit};

use dropbox::{DropboxProfile, DropboxFile};

use crate::{auth::DropboxAuthentication, console::console::Console};

pub mod auth;
pub mod console;
pub mod dropbox;

fn authenticate(console: &mut Console) -> DropboxAuthentication {
    let mut auth = DropboxAuthentication::create();
    let url = auth.get_auth_link();
    if !auth.authorized() {
        console.print(&format!("Open this URL and paste the code: {}\n\nCode: ", url));
        let token = console.get();
        auth.set_code(token);
    }

    auth
}

fn logout(console: &mut Console, auth: &mut DropboxAuthentication) {
    auth.logout();
    console.print(&String::from("Signed out."));
    exit(0);
}

fn whoami(console: &mut Console, auth: &DropboxAuthentication) {
    if let Some(profile) = DropboxProfile::from(auth) {
        console.print(&format!("You're signed into Dropbox as {}", profile.name.display_name));
    } else {
        println!("There was an error checking your profile.");
    }
}

fn dl(console: &mut Console, auth: &DropboxAuthentication, args: &[String]) {
    if args.len() == 0 {
        console.print_static("dl requires an argument (file path).");
    } else {
        if let Some(file) = DropboxFile::download(&args[0], auth) {
            file.write(file.path())
        } else {
            console.print_static("Cannot download this file, are you sure it exists?");
        }
    }
}

fn zip(console: &mut Console, auth: &DropboxAuthentication, args: &[String]) {
    if args.len() == 0 {
        console.print_static("zip requires an argument (file path).");
    } else {
        if let Some(file) = DropboxFile::download_zip(&args[0], auth) {
            file.write(file.path())
        } else {
            console.print_static("Cannot download this file, are you sure it exists?");
        }
    }
}

fn main() {
    let mut console = Console::create();
    let mut authentication = authenticate(&mut console);

    let input_args = env::args().collect::<Vec<String>>();
    let cmd = input_args[1].to_string();
    let args = &input_args[2..];

    match cmd.as_str() {
        "logout" => logout(&mut console, &mut authentication),
        "whoami" => whoami(&mut console, &authentication),
        "dl" => dl(&mut console, &authentication, args),
        "zip" => zip(&mut console, &authentication, args),
        _ => {
            console.print(&format!("Unknown command {}!\n", cmd))
        }
    }
}
