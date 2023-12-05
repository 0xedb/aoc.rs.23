use std::collections::HashSet;

pub fn solution() {
    let mut sum = 0u32;
    const INPUT: &str = include_str!("./input/d4.txt");

    for line in INPUT.lines() {
        let parts: Vec<_> = line.split("|").collect();

        let left: Vec<_> = parts.get(0).unwrap().trim().split(":").collect();
        let right = parts.get(1).unwrap().trim();

        let wins: HashSet<_> = left
            .get(1)
            .unwrap()
            .trim()
            .split(" ")
            .filter(|item| !item.is_empty())
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        let nums: HashSet<_> = right
            .split(" ")
            .filter(|item| !item.is_empty())
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        let intersection = nums.intersection(&wins);
        let count = intersection.count();

        let points = if count == 0 {
            0
        } else {
            2u32.pow((count - 1) as u32)
        };

        sum += points;
    }

    println!("{sum}")
}
