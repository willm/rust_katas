mod game;
use anyhow::Error;
use game::{Coordinate, Game, Move};
use std::io;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

fn parse(corner_line: String, position_line: String) -> Result<Game, Error> {
    let corner_coordinates: Vec<&str> = corner_line[..corner_line.len() - 1].split(' ').collect();
    let position_bits: Vec<&str> = position_line[..position_line.len()].split(' ').collect();
    Ok(Game::new(
        Coordinate::new(
            str::parse::<i32>(corner_coordinates[0])?,
            str::parse::<i32>(corner_coordinates[1])?,
        ),
        game::Position::new(
            str::parse::<i32>(position_bits[0])?,
            str::parse::<i32>(position_bits[1])?,
            game::Orientation::N,
        ),
    ))
}

fn main() {
    let mut top_right_corner = String::new();
    io::stdin()
        .read_line(&mut top_right_corner)
        .expect("Failed to read stdin");

    let mut starting_position = String::new();
    io::stdin()
        .read_line(&mut starting_position)
        .expect("Failed to read stdin");
    match parse(top_right_corner, starting_position) {
        Ok(mut game) => {
            loop {
                let mut cmd = String::new();
                io::stdin()
                    .read_line(&mut cmd)
                    .expect("Failed to read stdin");
                if cmd == "\n" {
                    break;
                }
                let parsed_move: Result<Move, enum_derive::ParseEnumError> =
                    cmd[..cmd.len() - 1].parse();
                if let Ok(m) = parsed_move {
                    game.make_move(m);
                }
            }

            println!("{}", game.position());
        }
        Err(err) => panic!("Invalid input {:?}", err),
    }
}
