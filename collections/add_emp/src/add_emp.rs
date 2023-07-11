/******
 * Using a hash map and vectors, create a text interface to allow a user to add employee names 
 * to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
 * Then let the user retrieve a list of all people in a department or all people in the company 
 * by department, sorted alphabetically.
 * 
 * Commands:
 *  Add <First Middle Last> to <Department>.
 *  Remove <First Middle Last> from <Department>.
 *  List all in <Department>.
 *  List all by department. | List all
 * Quit
 */

 use std::io;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut emp_list: HashMap<&str, &mut Vec<&str>> = HashMap::new();

    let re_cmd = Regex::new( r"^\b(?<cmd>Add|Remove|List|Quit)\b" ).unwrap();
    loop {
        println!("Please enter command: ");
        let mut line_read = String::new();
        io::stdin().read_line(&mut line_read).expect("Failed to read line");
        line_read.pop();
        let stmt: &str = &line_read[..];

        let Some( caps ) = re_cmd.captures(stmt) else {
            println!( "'{stmt}' does not start with a command(Add,Remove,List or Quit). Try again." );
            continue;
        };
        let cmd: &str = &caps["cmd"];

        match cmd {
            "Add" => add_empl( &emp_list, stmt ),
            // "Remove" => del_empl( &emp_list, stmt ),
            // "List" => lst_empl( &emp_list, stmt ),
            "Quit" => return,
            &_ => {
                println!( "Command '{cmd}' unknown. Try again." );
                continue;
            }   
        } 
    }
}

fn add_empl( list: &HashMap<&str, &mut Vec<&str>>, stmt: &str ) {
    let re_add = Regex::new( r"^\w+ (?<name>(?:[A-Z][a-z]+ )+)to (?<dept>[A-Z][a-z]+).$" ).unwrap();
    let Some( caps ) = re_add.captures(stmt) else {
        println!( "'{stmt}' not comprehensible. Try again." );
        return;
    };
    let (_, [name, dept]) = caps.extract();
    println!( "Cmd: Add; Name: {name}; Dept: {dept}" );

    let vec: Option<&&mut Vec<&str>> = list.get(dept);
    match vec {
        None =>{
            let vec: &mut Vec<&str> = &mut vec![name];
            list.insert( dept, &mut vec );
            println!( "{dept} has been added to list." );
            println!( "{name} has been added to {dept}." );
        },
        Some(vec) => {
            if vec.contains(&name) {
                println!( "{name} is already in {dept}!" );
            } else {
                vec.push(name);
                println!( "{name} has been added to {dept}." );
            }
        },
    }
}

// fn del_empl( _list: &HashMap<&str, mut Vec<&str>>, stmt: &str ) {
//     let re_del = Regex::new( r"^\w+ (?<name>(?:[A-Z][a-z]+ )+)from (?<dept>[A-Z][a-z]+).$" ).unwrap();
//     let Some( caps ) = re_del.captures(stmt) else {
//         println!( "'{stmt}' not comprehensible. Try again." );
//         return;
//     };
//     let (_, [name, dept]) = caps.extract();

//     println!( "Cmd: Add; Name: {name}; Dept: {dept}" );
// }
// fn lst_empl( _list: &HashMap<&str, mut Vec<&str>>, stmt: &str ) {
//     let re_lst = Regex::new( r"\^List all( ?:in|by (?<dept>\w+))?.$" ).unwrap();
//     let Some( caps ) = re_lst.captures(stmt) else {
//         println!( "'{stmt}' not comprehensible. Try again." );
//         return;
//     };
//     let (_, [dept]) = caps.extract();

//     println!( "Cmd: Add; Dept: {dept}" );
// }
