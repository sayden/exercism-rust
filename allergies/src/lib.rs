#[derive(PartialEq, Debug)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    pub fn get_level(&self) -> usize {
        match self {
            &Allergen::Eggs => return 1,
            &Allergen::Peanuts => return 2,
            &Allergen::Shellfish => return 4,
            &Allergen::Strawberries => return 8,
            &Allergen::Tomatoes => return 16,
            &Allergen::Chocolate => return 32,
            &Allergen::Pollen => return 64,
            &Allergen::Cats => return 128,
        }
    }
}


pub struct Allergies(pub usize);

impl Allergies {
    pub fn is_allergic_to(&self, a: &Allergen) -> bool {
        let allergens = Allergies::get_allergens();

        let mut pos = 0;
        for i in allergens {
            if a == &i {
                break;
            } else {
                pos += 1;
            }
        }

        match self.0 & 1 << pos {
            0 => false,
            _ => true,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        return Allergies::get_allergens()
                   .into_iter()
                   .filter(|a| self.is_allergic_to(a))
                   .collect();
    }

    fn get_allergens() -> Vec<Allergen> {
        vec![Allergen::Eggs,
             Allergen::Peanuts,
             Allergen::Shellfish,
             Allergen::Strawberries,
             Allergen::Tomatoes,
             Allergen::Chocolate,
             Allergen::Pollen,
             Allergen::Cats]
    }
}
