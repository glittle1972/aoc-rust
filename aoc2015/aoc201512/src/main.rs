use std::fs;

use regex::Regex;

fn main() {
    let results1 = part1("input.txt", false);
    println!("Results1 are {:?}", results1);
    let results2 = part1("input.txt", true);
    println!("Results2 are {:?}", results2);
}
/**
 * For part2, we need to find each inner object ("[]" without "{}" inside it
 * or "{}" without "[]" inside it). Calculate the sum of numbers for each internal
 * object. Work outwards.
 * If the current string contains :"red" then the sum is 0.
 * This would work for part1 but not processing the "red" ignore.
 */
fn part1(filepath: &str, ignore_red: bool) -> Vec<i32> {

    let re_i32 = Regex::new(r#"-{0,1}[0-9]+"#).unwrap();
    let re_obj = Regex::new(r#"\{["a-z:0-9,)(-]+\}"#).unwrap();
    let re_arr = Regex::new(r#"\[["a-z:0-9,)(-]+\]"#).unwrap();
    
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file");
    
    let mut results: Vec<i32> = Vec::new();
    for l in contents.lines() {
        let sum = process_line(&re_i32, &re_obj, &re_arr, l, ignore_red);
        results.push(sum);
    }

    return results;
}

fn process_line(re_i32: &Regex, re_obj: &Regex, re_arr: &Regex, line: &str, ignore_red: bool) -> i32 {
    let mut replaced = line.to_string();
    loop {
        // find every object, record the sum and then remove it.
        let replaced_obj = re_obj.replace_all(&replaced, |caps: &regex::Captures| {
            let obj = &caps[0];
            let obj_sum = do_sum(re_i32, obj, ignore_red);
            let repl = format!("{}{}{}", "(", obj_sum.to_string(), ")");
            return repl;
        }).to_string();
        println!("{}", replaced_obj);
        // find every object, record the sum and then remove it.
        let replaced_arr = re_arr.replace_all(&replaced_obj, |caps: &regex::Captures| {
            let obj = &caps[0];
            // We never ignore red for arrays
            let obj_sum = do_sum(re_i32, obj, false);
            let repl = format!("{}{}{}", "(", obj_sum.to_string(), ")");
            return repl;
        }).to_string();
        println!("{}", replaced_arr);
        if replaced_arr == replaced { // nothing changed, we're done
            break;
        }
        replaced = replaced_arr;
    }
    return do_sum(re_i32, &replaced, ignore_red);
}

fn do_sum(re_i32: &Regex, json: &str, ignore_red: bool) -> i32 {
    if ignore_red && json.contains("red") {
        return 0;
    }
    let result = re_i32.find_iter(json).map(|m| m.as_str().parse::<i32>().unwrap()).sum();
    return result;
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test1() {
        let results = part1("test1.txt", false);
        let expected: Vec<i32> = vec![6, 6, 3, 3, 0, 0, 0, 0];
        assert_eq!(expected, results);
    }

}
