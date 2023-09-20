// // use std::io;
//
// struct Vector2d {
//     x: i32,
// }
//
// fn add(v: i32) {
//     let x = v + 2;
//     println!("<<<{x}>>>")
// }
//
// fn aaa () {
//     let vec =  Vector2d {
//         x: 1
//     };
//     let list: [Vector2d; 1] = [vec];
//     let mut v = list[0].x;
//
//     add(v);
//
//     {
//         let f = 23;
//         v = v - f;
//     }
//
//     println!("{v} <");
//
//
//     let s = format!("dad {v}");
//     str(s.clone());
//     println!("{s} 2")
// }
//
// fn str(s: String) {
//     println!("{s} 1")
// }
//
// fn abc(num: &mut i32) {
//     *num = 2;
// }
//
// fn abbb() {
//     let mut a = 123;
//     abc(&mut a);
//     println!("{a}");
// }
//
// fn vec() {
//     let mut vec: Vec<u32> = Vec::new();
//     vec.push(34);
//
//     println!("{}", vec.capacity())
// }
//
// fn loops() {
//     let data = vec![1, 2, 3];
//     for datum in &data {
//         println!("{}", datum)
//     }
//     println!("{}", data[0]);
//
//     let mut counter = 10;
//     while counter > 0 {
//         counter -= 1;
//         println!("{}", counter);
//         if counter == 5 {
//             break;
//         }
//     }
//
//     let value = loop {
//         println!("hello");
//         break 123;
//     };
//     println!("{}", value);
// }
//
// struct Position(i64, i64);
// impl Position {
//     fn new(x: i64, y: i64) -> Self {
//         Position(x, y)
//     }
// }
//
// struct Player {
//     name: String,
//     hp: u8,
//     location: Position
// }
//
// fn die(Player { name, location: Position(x, y), .. }: Player) {
//     println!("Player {name} die {x} {y}")
// }
//
// fn ply() {
//
//     let player = Player {
//         name: "Pablo".to_string(),
//         hp: 100,
//         location: Position::new(0, 0)
//     };
//
//     let Player { name, .. } = &player;
//     println!("{name}");
//     die(player);
// }
//
// enum Type {
//     NORMAL { hp: i8 },
//     REGULAR
// }
//
// fn type_normal() {
//     println!("Normal")
// }
//
// fn m () {
//     let mut state = Type::NORMAL { hp: 1 };
//     match state {
//         Type::NORMAL { hp: 1 } => type_normal(),
//         Type::NORMAL { hp } => {
//             state = Type::REGULAR
//         },
//         Type::REGULAR => {}
//     }
//
//     let value = match true {
//         true => "Hello",
//         false => "Non hello"
//     };
//     println!("{value}")
// }
//
// fn get_player(id: u8) -> Option<String> {
//     if id == 0 {
//         return Some("Hello".to_string());
//     }
//     return None;
// }
//
// pub fn main() {
//     let player = match get_player(1) {
//         Some(player) => {
//             println!("{}", player)
//         }
//         None => {}
//     };
// }