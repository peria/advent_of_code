use std::io::BufRead;

fn main() {
    let mut control = String::new();
    let _ = std::io::stdin().lock().read_line(&mut control);
    let control = control.trim();

    println!("{}", get_height(&control, 2022));
    println!("{}", get_height(&control, 1000000000000));
}

fn get_height(controls: &str, n: usize) -> usize {
    // loop (block: 35, height: 53) after 50th control for test
    // loop (block: 1745, height: 2750) afte 5230th control for input
    let is_test = controls.len() < 100;
    let base = if is_test { 50 } else { 5230 };
    let loop_blocks = if is_test { 35 } else { 1745 };
    let loop_height = if is_test { 53 } else { 2750 };

    let base_n = if n < base {
        n
    } else {
        (n - base) % loop_blocks + base
    };
    let height = simulate(controls, base_n);
    let loop_times = if n < base {
        0
    } else {
        (n - base) / loop_blocks
    };
    height + loop_height * loop_times
}

fn simulate(controls: &str, n: usize) -> usize {
    // (dy, dx)
    let blocks: Vec<Vec<(usize, usize)>> = vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    let controls: Vec<char> = controls.chars().collect();
    let mut height = 0;
    let mut j = 0;
    let mut field = vec![vec!['.'; 7]];
    for i in 0..n {
        let (dy, dx) = (height + 3, 2usize);
        let block_id = i % blocks.len();

        let mut block: Vec<(usize, usize)> = blocks[block_id]
            .iter()
            .map(|(y, x)| (y + dy, x + dx))
            .collect();
        let top = block.iter().map(|(y, _x)| *y).max().unwrap();
        field.resize_with(top as usize + 1, || vec!['.'; 7]);
        let mut cs = String::new();
        loop {
            let c = controls[j];
            j = (j + 1) % controls.len();
            let dx = if c == '>' { 1 } else { -1 };
            cs += &c.to_string();

            let new_block: Vec<(usize, i64)> =
                block.iter().map(|&(y, x)| (y, x as i64 + dx)).collect();
            if new_block
                .iter()
                .all(|&(y, x)| 0 <= x && x < 7 && field[y][x as usize] == '.')
            {
                block = new_block.iter().map(|&(y, x)| (y, x as usize)).collect();
            }

            let new_block: Vec<(i64, usize)> =
                block.iter().map(|&(y, x)| (y as i64 - 1, x)).collect();
            if new_block
                .iter()
                .all(|&(y, x)| y >= 0 && field[y as usize][x] == '.')
            {
                block = new_block.iter().map(|&(y, x)| (y as usize, x)).collect();
                continue;
            }

            // Falling blocks conflict with fixed blocks, so the blocks under control
            // are fixed.
            break;
        }

        for &(y, x) in block.iter() {
            field[y as usize][x as usize] = '#';
        }
        height = height.max(block.iter().map(|(y, _x)| *y).max().unwrap() + 1);
    }
    height
}
