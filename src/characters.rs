// pub struct Attacker {
//     pub proficiency: i32,
//     pub str: i32,
// }

struct Character {
    str: i32,
    proficiency: i32,
    weapon: Weapon,
    off_hand_weapon: Option<Weapon>,
}
