fn parse_insects(str: &str) -> Vec<Option<InsectType>> {
    str.chars()
        .map(|c| match c {
            'A' => Some(InsectType::Ant),
            'B' => Some(InsectType::Beetle),
            'C' => Some(InsectType::Cockroach),
            'D' => Some(InsectType::DragonFly),
            _ => None,
        })
        .collect()
}

#[derive(Debug)]
enum InsectType {
    Ant,
    Beetle,
    Cockroach,
    DragonFly,
}

fn calculate_base_potions(insect: &Option<InsectType>) -> usize {
    match insect {
        Some(InsectType::Ant) => 0,
        Some(InsectType::Beetle) => 1,
        Some(InsectType::Cockroach) => 3,
        Some(InsectType::DragonFly) => 5,
        None => 0,
    }
}

fn calculate_potions(input: &[Option<InsectType>], insect_group_size: usize) -> usize {
    input
        .chunks(insect_group_size)
        .fold(0usize, |acc, insects| {
            let base_potions = insects.iter().map(calculate_base_potions).sum::<usize>();
            let insect_count = insects.iter().filter(|x| x.is_some()).count();
            let additional_potions = if insect_count > 0 {
                (insect_count - 1) * insect_count
            } else {
                0
            };
            acc + additional_potions + base_potions
        })
}
fn main() {
    // Part 1
    let p1input = parse_insects(include_str!("../../input/everybody_codes_e2024_q1_p1.txt"));
    let pt1result = calculate_potions(&p1input, 1);
    println!("Part 1: {}", pt1result);
    // Part 2
    let p2input = parse_insects(include_str!("../../input/everybody_codes_e2024_q1_p2.txt"));
    let pt2result = calculate_potions(&p2input, 2);
    println!("Part 2: {}", pt2result);
    // Part 3
    let p3input = parse_insects(include_str!("../../input/everybody_codes_e2024_q1_p3.txt"));
    let pt3result = calculate_potions(&p3input, 3);
    println!("Part 3: {}", pt3result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pt1() {
        let input = parse_insects("ABBAC");
        assert_eq!(calculate_potions(&input, 1), 5);
    }
    #[test]
    fn test_pt2() {
        let input = parse_insects("AxBCDDCAxD");
        assert_eq!(calculate_potions(&input, 2), 28);
    }
    #[test]
    fn test_pt3() {
        let input = parse_insects("xBxAAABCDxCC");
        assert_eq!(calculate_potions(&input, 3), 30);
    }
}
