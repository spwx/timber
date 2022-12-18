mod character;
mod die;
mod weapons;

// use weapons::AttackResult;
use crate::weapons::Timber;
use character::Character;

fn main() {
    let mut jengu = Character {
        str: 4,
        proficiency: 2,
        weapon: Timber::new(),
    };

    println!("{}", jengu.attack(10));
    println!("{}", jengu.attack(10));
    println!("{}", jengu.attack(10));
    println!("{}", jengu.attack(10));

    // let damage = match timber.attack(&jengu, 12) {
    //     AttackResult::Miss => 0,
    //     AttackResult::Hit(dmg) => dmg,
    //     AttackResult::Crit(dmg) => dmg,
    // };

    // println!("{}", timber.attack(&jengu, 12));
}
