use std::{collections::HashMap, fmt::Debug};

use CaveParts::*;

fn main() {
    // let input = include_str!("test.txt").trim();
    let input = include_str!("input.txt").trim();
    let mut cave = Cave::new();

    for line in input.lines() {
        let mut crds = line.split(" -> ");
        let mut start: [usize; 2] = parse_crd(crds.next().unwrap());

        for crd in crds {
            let parsed = parse_crd(crd);
            cave.draw_rock_line(start, parsed).unwrap();
            start = parsed;
        }
    }

    println!("Finished mapping");
    let mut count = 0;
    while matches!(cave.produce_sand(), ProduceSandReturn::Other) {
        //println!("{count}");
        count += 1;
    }
    println!("Part 1: {}", count);
    while !matches!(cave.produce_sand(), ProduceSandReturn::Hole) {
        // cave.print();
        count += 1;
    }
    println!("Part 2: {}", count + 2);
    // cave.print();
}

///Parses "<usize>,<usize>" -> [usize;2]
fn parse_crd(input: &str) -> [usize; 2] {
    input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

#[derive(Debug)]
enum ProduceSandReturn {
    Floor,
    Hole,
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CaveParts {
    Air,
    Rock,
    Sand,
}

impl Debug for CaveParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Air => '.',
                Rock => '#',
                Sand => 'o',
            }
        )
    }
}

impl Default for CaveParts {
    fn default() -> Self {
        Air
    }
}

struct Cave {
    cave: std::collections::HashMap<[usize; 2], CaveParts>,
    y_max: usize,
}

impl Cave {
    fn new() -> Cave {
        Cave {
            cave: HashMap::new(),
            y_max: 0,
        }
    }

    //returns true if the sand falls into the abyss
    fn produce_sand(&mut self) -> ProduceSandReturn {
        let mut sand_x = 500;
        let mut sand_pos = [0; 2];

        let mut r = ProduceSandReturn::Other;

        for y in 0..self.y_max + 2 {
            sand_pos = [sand_x, y];
            if y >= self.y_max {
                r = ProduceSandReturn::Floor;
            }
            if self.get(&[sand_x, y + 1]) == Air {
                continue;
            } else if self.get(&[sand_x - 1, y + 1]) == Air {
                sand_x -= 1;
                continue;
            } else if self.get(&[sand_x + 1, y + 1]) == Air {
                sand_x += 1;
                continue;
            } else {
                break;
            }
        }

        if sand_pos == [500, 0] {
            r = ProduceSandReturn::Hole;
        }

        self.cave.insert(sand_pos, Sand);

        // println!("{:?}", &r);
        r
    }

    fn print(&self) {
        let [x_min, x_max] = self.x_bounds();

        let y_max = self.y_max();

        for y in 0..=y_max {
            for x in x_min..=x_max {
                if y == 0 && x == 500 {
                    print!("+");
                } else {
                    print!("{:?}", self.get(&[x, y]));
                }
            }
            println!();
        }
    }

    fn y_max(&self) -> usize {
        self.cave
            .keys()
            .map(|[_, y]| y)
            .max_by_key(|y| *y)
            .unwrap()
            .to_owned()
    }

    fn x_bounds(&self) -> [usize; 2] {
        [
            self.cave
                .keys()
                .map(|[x, _]| x)
                .min_by_key(|x| *x)
                .unwrap()
                .to_owned(),
            self.cave
                .keys()
                .map(|[x, _]| x)
                .max_by_key(|x| *x)
                .unwrap()
                .to_owned(),
        ]
    }

    fn get(&self, index: &[usize; 2]) -> CaveParts {
        self.cave.get(index).unwrap_or(&Air).to_owned()
    }

    fn draw_rock(&mut self, pos: [usize; 2]) {
        self.cave.insert(pos, Rock);
        self.y_max = self.y_max();
    }

    fn draw_rock_line(&mut self, [sx, sy]: [usize; 2], [ex, ey]: [usize; 2]) -> Result<(), ()> {
        if sx != ex && sy != ey {
            return Err(());
        }

        if sx == ex {
            let range = if sy <= ey { sy..=ey } else { ey..=sy };
            for y in range {
                self.draw_rock([sx, y]);
            }
        } else {
            let range = if sx <= ex { sx..=ex } else { ex..=sx };
            for x in range {
                self.draw_rock([x, sy]);
            }
        }

        Ok(())
    }
}
