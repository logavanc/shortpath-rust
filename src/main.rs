mod cli;
mod crawler;

use cli::Cli;
use crawler::Crawler;

use clap::Parser;
use std::env;

fn main() {
    let args = Cli::parse();
    let crawler = Crawler::new(args.shortest, args.indicator);
    let long_path = env::current_dir().unwrap();
    let short_path = crawler.crawl(long_path.as_path());
    println!("{}", short_path);
}
