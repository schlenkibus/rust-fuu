#![allow(non_snake_case)]
#![allow(unused)]

//include sqlite
use sqlite;
extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn getBankSchema() -> Vec::<DatabaseEntry> {
    let mut ret = Vec::<DatabaseEntry>::new();
    ret.push(DatabaseEntry{row_name = "uuid", row_type = "TEXT"});
    ret.push(DatabaseEntry{row_name = "name", row_type = "TEXT"});
    ret.push(DatabaseEntry{row_name = "version", row_type = "INTEGER"});
    return ret;
}

fn getPresetSchema() -> Vec::<DatabaseEntry> {
    let mut ret = Vec::<DatabaseEntry>::new();
    ret.push(DatabaseEntry{row_name = "uuid", row_type = "TEXT"});
    ret.push(DatabaseEntry{row_name = "bank-uuid", row_type = "TEXT"});
    return ret;
}

fn getParameterSchema() -> Vec::<DatabaseEntry> {
    let mut ret = Vec::<DatabaseEntry>::new();
    ret.push(DatabaseEntry{row_name = "id", row_type = "TEXT"});
    ret.push(DatabaseEntry{row_name = "preset-uuid", row_type = "TEXT"});
    ret.push(DatabaseEntry{row_name = "value", row_type = "FLOAT"});
    ret.push(DatabaseEntry{row_name = "mc", row_type = "TEXT"});
    ret.push(DatabaseEntry{row_name = "mod-amt", row_type = "FLOAT"});
    return ret;
} 

struct Bank {
    uuid: String,
    name: String,
    version: i32
}

struct Preset {
    uuid: String,
    bank-uuid: String
}

struct Parameter {
    id: String,
    preset-uuid: String,
    value: f32,
    mc: String, 
    mod_amt: f32
}

struct DatabaseEntry {
    row_name: String,
    row_type: String
}

fn createTable(dbConnection: &sqlite::Connection, tableName: String, schema: Vec<DatabaseEntry>) {
    //create a SQL string like (name TEXT, age INTEGER) from a corresponding Vec<DatabaseEntry>
    let mut schemaString = String::new();
    schemaString.push_str(" (");
    for i in 0..schema.len() {
        let endline = if i == schema.len() - 1 {""} else {", "};
        let entry = schema[i].row_name.clone() + " " + &schema[i].row_type.clone() + endline;
        schemaString.push_str(&entry);
    }
    schemaString.push_str(")");

    println!("{}", schemaString);
    
    let createCmd = format!("CREATE TABLE {} {};", tableName, schemaString);

    dbConnection.execute(
        createCmd,
    ).unwrap();
}

fn insertIntoTable(dbConnection: &sqlite::Connection, tableName: String, name: String, age: String) {
    let insertCmd = format!("INSERT INTO {} VALUES ('{}', '{}');", tableName, name, age);
    dbConnection.execute(
        insertCmd,
    ).unwrap();
}

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("welcome to the NonlinearLabs Bank SQL interface");
    let connection = sqlite::open(":memory:").unwrap();

    let mut bank

    let mut schema = Vec::<DatabaseEntry>::new();
    schema.push(DatabaseEntry{row_name: "name".to_string(), row_type: "TEXT".to_string()});
    schema.push(DatabaseEntry{row_name: "age".to_string(), row_type: "INTEGER".to_string()});
    
    createTable(&connection, "users".to_string(), schema);
    insertIntoTable(&connection, "users".to_string(), "John".to_string(), 60.to_string());

    connection
        .iterate("SELECT * FROM users WHERE age > 50", |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        })
        .unwrap();
}


/*
let colour:char = match tp.color {
		Color::Black => 'b',
		Color::White => 'w'
};
*/