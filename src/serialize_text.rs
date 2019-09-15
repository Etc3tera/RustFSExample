use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
enum Role {
    Attacker,
    Support,
    Tanker
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    id: u32,
    name: String,
    role: Role
}

#[derive(Debug, Serialize, Deserialize)]
struct Party {
    name: String,
    members: Vec<Player>
}

fn main() {
    let alice = Player { id: 1, name: "Alice".to_string(), role: Role::Support };
    let bob = Player { id: 2, name: "Bob".to_string(), role: Role::Tanker };
    let prim = Player { id: 3, name: "Primula".to_string(), role: Role::Attacker };

    let x = Party { name: "For Fun".to_string(), members: vec!(alice, bob, prim) };

    let json = serde_json::to_string(&x).unwrap();
    println!("{}", json);
    println!();

    {
        let mut fp = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open("party.json")
            .unwrap();
    
        fp.write_all(json.as_bytes());
    }

    let mut fp = fs::OpenOptions::new()
        .read(true)
        .open("party.json")
        .unwrap();
    
    let mut buffer = String::new();
    fp.read_to_string(&mut buffer);

    let x_ds: Party = serde_json::from_str(&buffer).unwrap();
    println!("{:?}", x_ds);
}