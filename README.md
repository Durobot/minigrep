#### Minigrep example from The Rust Programming Language book

This is the [Minigrep example project](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html) from [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) book, my take on it anyway (not 100% verbatim).

To run, use `cargo run <search term> <file name>`

For example `cargo run to poem.txt`

To run a case-insensitive search, set `CASE_INSENSITIVE` environment variable first, for example

`CASE_INSENSITIVE=1 cargo run to poem.txt`

To run the tests, issue the usual `cargo test` command.