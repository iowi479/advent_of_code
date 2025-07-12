use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day15 {}

impl Day15 {
    pub fn new() -> Self {
        Self {}
    }
}

struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    input
        .lines()
        .map(|line| {
            let parts = line.replace(",", "");
            let parts = parts.split(" ").collect::<Vec<_>>();

            Ingredient {
                capacity: parts[2].parse().unwrap(),
                durability: parts[4].parse().unwrap(),
                flavor: parts[6].parse().unwrap(),
                texture: parts[8].parse().unwrap(),
                calories: parts[10].parse().unwrap(),
            }
        })
        .collect()
}

struct Amounts {
    length: usize,
    max: i64,
    current: Option<Vec<i64>>,
}

impl Amounts {
    fn new(length: usize, max: i64) -> Self {
        let mut current = vec![0; length];
        current[length - 1] = max;

        Self {
            length,
            max,
            current: Some(current),
        }
    }
}

impl Iterator for Amounts {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.take() {
            let mut next = current.clone();

            for i in (0..self.length - 1).rev() {
                let sum = next[..self.length - 1].iter().sum::<i64>();
                if sum == self.max {
                    next[i] = 0;
                } else if next[i] == self.max {
                    next[i] = 0;
                } else {
                    next[i] += 1;
                    break;
                }

                if i == 0 {
                    self.current = None;
                    return Some(current);
                }
            }

            next[self.length - 1] = self.max - next[..self.length - 1].iter().sum::<i64>();
            self.current = Some(next);
            return Some(current);
        }
        None
    }
}

impl Challenge for Day15 {
    fn part1(&self, input: &str) -> Result<String> {
        let ingredients = parse_input(input);
        let mut result = 0;
        let amounts = Amounts::new(ingredients.len(), 100);

        for a in amounts {
            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;

            for i in 0..ingredients.len() {
                capacity += ingredients[i].capacity * a[i];
                durability += ingredients[i].durability * a[i];
                flavor += ingredients[i].flavor * a[i];
                texture += ingredients[i].texture * a[i];
            }

            if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
                continue;
            }

            let score = capacity * durability * flavor * texture;
            if score > result {
                result = score;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let ingredients = parse_input(input);
        let mut result = 0;
        let amounts = Amounts::new(ingredients.len(), 100);

        for a in amounts {
            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            let mut calories = 0;

            for i in 0..ingredients.len() {
                capacity += ingredients[i].capacity * a[i];
                durability += ingredients[i].durability * a[i];
                flavor += ingredients[i].flavor * a[i];
                texture += ingredients[i].texture * a[i];
                calories += ingredients[i].calories * a[i];
            }

            if calories != 500 {
                continue;
            }
            if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
                continue;
            }

            let score = capacity * durability * flavor * texture;
            if score > result {
                result = score;
            }
        }

        Ok(result.to_string())
    }
}
