fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let mut times: Vec<u64> = Vec::new();
    let mut dists: Vec<u64> = Vec::new();
    for line in input.lines() {
        let Some((metric, values)) = line.split_once(':') else {
            continue;
        };
        let mut values: Vec<u64> = values
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        match metric {
            "Time" => times.append(&mut values),
            "Distance" => dists.append(&mut values),
            _ => panic!(),
        }
    }
    times.into_iter().zip(dists.into_iter()).collect()
}

fn count_wins(input: Vec<(u64, u64)>) -> u32 {
    let mut result: u32 = 1;
    for (racetime, racedist) in input.iter() {
        let mut wins = 0;
        for i in 1..*racetime {
            let time = racetime - i;
            let dist = time * i;

            if dist > *racedist {
                wins += 1;
            }
        }
        result *= wins;
    }
    result
}

fn part1(input: &str) -> u32 {
    count_wins(parse_input(input))
}

fn part2(input: &str) -> u32 {
    let mut time = String::new();
    let mut distance = String::new();
    let mut newinput: Vec<(u64, u64)> = Vec::new();

    for data in parse_input(input) {
        time += &data.0.to_string();
        distance += &data.1.to_string();
    }

    newinput.push((time.parse().unwrap(), distance.parse().unwrap()));
    count_wins(newinput)
}

const INPUT: &str = r#"Time:        61     70     90     66
Distance:   643   1184   1362   1041"#;

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_RESULT: u32 = 288;
    const PART2_RESULT: u32 = 71503;
    const TESTINPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test1() {
        assert_eq!(part1(TESTINPUT), PART1_RESULT);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TESTINPUT), PART2_RESULT);
    }
}
