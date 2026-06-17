#[derive(Debug)]
enum Direction {
  Left,
  Right,
}

#[derive(Debug)]
enum PlayerMove {
  Pass,
  Run(Direction),
  Teleport { x: u32, y: u32 },
}

fn main() {
    let dir = Direction::Left;
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: {player_move:?}");
    let another_move :PlayerMove =  PlayerMove::Pass;
    println!("On this turn: {another_move:?}");
    let teleport :PlayerMove = PlayerMove::Teleport{y:12,x:13};
    println!("On this turn: {teleport:?}");
}
