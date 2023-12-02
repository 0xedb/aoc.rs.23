use std::collections::HashMap;

pub fn solution() -> u32 {
    let mut sum = 0;

    let mut color_map = HashMap::with_capacity(3);

    color_map.insert("red", 12);
    color_map.insert("green", 13);
    color_map.insert("blue", 14);

    let input = include_str!("./input/d2.txt");

    for line in input.lines() {
        let mut is_possible = true;

        let mut input_details = line.split(":");
        let game_number = input_details.next().unwrap().split("Game").nth(1).unwrap();

        let gameplay_details = input_details.next().unwrap().split(";");

        for game in gameplay_details {
            let colors = game.split(",");

            for color in colors {
                if is_possible {
                    let mut component = color.trim().split(" ");

                    let value = component.next().unwrap().parse::<u32>().unwrap();
                    let color_name = component.next().unwrap().trim();

                    let limit = color_map.get(color_name).unwrap();

                    if value > *limit {
                        is_possible = false;
                        break;
                    }
                }
            }
        }

        if is_possible {
            sum += game_number.trim().parse::<u32>().unwrap();
        }
    }

    sum
}

pub fn solution2() -> u32 {
    let mut sum = 0;

    let mut color_map = HashMap::with_capacity(3);

    color_map.insert("red", 12);
    color_map.insert("green", 13);
    color_map.insert("blue", 14);

    let input = include_str!("./input/d2.txt");

    for line in input.lines() {
        let mut input_details = line.split(":");
        input_details.next();

        let gameplay_details = input_details.next().unwrap().split(";");

        let mut max_map: HashMap<_, u32> = HashMap::with_capacity(3);

        max_map.insert("red", 0);
        max_map.insert("green", 0);
        max_map.insert("blue", 0);

        for game in gameplay_details {
            let colors = game.split(",");

            for color in colors {
                let mut component = color.trim().split(" ");

                let value = component.next().unwrap().parse::<u32>().unwrap();
                let color_name = component.next().unwrap().trim();

                let max = max_map.get(color_name).unwrap();

                if value > *max {
                    max_map.insert(color_name, value);
                }
            }
        }

        let mut product = 1u32;
        for val in max_map.values() {
            product *= val;
        }

        sum += product;
    }

    sum
}
