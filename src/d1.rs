use std::collections::HashMap;

pub fn solution() -> u32 {
    let mut sum: u32 = 0;
    let input = include_str!("./input/d1.txt");

    for line in input.lines() {
        let mut last = None;
        let mut first = None;

        for ch in line.chars() {
            if ch.is_digit(10) {
                if first.is_none() {
                    first = Some(ch)
                } else {
                    last = Some(ch)
                }
            }
        }

        if last.is_none() {
            last = first.clone()
        }

        let total = format!(
            "{}{}",
            first.unwrap().to_string().parse::<u32>().unwrap(),
            last.unwrap().to_string().parse::<u32>().unwrap()
        )
        .parse::<u32>()
        .unwrap();

        sum += total;
    }

    sum
}

pub fn solution2() -> u32 {
    let mut sum: u32 = 0;
    let input = include_str!("./input/d1.txt");

    let mut number_map: HashMap<&str, u32> = HashMap::with_capacity(9);
    number_map.insert("one", 1);
    number_map.insert("two", 2);
    number_map.insert("three", 3);
    number_map.insert("four", 4);
    number_map.insert("five", 5);
    number_map.insert("six", 6);
    number_map.insert("seven", 7);
    number_map.insert("eight", 8);
    number_map.insert("nine", 9);

    for line in input.lines() {
        let mut last = None;
        let mut first = None;

        for (i, ch) in line.chars().enumerate() {
            if ch.is_digit(10) && ch != '0' {
                if first.is_none() {
                    first = Some((i, ch.to_string().parse::<u32>().unwrap()))
                } else {
                    last = Some((i, ch.to_string().parse::<u32>().unwrap()))
                }
            }
        }

        if last.is_none() {
            last = first.clone();
        }

        number_map.keys().for_each(|key| {
            let k_index = line.find(*key);
            let kl_index = line.rfind(*key);
            let value = number_map.get(*key);

            if let Some(i) = k_index {
                if first.is_none() {
                    first = Some((i, *value.unwrap()))
                } else if first.is_some() {
                    if i < first.unwrap().0 {
                        first = Some((i, *value.unwrap()))
                    }
                }

                if last.is_none() {
                    last = Some((i, *value.unwrap()))
                } else if last.is_some() {
                    if i > last.unwrap().0 {
                        last = Some((i, *value.unwrap()))
                    }
                }
            }

            if let Some(i) = kl_index {
                if first.is_none() {
                    first = Some((i, *value.unwrap()))
                } else if first.is_some() {
                    if i < first.unwrap().0 {
                        first = Some((i, *value.unwrap()))
                    }
                }

                if last.is_none() {
                    last = Some((i, *value.unwrap()))
                } else if last.is_some() {
                    if i > last.unwrap().0 {
                        last = Some((i, *value.unwrap()))
                    }
                }
            }
        });

        let total = format!("{}{}", first.unwrap().1, last.unwrap().1)
            .parse::<u32>()
            .unwrap();

        sum += total;
    }
    sum
}
