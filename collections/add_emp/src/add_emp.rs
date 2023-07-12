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
 *  List all by department. | List all.
 *  Quit
 */

use std::io;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut emp_list: HashMap<String, Vec<String>> = HashMap::new();

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
            "Add"  => add_empl( &mut emp_list, stmt ),
            // "Remove" => del_empl( &emp_list, stmt ),
            "List" => lst_empl( &emp_list, stmt ),
            "Quit" => return,
            &_ => println!( "Command '{cmd}' unknown. Try again." ),
        } 
    }
}

fn lst_empl( list: &HashMap<String, Vec<String>>, stmt: &str ) {
    let re_lst = Regex::new( r"^List all(?: (?:in|by) (?<dept>[_0-9A-Za-z]+))?.$" ).unwrap();
    let Some( caps ) = re_lst.captures(stmt) else {
        println!( "\t'{stmt}' not comprehensible. Try again." );
        return;
    };
    let dept = caps.name("dept").map_or("department", |m| m.as_str());

    if dept == "department" {
        for key in list.keys() {
            let vec = list.get(key).unwrap();
            println!( "\tIn department {key}:" );
            for emp in vec {
                println!( "\t\t{emp}" );
            }
        }
    } else {
        let vec: Option<&Vec<String>> = list.get(dept);
        match vec {
            None => { 
                println!( "\t'{dept}' - no such department." );
                return;
            },
            Some(vec) => {
                println!( "\tFor department {dept}:" );
                for emp in vec {
                    println!( "\t\t{emp}" );
                }
            },
        }
    }
}

fn add_empl( list: &mut HashMap<String, Vec<String>>, stmt: &str ) {
    let re_add = Regex::new( r"^Add (?<name>(?:[A-Z][a-z]+(?: |-))+)to (?<dept>[A-Z][_0-9a-z]+).$" ).unwrap();
    let Some( caps ) = re_add.captures(stmt) else {
        println!( "\t'{stmt}' not comprehensible. Try again." );
        return;
    };
    let (_, [name, dept, ]) = caps.extract();
    let name = name.trim();

    let vec: Option<&mut Vec<String>> = list.get_mut(dept);
    match vec {
        None => {
            let mut vec = Vec::new();
            vec.push( name.to_string() );
            list.insert( dept.to_string(), vec );
            println!( "\tNew department {dept} with {name} has been added." );
        },
        Some(vec) => {
            if vec.contains( &name.to_string() ) {
                    println!( "\t{name} is already in {dept}!" );
            } else {
                vec.push( name.to_string() );
                println!( "\t{name} has been added to {dept}." );
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
