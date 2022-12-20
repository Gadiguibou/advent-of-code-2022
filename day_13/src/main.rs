use anyhow::{anyhow, bail};
use std::{
    cmp::Ordering,
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug, Clone)]
enum PacketItem {
    Integer(usize),
    List(Vec<PacketItem>),
}

enum Token {
    Integer(usize),
    ListStart,
    ListEnd,
    ListSeparator,
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = match self.chars.peek() {
            Some('0'..='9') => {
                let mut number = String::new();
                number.push(self.chars.next().unwrap());

                while let Some('0'..='9') = self.chars.peek() {
                    number.push(self.chars.next().unwrap());
                }

                Token::Integer(usize::from_str(&number).unwrap())
            }
            Some('[') => {
                self.chars.next();
                Token::ListStart
            }
            Some(']') => {
                self.chars.next();
                Token::ListEnd
            }
            Some(',') => {
                self.chars.next();
                Token::ListSeparator
            }
            Some(' ') => {
                self.chars.next();
                return self.next();
            }
            Some(_) => panic!("Unexpected character"),
            None => return None,
        };

        Some(token)
    }
}

impl FromStr for PacketItem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lexer = Lexer {
            chars: s.chars().peekable(),
        };

        let mut stack = Vec::new();

        loop {
            let Some(token) = lexer.next() else {
                return Err(anyhow!("Unexpected end of input"));
            };

            match token {
                Token::Integer(value) => {
                    let Some(PacketItem::List(list)) = stack.last_mut() else {
                        bail!("Unexpected integer");
                    };

                    list.push(PacketItem::Integer(value));
                }
                Token::ListStart => {
                    let list = PacketItem::List(Vec::new());
                    stack.push(list);
                }
                Token::ListEnd => {
                    let Some(PacketItem::List(list)) = stack.pop() else {
                        bail!("Unexpected list end");
                    };

                    if let Some(PacketItem::List(parent_list)) = stack.last_mut() {
                        parent_list.push(PacketItem::List(list));
                    } else {
                        return Ok(PacketItem::List(list));
                    }
                }
                Token::ListSeparator => {
                    if stack.is_empty() {
                        bail!("Unexpected list separator");
                    }
                }
            }
        }
    }
}

impl PartialEq for PacketItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PacketItem::Integer(left), PacketItem::Integer(right)) => left == right,
            (PacketItem::List(left), PacketItem::List(right)) => left == right,
            (PacketItem::Integer(left), PacketItem::List(right)) => {
                let left = vec![PacketItem::Integer(*left)];
                left == *right
            }
            (PacketItem::List(left), PacketItem::Integer(right)) => {
                let right = vec![PacketItem::Integer(*right)];
                *left == right
            }
        }
    }
}

impl Eq for PacketItem {}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (PacketItem::Integer(left), PacketItem::Integer(right)) => left.partial_cmp(right),
            (PacketItem::List(left), PacketItem::List(right)) => left.partial_cmp(right),
            (PacketItem::Integer(left), PacketItem::List(right)) => {
                let left = vec![PacketItem::Integer(*left)];
                left.partial_cmp(right)
            }
            (PacketItem::List(left), PacketItem::Integer(right)) => {
                let right = vec![PacketItem::Integer(*right)];
                left.partial_cmp(&right)
            }
        }
    }
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    let packet_pairs = input
        .split("\n\n")
        .map(|packet_pair| packet_pair.split_once('\n').unwrap());

    // Indices of pairs in the right order
    let mut part_1 = 0;

    for (index, (left, right)) in packet_pairs.enumerate() {
        let left = PacketItem::from_str(left)?;
        let right = PacketItem::from_str(right)?;

        if left <= right {
            part_1 += index + 1;
        }
    }

    println!("Part 1: {part_1}");

    // Part 2
    let packets = input
        .split("\n\n")
        .flat_map(|packet_pair| packet_pair.split('\n'))
        .filter(|packet| !packet.is_empty());

    let mut packets: Vec<PacketItem> = packets
        .map(PacketItem::from_str)
        .collect::<Result<_, _>>()?;

    let first_delimiter = PacketItem::List(vec![PacketItem::List(vec![PacketItem::Integer(2)])]);
    let second_delimiter = PacketItem::List(vec![PacketItem::List(vec![PacketItem::Integer(6)])]);

    packets.extend(vec![first_delimiter.clone(), second_delimiter.clone()]);

    packets.sort();

    let part_2 = (1 + packets.iter().position(|packet| packet == &first_delimiter).unwrap()) // First delimiter
        * (1 + packets.iter().position(|packet| packet == &second_delimiter).unwrap()); // Second delimiter

    println!("Part 2: {part_2}");

    Ok(())
}
