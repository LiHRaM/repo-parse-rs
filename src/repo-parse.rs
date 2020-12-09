use std::{env, ffi::OsStr, path::Path};

use repo_parse_rs::{RepoUrl, parse};

fn main() {
    let mut args = env::args();
    let current_exe = args.next();
    let current_exe = current_exe
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .unwrap_or("repo-parse");

    if let Some(url) = args.next() {
        let (server, path) = match parse(&url) {
            RepoUrl::Ssh {server, path, ..} => (server, path),
            RepoUrl::Https {server, path}  => (server, path),
        };
        println!("{}/{}", server, path);
    } else {
        println!("Usage: {} <url>", current_exe);
        std::process::exit(1);
    }
}
