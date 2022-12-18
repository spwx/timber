use crate::die::Die;

pub enum AttackResult {
    Miss,
    Hit,
    Crit,
}

pub enum DamageType {
    Force,
    Slashing,
}

pub struct Damage {
    damage_die: Vec<Die>,
    damage_type: DamageType,
    damage: Optional<i32>,
}

pub struct Timber {
    pub dmg_die: Die,
    pub bonus: i32,
}

impl Timber {
    pub fn new() -> Self {
        Timber {
            dmg_die: Die(6),
            bonus: 0,
        }
    }

    pub fn damage(&mut self, damage_bonus: i32) -> Vec<Damage> {
        // let mut damage = self.dmg_die.roll() + self.bonus + damage_bonus;
        let mut damage = Damage {
            damage_die: vec![Die(6)],
            damage_type: DamageType::Slashing,
            damage: None,
        };

        if self.bonus == 3 {
            damage += self.dmg_die.roll() + self.dmg_die.roll() + self.dmg_die.roll();
        }

        if self.bonus >= 3 {
            self.bonus = 0;
        } else {
            self.bonus += 1;
        }

        damage
    }
}
