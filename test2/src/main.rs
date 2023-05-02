extern crate libc;

use libc::{getpid, kill};
use std::{env, fs, io::stdin, io::Error, io::ErrorKind};

mod game;

#[allow(non_camel_case_types, dead_code)]
enum Commands {
    solver(String),
    quit(String),
    input(String),
}

#[allow(dead_code)]
fn print<T: std::fmt::Display>(var: T) {
    let str_var = var.to_string();
    println!("{}", str_var);
}

fn parse_arguments() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();
    let mut filepath = "test2.txt";

    if args.len() == 2 {
        filepath = &args[1];
        println!("\"{}\" as input file.", filepath);
    } else if args.len() == 1 {
        println!("Default input file. (test2.txt)");
    } else {
        println!("Us: ./code.rs <filepath> o ./code.rs");
        return Err(Error::new(ErrorKind::Other, "Bad Args"));
    }

    let contents = fs::read_to_string(filepath)?;
    return Ok(contents);
}

fn selection_sort(list: &String) -> String {
    let mut temp_list: Vec<i32> = list
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let len = temp_list.len();
    for i in 0..len - 1 {
        let mut min_index = i;
        for j in i + 1..len {
            if temp_list[j] < temp_list[min_index] {
                min_index = j;
            }
        }
        temp_list.swap(i, min_index);
    }

    temp_list
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn time_solver(path: String) {
    //TODO: Hauria de crear un subprocess per cada solver cridant a un altre codi que rebi com a parametre un archiu cnf
    println!("estic a solve lolxd:{}", path)
}

fn do_solver() {
    println!("Lol tesperaves un solver xd\n");

    let mut line = String::new();
    println!("Indica la carpeta de benchmarks.(bench)");
    if let Err(e) = stdin().read_line(&mut line) {
        panic!("Error reading input: {}", e)
    }


    line = line.trim().to_owned();
    if line.eq("quit") {
        quit()
    }

    match fs::read_dir(line) {
        Err(e) => return println!("{}, Error al readDir", e),
        Ok(paths) => {
            for path in paths {
                if let Ok(entry) = path {
                    if let Some(path_str) = entry.path().to_str() {
                        time_solver(path_str.to_string());
                    }
                }
            }
        }
    }
}

fn quit() {
    println!("Hasta la proximaaa...");
    let pid = unsafe { getpid() };
    unsafe { kill(pid, libc::SIGINT) };
}

fn print_input(unsorted: &String) {
    println!(
        "Input file had: {} \nSorted input is: {} \n",
        unsorted,
        selection_sort(&unsorted)
    )
}
fn handle_game(){
    match game::run_game(){
        _ => println!("ole"),
    }
}
fn main() {
    let input = match parse_arguments() {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    println!("Implemented: <solver> <quit> <input> <game>");
    loop {
        let mut line = String::new();
        println!("##########     Cursed app: digues algo idk:     ##########");
        match stdin().read_line(&mut line) {
            Ok(it) => it,
            Err(e) => panic!("{}", e),
        };

        match line.trim(){
            "quit" => quit(),
            "solver" => do_solver(),
            "input" => print_input(&input),
            "game" => handle_game(),
            _ => println!("\n{} ta puta mare crack.\n", line),
        }
    }
}
