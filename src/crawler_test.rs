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

#[test]
fn test_home() {
    let mut c = Crawler::new(3, '…');
    c.home = Some(PathBuf::from("/home/user"));
    assert!(c.is_home(Path::new("/home/user")));
    assert!(!c.is_home(Path::new("/home/users")));
}

#[test]
fn test_no_home() {
    let mut c = Crawler::new(3, '…');
    c.home = None;
    assert!(!c.is_home(Path::new("/home/user")));
}
