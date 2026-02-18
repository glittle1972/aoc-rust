use std::fmt;
use std::fs;
use std::time::Instant;

struct Matrix {
    width: usize,
    height: usize,
    array: Vec<char>,
}

impl Matrix {
    pub fn new(row: usize, col: usize) -> Self {
        Self { width: col, height: row, array: vec!['.'; row * col]}
    }

    pub fn set(&mut self, row: usize, col: usize, value: char) {
        self.array[row * self.width + col] = value;
    }

    pub fn get(&self, row: isize, col: isize) -> Option<char> {
        if row < 0 || (row as usize) >= self.height || col < 0 || (col as usize) >= self.width {
            return None;
        }
        return Some(self.array[(row as usize) * self.width + (col as usize)]);
    }

    fn neighbours(&self, row: usize, col: usize) -> u8 {
        let mut count: u8 = 0;
        match self.get((row as isize) - 1, (col as isize) - 1) {
            Some('#') => count += 1,
            _ => (),
        }
        match self.get((row as isize) - 1, col as isize) {
            Some('#') => count += 1,
            _ => (),
        }
        match self.get((row as isize) - 1, (col as isize) + 1) {
            Some('#') => count += 1,
            _ => (),
        }
        match self.get(row as isize, (col as isize) + 1) {
            Some('#') => count += 1,
            _ => (),
        }
        match self.get((row as isize) + 1, (col as isize) + 1) {
            Some('#') => count += 1,
            _ => (),
        }
        match self.get((row as isize) + 1, col as isize) {
            Some('#') => count += 1,
            _ => (),
        }
        match self.get((row as isize) + 1, (col as isize) - 1) {
            Some('#') => count += 1,
            _ => (),
        }
        match self.get(row as isize, (col as isize) - 1) {
            Some('#') => count += 1,
            _ => (),
        }
        return count;
    }

    pub fn iterate(&mut self) {
        let mut new_array = vec!['.'; self.height * self.width];
        for r in 0..self.height {
            for c in 0..self.width {
                let count = self.neighbours(r, c);
                if self.get(r as isize, c as isize) == Some('#') && ( count == 2 || count == 3) {
                    new_array[r * self.width + c] = '#';
                } else if self.get(r as isize, c as isize) == Some('.') && count == 3 {
                    new_array[r * self.width + c] = '#';            
                } else {
                    new_array[r * self.width + c] = '.';
                }
            }
        }
        for i in 0..self.array.len() {
            self.array[i] = new_array[i];
        }
    }

    pub fn num_lights(&mut self) -> u32 {
        let mut count = 0;
        for c in &self.array {
            if c ==  &'#' {
                count += 1;
            }
        }
        return count;
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.height {
            for c in 0..self.height {
                let _ = write!(f, "{}", self.get(r as isize, c as isize).unwrap());
            }
            let _ = writeln!(f, "{}", "");
        }
        writeln!(f, "{}", "")
    }
}

fn main() {
    let num_lights = part1("input.txt", 100);
    println!("result1 is {}", num_lights);
}

fn part1(filepath: &str, iterations: usize) -> u32 {
    let start = Instant::now();

    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");    
    let mut matrix = parse_lines(contents);
    dbg!(&matrix);

    for _ in 0..iterations {
        matrix.iterate();
        dbg!(&matrix);
    }
    let num_lights = matrix.num_lights();

    let dur = start.elapsed();
    println!("Duration = {:?}", dur);

    return num_lights;
}

fn parse_lines(lines: String) -> Matrix{
    let height = lines.lines().count();
    let width = lines.lines().last().unwrap().len();
    let mut matrix = Matrix::new(height, width);
    for (r, line) in lines.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            matrix.set(r, c, ch);
        }
    }
    return matrix;
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let num_lights = part1("test.txt", 4);
        assert_eq!(4, num_lights);
    }

}
