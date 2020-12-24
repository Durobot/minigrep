use std::error::Error;
use std::fs;

// Note the use of "pub".
// This library crate has a public API that can be tested.

pub struct Config<'a>
{
    pub query: &'a str, //or we could use String, and avoid references and lifetimes (<'a>)
    pub filename: &'a str, //String,
}

impl<'a> Config<'a>
{
    // https://stackoverflow.com/questions/30794235/what-is-the-difference-between-a-slice-and-an-array
    //
    // [T; n]  is an array of length n, represented as n adjacent T instances.
    // &[T; n] is purely a reference to that array, represented as a thin pointer to the data.
    // [T]     is a slice, an unsized type; it can only be used through some form of indirection.
    // &[T],   called a slice, is a sized type.
    //         It's a fat pointer, represented as a pointer to the first item and the length of the slice.
    // Arrays thus have their length known at compile time while slice lengths are a runtime matter.
    pub fn new(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3
        { return Err("Not enough command line arguments!"); }

        let query = args[1].as_str(); //args[1].clone(); to get a copy of this String
        let filename = args[2].as_str(); //args[2].clone(); to get a copy of this String

        Ok(Config { query, filename })
    }
}

// trait object Box<dyn Error> means the function will return a type that implements the Error trait,
// but we don’t have to specify what particular type the return value will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>> // () if Ok, see below
{
    // Rather than panic! on an error, the ? operator
    // will return the error value from the current function for the caller to handle,
    // should fs::read_to_string() return Result::Err
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);

    // Using () in Result::Ok variant means we don't really
    // want to return any value if everything's good
    Ok(())
}
