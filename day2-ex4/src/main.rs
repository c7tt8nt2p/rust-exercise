#![allow(unused_variables, dead_code)]

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    println!("[{}]   [{}]", prefix, request_path);
    let mut prefix_iter = prefix.split('/');
    let mut request_path_iter = request_path.split('/');
    let matched = loop {
        match (prefix_iter.next(), request_path_iter.next()) {
            (Some(left), Some(right)) => {
                if is_wildcard(left) {
                    let Some(l) = prefix_iter.next() else { break false; };
                    if string_match(l, right) {
                        continue;
                    } else {
                        let matched = loop {
                            let Some(r) = request_path_iter.next() else { break false; };
                            if string_match(l, r) {
                                break true;
                            }
                        };
                        if matched {
                            continue;
                        }
                        break false;
                    }
                }
                if !string_match(left, right) {
                    break false;
                }
            }
            (Some(x), None) => break false,
            (None, Some(y)) => break true,
            (None, None) => break true,
        }
    };
    println!();
    matched
}

fn string_match(x: &str, y: &str) -> bool {
    x == y
}

fn is_wildcard(s: &str) -> bool {
    s == "*"
}


#[cfg(test)]
mod tests {
    use crate::prefix_matches;

    #[test]
    fn test_matches_without_wildcard() {
        assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
        assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
        assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

        assert!(!prefix_matches("/v1/publishers", "/v1"));
        assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
        assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
    }

    #[test]
    fn test_matches_with_wildcard() {
        assert!(prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/foo/books"
        ));
        assert!(prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/bar/books"
        ));
        assert!(prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/foo/books/book1"
        ));

        assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
        assert!(!prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/foo/booksByAuthor"
        ));
    }
}
