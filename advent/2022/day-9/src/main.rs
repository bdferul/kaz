const INPUT: &'static str = include_str!("input");
use std::{collections::{HashSet,HashMap}, ops::{Add, Sub, Div, AddAssign}, fmt::{Debug, format, Display}, iter::{TakeWhile, Product}};

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x:i32,y:i32) -> Self {
        Point { x, y }
    }

    pub fn is_touching(&self, other: Point) -> bool {
        let start =  Point::new(self.x-1, self.y-1);
        for y in 0..3 {
            for x in 0..3 {
                let delta = Point::new(x, y);
                if other == start + delta {
                    return true;
                }
            }
        }
        false
    }

    pub fn to_tuple(&self) -> (i32,i32) {
        (self.x,self.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Div for Point {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Point { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

fn main() {
    let raw_moves = INPUT
        .lines()
        .map(|s| s.split(' ')
        .collect::<Vec<&str>>())
        .map(|v| (v[0].chars().nth(0).unwrap(), v[1].parse::<i32>().unwrap()));

    let mut rope = vec![(0,0).to_point(); 10];
    let mut visits: HashMap<usize, HashSet<Point>> = HashMap::new();
    for i in 0..rope.len() {
        let mut tmp = HashSet::new();
        tmp.insert(rope[i]);
        visits.insert(i, tmp);
    }

    for (dir, len) in raw_moves {
        let delta = dir.to_point();

        for _ in 0..len {
            let mut new_rope = rope.clone();
            new_rope[0] = rope[0] + delta;
            visits.get_mut(&0).unwrap().insert(new_rope[0]);
            for j in 1..rope.len() {
                if !rope[j].is_touching(new_rope[j-1]) {
                    catch_up(new_rope[j-1], &mut new_rope[j]);
                    visits.get_mut(&j).unwrap().insert(new_rope[j]);
                }
            }
            //dbg!(rope, &new_rope);
            rope = new_rope;
        }
    }

    for i in 0..10 {
        println!("{i}: {:?}", visits.get(&i).unwrap().len());
    }
}

fn catch_up(head: Point, tail: &mut Point) {
    if head.is_touching(*tail) {
        return
    }
    let (mut dx,mut dy) = (head - *tail).to_tuple();
    dx = dx.checked_div(dx.abs()).unwrap_or(0);
    dy = dy.checked_div(dy.abs()).unwrap_or(0);
    *tail += (dx,dy).to_point();
}

trait ToPoint {
    fn to_point(&self) -> Point;
}

impl ToPoint for (i32,i32) {
    fn to_point(&self) -> Point {
        Point { x: self.0, y: self.1 }
    }
}

impl ToPoint for char {
    fn to_point(&self) -> Point {
        match self {
            'U' => Point::new(0,1),
            'D' => Point::new(0,-1),
            'R' => Point::new(1,0),
            'L' => Point::new(-1,0),
            _ => panic!(),
        }
    }
}