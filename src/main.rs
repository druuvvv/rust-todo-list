use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::fs::{OpenOptions};
use std::collections::HashSet;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let my_empty_string = String::from("");
    let task = args.get(1).unwrap_or_else(|| {&my_empty_string}).clone();
    
    let _ = match task.as_str() {
        "add" => add_items(&mut args),
        "remove" => remove_items(& mut args),
        "" => print_todo_list(),
        other => panic!("No command named {} found!", other)
    };

}

fn print_todo_list()-> Result<(), Error> {
    let path = "lines.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for (index,line) in buffered.lines().enumerate() {
        println!("{}. {}",index+1 ,line?);
    }
        Ok(())
}

fn get_remaining_args(args: &mut Vec<String>)-> Vec<String>{
    if args.len() > 2 {
        args.split_off(2)
    } else {
        panic!("Found less than expected arguments :(")
    }
}

fn add_items(indexes : & mut Vec<String>)-> Result<(), Error> {
    
    let items:Vec<String> = get_remaining_args(indexes);
    let path = "lines.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    // Collect existing lines into a HashSet to remove duplicates
    let existing_lines: HashSet<String> = buffered
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect();

    let mut output = OpenOptions::new().append(true).open(path)?;
    for item in items {
        if existing_lines.contains(&item) {
            println!("Skipped {} because it already exists in the list", item);
        } else {
        write!(output,"{}\n" , item.as_str()).unwrap();
        }
    }

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for (index,line) in buffered.lines().enumerate() {
        println!("{}. {}",index+1 ,line?);
    }

        Ok(())
}
fn remove_items(indexes : &mut Vec<String>)-> Result<(), Error> {
    let items = get_remaining_args(indexes);
    let mut parsed_indexes:Vec<usize> = Vec::new();
    for item in items {
        
        match item.parse::<usize>() {
            Ok(num) => {
                parsed_indexes.push(num);
            }
            Err(_) => {
                eprintln!("Error parsing {}" , item);
                println!("Skipped {}", item);
            }
        }
    }

    parsed_indexes.sort();
    
    let path = "lines.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let my_lines:Vec<String> = buffered
        .lines()
        .enumerate()
        .filter_map(|(index,line)| {
            if !parsed_indexes.contains(&(index+1)) {
                line.ok()
            }
            else {
                match line.ok() {
                    Some(text) => {println!("Deleted {}", text);},
                    None => {}
                }
                None
            }
        }) 
        .collect();
    
    let mut output = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(path)?;

    for line in my_lines {
        writeln!(output, "{}", line)?;
    }

    Ok(())
}