use std::fs::File;
use std::io::stdin;
use std::io::{self, BufRead, Write};
use std::path::Path;
use sysinfo::System;
use termion::terminal_size;

fn install(package: &str) {
    println!("Please add {} to your Cargo.toml dependencies.", package);
}

fn get_monitors() -> (u16, u16) {
    let (width, height) = terminal_size().unwrap();
    (width, height)
}

fn set_terminal_size(rows: u16, cols: u16) {
    println!("\x1b[8;{};{}t", rows, cols);
}

fn print_success() {
    println!("\nThe program finished this task successfully!");
}

fn read_table(fname: &str) -> (Vec<Vec<String>>, String) {
    let mut table = Vec::new();
    let mut version = String::new();

    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(ip) = line {
                let cols: Vec<String> = ip.split('\t').map(|s| s.to_string()).collect();
                version = if cols.len() == 5 {
                    "old".to_string()
                } else {
                    "new".to_string()
                };
                table.push(cols);
            }
        }
    }

    (table, version)
}

fn read_list(fname: &str) -> (String, Vec<String>) {
    let mut list = Vec::new();
    let filename = String::new();

    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(ip) = line {
                list.push(ip);
            }
        }
    }

    (filename, list)
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_table(
    version: &str,
    lines: Vec<Vec<String>>,
    fname: &str,
    mod_choice: i32,
    cols: &Vec<String>,
) {
    let mut file = File::create(fname).expect("Unable to create file");
    for j in 0..lines.len() {
        let mut line = Vec::new();
        let freq = if mod_choice == 2 || mod_choice == 3 || mod_choice == 6 {
            &lines[j][1]
        } else {
            &lines[j][0]
        };

        for (i, elem) in [
            cols[0].clone(),
            freq.to_string(),
            cols[1].clone(),
            cols[2].clone(),
        ]
        .iter()
        .enumerate()
        {
            if i == 0 {
                line.push(elem.clone());
            } else if i == 1 {
                line.push(freq.to_string());
            } else if i == 2 {
                line.push(elem.clone());
            } else if i == 3 {
                line.push(elem.clone());
            }
            line.push("\t".to_string());
        }

        if version == "old" {
            line.insert(2, "0\t".to_string());
        }

        let line_str = line.join("").trim().to_string();
        if j < lines.len() - 1 {
            writeln!(file, "{}", line_str).expect("Unable to write data");
        } else {
            write!(file, "{}", line_str).expect("Unable to write data");
        }
    }
}

fn main() {
    let sys = System::new_all();
    let platform = sysinfo::System::name().unwrap_or_default();

    if platform == "Linux" {
        let (width, height) = get_monitors();
        set_terminal_size(height / 10, width / 10);
    } else if platform == "Windows" {
        // Windows terminal resizing is not directly supported in Rust
        println!("Windows terminal resizing is not supported in this example.");
    }

    println!("\nWelcome to the Velleman List Editor!\n");

    let selection = "Please select one of the following modi: \n \
            1.) \tFor the conversion of the (old|new) table into the list in 'frequenzen.txt' press '1'! \n \
            2.) \tFor conversion of old table format (PCGU1000) into the new one (PCSU200) press '2'! \n \
            3.) \tFor conversion of new table format (PCSU200) into the old one (PCGU1000) press '3'! \n \
            4.) \tFor creation of the old table format from the list in 'frequenzen.txt' press '4'! \n \
            5.) \tFor creation of the new table format from the list in 'frequenzen.txt' press '5'!\n \
            6.) \tFor changes to the values only press '6'!";

    println!("{}", selection);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let mod_choice: i32 = input.trim().parse().expect("Please enter a number");

    let mut fname = String::new();

    if mod_choice < 4 || mod_choice == 6 {
        println!(
            "As a help, type in a fraction of the filename.\nAll similar filenames will be shown:"
        );
        let mut partial_str = String::new();
        stdin()
            .read_line(&mut partial_str)
            .expect("Failed to read line");

        let paths = glob::glob(&format!("./*{}*.txt", partial_str.trim()))
            .expect("Failed to read glob pattern");
        for path in paths {
            println!("{}", path.unwrap().display());
        }

        println!("Type in the filename:");
        stdin().read_line(&mut fname).expect("Failed to read line");
        fname = fname.trim().to_string();

        if !fname.ends_with(".txt") {
            fname.push_str(".txt");
        }
    }

    let mut cols = Vec::new();
    if mod_choice > 1 {
        println!("Type in the duration in [seconds]:");
        let mut dur = String::new();
        stdin().read_line(&mut dur).expect("Failed to read line");

        println!("Type in the peak-to-peak voltage in [Volt]:");
        let mut vpp = String::new();
        stdin().read_line(&mut vpp).expect("Failed to read line");

        println!("Type in the waveform: [1=sine, 2=rect, 3=tri]");
        let mut waveform = String::new();
        stdin()
            .read_line(&mut waveform)
            .expect("Failed to read line");

        cols.push(waveform.trim().to_string());
        cols.push(vpp.trim().to_string());
        cols.push(dur.trim().to_string());

        for s in &cols {
            if !s.replace(".", "").chars().all(char::is_numeric) {
                println!(
                    "The given String {} is no number - Press any key to exit!",
                    s
                );
                let mut exit_input = String::new();
                stdin()
                    .read_line(&mut exit_input)
                    .expect("Failed to read line");
                return;
            }
        }
    }

    match mod_choice {
        1 => {
            let (table, version) = read_table(&fname);
            let mut freqs = String::new();
            for line in table {
                freqs.push_str(&format!("{}, ", line[1]));
            }
            freqs = format!(
                "{}: {}",
                fname.replace(".txt", "").replace(" ", "_"),
                freqs.trim_end_matches(", ")
            );
            let mut file = File::create("frequenzen.txt").expect("Unable to create file");
            file.write_all(freqs.as_bytes())
                .expect("Unable to write data");
            print_success();
        }
        2 => {
            let (mut table, version) = read_table(&fname);
            fname = fname
                .replace(".txt", "")
                .replace("old", "")
                .replace(" ", "_")
                + "_new.txt";
            table = table
                .into_iter()
                .map(|line| vec![line[0].clone(), line[1].clone(), line[3].clone()])
                .collect();
            write_table("new", table, &fname, mod_choice, &cols);
            print_success();
        }
        3 => {
            let (table, version) = read_table(&fname);
            fname = fname
                .replace(".txt", "")
                .replace("_new", "")
                .replace("_pcsu200", "")
                .replace(" ", "_")
                + "_old.txt";
            write_table("old", table, &fname, mod_choice, &cols);
            print_success();
        }
        4 => {
            let (fname, freqs) = read_list("frequenzen.txt");
            let fname = fname
                .replace(" ", "_")
                .replace("_new", "")
                .replace("_old", "")
                + ".txt";
            write_table(
                "old",
                freqs.iter().map(|f| vec![f.clone()]).collect(),
                &fname,
                mod_choice,
                &cols,
            );
            print_success();
        }
        5 => {
            let (fname, freqs) = read_list("frequenzen.txt");
            let fname = fname
                .replace(" ", "_")
                .replace("_new", "")
                .replace("_old", "")
                + "_new.txt";
            write_table(
                "new",
                freqs.iter().map(|f| vec![f.clone()]).collect(),
                &fname,
                mod_choice,
                &cols,
            );
            print_success();
        }
        6 => {
            let (table, version) = read_table(&fname);
            fname = fname.replace(".txt", "") + "_copy.txt";
            write_table(&version, table, &fname, mod_choice, &cols);
            print_success();
        }
        _ => println!("Invalid mode selected"),
    }

    println!("\nPress any key to close the window:");
    let mut close_input = String::new();
    stdin()
        .read_line(&mut close_input)
        .expect("Failed to read line");
}
