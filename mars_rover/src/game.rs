use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Orientation {
  N,
  E,
  S,
  W,
}

custom_derive! {
#[derive(Debug, EnumFromStr)]
  pub enum Move {
    F,
    R,
    L,
  }
}

pub struct Coordinate {
  x: i32,
  y: i32,
}
impl Coordinate {
  pub fn new(x: i32, y: i32) -> Self {
    Coordinate { x, y }
  }
}

pub struct Position {
  coordinate: Coordinate,
  orientation: Orientation,
}

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} {} {:?}",
      self.coordinate.x, self.coordinate.y, self.orientation
    )
  }
}

impl Position {
  pub fn new(x: i32, y: i32, orientation: Orientation) -> Self {
    Position {
      coordinate: Coordinate { x, y },
      orientation,
    }
  }
}

pub struct Game {
  top_right_corner: Coordinate,
  position: Position,
}

impl Game {
  pub fn new(top_right_corner: Coordinate, starting_position: Position) -> Self {
    Game {
      top_right_corner,
      position: starting_position,
    }
  }
  pub fn make_move(&mut self, game_move: Move) {
    match game_move {
      Move::F => match self.position.orientation {
        Orientation::N => {
          self.position.coordinate.y += 1;
        }
        Orientation::E => {}
        Orientation::S => {
          self.position.coordinate.y -= 1;
        }
        Orientation::W => {}
      },
      Move::L => match self.position.orientation {
        Orientation::N => {
          self.position.orientation = Orientation::W;
        }
        Orientation::E => {
          self.position.orientation = Orientation::N;
        }
        Orientation::S => {
          self.position.orientation = Orientation::E;
        }
        Orientation::W => {
          self.position.orientation = Orientation::S;
        }
      },
      _ => {}
    }
  }
  pub fn position(&self) -> &Position {
    &self.position
  }
}

#[test]
fn turning_right() {
  Game::new(
    Coordinate { x: 3, y: 3 },
    Position::new(0, 0, Orientation::N),
  );
}
