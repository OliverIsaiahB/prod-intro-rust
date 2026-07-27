// Declare the modules that make up this crate.
mod note;
mod book;

use std::env;
use book::NoteBook;
use note::Priority;

const STORE: &str = "notes.txt";

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let mut book = NoteBook::default();
    // Load existing notes if the file is there (ignore "not found").
    let _ = book.load(STORE);

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            let text = args[2..].join(" ");
            book.add(text, Priority::Normal);
            book.save(STORE)?;
            println!("added; {} notes total", book.count());
        }
        Some("list") => book.list(),
        Some("help") | None => print_usage(),
        Some(other) => {
            println!("unknown command: {other}");
            print_usage();
        }
    }
    Ok(())
}

fn print_usage() {
    println!("usage: notes <add TEXT | list | help>");
}
