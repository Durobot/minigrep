use std::env;
use std::fs;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

// https://stackoverflow.com/questions/30794235/what-is-the-difference-between-a-slice-and-an-array
//
// [T; n]  is an array of length n, represented as n adjacent T instances.
// &[T; n] is purely a reference to that array, represented as a thin pointer to the data.
// [T]     is a slice, an unsized type; it can only be used through some form of indirection.
// &[T],   called a slice, is a sized type.
//         It's a fat pointer, represented as a pointer to the first item and the length of the slice.
// Arrays thus have their length known at compile time while slice lengths are a runtime matter.
fn parse_config(args: &[String]) -> (&str, &str)
{
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
