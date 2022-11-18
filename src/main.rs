mod cli;
mod crawler;

use cli::Cli;
use crawler::Crawler;

use clap::Parser;
use std::env;

fn main() {
    let args = Cli::parse();
    let crawler = Crawler::new(args.shortest, args.indicator);
    let path = env::current_dir().unwrap();
    println!("{}", crawler.crawl(path.as_path()));
}
