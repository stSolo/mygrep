#[cfg(test)]
use super::*;

#[test]
fn one_result() {
    let query = "duct";
    let content = "\
Rust:
safe, fast, productive
Download right now!
    ";
    assert_eq!(vec!["safe, fast, productive"], search(query, content));
}
