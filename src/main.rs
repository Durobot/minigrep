use std::env;
use std::process;

use minigrep::Config; // A struct from our library crate
// Alternatively, we could refer to it as minigrep::Config in the code

fn main()
{
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err_str|
    {
        println!("Problem parsing arguments: {}", err_str);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // If run() returns Result::Err, get its value and
    // assign it to e, then execute the {} block
    if let Err(e) = minigrep::run(config) // don't have to bring fns from the lib crate into scope
    {
        println!("Application error: {}", e);
        process::exit(1);
    }
    // Since run() returns () within Result::Ok, we don't
    // have to get Ok's value
}
