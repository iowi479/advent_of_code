use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::{HashMap, VecDeque};

pub struct Day21 {}

impl Day21 {
    pub fn new() -> Self {
        Self {}
    }
}

/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
fn numpad_position(c: &char) -> Result<(isize, isize)> {
    match c {
        '0' => Ok((3, 1)),
        '1' => Ok((2, 0)),
        '2' => Ok((2, 1)),
        '3' => Ok((2, 2)),
        '4' => Ok((1, 0)),
        '5' => Ok((1, 1)),
        '6' => Ok((1, 2)),
        '7' => Ok((0, 0)),
        '8' => Ok((0, 1)),
        '9' => Ok((0, 2)),
        'A' => Ok((3, 2)),
        _ => Err(anyhow::anyhow!("Invalid key")),
    }
}

///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
fn robopad_position(c: &char) -> Result<(isize, isize)> {
    match c {
        '^' => Ok((0, 1)),
        'v' => Ok((1, 1)),
        '<' => Ok((1, 0)),
        '>' => Ok((1, 2)),
        'A' => Ok((0, 2)),
        _ => Err(anyhow::anyhow!("Invalid key")),
    }
}

fn numpad_sequence(code: &str) -> Vec<String> {
    let mut sequences: Vec<String> = vec![String::new()];
    let mut pointer = (3, 2);

    for c in code.chars() {
        let (y1, x1) = numpad_position(&c).unwrap();
        let (dy, dx) = (y1 - pointer.0, x1 - pointer.1);

        let mut new_sequences = vec![];

        for seq in sequences {
            for path in possible_paths((3, 0), pointer, (dy, dx)) {
                let mut new_seq = seq.clone();
                new_seq.push_str(&path);
                new_sequences.push(new_seq);
            }
        }
        pointer.0 += dy;
        pointer.1 += dx;
        sequences = new_sequences;
    }

    sequences
}

fn possible_paths(
    blocked: (isize, isize),
    start: (isize, isize),
    delta: (isize, isize),
) -> Vec<String> {
    let mut queue = VecDeque::new();
    queue.push_back((delta, String::new()));
    let end = (start.0 + delta.0, start.1 + delta.1);

    let mut paths = vec![];

    while let Some((d, mut path)) = queue.pop_front() {
        if d == (0, 0) {
            path.push('A');
            paths.push(path);
            continue;
        }

        let (dy, dx) = d;

        if dy != 0 {
            let d1 = (dy - dy.signum(), dx);
            let pos_after = (end.0 - d1.0, end.1 - d1.1);
            if pos_after != blocked {
                let mut new_path = path.clone();
                new_path.push(if dy > 0 { 'v' } else { '^' });
                queue.push_back((d1, new_path));
            }
        }

        if dx != 0 {
            let d1 = (dy, dx - dx.signum());
            let pos_after = (end.0 - d1.0, end.1 - d1.1);
            if pos_after != blocked {
                let mut new_path = path.clone();
                new_path.push(if dx > 0 { '>' } else { '<' });
                queue.push_back((d1, new_path));
            }
        }
    }

    paths
}

fn robopad_sequences(code: &str) -> Vec<String> {
    let mut sequences: Vec<String> = vec![String::new()];
    let mut pointer = (0, 2);

    for c in code.chars() {
        let (y1, x1) = robopad_position(&c).unwrap();
        let (dy, dx) = (y1 - pointer.0, x1 - pointer.1);

        let mut new_sequences = vec![];

        for seq in sequences {
            for path in possible_paths((0, 0), pointer, (dy, dx)) {
                let mut new_seq = seq.clone();
                new_seq.push_str(&path);
                new_sequences.push(new_seq);
            }
        }
        pointer.0 += dy;
        pointer.1 += dx;
        sequences = new_sequences;
    }

    sequences
}

type Cache = HashMap<(usize, String), usize>;

fn sequence_len(seq: &str, depth: usize, cache: &mut Cache) -> usize {
    let mut seq_len = 0;

    let mut seq_rest = &seq[..];
    while seq_rest.len() > 0 {
        let (part_len, extra_as) = len_parts(seq_rest);
        let part = &seq_rest[..part_len];
        seq_rest = &seq_rest[(part_len + extra_as)..];

        if let Some(r) = cache.get(&(depth, part.to_string())) {
            seq_len += r + extra_as;
        } else {
            let part_seqs = robopad_sequences(part);

            let l = part_seqs
                .iter()
                .map(|s| match depth {
                    1 => s.len(),
                    _ => sequence_len(s, depth - 1, cache),
                })
                .min()
                .unwrap();

            cache.insert((depth, part.to_string()), l);

            seq_len += l + extra_as;
        }
    }

    seq_len
}

fn len_parts(text: &str) -> (usize, usize) {
    let mut len = (0, 0);
    let mut chars = text.chars();
    let mut a = false;

    while let Some(c) = chars.next() {
        if a {
            match c {
                'A' => len.1 += 1,
                _ => break,
            }
        } else {
            a = c == 'A';
            len.0 += 1;
        }
    }

    len
}

fn numeric_part(code: &str) -> usize {
    code.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

impl Challenge for Day21 {
    fn part1(&self, input: &str) -> Result<String> {
        let words = input.lines().collect::<Vec<&str>>();
        let mut result = 0;
        let mut cache = HashMap::new();

        for word in &words[..] {
            let seqs = numpad_sequence(word);

            let min_len = seqs
                .iter()
                .map(|s| sequence_len(&s, 2, &mut cache))
                .min()
                .unwrap();
            let numeric = numeric_part(word);

            result += min_len * numeric;
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let words = input.lines().collect::<Vec<&str>>();
        let mut result = 0;
        let mut cache = HashMap::new();

        for word in &words[..] {
            let seqs = numpad_sequence(word);

            let min_len = seqs
                .iter()
                .map(|s| sequence_len(&s, 25, &mut cache))
                .min()
                .unwrap();
            let numeric = numeric_part(word);

            result += min_len * numeric;
        }

        Ok(result.to_string())
    }
}
