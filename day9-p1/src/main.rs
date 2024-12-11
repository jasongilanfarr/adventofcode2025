
#[derive(Debug,Clone,PartialEq, Copy)]
enum Block {
    File(usize),
    Free
}

impl Block {
    fn file(id: usize) -> Block {
        Block::File(id)
    }

    fn free() -> Block {
        Block::Free
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

    if let Some(Ok(input)) = std::io::stdin().lines().next() {
        let mut is_free = false;
        blocks = input.chars().flat_map(|c| {
            let len = c.to_digit(10).unwrap() as usize;
            if !is_free {
                fileid += 1;
                is_free = true;
                vec![Block::file(fileid - 1); len].into_iter()
            } else {
                is_free = false;
                vec![Block::free(); len].into_iter()
            }
        }).collect()
    }

    let mut free_slot = 0;
    let mut last = blocks.len() - 1;

    while free_slot < last {
        while blocks[free_slot] != Block::Free {
            free_slot += 1;
        }
        while blocks[last] == Block::Free {
            last -= 1;
        }
        if free_slot > last {
            break;
        }

        blocks[free_slot] = blocks[last];
        blocks[last] = Block::Free;
        last -= 1;
        free_slot += 1;
    }

    let sum = blocks.iter().enumerate().map(|(index, block)| {
        match block {
            Block::File(id) => id * index,
            Block::Free => 0,
        }
    }).sum::<usize>();
    
    println!("{}", sum);

}
