extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use itertools::Itertools;
use std::io::Error;


fn main() -> io::Result<()>{
    let stdin = io::stdin();
    for (head, seq, _, _) in stdin.lock().lines().tuples() {
        let seq = seq?;
        let head = head?;
        println!(">{}{}\n{}", &head, &seq, &seq);
    }
    Ok(())
}
