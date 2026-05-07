pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}

#[derive(Debug, PartialEq)]
pub enum CatBreed {
    Siamese,
    Persian,
    MaineCoon,
    Ragdoll,
    Sphynx,
}

pub struct Cat {
    pub name: String,
    pub age: u8,
    pub breed: CatBreed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_subtracts() {
        let result = subtract(5, 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_creates_cat() {
        let my_cat = Cat {
            name: String::from("Zhima"),
            age: 3,
            breed: CatBreed::Ragdoll,
        };
        assert_eq!(my_cat.name, String::from("Zhima"));
        assert_eq!(my_cat.age, 3);
        assert_eq!(my_cat.breed, CatBreed::Ragdoll);
    }
}
