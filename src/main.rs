use std::env;
use std::process;

use minigrep::Config; // A struct from our library crate
// Alternatively, we could refer to it as minigrep::Config in the code

fn main()
{
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err_str|
    {
        eprintln!("Problem parsing arguments: {}", err_str);
        process::exit(1);
    });

    // If run() returns Result::Err, get its value and
    // assign it to e, then execute the {} block
    if let Err(e) = minigrep::run(config) // don't have to bring fns from the lib crate into scope
    {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    // Since run() returns () within Result::Ok, we don't
    // have to get Ok's value
}
