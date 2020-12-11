pub enum RepoUrl<'a> {
    Ssh {
        user: &'a str,
        server: &'a str,
        path: &'a str,
    },
    Https {
        server: &'a str,
        path: &'a str,
    },
}

fn split_once(url: &'_ str, delimiter: char) -> Option<(&'_ str, &'_ str)> {
    match url.find(delimiter) {
        Some(loc) => Some((&url[..loc], &url[(loc + 1)..])),
        None => None,
    }
}

pub fn parse(url: &'_ str) -> RepoUrl<'_> {
    let url = url.trim();
    let url = url.strip_suffix(".git").unwrap_or(url);

    if let Some(url) = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
    {
        let url = url.strip_prefix("www.").unwrap_or(url);
        if let Some((server, path)) = split_once(url, '/') {
            RepoUrl::Https { server, path }
        } else {
            oh_no("/");
        }
    } else if let Some((server, path)) = split_once(url, ':') {
        if let Some((user, server)) = split_once(server, '@') {
            RepoUrl::Ssh { user, server, path }
        } else {
            oh_no("@");
        }
    } else {
        oh_no(":");
    }
}

fn oh_no(expected: &str) -> ! {
    panic!("Oh no! Expected '{}'.", expected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_once_end() {
        assert_eq!(Some(("start", "")), split_once("start/", '/'))
    }

    #[test]
    fn split_once_start() {
        assert_eq!(Some(("", "end")), split_once("/end", '/'))
    }

    #[test]
    fn git_ssh() {
        let url = "git@example.com:hello/world.git";
        if let RepoUrl::Ssh { user, server, path } = parse(url) {
            assert_eq!(user, "git");
            assert_eq!(server, "example.com");
            assert_eq!(path, "hello/world");
        } else {
            unreachable!();
        }
    }

    #[test]
    fn git_http() {
        let url = "https://www.example.com/hello/world.git";
        if let RepoUrl::Https { server, path } = parse(url) {
            assert_eq!(server, "example.com");
            assert_eq!(path, "hello/world");
        } else {
            unreachable!();
        }
    }

    #[test]
    #[should_panic(expected = "Oh no! Expected ':'.")]
    fn fails_missing_protocol() {
        let url = "www.example.com/hello/world.git";
        parse(url);
    }

    #[test]
    #[should_panic(expected = "Oh no! Expected '/'.")]
    fn fails_missing_path() {
        let url = "https://www.example.com";
        parse(url);
    }

    #[test]
    #[should_panic(expected = "Oh no! Expected '@'.")]
    fn fails_missing_at_sign() {
        let url = "example.com:hello/world.git";
        parse(url);
    }
}
