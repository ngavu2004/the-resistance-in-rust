
use std::os::unix::net::{UnixListener, UnixStream};
use std::os::unix::fs::PermissionsExt;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use rand::Rng;
use rand::seq::IndexedRandom;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum Role {
    Merlin,
    Assassin,
    ResistanceOperative,
    GovernmentSpy,
    Undecided
}

impl Role {
    fn as_string(&self) -> String {
        match self {
            Role::Merlin => String::from("Merlin"),
            Role::Assassin => String::from("Assassin"),
            Role::ResistanceOperative => String::from("Resistance Operative"),
            Role::GovernmentSpy => String::from("Goverment Spy"),
            Role::Undecided => String::from("Undecided")
        }
    }
}

#[derive(Debug)] 
struct Player {
    id: i32,
    name: String,
    stream: UnixStream,
    role: Role
}

fn choose_player(id_list: &Vec<i32>, n: usize, rng:&mut impl Rng) -> Vec<i32> {
    id_list.sample(rng, n).copied().collect()
}

fn send_message(stream: &mut UnixStream, message: &String) {
    let message = message.to_owned();
    stream.write_all(message.as_bytes()).unwrap();
}

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/game.sock";

    // Remove old socket if it exists
    let _ = std::fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path)?;
    std::fs::set_permissions(socket_path, std::fs::Permissions::from_mode(0o666))?;

    // Get number of players
    let player_num: i32 = loop {
        let mut input = String::new();

        println!("Please enter the number of player:");
        io::stdin().read_line(&mut input)?;

        let number : i32 = input.trim().parse().unwrap();

        if number < 6 || number > 10 {
            println!("The game is only for 6 to 10 players. Please try again!");
            continue;
        } else {
            break number;
        }
    };
    
    // ==== CONNECTING PLAYERS ====
    let mut players_map : HashMap<i32, Player> = HashMap::new(); // map id to player object
    let mut player_count : i32 = 0;
    println!("Waiting for players to connect...");

    while player_count < player_num {
        let (mut stream, _) = listener.accept()?;
        let mut buffer = [0u8; 1024];
        let n = stream.read(&mut buffer)?;

        let player_name = String::from_utf8_lossy(&buffer[..n]).trim().to_owned();
        
        println!("{player_name} connected!");

        let message = format!("Welcome to the game, {player_name}!");
        stream.write_all(message.as_bytes())?;

        let new_player_id : i32 = player_count;
        player_count += 1;
        let new_player = Player { id: new_player_id, name: player_name, stream: stream, role: Role::Undecided};
        players_map.insert(new_player_id, new_player);
    }

    // Create a hash map to map roles to player ids;
    let merlin_c : usize = 1;
    let assassin_c : usize = 1;
    let governmentspy_c : usize = 3;
    let resistanceop_c : usize = (player_num as usize) - merlin_c - assassin_c - governmentspy_c;

    let mut role2player_map: HashMap<Role, Vec<i32>> = HashMap::new();

    // ==== ASSIGNING ROLES ====
    let mut unchosen_roles : Vec<(Role, usize)> = vec!((Role::Merlin, merlin_c), (Role::Assassin,assassin_c), (Role::GovernmentSpy,governmentspy_c), (Role::ResistanceOperative,resistanceop_c));
    let mut rng = rand::rng();

    while unchosen_roles.len() > 0 {
        let mut choices : Vec<i32> = Vec::new();
        
        // Get the players whose role hasnt been decided
        for id in 0..player_count {
            let player : &Player = players_map.get(&id).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "Missing player",
                )
            })?;
            if player.role == Role::Undecided {
                choices.push(id);
            }
        }

        let (role, count) = match unchosen_roles.pop() {
            Some((x, y)) => (x, y),
            None => (Role::Undecided, 0)
        };
        let chosen = choose_player(&choices, count, &mut rng);

        // Update the roles of the chosen
        for id in &chosen {
            let player : &mut Player = players_map.get_mut(id).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "Missing player",
                )
            })?;
            player.role = role.clone();
        }

        role2player_map.insert(role, chosen);
    }
    
    // ==== ROLE ANNOUCEMENT ====
    println!("\n== PLAYERS INFO ==\n");
    for id in 0..player_count {
        let mut player : &mut Player = players_map.get_mut(&id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Missing player",
            )
        })?;
        
        // Announce the role to player
        let role_message = format!("Your role is {}", player.role.as_string());

        send_message(&mut player.stream, &role_message);
        println!("{player:?}");
    }

    
    Ok(())
}

