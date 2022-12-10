use std::{vec};

const INPUT: &str = include_str!("input.txt");

struct Map {
    map: Vec<Vec<u32>>,
    visible: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

fn main() {
    let raw_map: Vec<Vec<u32>> = INPUT.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let width = raw_map[0].len();
    let height =  raw_map.len();
    let mut map = Map {
        map: raw_map,
        visible: vec![vec![false;width];height],
        width,
        height,
    };
    //let start = (1, 1);

    for y in 0..height {
        for x in 0..width {
            if y == 0 || x == 0 || y == height-1 || x == width-1 {
                map.visible[y][x] = true;
            }
        }
    }

    // top and bottom
    for x in 0..width {
        propogate(&mut map, x, 0, (0,1));
        propogate(&mut map, x, height-1, (0,-1));
    }

    // right and left
    for y in 0..width {
        propogate(&mut map, 0, y, (1,0));
        propogate(&mut map, width-1, y, (-1,0));
    }

    let mut cnt = 0;
    for y in map.visible.iter() {
        for x in y.iter() {
            print!("{} ", if *x {
                cnt += 1;
                '#'
            } else { ' ' });
        }
        println!();
    }    
    println!("Part 1: {cnt}");


    let mut view_scores = vec![];
    for y in 0..height {
        for x in 0..width {
            let a = count_visible(&map, x, y);
            print!("{a} ");
            view_scores.push(a);
        }
        println!();
    }
    println!("Part 2: {}", view_scores.iter().max().unwrap());
}

fn propogate(map: &mut Map, x: usize, y: usize, dir: (i32,i32)) -> i32 {
    let mut count = 1;
    let mut max = map.map[y][x];
    for i in 1.. {
        let nx = x as i32 + (dir.0*i);
        let ny = y as i32 + (dir.1*i);

        if nx < 0 || ny < 0 || nx >= map.width as i32 || ny >= map.height as i32 {
            break;
        }
        let ny = ny as usize;
        let nx = nx as usize;

        let mm = map.map[ny][nx];
        //dbg!(x,y,mm,max);
        if mm > max {
            max = mm;
            count += 1;
            map.visible[ny][nx] = true;
        }
    }
    count
}

fn count_visible(map: &Map, x: usize, y: usize) -> u32 {
    if x == 0 || y == 0 || x == map.width - 1 || y == map.height - 1 {
        return 0;
    }

    let mut r = 1;

    let dirs = [
        (0,1),
        (0,-1),
        (1,0),
        (-1,0),
    ];

    let max = map.map[y][x];
    for (dx,dy) in dirs {
        let mut cnt = 0;
        for i in 1.. {
            let nx = x as i32 + (dx*i);
            let ny = y as i32 + (dy*i);
            if nx < 0 || ny < 0 || nx >= map.width as i32 || ny >= map.height as i32 {
                break;
            }

            let mm = map.map[ny as usize][nx as usize];
            cnt += 1;
            if mm >= max {
                break;
            }
        }
        r *= cnt;
        if cnt == 0 {
            break;
        }
    }

    r
}

#[cfg(test)]
mod tests {
    #[test]
    fn count_visible() {
        let raw_map: Vec<Vec<u32>> = super::INPUT.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
        let width = raw_map[0].len();
        let height =  raw_map.len();
        let map = super::Map {
            map: raw_map,
            visible: vec![vec![false;width];height],
            width,
            height,
        };
        assert_eq!(super::count_visible(&map, 2, 3), 8);
    }
}