use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

fn main() {
    let builtins: HashSet<&str> = ["echo", "exit", "type", "pwd", "cd"]
        .iter()
        .cloned()
        .collect();
    let mut cache: HashMap<String, String> = HashMap::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let tokens = parse_input(&input);

        if let Some((command, arguments)) = tokens.split_first() {
            match command.as_str() {
                "exit" => {
                    exit_command(&arguments.iter().map(String::as_str).collect::<Vec<&str>>())
                }
                "echo" => echo_command(arguments),
                "type" => type_command(arguments, &builtins, &mut cache),
                "pwd" => pwd_command(),
                "cd" => cd_command(arguments),
                _ => run_program(
                    command,
                    &arguments.iter().map(String::as_str).collect::<Vec<&str>>(),
                    &mut cache,
                ),
            }
        } else {
            return;
        }
    }
}

// this is a nightmare
fn parse_input(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = None;
    let mut escape = false;

    for c in input.trim().chars() {
        if escape {
            match (in_quotes, c) {
                (Some('"'), '\\' | '$' | '"' | '\n') => {
                    current_token.push(c);
                }
                (Some('"'), _) => {
                    current_token.push('\\');
                    current_token.push(c);
                }
                (_, _) => current_token.push(c),
            }
            escape = false;
            continue;
        }

        match in_quotes {
            Some(quote) if c == quote => {
                in_quotes = None;
            }
            Some(_) => {
                if c == '\\' && in_quotes == Some('"') {
                    escape = true;
                } else {
                    current_token.push(c);
                }
            }
            None if c == '\\' => {
                escape = true;
            }
            None if c == '"' || c == '\'' => {
                in_quotes = Some(c);
                if !current_token.is_empty()
                    && current_token.chars().last().unwrap().is_whitespace()
                {
                    tokens.push(current_token);
                    current_token = String::new()
                };
            }
            None if !c.is_whitespace() => {
                current_token.push(c);
            }
            None => {
                if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new()
                }
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    tokens
}

fn cd_command(arguments: &[String]) {
    if let Some(path) = arguments.get(0) {
        let path = path.replace("~", &env::var("HOME").unwrap_or_default());
        let path = Path::new(&path);
        if path.exists() && path.is_dir() {
            let _ = env::set_current_dir(&path);
        } else {
            println!("cd: {}: No such file or directory", path.display());
        }
    };
}

fn pwd_command() {
    if let Ok(current_dir) = env::current_dir() {
        println!("{}", current_dir.display());
    }
}

fn run_program(command: &str, arguments: &[&str], cache: &mut HashMap<String, String>) {
    if let Some(path) = cache.get(command) {
        match Command::new(path)
            .args(arguments)
            .spawn()
            .and_then(|mut child| child.wait())
        {
            Ok(_) => (),
            Err(e) => eprintln!("Error executing {}: {}", command, e),
        }
    } else if let Some(path) = get_command_path(command) {
        cache.insert(command.to_string(), path.clone());
        match Command::new(path)
            .args(arguments)
            .spawn()
            .and_then(|mut child| child.wait())
        {
            Ok(_) => (),
            Err(e) => eprintln!("Error executing {}: {}", command, e),
        }
    } else {
        command_not_found(command);
    }
}

fn get_command_path(command: &str) -> Option<String> {
    let path_var = env::var("PATH").unwrap_or_else(|_| String::new());
    for dir in path_var.split(':') {
        let path = format!("{}/{}", dir, command);
        if let Ok(metadata) = fs::metadata(&path) {
            if metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0) {
                return Some(path);
            }
        }
    }
    None
}

fn exit_command(arguments: &[&str]) {
    let exit_code = arguments
        .get(0)
        .and_then(|arg| arg.parse::<i32>().ok())
        .unwrap_or(0);
    std::process::exit(exit_code);
}

fn echo_command(arguments: &[String]) {
    let output = arguments.join(" ");
    println!("{}", output);
}

fn type_command(
    arguments: &[String],
    builtins: &HashSet<&str>,
    cache: &mut HashMap<String, String>,
) {
    if arguments.is_empty() {
        return;
    }

    for command in arguments {
        if builtins.contains(command.as_str()) {
            println!("{} is a shell builtin", command);
        } else if let Some(path) = cache.get(command) {
            println!("{} is {}", command, path)
        } else if let Some(path) = get_command_path(command) {
            cache.insert(command.to_string(), path.clone());
            println!("{} is {}", command.to_string(), path)
        } else {
            println!("{}: not found", command);
        }
    }
}

fn command_not_found(command: &str) {
    println!("{}: command not found", command);
}
