struct Map {
    pub topology: Vec<Vec<char>>,
    pub path: Vec<Vec<Option<u32>>>,
    pub dirs: Vec<Vec<Vec<(usize, usize)>>>,
    start: (usize,usize),
}

impl Map {
    pub fn height(&self) -> usize {
        self.topology.len()
    }

    pub fn width(&self) -> usize {
        self.topology[0].len()
    }

    pub fn start(&self) -> Option<u32> {
        self.path[self.start.1][self.start.0]
    }
}

fn main() {
    let input = include_str!("input.txt");
    let raw_map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let width = raw_map[0].len();
    let height = raw_map.len();
    let mut map = Map {
        path: vec![vec![None; width]; height],
        dirs: vec![vec![vec![]; width]; height],
        topology: raw_map,
        start: (0,0),
    };

    init(&mut map);
    while propogate(&mut map){};

    /*
    for y in 0..map.height() {
        for x in 0..map.width() {
            print!(
                "{:_<3} ",
                match map.path[y][x] {
                    Some(n) => n.to_string(),
                    None => "".to_string(),
                }
            );
        }
        println!();
    }
    */
    

    println!("Part 1: {}", map.start().unwrap());
    println!("Part 2: {}", part2(&map));
}

fn part2(map: &Map) -> u32 {
    let mut min = None;
    for y in 0..map.height() {
        for x in 0..map.width() {
            if matches!(map.topology[y][x], 'a'|'S') {
                if let Some(a) = min {
                    if let Some(p) = map.path[y][x] {
                        if p < a {
                            min = map.path[y][x];
                        }
                    }
                } else {
                    min = map.path[y][x]
                }
            }
        }
    }

    min.unwrap()
}

fn init(map: &mut Map) {
    let mut no_start_end = map.topology.clone();
    for (nse, top) in no_start_end.iter_mut().zip(&map.topology) {
        for (a, b) in nse.iter_mut().zip(top) {
            *a = match *b {
                'S' => 'a',
                'E' => 'z',
                other => other,
            };
        }
    }
    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.topology[y][x] == 'E' {
                map.path[y][x] = Some(0);
            }
            if map.topology[y][x] == 'S' {
                map.start = (x,y);
            }

            let mut dirs = vec![];
            if x + 1 < map.width() {
                dirs.push((x + 1, y));
            }
            if x > 0 {
                dirs.push((x - 1, y));
            }
            if y + 1 < map.height() {
                dirs.push((x, y + 1));
            }
            if y > 0 {
                dirs.push((x, y - 1));
            }

            dirs.retain(|&(dx, dy)| {
                ('a'..=(no_start_end[y][x] as u8 + 1) as char).contains(&no_start_end[dy][dx])
            });

            map.dirs[y][x] = dirs;
        }
    }
}

///returns false if is done propogating
fn propogate(map: &mut Map) -> bool {
    let mut done = true;
    let h = map.height();
    let w = map.width();
    for y in 0..h {
        for x in 0..w {
            if map.topology[y][x] == 'E' {
                map.path[y][x] = Some(0);
            }

            for &(dx, dy) in &map.dirs[y][x] {
                if let Some(them) = map.path[dy][dx] {
                    if let Some(me) = map.path[y][x] {
                        if me > them + 1 {
                            map.path[y][x] = Some(them+1);
                            done = false;
                        }
                    } else {
                        map.path[y][x] = Some(them+1);
                        done = false
                    }
                }
            }
        }
    }

    !done
}
