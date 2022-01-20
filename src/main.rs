use std::{env, process::exit};
pub use csc411_image::{Read, Write, RgbImage, GrayImage};
use array2::Array2;
use std::collections::HashMap;

struct RowColCheck{
   pub rc_check:HashMap<u16, bool>,
}

impl RowColCheck{
    fn new() -> Self {
       RowColCheck{
           rc_check: HashMap::new()
       }
    }

    fn hash_reset(&mut self) {
        for i in 1..10{
            self.rc_check.insert(i, false);
        }
    }
}


fn main() {
	let input = env::args().nth(1);
    let puzzle: GrayImage = GrayImage::read(input.as_deref()).unwrap();

	if puzzle.denominator != 9 || puzzle.height != 9 || puzzle.width != 9 {
		exit(1);
	}
    
    let puzzle_a2 = Array2::new_h_w(puzzle.pixels, puzzle.height as usize, puzzle.width as usize);    
    let s_iter_row_major = puzzle_a2.make_iter(array2::TypeOfIteration::RowMajor);
    let s_iter_col_major = puzzle_a2.make_iter(array2::TypeOfIteration::ColumnMajor);
    let s_iter_blocks = puzzle_a2.make_iter(array2::TypeOfIteration::DctIter);

    let mut count:usize = 0;
	let mut check = RowColCheck::new();
    check.hash_reset();

    for i in s_iter_row_major{

        if check.rc_check.contains_key(&i.value) && !check.rc_check[&i.value]{
            check.rc_check.remove(&i.value);
        }
        else{
            exit(1);
        }

        if (count + 1) % puzzle_a2.height == 0 {
            check.hash_reset();
        }
        count += 1;
    }

    for i in s_iter_col_major{
        if check.rc_check.contains_key(&i.value) && !check.rc_check[&i.value]{
            check.rc_check.remove(&i.value);
        }
        else{
            exit(1);
        }

        if (count + 1) % puzzle_a2.height == 0 {
            check.hash_reset();
        }
        count += 1;
    }

    for i in s_iter_blocks{
        if check.rc_check.contains_key(&i.value) && !check.rc_check[&i.value]{
            check.rc_check.remove(&i.value);
        }
        else{
            exit(1);
        }

        if (count + 1) % puzzle_a2.height == 0 {
            check.hash_reset();
        }
        count += 1;
    }

    println!("{}", "All good!");
}