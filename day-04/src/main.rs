use std::cmp::{max, min};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day-04/input.txt")?;
    let prepared = input
        .lines()
        .filter(|line| !line.is_empty() )
        .map(|line| line.splitn(4, |c| c == ',' || c == '-').map(|num| num.parse::<i32>().unwrap_or_default()).collect());

    let full_overlaps = prepared
        .clone()
        .filter(|nums: &Vec<i32>| nums[2] >= nums[0] && nums[3] <= nums[1] || nums[0] >= nums[2] && nums[1] <= nums[3])
        .count();

    println!("Task 1: {}", full_overlaps);

    let overlaps = prepared
        .filter(|nums: &Vec<i32>| max(nums[0], nums[2]) <= min(nums[1], nums[3]))
        .count();

    println!("Task 2: {}", overlaps);

    Ok(())
}
