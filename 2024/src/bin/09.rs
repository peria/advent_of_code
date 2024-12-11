use std::io::BufRead;

fn main() {
    let file_system = input();
    println!("{}", solve1(&file_system));
    println!("{}", solve2(&file_system));
}

fn input() -> FileSystem {
    let mut data = String::new();
    let _ = std::io::stdin().lock().read_line(&mut data);
    FileSystem::from(&data as &str)
}

fn solve1(file_system: &FileSystem) -> i64 {
    let mut file_system = file_system.clone();
    file_system.compress_by_block();
    file_system.checksum()
}

fn solve2(file_system: &FileSystem) -> i64 {
    let mut file_system = file_system.clone();
    file_system.compress_by_file();
    file_system.checksum()
}

#[derive(Debug, Copy, Clone)]
struct File {
    id: usize,
    address: usize,
    size: usize,
}

impl File {
    fn new(id: usize, address: usize, size: usize) -> Self {
        File { id, address, size }
    }

    fn check_sum(&self) -> usize {
        let begin = self.address;
        let end = self.address + self.size - 1;
        self.id * self.size * (begin + end) / 2
    }
}

#[derive(Debug, Clone)]
struct FileSystem {
    files: Vec<File>,
}

impl FileSystem {
    fn compress_by_block(&mut self) {
        loop {
            let n = self.files.len();
            let file = self.files.remove(n - 1);
            if let Some(space) = self.find_empty_space(1) {
                let index = space.id;
                if space.size >= file.size {
                    self.files
                        .insert(index, File::new(file.id, space.address, file.size));
                } else {
                    self.files
                        .insert(index, File::new(file.id, space.address, space.size));
                    let mut file = file;
                    file.size -= space.size;
                    self.files.push(file);
                }
            } else {
                let last_file = self.files.last().unwrap();
                let address = last_file.address + last_file.size;
                self.files.push(File::new(file.id, address, file.size));
                break;
            }
        }
    }

    fn compress_by_file(&mut self) {
        let max_id = self.files.len() - 1;
        for id in (0..=max_id).rev() {
            let index = self.files.iter().position(|x| x.id == id).unwrap();
            let file_size = self.files[index].size;
            if let Some(space) = self.find_empty_space(file_size) {
                if space.id <= index {
                    let mut file = self.files.remove(index);
                    file.address = space.address;
                    self.files.insert(space.id, file);
                }
            }

            let mut address = 0;
            for f in self.files.iter() {
                assert!(address <= f.address);
                address = f.address + f.size;
            }
        }
    }

    fn checksum(&self) -> i64 {
        self.files.iter().map(|x| x.check_sum()).sum::<usize>() as i64
    }

    fn find_empty_space(&self, min_size: usize) -> Option<File> {
        let mut address = 0;
        for (id, f) in self.files.iter().enumerate() {
            let size = f.address - address;
            if size >= min_size {
                return Some(File::new(id, address, size));
            }
            address = f.address + f.size;
        }

        None
    }
}

impl From<&str> for FileSystem {
    fn from(value: &str) -> Self {
        let mut files = Vec::new();

        let mut is_file = true;
        let mut address = 0;
        let mut id = 0;
        for l in value.trim().chars() {
            assert!(l.is_numeric());
            let len = (l as u8 - '0' as u8) as usize;
            if is_file {
                let file = File::new(id, address, len);
                files.push(file);
                id += 1;
            }
            is_file = !is_file;
            address += len;
        }

        FileSystem { files }
    }
}

#[cfg(test)]
mod test {
    use crate::solve1;
    use crate::solve2;
    use crate::FileSystem;

    const SAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_solve1() {
        let file_system = FileSystem::from(SAMPLE_INPUT);
        let actual = solve1(&file_system);
        let expect = 1928;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_solve2() {
        let file_system = FileSystem::from(SAMPLE_INPUT);
        let actual = solve2(&file_system);
        let expect = 2858;
        assert_eq!(expect, actual);
    }
}
