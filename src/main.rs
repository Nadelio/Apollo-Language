fn main() {
    // take in cmdline args
    let args: Vec<String> = std::env::args().collect();

    // collect file name from args, or if `--dir`, then the directory passed in from args

    // iterate through every file and send them to a compile task (thread pool, max 5 threads/tasks)

    // lexer -> parser -> compiler

    // if `-d` or `--debug`, then enable debug mode in every stage

    // if `-v` or `--verbose`, then enable verbose mode in every stage
    // if `-q` or `--quiet`, then disable all output except for errors
    // if `-o` or `--output`, then specify the output directory, defaults to `./out/rs`
    // if `-l` or `--lib`, then specify the library or libraries to pull and compile

    // if `-h` or `--help`, then print the help message

    
}
