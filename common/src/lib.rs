use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let md_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let filepath = md_path.join("..").join("data").join(filename);

    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_numbers<P: std::str::FromStr>(line: &str, start_idx: Option<usize>, end_idx: Option<usize>) -> Vec<P> {
    let mut result = vec![];

    if let Some(sl) = line.get(start_idx.unwrap_or(0)..end_idx.unwrap_or(line.len())) {
        for num_str in sl.split(' ') {
            if let Ok(number) = num_str.trim().parse::<P>() {
                result.push(number);
            }
        }
    }

    return result;
}
