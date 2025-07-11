pub mod lexer;
pub mod util;

use lexer::Lexer;
use util::{ ERR, SUCCESS, INFO, MSG, DEBUG, RESET };

const VERSION: &str = "0.0.0-A";

fn main() {
    // take in cmdline args
    let args: Vec<String> = std::env::args().map(|x| x.to_lowercase()).collect();

    // if `-f` or `--file`, then specify the file to compile, fail if both `-f` and `--dir` flags are not found
    // if `--dir`, then specify the directory to compile, fail if `-f` flag is also found`

    // (`-f` || `--file`) ^ `--dir` ? pass : fail

    // if `-d` or `--debug`, then enable debug mode in every stage
    // if `-v` or `--verbose`, then enable verbose mode in every stage
    // if `-q` or `--quiet`, then disable all output except for errors
    // if `-o` or `--output`, then specify the output directory, defaults to `./out/rs`
    // if `-l` or `--lib`, then specify the library or libraries to pull and compile
    // if `-h` or `--help`, then print the help message
    // if `--version`, then print the version number

    // if no flags are found, then print the help message
    if args.len() < 2 {
        println!("{}Usage: apollo [options]\nType 'apollo --help' for more information.{}", MSG, RESET);
        std::process::exit(0);
    }

    let file = args.iter().position(|x| x == "-f" || x == "--file");
    let dir = args.iter().position(|x| x == "--dir");

    let help_flag = args.contains(&"-h".to_string()) || args.contains(&"--help".to_string());
    let version_flag = args.contains(&"--version".to_string());
    if help_flag {
        println!("{}Usage: apollo [options]\nOptions:\n  -f, --file <file>    Specify a file to compile\n  --dir <directory>    Specify a directory to compile\n  -d, --debug          Enable debug mode\n  -v, --verbose        Enable verbose mode\n  -q, --quiet          Disable all output except for errors\n  -o, --output <dir>   Specify the output directory (default: ./out)\n  -l, --lib <libs>     Specify libraries to pull and compile\n  -h, --help           Show this help message\n  --version            Show version number\nVersions are in the format <major>.<minor>.<patch>-<Alpha/Beta/Release>\n{}", MSG, RESET);
        std::process::exit(0);
    }
    if version_flag {
        println!("{}Apollo Compiler Version: {}{}{}", MSG, INFO, VERSION, RESET);
        std::process::exit(0);
    }

    if file.is_some() && dir.is_some() {
        eprintln!("{}Error: {}Cannot specify both -f/--file and --dir flags.{}", ERR, MSG, RESET);
        std::process::exit(1);
    } else if file.is_none() && dir.is_none() {
        eprintln!("{}Error: {}Must specify either -f/--file or --dir flag.{}", ERR, MSG, RESET);
        std::process::exit(1);
    }

    // Process other flags like `-d`, `-v`, `-q`, `-o`, `-l`, `-h`, and `--version`
    let debug = args.contains(&"-d".to_string()) || args.contains(&"--debug".to_string());
    let verbose = args.contains(&"-v".to_string()) || args.contains(&"--verbose".to_string());
    let quiet = args.contains(&"-q".to_string()) || args.contains(&"--quiet".to_string());

    // debug mode does small things like print the file being compiled, verbose mode prints additional information
    // verbose is only available if debug mode is enabled, quiet mode is only available if debug mode is off
    if !debug && verbose {
        eprintln!("{}Error: {}-v/--verbose flag requires -d/--debug flag to be enabled.{}", ERR, MSG, RESET);
        std::process::exit(1);
    }

    if (debug || verbose) && quiet {
        eprintln!("{}Error: {}-q/--quiet flag cannot be used with -d/--debug or -v/--verbose flags.{}", ERR, MSG, RESET);
        std::process::exit(1);
    }

    let _output_dir = args.iter().find(|&x| x == "-o" || x == "--output").and_then(|x| {
        let index = args.iter().position(|y| y == x)?;
        if index + 1 < args.len() {
            Some(args[index + 1].clone())
        } else {
            eprintln!("{}Error: {}-o/--output flag requires a directory argument.{}", ERR, MSG, RESET);
            std::process::exit(1);
        }
    }).unwrap_or_else(|| "./out/".to_string());
    // -l and --lib flags would be processed similarly, but for now we will skip them

    // iterate through every file and send them to a compile task (thread pool, max 5 threads/tasks)
    // lexer -> parser -> compiler

    let mut mode: u8 = 0; // 0: quiet, 1: debug, 2: verbose
    
    if quiet { mode = 0; }
    else if verbose { mode = 2; }
    else if debug { mode = 1; }

    if dir.is_some() {
        let dir = args[dir.unwrap() + 1].clone();
        if mode > 0 { println!("{}Compiling directory: {}{}{}", DEBUG, INFO, dir, RESET); }
        // Here you would implement the logic to read files from the directory and compile them
    } else if file.is_some() {
        let file = args[file.unwrap() + 1].clone(); // filepath
        if mode > 0 { println!("{}Compiling file: {}{}{}", DEBUG, INFO, file, RESET); }
        
        let result = Lexer::new(file, mode).begin();
        match result {
            Ok(_tokens) => {
                if mode > 0 { println!("{}Lexing completed successfully.{}", SUCCESS, RESET); }
            },
            Err(e) => {
                e.print();
                std::process::exit(1);
            }
        }
    }
}