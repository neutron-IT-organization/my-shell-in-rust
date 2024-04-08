use std::io::{self, Write};
use std::process::{Command, exit};

// Builtin function implementations.
fn neutron_shell_cd(args: &[&str]) -> i32 {
    if args.len() < 2 {
        eprintln!("neutron_shell: expected argument to \"cd\"");
    } else {
        if let Err(err) = std::env::set_current_dir(args[1]) {
            eprintln!("{}", err);
        }
    }
    1
}


fn neutron_shell_exit(_: &[&str]) -> i32 {
    exit(0);
}

// Launch a program and wait for it to terminate.
fn neutron_shell_launch(args: &[&str]) -> i32 {
    match Command::new(args[0]).args(&args[1..]).spawn() {
        Ok(mut child) => {
            child.wait().expect("Failed to wait on child process");
        }
        Err(err) => eprintln!("neutron_shell: {}", err),
    }
    1
}

// Execute shell built-in or launch program.
fn neutron_shell_exec(args: &[&str]) -> i32 {
    match args.get(0) {
        Some(&"cd") => neutron_shell_cd(args),
        Some(&"exit") => neutron_shell_exit(args),
        Some(command) => neutron_shell_launch(&[command]),
        None => 1,
    }
}

// Read a line of input from stdin.
fn neutron_shell_read() -> String {
    print!("neutron-shell > ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// Split a line into tokens.
fn neutron_shell_split(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

// Loop getting input and executing it.
fn neutron_shell_loop() {
    loop {
        let line = neutron_shell_read();
        let args = neutron_shell_split(&line);
        let status = neutron_shell_exec(&args);
        if status == 0 {
            break;
        }
    }
}

fn main() {
    // Run command loop.
    neutron_shell_loop();
}

