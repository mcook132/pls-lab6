use std::io::{self, Write, BufRead, BufReader};
use std::fs::File;


fn main() {

    // Read Bible
    for (index, line) in BufReader::new(File::open("Bible.txt").expect("Unable to upen file")).lines().enumerate() {
        let line = line.expect("Unable to read line");
        
    }

    loop {
        // Variable declarations
        let mut book = String::new();
        let mut chapter = String::new();
        let mut verse = String::new();

        // Intro & ask for reference
        println!("Welcome to Bible Lookup! Please enter the reference:");

        // Ask what book
        print!("     Book: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut book).expect("Invalid input");
        let book = book.trim();

        // Ask what chapter
        print!("  Chapter: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut chapter).expect("Invalid input");
        let chapter = chapter.trim();

        // Ask what verse
        print!("    Verse: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut verse).expect("Invalid input");
        let verse = verse.trim();

        // Ask for repeat
        print!("Would you like to run the program again? (Y/N) ");
        io::stdout().flush().unwrap();
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Invalid input");
        if again.trim().to_uppercase() == "N" {
            return;
        }
    }
}