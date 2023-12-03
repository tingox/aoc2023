// aoc2023 - day 01 - trebuchet, calibration values
//
//
advent_of_code::solution!(1);

// find first and last digit in a line of input - in that order, combine to make two-digit number.
// return the sum of all numbers
//
pub fn part_one(input: &str) -> Option<u32> {
    let mut first = 0;
    let mut last = 0; 
    let numbers = [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut sum = 0;
    let mut numstring = String::new();
    
    for line in input.lines() {
    	//println!("{}", line);
    	let Some(first) = line.chars().position(|c| numbers.contains(&c)) else { todo!() };
    	let rev_input = line.chars().rev().collect::<String>();
    	last = rev_input.chars().position(|c| numbers.contains(&c)).unwrap();
    	//let numstring = &line[first..first+1] + &rev_input[last..last+1];
    	numstring.clear();
    	numstring.push_str(&line[first..first+1]);
    	numstring.push_str(&rev_input[last..last+1]);
    	//println!("numstring = {}", numstring);
    	sum = sum + numstring.parse::<i32>().unwrap();
    }
    //None
    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, None);
        assert_eq!(result, Some(142));        
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
