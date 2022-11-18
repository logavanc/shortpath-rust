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
    fn shortest(&self, s: String, others: Vec<String>) -> String {
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
        if shortest != s {
            // Add the indicator if the original string was truncated.
            shortest.push(self.indicator);
        }
        shortest
    }

    fn is_home(&self, path: &Path) -> bool {
        if let Some(home) = &self.home {
            return path == home.as_path();
        }
        false
    }

    pub fn crawl(&self, path: &Path) -> String {
        if self.is_home(path) {
            return "~".to_string();
        }
        // if dirPath == os.Getenv("HOME") {
        //     return "~"
        // } else if dirPath == "/" {
        //     return "/"
        // } else if dirPath == "" {
        //     return ""
        // }
        let mut path_string = String::new();

        for component in path.components() {
            let component_string = component.as_os_str().to_string_lossy().to_string();
            path_string.push_str(component_string.as_str());
            path_string.push('X');
        }

        path_string
    }
}
