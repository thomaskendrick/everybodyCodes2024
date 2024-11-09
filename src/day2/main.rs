use std::collections::HashSet;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let runes = lines
        .next()
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .collect();

    let text_lines = lines.skip(1).collect();

    (runes, text_lines)
}

fn part1(runewords: &[&str], text_lines: &[&str]) -> usize {
    runewords.iter().fold(0, |acc, runeword| {
        acc + text_lines[0].matches(runeword).count()
    })
}
fn part2(runewords: &[&str], text_lines: &[&str]) -> usize {
    let mut runes_with_revs: Vec<String> = Vec::new();
    for runeword in runewords {
        runes_with_revs.push(runeword.to_string());
        if runeword.len() > 1 {
            runes_with_revs.push(runeword.chars().rev().collect());
        }
    }
    text_lines.iter().fold(0, |acc, line| {
        let symbols: HashSet<usize> = runes_with_revs
            .iter()
            .flat_map(|runeword| {
                line.match_indices(runeword)
                    .flat_map(|(i, _)| i..i + runeword.len())
                    .collect::<HashSet<usize>>()
            })
            .collect();

        acc + symbols.len()
    })
}

fn main() {
    let (p1runes, p1text) =
        parse_input(include_str!("../../input/everybody_codes_e2024_q2_p1.txt"));
    let pt1_result = part1(&p1runes, &p1text);
    println!("Part 1: {}", pt1_result);
    let (p2runes, p2text) =
        parse_input(include_str!("../../input/everybody_codes_e2024_q2_p2.txt"));
    let pt2_result = part2(&p2runes, &p2text);
    println!("Part 2: {}", pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pt1() {
        assert_eq!(
            part1(
                &["THE", "OWE", "MES", "ROD", "HER"],
                &["AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"],
            ),
            4
        );
        assert_eq!(
            part1(
                &["THE", "OWE", "MES", "ROD", "HER"],
                &["THE FLAME SHIELDED THE HEART OF THE KINGS"],
            ),
            3
        );
        assert_eq!(
            part1(
                &["THE", "OWE", "MES", "ROD", "HER"],
                &["POWE PO WER P OWE R"],
            ),
            2
        );
        assert_eq!(
            part1(&["THE", "OWE", "MES", "ROD", "HER"], &["THERE IS THE END"],),
            3
        );
    }
    #[test]
    fn test_pt2() {
        assert_eq!(
            part2(
                &["THE", "OWE", "MES", "ROD", "HER"],
                &[
                    "AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE",
                    "THE FLAME SHIELDED THE HEART OF THE KINGS",
                    "POWE PO WER P OWE R",
                    "THERE IS THE END"
                ],
            ),
            37
        );
    }
}
