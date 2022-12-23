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
    ret.push(DatabaseEntry{ row_name: "uuid".to_string(), row_type: "TEXT".to_string()});
    ret.push(DatabaseEntry{ row_name: "name".to_string(), row_type: "TEXT".to_string()});
    ret.push(DatabaseEntry{ row_name: "version".to_string(), row_type: "INTEGER".to_string()});
    return ret;
}

fn getPresetSchema() -> Vec::<DatabaseEntry> {
    let mut ret = Vec::<DatabaseEntry>::new();
    ret.push(DatabaseEntry{row_name: "uuid".to_string(), row_type: "TEXT".to_string()});
    ret.push(DatabaseEntry{row_name: "bankuuid".to_string(), row_type: "TEXT".to_string()});
    return ret;
}

fn getParameterSchema() -> Vec::<DatabaseEntry> {
    let mut ret = Vec::<DatabaseEntry>::new();
    ret.push(DatabaseEntry{row_name: "id".to_string(), row_type: "TEXT".to_string()});
    ret.push(DatabaseEntry{row_name: "presetuuid".to_string(), row_type: "TEXT".to_string()});
    ret.push(DatabaseEntry{row_name: "value".to_string(), row_type: "FLOAT".to_string()});
    ret.push(DatabaseEntry{row_name: "mc".to_string(), row_type: "TEXT".to_string()});
    ret.push(DatabaseEntry{row_name: "modamt".to_string(), row_type: "FLOAT".to_string()});
    return ret;
} 

struct Attribute {
    name: String,
    value: String
}

struct Bank {
    uuid: String,
    name: String,
    version: i32
}

struct Preset {
    uuid: String,
    bank_uuid: String
}

struct Parameter {
    id: String,
    preset_uuid: String,
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

    println!("schema for {}: {}", tableName, schemaString);
    
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
    println!("insert for {}: {} ({})", tableName, name, age);
}

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("welcome to the NonlinearLabs Bank SQL interface");
    let connection = sqlite::open(":memory:").unwrap();

    createTable(&connection, "banks".to_string(), getBankSchema());
    createTable(&connection, "presets".to_string(), getPresetSchema());
    createTable(&connection, "parameters".to_string(), getParameterSchema());

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


    let file = File::open("Kontour_01_Keys_ssc.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;

    // let mut currentBank = Bank{};
    // let mut currentPreset = Preset{};
    // let mut currentParameter = Parameter{};

    let mut currentTag = String::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if (name.to_string() == "value".to_string()) {
                    return;
                }
                println!("{}+{}", indent(depth), name);
                for ownedAttribute in attributes.iter() {
                    println!("{}+{}: {}", indent(depth), name, ownedAttribute);
                }
                depth += 1;
                currentTag = name.to_string();
            }
            Ok(XmlEvent::Characters(text)) => {
                

                match &currentTag as &str {
                    "modSrc" => {
                        
                    },
                    "modAmount" => {
                        
                    },
                    "value" => {

                    },
                    {
                        println!("{} {}", indent(depth), text)
                    }
                }
            }
            Ok(XmlEvent::CData(text)) => {
                println!("{} {}", indent(depth), text);

                
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}


/*
let colour:char = match tp.color {
		Color::Black => 'b',
		Color::White => 'w'
};
*/