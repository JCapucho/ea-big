use std::{env, fs, io, process};

fn main() -> io::Result<()> {
    let path = if let Some(path) = env::args().skip(1).next() {
        path
    } else {
        eprintln!("A second argument with the path to the file must be passed");
        process::exit(1)
    };

    let file = fs::File::open(path)?;

    let (header, entries) = ea_big::from_reader(&file)?;

    println!("======= Header =======");
    println!("name: {}", header.name);
    println!("size: {}", header.size);
    println!("files: {}", header.files);
    println!("indices: {}", header.indices);
    println!("======= Entries ======");
    for entry in entries {
        println!(
            "name: {} | offset: {} | size: {}",
            entry.name, entry.pos, entry.size
        );
    }

    Ok(())
}
