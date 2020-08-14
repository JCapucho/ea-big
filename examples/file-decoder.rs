use std::{
    env, fs,
    io::{Read, Result},
    path::PathBuf,
    process,
};

fn main() -> Result<()> {
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

        let mut embed = ea_big::open_file(&file, &entry);

        let mut buf = Vec::with_capacity(entry.size as usize);

        assert_eq!(embed.read_to_end(&mut buf)?, entry.size as usize);

        let path = PathBuf::from(entry.name.replace("\\", "/"));

        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, buf)?;
    }

    Ok(())
}
