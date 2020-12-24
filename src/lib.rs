use std::error::Error;
use std::fs;
use std::env;

// Note the use of "pub".
// This library crate has a public API that can be tested.

pub struct Config<'a>
{
    pub query: &'a str, //or we could use String, and avoid references and lifetimes (<'a>)
    pub filename: &'a str, //String,
    pub case_sensitive: bool,
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
        // fn env::var() returns a Result that will be the successful Ok
        // variant that contains the value of the environment variable
        // if the environment variable is set, or the Err variant if
        // the environment variable is not set.
        // Result::is_err() method returns true if this Result is set to Err,
        // thus case_sensitive will be true if CASE_INSENSITIVE environment
        // variable is NOT set.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
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

    let results = if config.case_sensitive
    { search(&config.query, &contents) }
    else
    { search_case_insensitive(&config.query, &contents) };

    for line in results
    { println!("{}", line); }

    // Using () in Result::Ok variant means we don't really
    // want to return any value if everything's good
    Ok(())
}

// Returned vector contains string slices that reference file contents,
// i.e. they can live as long as contents (rather than as long as query).
// None of the 3 rules of function parameters' lifetimes cover this case
// (2 reference parameters, 1 reference return value), so we must specify
// return value's lifetime explicitly.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut results = Vec::new();
    for line in contents.lines()
    {
        if line.contains(query)
        { results.push(line); }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    // (1) to_lowercase() returns String, calling to_lowercase() creates new data
    // (2) query on the left is a new variable that is going to shadow old query
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines()
    {
        // Ampersans is necessary because the signature of contains()
        // is defined to take a string slice
        if line.to_lowercase().contains(&query)
        { results.push(line); }
    }

    results
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn case_sensitive()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."; // "duct" != "Duct"

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive()
    {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me."; // "rUsT" is in "Rust:" AND "Trust me."

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
