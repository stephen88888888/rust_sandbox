use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn exec() -> io::Result<()> {
    let file = File::open("example.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
