// $ cargo run -- searchstring example-filename.txt
// dbg! macro is used to print the value of args
// $ IGNORE_CASE=1 cargo run -- to poem.txt
// $ cargo run -- to ./poem.txt > output.txt (output to file)

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    // simiar: let config = Config::build(&args).unwrap_or_else(|err| {
    // passing ownership of args to Config::build
    let config = match Config::build(env::args()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Problem parsing arguments: {e}"); // eprintln! prints to standard error system
            process::exit(1);
        },
    };

    // we don’t use `unwrap_or_else` because we dont need to return the unwrapped value, which would only be ().
    // this means we use `if let` to check if the run function returns an error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
