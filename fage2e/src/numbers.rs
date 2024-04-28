#![allow(dead_code)]

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Dice {
    d6(u8),
    d3(u8),
}

impl Dice {
    pub fn size(&self) -> u8 {
        match self {
            Self::d6(_) => 6,
            Self::d3(_) => 3,
        }
    }

    pub fn count(&self) -> u8 {
        match self {
            Self::d6(num) => *num,
            Self::d3(num) => *num,
        }
    }

    pub fn min_value(&self) -> i16 {
        self.count() as i16
    }

    pub fn max_value(&self) -> i16 {
        (self.size() as i16) * (self.count() as i16)
    }
}

impl std::fmt::Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.count(), self.size())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DiceWithMod {
    pub dice: Dice,
    pub modifier: i8
}

impl DiceWithMod {
    pub fn min_value(&self) -> i16 {
        self.dice.min_value() + self.modifier as i16
    }

    pub fn max_value(&self) -> i16 {
        self.dice.max_value() + self.modifier as i16
    }
}

impl std::fmt::Display for DiceWithMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.modifier < 0 {
            write!(f, "{} - {}", self.dice, -self.modifier)
        }
        else {
            write!(f, "{} + {}", self.dice, self.modifier)
        }
    }
}
