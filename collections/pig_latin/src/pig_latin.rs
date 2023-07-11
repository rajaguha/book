/******** 
 Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is 
 added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead 
 (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
********/

#[allow(dead_code)]
use std::io;

fn main() {
    let vowels = [ 'a', 'e', 'i', 'o', 'u' ];

    println!("Your word is? ");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");
    word.pop();
    let first :char = word.chars().next().unwrap();
    let mut pig = String::new();
    if vowels.contains(&first) {
        pig = format!( "{}{}", word, "-hay" );
    } else {
        pig = format!( "{}{}{}{}", &word[1..], '-', first, "ay" );
    }

    println!( "word is {}; first is {}; pig is {}", word, first, pig );
}
