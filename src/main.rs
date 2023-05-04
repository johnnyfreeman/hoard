use std::env;
use std::fs;
use std::io::Write;
use std::process;

trait Command {
    fn handle(&self) -> i32;
}

struct AddCommand {
    args: Vec<String>,
}

impl AddCommand {
    fn new(args: Vec<String>) -> Self {
        AddCommand { args }
    }
}

impl Command for AddCommand {
    fn handle(&self) -> i32 {
        let description = &self
            .args
            .iter()
            .skip(2)
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("tasks.txt")
            .expect("File not found.");

        writeln!(file, "{description}").expect("File not writable.");

        return 0;
    }
}

struct ListCommand {}

impl ListCommand {
    fn new() -> Self {
        ListCommand {}
    }
}

impl Command for ListCommand {
    fn handle(&self) -> i32 {
        let tasks = fs::read_to_string("tasks.txt").expect("File not found.");

        println!("{tasks}");

        0
    }
}

struct RemoveCommand {
    args: Vec<String>,
}

impl RemoveCommand {
    fn new(args: Vec<String>) -> Self {
        RemoveCommand { args }
    }
}

impl Command for RemoveCommand {
    fn handle(&self) -> i32 {
        let description = &self
            .args
            .iter()
            .skip(2)
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let tasks = fs::read_to_string("tasks.txt")
            .expect("File not found.")
            .lines()
            .filter(|task| task != &description)
            .collect::<Vec<&str>>()
            .join("\n");

        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("tasks.txt")
            .expect("File not found.");

        writeln!(file, "{tasks}").expect("File not writable.");

        return 0;
    }
}

struct UnknownCommand {}

impl UnknownCommand {
    fn new() -> Self {
        UnknownCommand {}
    }
}

impl Command for UnknownCommand {
    fn handle(&self) -> i32 {
        println!("Unknown command.");

        1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args.get(1).unwrap_or_else(|| {
        display_help();

        process::exit(0);
    });

    let exit_code: i32 = match command.as_str() {
        "add" => AddCommand::new(args).handle(),
        "list" => ListCommand::new().handle(),
        "remove" => RemoveCommand::new(args).handle(),
        _ => UnknownCommand::new().handle(),
    };

    process::exit(exit_code);
}

fn display_help() {
    println!("Usage: task <command> <args>");
    println!();
    println!("Commands:");
    println!("   add <description> - adds task");
    println!("   list              - displays task list");
    println!();
}
