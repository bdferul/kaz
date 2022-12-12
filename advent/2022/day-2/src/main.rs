const INPUT: &str = include_str!("input.txt");

trait ToScore {
    fn to_score(&self) -> i32;
}

impl ToScore for str {
    fn to_score(&self) -> i32 {
        match self {
            "A"|"X" => 1,//rock
            "B"|"Y" => 2,//paper
            "C"|"Z" => 3,//scissors
            _ => panic!("Invalid input char"),
        }
    }
}

trait Winner {
    fn part1(&self) -> i32;
    fn part2(&self) -> i32;
}

impl Winner for (i32,i32) {
    /// 0 for Player1, 6 for Player2, 3 for tie
    fn part1(&self) -> i32 {
        let (p1,p2) = self;
        let mut order = (1..=3).cycle();
        if p1 == p2 {
            return 3 + p2
        }

        if !(1..=3).contains(p1) || !(1..=3).contains(p2) {
            panic!();
        }

        order.find(|x| x==p1);
        if order.next().unwrap() == *p2 {
            6 + p2
        } else {
            *p2
        }
    }

    fn part2(&self) -> i32 {
        let (p1, outcome) = self;
        let order = 1..=3;
        if !order.contains(p1) {
            panic!()
        }

        let mut cycle = order.cycle();

        cycle.find(|x| x==p1);
        let p2 = cycle.clone().nth(match outcome {
            1 => 1,//lose
            2 => 2,//tie
            3 => 0,//win
            _ => panic!(),
        }).unwrap();

        println!("{:?}", (p1,p2,outcome));

        match outcome {
            1 => p2,
            2 => 3 + p2,
            3 => 6 + p2,
            _ => panic!()
        }
    }
}

fn main() {
    let res: i32 = INPUT
        .lines()
        .map(|s| s.split(' '))
        .map(|mut x| (x.next().unwrap().to_score(), x.next().unwrap().to_score()))
        .map(|x| x.part2())
        //.map(|x| x.part1()) //Part 1
        .sum();

    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::Winner;
    #[test]
    fn winner() {
        assert_eq!((1,2).part1(),8);
        assert_eq!((2,1).part1(),1);
        assert_eq!((3,3).part1(),6);
    }

    #[test]
    fn part2() {
        assert_eq!((1,2).part2(), 4);
        assert_eq!((2,1).part2(), 1);
        assert_eq!((3,3).part2(), 7);
    }
}