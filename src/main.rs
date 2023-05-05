use std::env;
use std::fs;
use std::io::Write;
use std::process;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

struct Tasks {
    tasks: Vec<String>,
}

impl Tasks {
    fn get() -> Self {
        Tasks {
            tasks: fs::read_to_string("tasks.txt")
                .expect("File not found.")
                .lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>(),
        }
    }

    pub fn add(&mut self, descriptions: Vec<String>) -> &mut Self {
        for description in descriptions {
            self.tasks.push(description);
        }

        self
    }

    pub fn list(self) -> Self {
        println!("{}", self.tasks.join("\n"));

        self
    }

    pub fn remove(&mut self, descriptions: Vec<String>) -> &mut Self {
        for description in descriptions {
            if let Some(index) = self.tasks.iter().position(|task| task == &description) {
                self.tasks.remove(index);
            } else {
                eprintln!("Error: {:?} not found.", description);
            }
        }

        self
    }

    pub fn save(&self) -> i32 {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("tasks.txt")
            .expect("File not found.");

        let contents = &self.tasks.join("\n");
        writeln!(file, "{contents}").expect("File not writable.");

        0
    }
}

#[derive(EnumString, Display)]
enum Command {
    #[strum(serialize = "add")]
    Add,
    #[strum(serialize = "remove")]
    Remove,
    #[strum(serialize = "list")]
    List,
}

fn parse_args(args: env::Args) -> (Command, Vec<String>) {
    let binding = args.collect::<Vec<String>>();
    let command = binding.get(1).unwrap_or_else(|| {
        display_help();

        std::process::exit(0);
    });

    let command = Command::from_str(command.as_str()).unwrap_or_else(|_| {
        eprintln!("Error: Invalid command.");

        process::exit(1);
    });

    return (
        command,
        binding
            .iter()
            .skip(2)
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>(),
    );
}

fn main() {
    let (command, args) = parse_args(env::args());

    let mut tasks = Tasks::get();

    let exit_code = match command {
        Command::Add => tasks.add(args).save(),
        Command::List => tasks.list().save(),
        Command::Remove => tasks.remove(args).save(),
    };

    process::exit(exit_code)
}

fn display_help() {
    println!("Usage: task <command> <args>");
    println!();
    println!("Commands:");
    println!("   add <description> - adds task");
    println!("   list              - displays task list");
    println!("   remove            - removes a task");
    println!();
}
