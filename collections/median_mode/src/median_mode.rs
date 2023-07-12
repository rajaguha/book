/******
 * Given a list of integers, use a vector and return the median (when 
 * sorted, the value in the middle position) and mode (the value that 
 * occurs most often; a hash map will be helpful here) of the list.
 **/

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut vec: Vec<u32> = Vec::new();
    for _i in 1..200 {
        let rn: u32 = rand::thread_rng().gen_range(1..=100);
        vec.push(rn);
    }
    vec.sort();

    // Median
    let count = vec.len();
    let mid: f32 = count as f32 / 2.0;
    if count % 2 == 0 {
        let mid_lo: usize = mid.floor() as usize;
        let mid_hi: usize = mid.ceil() as usize;
        let median = ( vec[mid_lo] + vec[mid_hi] ) as f32 / 2.0;
        println!( "The median is {median}." );
    } else {
        println!( "The median is {}.", vec[mid.ceil() as usize] );
    }

    // Mode
    let mut hm: HashMap<u32, u32> = HashMap::new();
    let mut it: (u32, u32) = (0, 0);
    for &val in vec.iter() {
        let nxt = *(hm.entry(val).or_insert(0)) + 1;
        hm.insert(val, nxt);
        if nxt > it.1 {
            it = (val, nxt);
        }
   }
   println!("The mode is {} with {} occurances.", it.0, it.1);
}
