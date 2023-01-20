/// Returns the Manhattan distance between two points
fn man_dist(a: (i64, i64), b: (i64, i64)) -> i64 {
    i64::abs(a.0 - b.0) + i64::abs(a.1 - b.1)
}

/// A sensor has its center position and its exclusion range
struct Sensor {
    position: (i64, i64),
    range: i64,
}

impl Sensor {
    pub fn init(pos: (i64, i64), beacon: (i64, i64)) -> Sensor {
        Self {
            position: pos,
            range: man_dist(beacon, pos),
        }
    }

    /// Returns True if point is inside this sensor's exclusion range
    pub fn in_exclusion_range(&self, point: (i64, i64)) -> bool {
        self.range >= man_dist(self.position, point)
    }
}

/// Parses the advent input and returns the list of sensors
fn parse(input: &str) -> Vec<Sensor> {
    let mut result = vec![];

    for line in input.lines() {
        // line: "Sensor at x=2391367, y=3787759: closest beacon is at x=2345659, y=4354867"
        let (first, second) = line.split_once(": ").unwrap();

        let (sensor_x, sensor_y) = first[12..].split_once(", ").unwrap();
        let sensor_x = sensor_x.parse().unwrap();
        let sensor_y = sensor_y[2..].parse().unwrap();

        let (beacon_x, beacon_y) = second[23..].split_once(", ").unwrap();
        let beacon_x = beacon_x.parse().unwrap();
        let beacon_y = beacon_y.parse().unwrap();

        result.push(Sensor::init((sensor_x, sensor_y), (beacon_x, beacon_y)));
    }

    result
}

fn main() {}
