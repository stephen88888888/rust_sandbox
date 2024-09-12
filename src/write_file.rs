use std::fs::File;
use std::io::{ self, Write };

fn exec() -> io::Result<()> {
    let mut file = File::create("output.txt")?;

    writeln!(file, "Hello, World!")?;
    writeln!(file, "This is a new line of text.")?;

    Ok(())
}
