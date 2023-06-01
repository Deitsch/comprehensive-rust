// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefix_parts = prefix.split("/");
    let request_parts = request_path.split("/");

    if prefix_parts.clone().count() > request_parts.clone().count() {
        return false
    }
    
    for (pre, req) in prefix_parts.zip(request_parts) {
        if pre != req && pre != "*" {
            return false
        }
    };
    return true;
}

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
    assert!(prefix_matches("/v1/publishers/*/books", "/v1/publishers/foo/books"));
    assert!(prefix_matches("/v1/publishers/*/books", "/v1/publishers/bar/books"));
    assert!(prefix_matches("/v1/publishers/*/books", "/v1/publishers/foo/books/book1"));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers/foo/booksByAuthor"));
}

fn main() {
    let yo = prefix_matches("/v1/publishers", "/v1");
    println!("{yo}")
}
