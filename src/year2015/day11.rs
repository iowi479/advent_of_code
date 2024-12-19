use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day11 {}

impl Day11 {
    pub fn new() -> Self {
        Self {}
    }
}

fn find_next(pw: &str) -> String {
    let mut pw = pw.chars().collect::<Vec<char>>();

    loop {
        // next pw
        let mut i = pw.len() - 1;
        loop {
            if pw[i] == 'z' {
                pw[i] = 'a';
                i -= 1;
            } else {
                pw[i] = (pw[i] as u8 + 1) as char;
                break;
            }
        }

        if pw.iter().any(|&c| c == 'i' || c == 'o' || c == 'l') {
            for i in 0..pw.len() {
                if pw[i] == 'i' || pw[i] == 'o' || pw[i] == 'l' {
                    for j in i + 1..pw.len() {
                        pw[j] = 'z';
                    }
                    break;
                }
            }
            continue;
        }

        // three consecutive letters
        if pw
            .windows(3)
            .any(|w| w[0] as u8 + 1 == w[1] as u8 && w[1] as u8 + 1 == w[2] as u8)
        {
            // two pairs of letters
            let pairs = pw
                .windows(2)
                .enumerate()
                .filter(|(_, w)| w[0] == w[1])
                .collect::<Vec<_>>();

            if pairs.len() >= 2 {
                if pairs.windows(2).any(|w| w[0].0 + 1 != w[1].0) {
                    break;
                }
            }
        }
    }

    pw.into_iter().collect()
}

impl Challenge for Day11 {
    fn part1(&self, input: &str) -> Result<String> {
        let pw = input.trim().to_string();
        let next = find_next(&pw);

        Ok(next)
    }

    fn part2(&self, input: &str) -> Result<String> {
        let pw = input.trim().to_string();
        let mut next = find_next(&pw);
        next = find_next(&next);

        Ok(next)
    }
}
