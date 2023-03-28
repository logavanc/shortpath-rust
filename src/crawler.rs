#[cfg(test)]
#[path = "./crawler_test.rs"]
mod crawler_test;

use dirs::home_dir;
use std::path::{Path, PathBuf};

pub struct Crawler {
    pub shortest: usize,
    pub indicator: char,

    home: Option<PathBuf>,
}

impl Crawler {
    /// Creates a new [`Crawler`].
    pub fn new(shortest: usize, indicator: char) -> Self {
        Self {
            shortest,
            indicator,
            home: home_dir(),
        }
    }

    /// Returns the shortest slice of s that is still unique among the others.
    /// The shortest slice is at least `self.shortest` characters long, and
    /// will have the `self.indicator` character appended to it if it has been
    /// truncated.
    fn shortest(&self, s: &String, mut others: Vec<String>) -> String {
        others.retain(|other| other != s);
        let mut shortest = String::new();
        's_chars_loop: for (i, c) in s.chars().enumerate() {
            if i < self.shortest {
                shortest.push(c);
                continue 's_chars_loop;
            }
            for other in &others {
                if other.starts_with(&shortest) {
                    shortest.push(c);
                    continue 's_chars_loop;
                }
            }
            break 's_chars_loop;
        }
        if &shortest != s {
            // Add the indicator if the original string was truncated.
            shortest.push(self.indicator);
        }
        shortest
    }

    fn is_home(&self, path: &Path) -> bool {
        if let Some(home) = &self.home {
            return path == home;
        }
        false
    }

    pub fn crawl(&self, path: &Path) -> String {
        // Create a list of strings to be joined at the end.
        let mut result: Vec<String> = Vec::new();

        let canon_path = path.canonicalize().unwrap();
        for (i, ancestor) in canon_path.ancestors().enumerate() {
            if self.is_home(ancestor) {
                result.push(String::from("~"));
                break;
            }
            if ancestor.as_os_str() == "/" {
                result.push(String::from("/"));
                break;
            }
            let leaf = ancestor.file_name().unwrap().to_str().unwrap();
            if i == 0 {
                result.push(leaf.to_string());
                continue;
            }
            let parent = ancestor.parent().unwrap();
            let others: Vec<String> = parent
                .read_dir()
                .unwrap()
                .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
                .collect();
            let shortest = self.shortest(&leaf.to_string(), others);
            result.push(shortest + "/");
        }

        result.reverse();
        result.join("")
    }
}
