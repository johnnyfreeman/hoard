use std::env;
use std::fs;
use std::io::Write;
use std::process;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

fn main() {
    let (command, args) = parse_args(env::args().collect::<Vec<String>>());

    let filename: String = "tasks.txt".to_string();

    let exit_code = match command {
        Command::Add => Tasks::read(&filename).add(args).write(&filename),
        Command::Init => Tasks::create(&filename),
        Command::List => Tasks::read(&filename).list(),
        Command::Remove => Tasks::read(&filename).remove(args).write(&filename),
        Command::Sort => Tasks::read(&filename).sort().write(&filename),
    };

    process::exit(exit_code)
}

fn parse_args(args: Vec<String>) -> (Command, Vec<String>) {
    let command = args.get(1).unwrap_or_else(|| {
        display_help();

        std::process::exit(0);
    });

    let command = Command::from_str(command.as_str()).unwrap_or_else(|_| {
        eprintln!("Error: Invalid command.");

        process::exit(1);
    });

    return (
        command,
        args.iter()
            .skip(2)
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>(),
    );
}

struct Tasks {
    tasks: Vec<String>,
}

impl Tasks {
    fn read(storage: &String) -> Self {
        Tasks {
            tasks: fs::read_to_string(storage)
                .unwrap_or_else(|_| {
                    eprintln!("File {} could not be found.", storage);
                    process::exit(1);
                })
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

    pub fn list(self) -> i32 {
        println!("{}", self.tasks.join("\n"));

        0
    }

    pub fn remove(&mut self, descriptions: Vec<String>) -> &mut Self {
        for description in descriptions {
            if let Some(index) = self.tasks.iter().position(|task| task == &description) {
                self.tasks.remove(index);
            } else {
                eprintln!("Error: {} not found.", description);
            }
        }

        self
    }

    pub fn write(&self, filename: &String) -> i32 {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(filename)
            .unwrap_or_else(|_| {
                eprintln!("File {} could not be found.", filename);
                process::exit(1);
            });

        let contents = &self.tasks.join("\n");
        writeln!(file, "{contents}").expect("File not writable.");

        0
    }

    pub fn create(filename: &String) -> i32 {
        match fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(filename)
        {
            Ok(_) => {
                println!("File {} has been created successfullly.", filename);
                0
            }
            Err(_) => {
                eprintln!("File {} could not be created.", filename);
                1
            }
        }
    }

    pub fn sort(&mut self) -> &mut Self {
        self.tasks.sort();

        self
    }
}

#[derive(EnumString, Display)]
enum Command {
    #[strum(serialize = "add")]
    Add,
    #[strum(serialize = "init")]
    Init,
    #[strum(serialize = "list")]
    List,
    #[strum(serialize = "sort")]
    Sort,
    #[strum(serialize = "remove")]
    Remove,
}

fn display_help() {
    println!("Usage: task <command> <args>");
    println!();
    println!("Commands:");
    println!("   add <description>    - adds task");
    println!(
        "   init                 - creates file for task storage if one does not already exist"
    );
    println!("   list                 - displays task list");
    println!("   remove <description> - removes task");
    println!("   sort                 - sort tasks");
}
