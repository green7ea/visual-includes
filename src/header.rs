use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug)]
pub struct Header {
    pub path: String,
    pub depth: u32,
    pub dependencies: u32,
    pub visible: bool,
}

pub fn headers_from_file(filename: &str) -> Result<Vec<Header>> {
    let file = File::open(filename)?;
    let mut headers: Vec<Header> = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|line| parse_line(line.trim()))
        .collect();
    count_dependencies(&mut headers);

    Ok(headers)
}

fn parse_line(line: &str) -> Option<Header> {
    let depth = String::from(line).chars().take_while(|&x| x == '.').count() as u32;

    if depth == 0 {
        return None;
    }

    let path: String = String::from(line)
        .chars()
        .skip_while(|&x| x == '.')
        .collect();

    Some(Header {
        path: String::from(path.trim()),
        depth: depth,
        dependencies: 1,
        visible: depth == 1,
    })
}

fn count_dependencies(headers: &mut Vec<Header>) {
    // We could do this much more efficiently by traversing the vector backwards
    // and keeping a stack but that is longer to implement. It would be a good
    // idea to use this implementation to validate the next, more complicated
    // one.
    for i in 0..headers.len() {
        let current_depth = headers[i].depth;

        let deps = &headers.as_slice()[i + 1..headers.len()]
            .iter()
            .take_while(|x| x.depth > current_depth)
            .count();
        headers[i].dependencies = deps.clone() as u32 + 1;
    }
}
