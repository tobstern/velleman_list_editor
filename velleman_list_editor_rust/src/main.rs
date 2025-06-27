/// Velleman List Editor - A tool for converting between different table formats
/// and managing frequency lists for Velleman devices.
///
/// This application provides functionality to:
/// - Convert tables to frequency lists
/// - Transform between old (PCGU1000) and new (PCSU200) table formats
/// - Create tables from frequency lists
/// - Modify table values
use crossterm::terminal;
use std::fs::File;
use std::io::{self, stdin, BufRead, Write};
use std::path::Path;
use sysinfo::System;

/// Represents the version/format of a table
#[derive(Debug, Clone, PartialEq)]
enum TableVersion {
    Old, // PCGU1000 format (5 columns)
    New, // PCSU200 format (3 columns)
}

impl std::fmt::Display for TableVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableVersion::Old => write!(f, "old"),
            TableVersion::New => write!(f, "new"),
        }
    }
}

/// Configuration for waveform parameters
#[derive(Debug)]
struct WaveformConfig {
    waveform: String, // 1=sine, 2=rect, 3=tri
    voltage: String,  // Peak-to-peak voltage in Volts
    duration: String, // Duration in seconds
}

impl WaveformConfig {
    /// Creates a new WaveformConfig with validation
    fn new(waveform: String, voltage: String, duration: String) -> Result<Self, String> {
        // Validate numeric inputs
        for (_, value) in [
            ("waveform", &waveform),
            ("voltage", &voltage),
            ("duration", &duration),
        ] {
            if !value.replace(".", "").chars().all(char::is_numeric) {
                return Err(format!("The given String {} is not a number", value));
            }
        }
        Ok(WaveformConfig {
            waveform,
            voltage,
            duration,
        })
    }

    /// Converts the configuration to a vector for table operations
    fn to_vec(&self) -> Vec<String> {
        vec![
            self.waveform.clone(),
            self.voltage.clone(),
            self.duration.clone(),
        ]
    }
}

/// Gets the terminal size, returning an IO error if unable to determine
fn get_terminal_size() -> io::Result<(u16, u16)> {
    terminal::size().map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// Sets up the terminal based on the current platform
fn setup_terminal() {
    let platform = System::name().unwrap_or_default();

    if platform == "Linux" {
        if let Ok((width, height)) = get_terminal_size() {
            println!("\x1b[8;{};{}t", height / 10, width / 10);
        }
    } else if platform == "Windows" {
        println!("Windows terminal resizing is not directly supported.");
    }
}

fn get_user_input(prompt: &str) -> io::Result<String> {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn get_waveform_config() -> Result<WaveformConfig, String> {
    let waveform = get_user_input("Type in the waveform: [1=sine, 2=rect, 3=tri]")
        .map_err(|e| e.to_string())?;
    let voltage =
        get_user_input("Type in the peak-to-peak voltage in [Volt]:").map_err(|e| e.to_string())?;
    let duration =
        get_user_input("Type in the duration in [seconds]:").map_err(|e| e.to_string())?;

    WaveformConfig::new(waveform, voltage, duration)
}

fn find_files_with_pattern(pattern: &str) -> Result<Vec<String>, String> {
    let glob_pattern = format!("./*{}*.txt", pattern);
    let paths =
        glob::glob(&glob_pattern).map_err(|e| format!("Failed to read glob pattern: {}", e))?;

    let mut files = Vec::new();
    for path in paths {
        match path {
            Ok(p) => {
                println!("{}", p.display());
                files.push(p.display().to_string());
            }
            Err(e) => eprintln!("Error reading path: {}", e),
        }
    }
    Ok(files)
}

fn get_filename() -> io::Result<String> {
    println!(
        "As a help, type in a fraction of the filename.\nAll similar filenames will be shown:"
    );
    let partial_str = get_user_input("")?;

    if let Err(e) = find_files_with_pattern(&partial_str) {
        eprintln!("Warning: {}", e);
    }

    let mut fname = get_user_input("Type in the filename:")?;
    if !fname.ends_with(".txt") {
        fname.push_str(".txt");
    }
    Ok(fname)
}

/// Prints a success message
fn print_success() {
    println!("\nThe program finished this task successfully!");
}

/// Reads a table file and determines its version based on column count
/// Returns the table data and version type
fn read_table(fname: &str) -> io::Result<(Vec<Vec<String>>, TableVersion)> {
    let mut table = Vec::new();
    let mut version = TableVersion::New;

    let lines = read_lines(fname)?;
    for line in lines {
        let ip = line?;
        let cols: Vec<String> = ip.split('\t').map(|s| s.to_string()).collect();
        version = if cols.len() == 5 {
            TableVersion::Old
        } else {
            TableVersion::New
        };
        table.push(cols);
    }

    Ok((table, version))
}

/// Reads a simple list from a file (one item per line)
fn read_list(fname: &str) -> io::Result<Vec<String>> {
    let mut list = Vec::new();

    let lines = read_lines(fname)?;
    for line in lines {
        let ip = line?;
        list.push(ip);
    }

    Ok(list)
}

/// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Writes table data to a file with proper formatting based on version
fn write_table(
    version: &TableVersion,
    lines: Vec<Vec<String>>,
    fname: &str,
    mod_choice: i32,
    config: &WaveformConfig,
) -> io::Result<()> {
    let mut file = File::create(fname)?;
    let cols = config.to_vec();

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

        if *version == TableVersion::Old {
            line.insert(2, "0\t".to_string());
        }

        let line_str = line.join("").trim().to_string();
        if j < lines.len() - 1 {
            writeln!(file, "{}", line_str)?;
        } else {
            write!(file, "{}", line_str)?;
        }
    }
    Ok(())
}

fn generate_output_filename(input_fname: &str, suffix: &str, version: &str) -> String {
    input_fname
        .replace(".txt", "")
        .replace("old", "")
        .replace("_new", "")
        .replace("_pcsu200", "")
        .replace(" ", "_")
        + suffix
        + version
        + ".txt"
}

fn handle_conversion_mode_1(fname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (table, _) = read_table(fname)?;
    let mut freqs = String::new();
    for line in table {
        if line.len() > 1 {
            freqs.push_str(&format!("{}, ", line[1]));
        }
    }
    let output = format!(
        "{}: {}",
        fname.replace(".txt", "").replace(" ", "_"),
        freqs.trim_end_matches(", ")
    );
    let mut file = File::create("frequenzen.txt")?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

fn handle_conversion_mode_2(
    fname: &str,
    config: &WaveformConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut table, _) = read_table(fname)?;
    let new_fname = generate_output_filename(fname, "_", "new");
    table = table
        .into_iter()
        .filter_map(|line| {
            if line.len() >= 4 {
                Some(vec![line[0].clone(), line[1].clone(), line[3].clone()])
            } else {
                None
            }
        })
        .collect();
    write_table(&TableVersion::New, table, &new_fname, 2, config)?;
    Ok(())
}

fn handle_conversion_mode_3(
    fname: &str,
    config: &WaveformConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let (table, _) = read_table(fname)?;
    let new_fname = generate_output_filename(fname, "_", "old");
    write_table(&TableVersion::Old, table, &new_fname, 3, config)?;
    Ok(())
}

fn handle_conversion_mode_4(config: &WaveformConfig) -> Result<(), Box<dyn std::error::Error>> {
    let freqs = read_list("frequenzen.txt")?;
    let output_fname = "output.txt";
    write_table(
        &TableVersion::Old,
        freqs.iter().map(|f| vec![f.clone()]).collect(),
        output_fname,
        4,
        config,
    )?;
    Ok(())
}

fn handle_conversion_mode_5(config: &WaveformConfig) -> Result<(), Box<dyn std::error::Error>> {
    let freqs = read_list("frequenzen.txt")?;
    let output_fname = "output_new.txt";
    write_table(
        &TableVersion::New,
        freqs.iter().map(|f| vec![f.clone()]).collect(),
        output_fname,
        5,
        config,
    )?;
    Ok(())
}

fn handle_conversion_mode_6(
    fname: &str,
    config: &WaveformConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let (table, version) = read_table(fname)?;
    let new_fname = fname.replace(".txt", "") + "_copy.txt";
    write_table(&version, table, &new_fname, 6, config)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_terminal();

    println!("\nWelcome to the Velleman List Editor!\n");

    let selection = "Please select one of the following modi: \n \
            1.) \tFor the conversion of the (old|new) table into the list in 'frequenzen.txt' press '1'! \n \
            2.) \tFor conversion of old table format (PCGU1000) into the new one (PCSU200) press '2'! \n \
            3.) \tFor conversion of new table format (PCSU200) into the old one (PCGU1000) press '3'! \n \
            4.) \tFor creation of the old table format from the list in 'frequenzen.txt' press '4'! \n \
            5.) \tFor creation of the new table format from the list in 'frequenzen.txt' press '5'!\n \
            6.) \tFor changes to the values only press '6'!";

    println!("{}", selection);

    let input = get_user_input("")?;
    let mod_choice: i32 = input.parse().map_err(|_| "Please enter a valid number")?;

    let mut fname = String::new();
    if mod_choice < 4 || mod_choice == 6 {
        fname = get_filename()?;
    }

    let mut config: Option<WaveformConfig> = None;
    if mod_choice > 1 {
        config = Some(get_waveform_config()?);
    }

    match mod_choice {
        1 => {
            handle_conversion_mode_1(&fname)?;
            print_success();
        }
        2 => {
            if let Some(cfg) = config {
                handle_conversion_mode_2(&fname, &cfg)?;
                print_success();
            }
        }
        3 => {
            if let Some(cfg) = config {
                handle_conversion_mode_3(&fname, &cfg)?;
                print_success();
            }
        }
        4 => {
            if let Some(cfg) = config {
                handle_conversion_mode_4(&cfg)?;
                print_success();
            }
        }
        5 => {
            if let Some(cfg) = config {
                handle_conversion_mode_5(&cfg)?;
                print_success();
            }
        }
        6 => {
            if let Some(cfg) = config {
                handle_conversion_mode_6(&fname, &cfg)?;
                print_success();
            }
        }
        _ => {
            println!("Invalid mode selected");
            return Err("Invalid mode selection".into());
        }
    }

    get_user_input("\nPress any key to close the window:")?;
    Ok(())
}
