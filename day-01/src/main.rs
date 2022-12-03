use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("day-01/input.txt")?;
    let mut cals: Vec<i32> = content.split("\n\n").map(|elv| {
        elv.split("\n").filter(|cal| { !cal.is_empty() }).map(|cal| { cal.parse::<i32>().unwrap() }).sum()
    }).collect();
    cals.sort_by(|a, b| { b.cmp(a) });
    let highest = cals.get(0).expect("no element found");
    println!("Most calories: {}", highest);

    let second = cals.get(1).expect("no element found");
    let third = cals.get(2).expect("no element found");
    println!("Top three sum: {}", highest + second + third);

    Ok(())
}
