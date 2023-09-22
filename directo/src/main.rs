
struct Character {
    id: u32,
}

impl Character {

    fn new(id: u32) -> Character {
        Character {
            id
        }
    }
}

fn do_character_things (character: &mut Character) {
    character.id = 2;
}


fn main() {
    let mut character = Character::new(1);

    do_character_things(&mut character);

    println!("Character id: {}", character.id);
}