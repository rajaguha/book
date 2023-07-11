#[allow(dead_code)]

fn main() {
   let v = vec![1, 2, 3, 4, 5];

    let third: Option<&i32> = v.get(200);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let third: &i32 = &v[200];
    println!("The third element is {third}");
}
