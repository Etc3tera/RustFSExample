use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::io::SeekFrom;

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
    role: Role,
    money: u32
}

#[derive(Debug, Serialize, Deserialize)]
struct Party {
    name: String,
    members: Vec<Player>
}

fn main() {
    let alice = Player { id: 10, name: "Alice".to_string(), role: Role::Support, money: 10 };
    let bob = Player { id: 1000, name: "Bob".to_string(), role: Role::Tanker, money: 255 };
    let prim = Player { id: 100000, name: "Primula".to_string(), role: Role::Attacker, money: 1000 };

    let x = Party { name: "For Fun".to_string(), members: vec!(alice, bob, prim) };

    let bin = bincode::serialize(&x).unwrap();
    println!("{:?}", bin);
    println!();

    // We write file in closure, after closure end, fp is destroy, file is successfully close
    // If file is not successfully closed, we cannot use read_to_end() method, because EOF is not found.
    {
        let mut fp = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open("party.dat")
            .unwrap();

        fp.write_all(&bin);
    }

    // Read file and deserialize
    {
        let mut fp = fs::OpenOptions::new()
        .read(true)
        .open("party.dat")
        .unwrap();

        let mut buffer = Vec::new();
        let len = fp.read_to_end(&mut buffer).unwrap();
        println!("file lenght = {}", len);

        let x_ds: Party = bincode::deserialize(&buffer[..]).unwrap();
        println!("{:?}", x_ds);
    }

    // Uncomment this to play with File.seek
    // Alice's money will be rewritten.
    // {
    //     let mut fp = fs::OpenOptions::new()
    //         .create(true)
    //         .read(true)
    //         .write(true)
    //         .open("party.dat")
    //         .unwrap();

    //     fp.seek(SeekFrom::Start(
    //         8       // String Party.name length in u64 (value = 7)
    //         + 7     // 'For Fun' (7 bytes)
    //         + 8     // Vector Party.members store length in u64
    //         + 4     // Party.members[0].id in (u32)
    //         + 8     // String Party.members[0].name length in u64 (value = 5)
    //         + 5     // 'Alice' (5 bytes)
    //         + 4     // Enum Party.members[0].role is u32
    //     ));

    //     let new_money = bincode::serialize(&1000000u32).unwrap();
    //     fp.write_all(&new_money[..]);

    //     // read again
    //     fp.seek(SeekFrom::Start(0));
    //     let mut buffer = Vec::new();
    //     let len = fp.read_to_end(&mut buffer).unwrap();
    //     println!("file lenght = {}", len);

    //     let x_ds: Party = bincode::deserialize(&buffer[..]).unwrap();
    //     println!("{:?}", x_ds);
    // }
}