use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq, PartialOrd)]
enum PacketItem {
    Int(u32),
    List(Vec<PacketItem>),
}

impl std::fmt::Debug for PacketItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(n) => write!(f, "{}", n),
            Self::List(l) => write!(f, "{:?}", l),
        }
    }
}

impl PacketItem {
    fn unwrap_int(&self) -> u32 {
        match self {
            Self::Int(n) => *n,
            _ => panic!("unwrap_int"),
        }
    }
}

fn main() {
    let input = include_str!("test.txt").trim();
    let mut sum = 0;

    for (pair, i) in input.split("\n\n").zip(1..) {
        //println!("{pair}");
        let parsed = parse_input(pair);
        if compare(&parsed) {
            sum += i;
        } else {
            //println!("{i}\n{pair}\n");
        }
        //println!("{}: {}", i + 1, compare(&parsed_pair));
    }

    println!("Part 1: {}", sum);

    let mut input2 = input
        .lines()
        .chain("[[2]]\n[[6]]".lines())
        .filter(|s| !s.is_empty())
        .flat_map(parse_input)
        .collect::<Vec<Vec<PacketItem>>>();
    input2.sort_by(|a, b| {
        if compare(&[a.to_vec(), b.to_vec()]) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    //dbg!(&input2);
    input2.iter().for_each(|s| println!("{s:?}"));
}

fn parse_input(input: &str) -> Vec<Vec<PacketItem>> {
    let mut parsed_pair = vec![];
    for line in input.lines() {
        let packet = parse_packet(line);
        //println!("{:?}", packet);

        parsed_pair.push(packet);
    }
    parsed_pair
}

fn parse_packet(arg: &str) -> Vec<PacketItem> {
    use PacketItem::*;
    let mut nest_level = 0;
    let mut start = 1;

    let mut raw_items = vec![];

    for (i, c) in arg.chars().enumerate() {
        match c {
            '[' => nest_level += 1,
            ']' => {
                nest_level -= 1;
                if nest_level == 0 {
                    raw_items.push(arg[start..i].to_string());
                }
            }
            ',' => {
                if nest_level > 1 {
                    continue;
                }
                raw_items.push(arg[start..i].to_string());
                start = i + 1;
            }
            _ => (),
        }
    }

    raw_items
        .into_iter()
        .map(|s| {
            if s.starts_with('[') {
                List(parse_packet(&s))
            } else if let Ok(n) = s.parse() {
                Int(n)
            } else {
                List(vec![])
            }
        })
        .collect()
}

fn compare(packets: &[Vec<PacketItem>]) -> bool {
    use PacketItem::*;
    for i in 0.. {
        if i >= packets[0].len() {
            return true;
        }
        if i >= packets[1].len() {
            return false;
        }
        let p1 = packets[0][i].clone();
        let p2 = packets[1][i].clone();

        macro_rules! list_int {
            ($a:expr, $b:expr) => {
                {
                    let a = list_to_int(List($a));
                    if let Some(a) = a {
                        if a == $b {
                            continue;
                        }
        
                        a < $b
                    } else {
                        true
                    }
                }
            };
        }

        let outcome = match (p1.clone(), p2.clone()) {
            (Int(a), Int(b)) => {
                if a == b {
                    continue;
                }
                a < b
            }
            (Int(a), List(b)) => !list_int!(b, a),
            (List(a), Int(b)) => list_int!(a, b),
            (List(a), List(b)) => compare(&[a, b]),
        };

        return outcome;
    }

    false
}

fn list_to_int(list: PacketItem) -> Option<u32> {
    let PacketItem::List(list) = list else {
        panic!("list not list list_to_int");
    };
    let Some(mut list_first) = list.first() else {
        return None;
    };

    while let PacketItem::List(l) = list_first {
        let Some(zero) = l.first() else {
            return None;
        };

        list_first = zero;
    }

    Some(list_first.unwrap_int())
}

fn cmp_list_int(list: PacketItem, int: PacketItem) -> bool {
    use PacketItem::*;
    let List(list) = list else {
        panic!("list not List");
    };
    if list.is_empty() {
        return true;
    }
    let Int(_) = int else {
        panic!("int not Int")
    };

    let mut list_first = list[0].clone();
    while let List(l) = list_first {
        if l.is_empty() {
            return true;
        }
        list_first = l[0].clone();
    }

    compare(&[vec![list_first], vec![int]])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cmp_int_int() {
        assert!(compare(&parse_input("[1,2,3,4,5]\n[2,3,4,5,5]")));
        assert!(!compare(&parse_input("[3]\n[2]")));
    }

    #[test]
    fn cmp_list_list() {
        assert!(compare(&parse_input("[]\n[2]")));
        assert!(compare(&parse_input("[1,1,1]\n[1,1,1,1]")));
        assert!(compare(&parse_input("[[[[1],1]]]\n[2]")));
        assert!(!compare(&parse_input("[[]]\n[]")));
    }
}
