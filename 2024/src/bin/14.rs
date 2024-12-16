use png;
use std::fs::File;
use std::io::BufWriter;
use std::io::Read;
use std::path::Path;

fn main() {
    let room = input();
    println!("{}", solve1(&room, None, None));
    println!("{}", solve2(&room));
}

fn input() -> BathRoom {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);
    BathRoom::from(buf.trim())
}

fn solve1(room: &BathRoom, width: Option<i64>, height: Option<i64>) -> i64 {
    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;
    const TIME: i64 = 100;

    let width = width.unwrap_or(WIDTH);
    let height = height.unwrap_or(HEIGHT);
    room.safety_factor((width, height), TIME)
}

fn solve2(room: &BathRoom) -> i64 {
    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;
    const TIME: i64 = 100;

    for t in (71..(WIDTH * HEIGHT)).step_by(WIDTH as usize) {
        room.output_png((WIDTH, HEIGHT), t);
    }
    0
}

#[derive(Debug, Clone)]
struct BathRoom {
    robots: Vec<Robot>,
}

impl BathRoom {
    fn safety_factor(&self, room: (i64, i64), time: i64) -> i64 {
        let midx = room.0 / 2;
        let midy = room.1 / 2;

        let get_index = |x: i64, y: i64| -> usize {
            let mut index = 0;
            if x == midx || y == midy {
                return 4;
            }
            if x > midx {
                index += 1;
            }
            if y > midy {
                index += 2;
            }
            index
        };

        let mut count: [i64; 5] = [0, 0, 0, 0, 0];
        for r in self.robots.iter() {
            let x = ((r.start.0 + r.velocity.0 * time) % room.0 + room.0) % room.0;
            let y = ((r.start.1 + r.velocity.1 * time) % room.1 + room.1) % room.1;

            let index = get_index(x, y);
            count[index] += 1;
        }

        count[0] * count[1] * count[2] * count[3]
    }

    fn output_png(&self, room: (i64, i64), time: i64) {
        let width = room.0 as usize;
        let height = room.1 as usize;

        let mut data = vec![255; width * height];
        for r in self.robots.iter() {
            let x = ((r.start.0 + r.velocity.0 * time) % room.0 + room.0) % room.0;
            let y = ((r.start.1 + r.velocity.1 * time) % room.1 + room.1) % room.1;
            let index = y as usize * width + x as usize;
            data[index] = 0;
        }
        let path = format!("images/image_14_{:03}.png", time);
        let path = Path::new(&path);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Grayscale);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&data).unwrap();
    }
}

impl From<&str> for BathRoom {
    fn from(value: &str) -> Self {
        let mut robots = Vec::new();
        for line in value.split('\n') {
            let robot = Robot::from(line);
            robots.push(robot);
        }

        BathRoom { robots }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    start: (i64, i64),
    velocity: (i64, i64),
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let parsed = sscanf::sscanf!(value, "p={i64},{i64} v={i64},{i64}")
            .ok()
            .unwrap();
        let start = (parsed.0, parsed.1);
        let velocity = (parsed.2, parsed.3);

        Robot { start, velocity }
    }
}

#[cfg(test)]
mod test {
    use crate::solve1;
    use crate::solve2;
    use crate::BathRoom;

    const SAMPLE_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_solve1() {
        let room = BathRoom::from(SAMPLE_INPUT);
        let actual = solve1(&room, Some(11), Some(7));
        let expect = 12;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_solve2() {
        let room = BathRoom::from(SAMPLE_INPUT);
        let actual = solve2(&room);
        let expect = 0;
        assert_eq!(actual, expect);
    }
}
