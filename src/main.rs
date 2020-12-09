use colored::*;
use rand::Rng;

use std::{convert::TryInto, env, process, thread::{sleep}, time::Duration};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut x = 0;
    let length = args.len().try_into().unwrap();

    println!("{} Resolving packages...", "Running".bright_green().bold());
    check_arguments(args.clone());

    println!("{} Checking for dependency conflicts...", "Running".bright_green().bold());
    sleep(Duration::from_millis(500));

    if length >= 1 {
        for argument in args {
            let state = argument.chars().next().unwrap();
            let package = argument.split_at(1).1.to_string();
    
            x = x+1;
    
            run_process(state, package, vec![x, length]) 
        }

        cleanup(length);
    } else if length == 0 {
        run_process('^', "core".to_string(), vec![1, 3]);
        run_process('^', "community".to_string(), vec![2, 3]);
        run_process('^', "linux".to_string(), vec![3, 3]);
        cleanup(3);
    }
}

fn check_arguments(args: Vec<String>) {
    for argument in args {
        let state = argument.chars().next().unwrap();

        if state != '+' && state != '-' && state != '^' { 
            println!("{} Failed to resolve packages, try checking the syntax and try again.", "Failure".bright_red().bold());
            process::exit(0);
        }
    }
}

fn cleanup(length: i16) {
    println!("{} Installed {} packages in {}s", "Finished".bright_green().bold(), length, rand::thread_rng().gen_range(length, length+5))
}

fn run_process(state: char, package: String, step: Vec<i16>) {
    let pretty_state = if state == '+' { "Installing" } else if state == '-' { "Removing" } else if state == '^' { "Upgrading" } else { "Missingno" };

    println!("  {} {} {}...", format!("({}/{})", step[0], step[1]).bright_black(), pretty_state, package);

    sleep(Duration::from_secs(rand::thread_rng().gen_range(1, 7)));
}