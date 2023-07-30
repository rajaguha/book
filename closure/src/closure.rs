// use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // let borrows = || println!("From closure: {:?}", list);
    let mut borrows = || {
        list.push(list.len() + 1);
        println!("From closure: {:?}", list);
    };
    // thread::spawn( move || {
    //     list.push(list.len() + 1);
    //     println!("From thread: {:?}", list );
    // } ).join().unwrap();

    // println!("After define, before calling closure: {:?}", list);
    borrows();
    borrows();
    println!("After calling closure: {:?}", list);
    // borrows();
}
