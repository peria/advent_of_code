use std::io::BufRead;

fn main() {
    let grid = get_input();
    let answer1 = solve1(&grid);
    let answer2 = solve2(&grid);
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn get_input() -> Vec<Vec<char>> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn remove_rolls(grid: &mut Vec<Vec<char>>) -> usize {
    let h = grid.len();
    let w = grid[0].len();

    let mut accessible = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] != '@' {
                continue;
            }
            let mut num_near = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if grid[ni][nj] == '@' {
                        num_near += 1;
                    }
                }
            }
            if num_near <= 4 {
                accessible.push((i, j));
            }
        }
    }
    for (i, j) in accessible.iter() {
        grid[*i][*j] = '.';
    }

    accessible.len()
}

fn solve1(grid: &Vec<Vec<char>>) -> i64 {
    let mut grid = grid.clone();
    remove_rolls(&mut grid) as i64
}

fn solve2(grid: &Vec<Vec<char>>) -> i64 {
    let mut grid = grid.clone();
    let mut total = 0;
    loop {
        let removed = remove_rolls(&mut grid);
        if removed == 0 {
            break;
        }
        total += removed;
    }
    total as i64
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_solve1() {
        let input = SAMPLE_INPUT
            .split_ascii_whitespace()
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(solve1(&input), 13);
    }

    #[test]
    fn test_solve2() {
        let input = SAMPLE_INPUT
            .split_ascii_whitespace()
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(solve2(&input), 43);
    }
}
