use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Block {
    File(usize),
    Free,
}

impl Block {
    fn file(id: usize) -> Block {
        Block::File(id)
    }

    fn free() -> Block {
        Block::Free
    }
}

#[derive(Debug)]
struct File {
    id: usize,
    start: usize,
    len: usize,
}

impl File {
    fn new(id: usize, start: usize, len: usize) -> File {
        File { id, start, len }
    }
}

#[derive(Debug)]
struct Free {
    start: usize,
    len: usize,
}

impl Free {
    fn new(start: usize, len: usize) -> Free {
        Free { start, len }
    }
}

#[allow(dead_code)]
fn show_blocks(blocks: &[Block]) {
    for block in blocks {
        match block {
            Block::File(id) => print!("{}", id),
            Block::Free => print!("."),
        }
    }
    println!();
}

fn main() {
    let mut fileid: usize = 0;
    let mut blocks = Vec::<Block>::new();
    let mut free_space = Vec::<Free>::new();
    let mut files = HashMap::<usize, File>::new();
    let mut index = 0_usize;

    if let Some(Ok(input)) = std::io::stdin().lines().next() {
        let mut is_free = false;
        blocks = input
            .chars()
            .flat_map(|c| {
                let len = c.to_digit(10).unwrap() as usize;
                let start = index;
                index += len;
                if !is_free {
                    let current_file_id = fileid;
                    fileid += 1;
                    is_free = true;
                    files.insert(current_file_id, File::new(current_file_id, start, len));
                    vec![Block::file(current_file_id); len].into_iter()
                } else {
                    is_free = false;
                    if len == 0 {
                        return vec![].into_iter();
                    }
                    free_space.push(Free::new(start, len));
                    vec![Block::free(); len].into_iter()
                }
            })
            .collect()
    }

    for i in (0..fileid).rev() {
        if let Some(file) = files.get(&i) {
            for (index, free) in free_space.iter().enumerate() {
                if file.start > free.start && free.len >= file.len {
                    for j in 0..file.len {
                        blocks[free.start + j] = Block::file(file.id);
                    }
                    for j in 0..file.len {
                        blocks[file.start + j] = Block::free();
                    }
                    free_space[index].len -= file.len;
                    free_space[index].start += file.len;

                    if free_space[index].len == 0 {
                        free_space.remove(index);
                    } else {
                        while let Some(next_free_block) = free_space.get(index + 1) {
                            if free_space[index].start + free_space[index].len
                                >= next_free_block.start
                            {
                                free_space[index].len += next_free_block.len;
                                free_space.remove(index + 1);
                            } else {
                                break;
                            }
                        }

                        // now we need to put the new free block in the right place _and_ merge it with free space around if if necessary.
                        for i in index..free_space.len() {
                            if file.start < free_space[i].start {
                                if file.start + file.len >= free_space[i].start {
                                    free_space[i].start = file.start;
                                    free_space[i].len += file.len;
                                    break;
                                } else {
                                    free_space.insert(i, Free::new(file.start, file.len));
                                    break;
                                }
                            }
                        }

                    }
                    break;
                }
            }
        }
    }


    let sum = blocks
        .iter()
        .enumerate()
        .map(|(index, block)| match block {
            Block::File(id) => id * index,
            Block::Free => 0,
        })
        .sum::<usize>();

    println!("{}", sum);
}
