use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {

    let md_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let filepath = md_path.join("data").join(filename);

    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
