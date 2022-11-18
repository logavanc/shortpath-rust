use super::*;

#[test]
fn test_shortest_minimum() {
    let s = "abcdefg".to_string();
    let others = vec!["x".to_string(), "y".to_string(), "z".to_string()];
    let shortest = Crawler::new(3, '…').shortest(s, others);
    assert_eq!(shortest, "abc…");
}

#[test]
fn test_shortest_maximum() {
    let s = "abcdefg".to_string();
    let others = vec!["abcdefgh".to_string(), "y".to_string(), "z".to_string()];
    let shortest = Crawler::new(3, '…').shortest(s, others);
    assert_eq!(shortest, "abcdefg");
}

#[test]
fn test_shortest_working() {
    let s = "abcdefg".to_string();
    let others = vec!["abcx".to_string(), "y".to_string(), "z".to_string()];
    let shortest = Crawler::new(3, '…').shortest(s, others);
    assert_eq!(shortest, "abcd…");
}
