use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Could not read file");
    
    let mut lines = vec![];
    parse_lines(&contents, &mut lines);
    
    let result = part1(&lines);
    println!("Result is {result}");

    let mut trans = vec![];
    transpose(&lines, &mut trans);
    let result2 = part1(&trans);
    println!("Resutl2 is {result2}")
}

fn parse_lines(contents: &str, lines: &mut Vec<Vec<usize>>) {
    for row in contents.lines() {
        let v: Vec<usize> = row.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        lines.push(v);
    }
}

fn transpose(rc: &Vec<Vec<usize>>, cr: &mut Vec<Vec<usize>>) {
    for c in 0..rc[0].len() {
        let mut row = vec![];
        for r in 0..rc.len() {
            row.push(rc[r][c]);            
        }
        cr.push(row);
    }
}

fn is_valid(triple: &Vec<usize>) -> bool {
    if triple[0] + triple[1] > triple[2] {
        true
    } else {
        false
    }
}

fn part1(lines: &Vec<Vec<usize>>) -> usize {

    let mut valid: usize = 0;

    // Work horizontally in groups of three
    for row in lines {
        for i in (0..row.len()).step_by(3) {
            let mut v = vec![row[i], row[i+1], row[i+2]];
            v.sort();
            if is_valid(&v) {
                valid += 1;
            }
        }
    }

    valid
}
