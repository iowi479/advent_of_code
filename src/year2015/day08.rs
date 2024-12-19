use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day08 {}

impl Day08 {
    pub fn new() -> Self {
        Self {}
    }
}

struct Literal<'a> {
    text: &'a str,
    chars: usize,
}

fn parse_input(input: &str) -> Vec<Literal> {
    input
        .lines()
        .map(|line| Literal {
            text: line,
            chars: line.len(),
        })
        .map(|lit| {
            let mut length = 0;
            let mut i = 0;
            while i < lit.chars {
                if lit.text[i..].starts_with(r#"\\"#) {
                    i += 2;
                    length += 1;
                } else if lit.text[i..].starts_with(r#"\""#) {
                    i += 2;
                    length += 1;
                } else if lit.text[i..].starts_with(r#"\x"#) {
                    i += 4;
                    length += 1;
                } else {
                    i += 1;
                    length += 1;
                }
            }

            Literal {
                text: lit.text,
                chars: length - 2,
            }
        })
        .collect()
}

impl Challenge for Day08 {
    fn part1(&self, input: &str) -> Result<String> {
        let literals = parse_input(input);
        let mut result = 0;

        for lit in literals {
            result += lit.text.len() - lit.chars;
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let lines = input
            .lines()
            .map(|line| {
                let mut escaped = String::new();
                escaped.push('"');
                for c in line.chars() {
                    if c == '\\' || c == '"' {
                        escaped.push('\\');
                    }
                    escaped.push(c);
                }
                escaped.push('"');
                escaped
            })
            .collect::<Vec<String>>();

        let result = self.part1(&lines.join("\n"))?;
        Ok(result.to_string())
    }
}
