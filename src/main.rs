use std::{env, process::exit, path::PathBuf, io::Cursor};

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
            file.write(file.filename())
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
            file.write(file.filename())
        } else {
            console.print_static("Cannot download this folder, are you sure it exists?");
        }
    }
}

fn folder(console: &mut Console, auth: &DropboxAuthentication, args: &[String]) {
    if args.len() == 0 {
        console.print_static("folder requires an argument (file path).");
    } else {
        if let Some(file) = DropboxFile::download_zip(&args[0], auth) {
            let local_path = file.filename().replace(".zip", "");
            let local_path_buf = PathBuf::from(local_path);
            _ = zip_extract::extract(Cursor::new(file.data()), &local_path_buf, true);
        } else {
            console.print_static("Cannot download this folder, are you sure it exists?");
        }
    }
}

fn upload(console: &mut Console, auth: &DropboxAuthentication, args: &[String]) {
    if args.len() == 0 {
        console.print_static("upload requires an argument (local file).");
    } else {
        if let Some(local) = DropboxFile::local(&args[0]) {
            if let Some(path) = local.upload(&auth) {
                console.print(&format!("Uploaded to {}", path))
            }
        }
    }
}
fn get_view_link(console: &mut Console, auth: &DropboxAuthentication, args: &[String]) {
    if args.len() == 0 {
        console.print_static("vl requires an argument (Dropbox file).");
    } else {
        if let Some(url) = DropboxFile::public_view_link(&args[0], auth) {
            console.print(&format!("View-only link: {}", url))
        } else {
            console.print_static("Cannot find the specified file.")
        }
    }
}
fn remove_links(console: &mut Console, auth: &DropboxAuthentication, args: &[String]) {
    if args.len() == 0 {
        console.print_static("private requires an argument (Dropbox file).");
    } else {
        if DropboxFile::remove_all_links(&args[0], auth) {
            console.print_static("Removed all links.")
        } else {
            console.print_static("Cannot not remove links.")
        }
    }
}

fn main() {
    let mut console = Console::create();
    let mut authentication = authenticate(&mut console);

    let input_args = env::args().collect::<Vec<String>>();
    if input_args.len() == 1 {
        console.print_static("Welcome to DPX. You can check out the commands here: https://github.com/jacksonzamorano/dpx.");
        exit(0);
    }
    let cmd = input_args[1].to_string();
    let args = &input_args[2..];

    match cmd.as_str() {
        "logout" => logout(&mut console, &mut authentication),
        "whoami" => whoami(&mut console, &authentication),
        "dl" => dl(&mut console, &authentication, args),
        "zip" => zip(&mut console, &authentication, args),
        "up" => upload(&mut console, &authentication, args),
        "vl" => get_view_link(&mut console, &authentication, args),
        "private" => remove_links(&mut console, &authentication, args),
        "folder" => folder(&mut console, &authentication, args),
        _ => {
            console.print(&format!("Unknown command {}!\n", cmd))
        }
    }
}
