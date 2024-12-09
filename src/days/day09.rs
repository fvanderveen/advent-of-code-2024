use std::str::FromStr;
use crate::days::Day;
use crate::util::number::parse_usize;

pub const DAY9: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let mut drive: Drive = input.parse().unwrap();

    drive.defrag_blocks();
    println!("Checksum of drive after block-defrag: {}", drive.checksum());
}

fn puzzle2(input: &String) {
    let mut drive: Drive = input.parse().unwrap();

    drive.defrag_files();
    println!("Checksum of drive after file-defrag: {}", drive.checksum());
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Block {
    // size
    Empty(usize),
    // id,size
    File(usize, usize)
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Drive {
    blocks: Vec<Block>,
}

impl Drive {
    fn first_empty_block(&self) -> usize {
        for i in 0..self.blocks.len() {
            match self.blocks[i] {
                Block::Empty(_) => return i,
                _ => {}
            }
        }

        self.blocks.len()
    }

    fn defrag_blocks(&mut self) {
        // Defragmentating the Drive is built by moving blocks from the back up to the first free space
        // in the front of the disk.
        let mut empty_index = self.first_empty_block();
        let mut copy_index = self.blocks.len() - 1;

        while empty_index < copy_index {
            let empty_block = self.blocks[empty_index].clone();
            let file_block = self.blocks[copy_index].clone();

            match (empty_block, file_block) {
                (Block::Empty(empty_size), Block::File(id, file_size)) if file_size <= empty_size => {
                    // File fits inside the empty block
                    self.blocks.remove(copy_index);
                    self.blocks.remove(empty_index);

                    self.blocks.insert(empty_index, Block::File(id, file_size));

                    empty_index += 1;
                    copy_index -= 1;

                    let leftover_space = empty_size - file_size;
                    if leftover_space > 0 {
                        self.blocks.insert(empty_index, Block::Empty(leftover_space));
                        copy_index += 1;
                    }
                }
                (Block::Empty(empty_size), Block::File(id, file_size)) => {
                    // File is bigger than the empty block, and needs to be split up.
                    let leftover_size = file_size - empty_size;
                    self.blocks.remove(empty_index);
                    self.blocks.insert(empty_index, Block::File(id, empty_size));

                    match &mut self.blocks.get_mut(copy_index) {
                        Some(Block::File(_, size)) => {
                            *size = leftover_size
                        },
                        _ => unreachable!()
                    }
                },
                _ => unreachable!("Invalid state?!") // We should always have an empty block and a file block
            }

            // Find next empty index
            while empty_index < self.blocks.len() {
                match self.blocks[empty_index] {
                    Block::Empty(_) => break,
                    _ => empty_index += 1,
                }
            }
            // Find next copy index
            while copy_index > 0 {
                match self.blocks[copy_index] {
                    Block::Empty(_) => {
                        self.blocks.remove(copy_index);
                        copy_index -= 1
                    },
                    _ => break
                }
            }
        }
    }

    fn defrag_files(&mut self) {
        // Same as defrag_blocks, but move whole files by finding an empty space to the left that fits them whole
        let max_file_id = match self.blocks.iter().filter_map(|b| match b {
            Block::File(id, _) => Some(id),
            _ => None
        }).max() {
            Some(id) => *id,
            None => return
        };

        // Find the block:
        for file_id in (0..=max_file_id).rev() {
            let file_index = self.find_file_index(file_id).unwrap();
            match self.blocks[file_index] {
                Block::File(_, file_size) => {
                    if let Some(block_index) = self.find_empty_slot(file_size, file_index) {
                        self.blocks.remove(file_index);
                        self.blocks.insert(file_index, Block::Empty(file_size));
                        match self.blocks.remove(block_index) {
                            Block::Empty(block_size) => {
                                self.blocks.insert(block_index, Block::File(file_id, file_size));
                                if block_size > file_size {
                                    self.blocks.insert(block_index + 1, Block::Empty(block_size - file_size));
                                }
                            }
                            _ => unreachable!()
                        }
                    }
                }
                _ => unreachable!()
            }
        }

        // Merge empty blocks, mainly for test purposes.
        let mut cur_index = 1;
        while cur_index < self.blocks.len() {
            let prev = &self.blocks[cur_index - 1];
            let cur = &self.blocks[cur_index];

            match (prev, cur) {
                (Block::Empty(prev_size), Block::Empty(cur_size)) => {
                    // Merge (note, cur_index will automatically point to the next one)
                    self.blocks[cur_index - 1] = Block::Empty(prev_size + cur_size);
                    self.blocks.remove(cur_index);
                },
                _ => {
                    // Continue
                    cur_index += 1;
                }
            }
        }
    }

    fn find_file_index(&self, id: usize) -> Option<usize> {
        for i in 0..self.blocks.len() {
            match self.blocks[i] {
                Block::File(file_id, _) if file_id == id => return Some(i),
                _ => {}
            }
        }

        None
    }

    fn find_empty_slot(&self, size: usize, max_index: usize) -> Option<usize> {
        for i in 0..max_index {
            match self.blocks[i] {
                Block::Empty(block_size) if block_size >= size => return Some(i),
                _ => {}
            }
        }

        None
    }

    fn checksum(&self) -> usize {
        let mut checksum = 0;
        let mut offset = 0;

        for block in &self.blocks {
            match block {
                Block::File(id, size) => {
                    for v in offset..(offset + size) {
                        checksum += v * id;
                    }
                    offset += size;
                },
                Block::Empty(size) => { offset += size; }
            }
        }

        checksum
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day09::{Drive, Block};

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_parse_drive() {
        let result: Result<Drive, _> = TEST_INPUT.parse();

        assert!(result.is_ok());
        let drive = result.unwrap();

        assert_eq!(drive.blocks, vec![
            Block::File(0, 2),
            Block::Empty(3),
            Block::File(1, 3),
            Block::Empty(3),
            Block::File(2, 1),
            Block::Empty(3),
            Block::File(3, 3),
            Block::Empty(1),
            Block::File(4, 2),
            Block::Empty(1),
            Block::File(5, 4),
            Block::Empty(1),
            Block::File(6, 4),
            Block::Empty(1),
            Block::File(7, 3),
            Block::Empty(1),
            Block::File(8, 4),
            Block::File(9, 2),
        ]);
    }

    #[test]
    fn test_defrag_blocks() {
        let mut drive: Drive = TEST_INPUT.parse().unwrap();

        drive.defrag_blocks();

        assert_eq!(drive.blocks, vec![
            Block::File(0, 2),
            Block::File(9, 2), Block::File(8, 1),
            Block::File(1, 3),
            Block::File(8, 3),
            Block::File(2, 1),
            Block::File(7, 3),
            Block::File(3, 3),
            Block::File(6, 1),
            Block::File(4, 2),
            Block::File(6, 1),
            Block::File(5, 4),
            Block::File(6, 1),
            Block::File(6, 1),
        ])
    }

    #[test]
    fn test_defrag_files() {
        let mut drive: Drive = TEST_INPUT.parse().unwrap();

        drive.defrag_files();

        assert_eq!(drive.blocks, vec![
            Block::File(0, 2),
            Block::File(9, 2),
            Block::File(2, 1),
            Block::File(1, 3),
            Block::File(7, 3),
            Block::Empty(1),
            Block::File(4, 2),
            Block::Empty(1),
            Block::File(3, 3),
            Block::Empty(4),
            Block::File(5, 4),
            Block::Empty(1),
            Block::File(6, 4),
            Block::Empty(5),
            Block::File(8, 4),
            Block::Empty(2),
        ]);
        assert_eq!(drive.checksum(), 2858);
    }

    #[test]
    fn test_checksum() {
        let mut drive: Drive = TEST_INPUT.parse().unwrap();

        drive.defrag_blocks();

        assert_eq!(drive.checksum(), 1928);
    }
}

impl FromStr for Drive {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The drive representation is a list of digits that represent file-size and free-space alternating.
        // The first digit is a file-size

        let mut file_id = 0;
        let mut blocks = vec![];

        for i in (0..s.len()).step_by(2) {
            let file_size = parse_usize(&s[i..(i+1)])?;

            blocks.push(Block::File(file_id, file_size));

            file_id += 1;

            if i+1 < s.len() {
                let offset= parse_usize(&s[(i+1)..(i+2)])?;
                if offset > 0 {
                    blocks.push(Block::Empty(offset));
                }
            }
        }

        Ok(Self { blocks })
    }
}
