#[cfg(test)]
use super::*;

#[test]
fn case_sensitive() {
    let query = "duct";
    let content = "\
Rust:
safe, fast, productive
Download right now!
    ";
    assert_eq!(vec!["safe, fast, productive"], search(query, content));
}

#[test]
fn case_insetitive() {
    let query = "duCt";
    let content = "\
Rust:
safe, fast, productive
Download right now!
    ";
    assert_eq!(
        vec!["safe, fast, productive"],
        search_case_insentitve(query, content)
    );
}
