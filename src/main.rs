use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Parser)]
#[command(name = "rtodo")]
#[command(about = "A simple TODO CLI")]
struct Cli {
    #[arg(long)]
    add: Option<String>,

    #[arg(long)]
    completed: Option<usize>,

    #[arg(long)]
    delete: Option<usize>,

    #[arg(long)]
    show: bool,

    #[arg(long, default_value = "todo.txt")]
    file: String,
}

// Read all lines from the file and return them as a vector
fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

// Write all lines back to the file with proper numbering
fn write_lines(filename: &str, lines: &[String]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for (i, line) in lines.iter().enumerate() {
        // Explicitly specify char type in the closure for type inference
        // Remove existing numbers, dots, and spaces from the start of the line
        let cleaned_line = line.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == ' ');
        writeln!(file, "{}. {}", i + 1, cleaned_line)?;
    }
    Ok(())
}

// Display todos with formatting
fn display_todos(filename: &str) -> io::Result<()> {
    let lines = read_lines(filename)?;
    println!("─────────────── TODO List ───────────────");
    for line in lines {
        println!("{}", line);
    }
    println!("─────────────── End List ────────────────");
    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    // Create the file if it doesn't exist
    if !Path::new(&cli.file).exists() {
        File::create(&cli.file)?;
    }

    // Handle the add command
    if let Some(todo_item) = cli.add {
        let lines = read_lines(&cli.file)?;
        let new_index = lines.len() + 1;
        let mut file = OpenOptions::new()
            .append(true)
            .open(&cli.file)?;
        writeln!(file, "{}. {}", new_index, todo_item.trim())?;
        println!("Added: {}", todo_item.trim());
    }

    // Handle the completed command - mark item with a star
    if let Some(index) = cli.completed {
        let mut lines = read_lines(&cli.file)?;
        if index > 0 && index <= lines.len() {
            let line = &lines[index - 1];
            if !line.contains('★') {
                lines[index - 1] = format!("{} ★", line);
                write_lines(&cli.file, &lines)?;
                println!("Marked item {} as completed", index);
            }
        } else {
            println!("Invalid item number");
        }
    }

    // Handle the delete command - remove item and renumber
    if let Some(index) = cli.delete {
        let mut lines = read_lines(&cli.file)?;
        if index > 0 && index <= lines.len() {
            lines.remove(index - 1);
            write_lines(&cli.file, &lines)?;
            println!("Deleted item {}", index);
        } else {
            println!("Invalid item number");
        }
    }

    // Handle the show command - display all items
    if cli.show {
        display_todos(&cli.file)?;
    }

    Ok(())
}
