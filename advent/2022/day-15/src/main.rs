const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

/// Beacon Sensor Pair
#[derive(Debug, Clone, Copy)]
struct BeaSenPair {
    sensor: Point,
    beacon: Point,
}

fn main() {
    let beasens = parse_input(INPUT);
    let sensor_ranges = beasens
        .iter()
        .map(|bsp| (bsp.sensor, get_manhattan(bsp.sensor, bsp.beacon)))
        .collect::<Vec<(Point, i32)>>();

    let closest_sensor_x = sensor_ranges.iter().min_by_key(|&&(s, _)| s.x).unwrap();
    let farthest_sensor_x = beasens.iter().map(|bsp| bsp.sensor.x).max().unwrap();

    let mut part1 = 0;
    let test_y = 2000000;
    let beacon_collums_on_test_y = beasens
        .iter()
        .map(|bsp| bsp.beacon)
        .filter(|b| b.y == test_y)
        .map(|b| b.x)
        .collect::<Vec<i32>>();

    let start = closest_sensor_x.0.x - closest_sensor_x.1;

    for x in start.. {
        let point = Point::new(x, test_y);
        if beacon_collums_on_test_y.contains(&x) {
            continue;
        }
        let mut good = true;

        for ranges in &sensor_ranges {
            let dis = get_manhattan(point, ranges.0);

            if dis <= ranges.1 {
                good = false;
                break;
            }
        }

        if !good {
            part1 += 1;
        } else if x > farthest_sensor_x {
            break;
        }
    }

    println!("Part 1: {part1}");

    let mut part2 = None;
    let part2_range = 0..=4000000;

    'out: for (sensor, range) in sensor_ranges.clone() {
        let rim = rim(sensor, range + 1);

        for p in rim {
            if !part2_range.contains(&p.x) || !part2_range.contains(&p.y) {
                continue;
            }

            let mut unscanned = true;
            for (sensor, range) in &sensor_ranges {
                if get_manhattan(p, *sensor) <= *range {
                    unscanned = false;
                    break;
                }
            }

            if unscanned {
                part2 = Some(p);
                break 'out;
            }
        }
    }

    let Point { x: px, y: py } = part2.unwrap();

    let px = px as i128;
    let py = py as i128;

    let part2 = (px * 4000000) + py;

    println!("Part 2: {part2}");
}

/// Finds the rim of points surrounding a point beyond a specified distance;
fn rim(point: Point, distance: i32) -> Vec<Point> {
    let mut r = vec![];

    let start_x = point.x;
    let start_y = point.y;

    for i in 0..distance {
        r.push(Point::new(start_x + i, start_y - distance + i)); // \
        r.push(Point::new(start_x + distance - i, start_y + i)); // /
        r.push(Point::new(start_x - i, start_y + distance - i)); // \
        r.push(Point::new(start_x - distance + i, start_y - i)); // /
    }

    r
}

fn get_manhattan(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn parse_input(input: &str) -> Vec<BeaSenPair> {
    let mut beasens = vec![];

    for line in input.lines() {
        let [_,sx,_,sy,_,bx,_,by] = &parse_line(line, &["=", ",", "=", ":", "=", ",", "="])[..] else {panic!()};

        let sensor = Point::parse(sx, sy);
        let beacon = Point::parse(bx, by);

        beasens.push(BeaSenPair { sensor, beacon });
    }

    beasens
}

fn parse_line(input: &str, delims: &[&str]) -> Vec<String> {
    let mut r = vec![];
    let mut line = input;

    for s in delims {
        let Some((a, b)) = line.split_once(s) else { break };
        r.push(a.to_string());
        line = b;
    }

    r.push(line.to_string());

    r
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", (self.x, self.y))
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn parse(x: &str, y: &str) -> Self {
        Self::new(x.parse().unwrap(), y.parse().unwrap())
    }
}
