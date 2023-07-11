#[allow(dead_code)]

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(5) => Some(10),
            Some(6) => Some(11),
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    println!( "Five: {:?}", plus_one(five) );
    println!( "Six: {:?}", plus_one(Some(6)) );
    println!( "Other: {:?}", plus_one(Some(27)) );
    println!( "None: {:?}", plus_one(None) );
}
