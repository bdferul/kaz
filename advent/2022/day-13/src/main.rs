use std::cmp::Ordering;

///Enum for the possible parsed items in a packet
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

// impl PacketItem {
//     fn unwrap_int(&self) -> u32 {
//         match self {
//             Self::Int(n) => *n,
//             _ => panic!("unwrap_int"),
//         }
//     }
// }

fn main() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("test.txt").trim();
    let mut sum = 0;

    for (pair, i) in input.split("\n\n").zip(1..) {
        // println!("{pair}");
        let parsed = parse_input(pair);
        if compare(&parsed).is_good_or_other() {
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
        if compare(&[a.to_vec(), b.to_vec()]).is_good_or_other() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let key1 = input2
        .iter()
        .position(|x| *x == parse_input("[[2]]")[0])
        .unwrap()
        + 1;
    let key2 = input2
        .iter()
        .position(|x| *x == parse_input("[[6]]")[0])
        .unwrap()
        + 1;
    println!("Part 2: {}", key1 * key2);
    //dbg!(&input2);
    input2
        .iter()
        .enumerate()
        .for_each(|(i, s)| println!("{i}: {s:?}"));
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

enum PacketOrder {
    Good,
    Bad,
    Other,
}

impl PacketOrder {
    fn is_good_or_other(&self) -> bool {
        matches!(self, Self::Good | Self::Other)
    }

    fn is_good(&self) -> bool {
        matches!(self, Self::Good)
    }
}

///hello:w
fn compare(packets: &[Vec<PacketItem>]) -> PacketOrder {
    use PacketItem::*;
    use PacketOrder::*;

    let mut packet1 = packets[0].clone();
    let mut packet2 = packets[1].clone();

    let mut i = 0;
    loop {
        if i >= packet1.len() {
            if packet1.len() == packet2.len() {
                break Other;
            }
            break Good;
        }
        if i >= packet2.len() {
            break Bad;
        }

        match (packet1[i].clone(), packet2[i].clone()) {
            (Int(a), Int(b)) => {
                if a < b {
                    break Good;
                }
                if a > b {
                    break Bad;
                }
                if a == b {
                    i += 1;
                }
            }
            (Int(a), List(_)) => packet1[i] = List(vec![Int(a)]),
            (List(_), Int(b)) => packet2[i] = List(vec![Int(b)]),
            (List(a), List(b)) => match compare(&[a, b]) {
                Good => break Good,
                Bad => break Bad,
                Other => i += 1,
            },
        }
    }
}

// fn compare(packets: &[Vec<PacketItem>]) -> bool {
//     use PacketItem::*;
//     for i in 0.. {
//         if i >= packets[0].len() {
//             return true;
//         }
//         if i >= packets[1].len() {
//             return false;
//         }
//         let p1 = packets[0][i].clone();
//         let p2 = packets[1][i].clone();

//         macro_rules! list_int {
//             ($a:expr, $b:expr) => {{
//                 let a = list_to_int(List($a));
//                 if let Some(a) = a {
//                     if a == $b {
//                         continue;
//                     }

//                     a < $b
//                 } else {
//                     true
//                 }
//             }};
//         }

//         let outcome = match (p1.clone(), p2.clone()) {
//             (Int(a), Int(b)) => {
//                 if a == b {
//                     continue;
//                 }
//                 a < b
//             }
//             (Int(a), List(b)) => !list_int!(b, a),
//             (List(a), Int(b)) => list_int!(a, b),
//             (List(a), List(b)) => compare(&[a, b]),
//         };

//         return outcome;
//     }

//     false
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cmp_int_int() {
        assert!(compare(&parse_input("[1,2,3,4,5]\n[2,3,4,5,5]")).is_good());
        assert!(!compare(&parse_input("[3]\n[2]")).is_good_or_other());
    }

    #[test]
    fn cmp_list_list() {
        assert!(compare(&parse_input("[]\n[2]")).is_good_or_other());
        assert!(compare(&parse_input("[1,1,1]\n[1,1,1,1]")).is_good_or_other());
        assert!(compare(&parse_input("[[[[1],1]]]\n[2]")).is_good_or_other());
        assert!(!compare(&parse_input("[[]]\n[]")).is_good_or_other());
    }
}
