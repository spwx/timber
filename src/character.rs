use crate::die::Die;
use crate::weapons::{AttackResult, Timber};

pub struct Character {
    pub str: i32,
    pub proficiency: i32,
    pub weapon: Timber,
}

impl Character {
    pub fn attack(&mut self, enemy_ac: i32) -> i32 {
        let roll = { self.attack_roll(enemy_ac) };

        match roll {
            AttackResult::Miss => 0,
            AttackResult::Hit => self.weapon.damage(self.str, false),
            AttackResult::Crit => self.weapon.damage(self.str, true),
        }
    }

    fn attack_roll(&mut self, enemy_ac: i32) -> AttackResult {
        let d20 = Die(20);
        let attack_roll = d20.roll();

        let attack_total = attack_roll + self.proficiency + self.str + self.weapon.bonus;

        match attack_roll {
            20 => AttackResult::Crit,
            1 => AttackResult::Miss,
            _ => match attack_total >= enemy_ac {
                true => AttackResult::Hit,
                false => AttackResult::Miss,
            },
        }
    }
}
