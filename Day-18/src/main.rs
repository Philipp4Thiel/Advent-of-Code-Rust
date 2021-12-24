use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Node {
    Value(i32),
    Pair(Box<Node>, Box<Node>),
}

impl Node {
    fn boxed(self) -> Box<Self> { Box::new(self) }

    fn depth(&self) -> usize {
        match self {
            Self::Value(_) => 0,
            Self::Pair(c1, c2) => 1 + max(c1.depth(), c2.depth())
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            Self::Value(x) => *x,
            Self::Pair(c1, c2) => 3 * c1.magnitude() + 2 * c2.magnitude()
        }
    }

    fn split(&self) -> Option<Self> {
        match self {
            Self::Value(x) if *x >= 10 => {
                let a = *x / 2;
                let b = *x - a;
                return Some(Self::Pair(Self::Value(a).boxed(), Self::Value(b).boxed()));
            }
            Self::Pair(c1, c2) => {
                if let Some(a) = c1.split() {
                    return Some(Self::Pair(a.boxed(), c2.clone()));
                }
                if let Some(b) = c2.split() {
                    return Some(Self::Pair(c1.clone(), b.boxed()));
                }
            }
            _ => {}
        }
        None
    }

    fn add_left(&self, c2: Option<Self>) -> Self {
        match (self, c2) {
            (_, None) => self.clone(),
            (Self::Value(x), Some(Self::Value(x2))) => Self::Value(*x + x2),
            (Self::Pair(a, b), c2) => Self::Pair(a.add_left(c2).boxed(), b.clone()),
            _ => panic!()
        }
    }

    fn add_right(&self, c2: Option<Self>) -> Self {
        match (self, c2) {
            (_, None) => self.clone(),
            (Self::Value(x), Some(Self::Value(x2))) => Self::Value(*x + x2),
            (Self::Pair(a, b), c2) => Self::Pair(a.clone(), b.add_right(c2).boxed()),
            _ => panic!()
        }
    }

    fn explode(&self, n: usize) -> Option<(Option<Self>, Self, Option<Self>)> {
        if let Self::Pair(c1, c2) = self {
            if n == 1 {
                return Some((Some(*c1.clone()), Self::Value(0), Some(*c2.clone())));
            }
            if let Some((left, a, right)) = c1.explode(n - 1) {
                let new = Self::Pair(a.boxed(), c2.add_left(right).boxed());
                return Some((left, new, None));
            }
            if let Some((left, b, right)) = c2.explode(n - 1) {
                let new = Self::Pair(c1.add_right(left).boxed(), b.boxed());
                return Some((None, new, right));
            }
        }
        None
    }

    fn reduce(mut self) -> Self {
        loop {
            let depth = self.depth();
            if depth > 4 {
                if let Some((_, n, _)) = self.explode(depth) {
                    self = n;
                    continue;
                }
            }

            match self.split() {
                Some(new_node) => self = new_node,
                None => { break self; }
            }
        }
    }
}

fn parse_input(s: &[u8], pos: usize) -> (usize, Node) {
    match s[pos] {
        b'[' => {
            let (pos, n1) = parse_input(s, pos + 1);
            let (pos, n2) = parse_input(s, pos + 1);

            (pos + 1, Node::Pair(n1.boxed(), n2.boxed()))
        }

        _ => (pos + 1, Node::Value((s[pos] - b'0') as i32))
    }
}

fn add(n1: &Node, n2: &Node) -> Node {
    Node::Pair(n1.clone().boxed(), n2.clone().boxed()).reduce()
}

fn main() {
    let input = BufReader::new(File::open("main.in")
        .expect("file wasn't found.")).lines()
        .map(|line| parse_input(line.unwrap().as_bytes(), 0).1)
        .collect::<Vec<_>>();

    let mut cur = input[0].clone();

    for i in input[1..].iter() {
        cur = add(&cur, i);
    }

    println!("Part 1: {}", cur.magnitude());

    let mut max: i32 = i32::MIN;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j {
                let cur = add(&input[i], &input[j]).magnitude();
                if cur > max {
                    max = cur;
                }
            }
        }
    }
    println!("Part 2: {}", max)
}
