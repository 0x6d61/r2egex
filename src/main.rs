extern crate r2egex;

use r2egex as other_r2egex;

fn main() {
	let pattern = "^t.*ab?".split("").collect::<Vec<&str>>();
	let text = "testbb".split("").collect::<Vec<&str>>();
    println!("{:?}",other_r2egex::matchs(pattern[1..(pattern.len()-1) as usize].to_vec(),text[1..text.len()-1].to_vec()));
}
