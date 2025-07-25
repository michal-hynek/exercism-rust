use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            score,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_val = *allergen as u32;
        self.score & allergen_val != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
