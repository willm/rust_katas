fn main() {
    println!("Hello, world!");
}

#[derive(Copy, Clone)]
struct Match {
    players: (usize, usize),
}

trait Tennis {
    fn new() -> Self;
    fn score(&self) -> String;
    fn player_scores(&mut self, player: Player) -> ();
}

fn get_leading_score(
    player_score: usize,
    other_player_score: usize,
    player_name: String,
) -> String {
    if player_score - other_player_score > 1 {
        return format!("Game - {}", player_name);
    }
    format!("Advantage - {}", player_name)
}

impl Tennis for Match {
    fn new() -> Match {
        Match { players: (0, 0) }
    }

    fn score(&self) -> String {
        let scores = ["Love", "15", "30", "40"];
        if self.players.0 >= scores.len() - 1 && self.players.0 == self.players.1 {
            return String::from("Deuce");
        }
        if self.players.0 >= scores.len() {
            return get_leading_score(self.players.0, self.players.1, String::from("Player A"));
        }
        if self.players.1 >= scores.len() {
            return get_leading_score(self.players.1, self.players.0, String::from("Player B"));
        }
        let formatted_player_a_score = scores[self.players.0];
        let formatted_player_b_score = scores[self.players.1];
        format!(
            "{} - {}",
            formatted_player_a_score, formatted_player_b_score
        )
    }

    fn player_scores(&mut self, player: Player) {
        match player {
            Player::A => self.players.0 += 1,
            Player::B => self.players.1 += 1,
        }
    }
}

enum Player {
    A,
    B,
}

#[test]
fn love_love() {
    let game: Match = Tennis::new();
    assert_eq!(game.score(), "Love - Love");
}

#[test]
fn fifteen_love() {
    let mut game: Match = Tennis::new();
    game.player_scores(Player::A);
    assert_eq!(game.score(), "15 - Love");
}

#[test]
fn love_fifteen() {
    let mut game: Match = Tennis::new();
    game.player_scores(Player::B);
    assert_eq!(game.score(), "Love - 15");
}

#[test]
fn game_player_a() {
    let mut game: Match = Tennis::new();
    game.player_scores(Player::A);
    game.player_scores(Player::A);
    game.player_scores(Player::A);
    game.player_scores(Player::A);
    assert_eq!(game.score(), "Game - Player A");
}

#[test]
fn game_player_b() {
    let mut game: Match = Tennis::new();
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    assert_eq!(game.score(), "Game - Player B");
}

#[test]
fn deuce() {
    let mut game: Match = Tennis::new();
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::A);
    game.player_scores(Player::A);
    game.player_scores(Player::A);

    assert_eq!(game.score(), "Deuce");
}

#[test]
fn advantage() {
    let mut game: Match = Tennis::new();
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::A);
    game.player_scores(Player::A);
    game.player_scores(Player::A);
    game.player_scores(Player::A);

    assert_eq!(game.score(), "Advantage - Player A");
}

#[test]
fn advantage_b() {
    let mut game: Match = Tennis::new();
    game.player_scores(Player::A);
    game.player_scores(Player::A);
    game.player_scores(Player::A);
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::B);

    assert_eq!(game.score(), "Advantage - Player B");
}

#[test]
fn advantage_game_b() {
    let mut game: Match = Tennis::new();
    game.player_scores(Player::A);
    game.player_scores(Player::A);
    game.player_scores(Player::A);
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::B);
    game.player_scores(Player::B);

    assert_eq!(game.score(), "Game - Player B");
}
