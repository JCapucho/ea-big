# ea-big

`ea-big` is a rust library to open EA's `.big` game files it provides an easy way to read the file header and indices table as well as an easy to use abstraction to read the embedded files.

## Usage

Add the library to your `Cargo.toml` like so:

```
ea-big = "0.1"
```

Calling `ea_big::from_reader` on a type implementing the [`Read`][0] trait will return a `Result` with the file `Header` and a `Vec` of the entries in the index table.

`ea_big::open_file` takes a type implementing the [`Read`][0] and [`Seek`][1] traits and a reference to a Entry in the Index table and provides a type that also implements [`Read`][0] and [`Seek`][1], it works like a normal file but it only refers to the data in the embedded file indicated by the table entry.

## Example

The examples provides a program that can read a `.big` file and print the header and indices table to the terminal to run use:

```
cargo run --example file-decoder -- FILE
```

Where `FILE` is the `.big` file

[0]: https://doc.rust-lang.org/std/io/trait.Read.html
[1]: https://doc.rust-lang.org/std/io/trait.Seek.html
