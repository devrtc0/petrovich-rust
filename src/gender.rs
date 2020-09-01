use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

const MALE: &str = "male";
const FEMALE: &str = "female";
const ANDROGYNOUS: &str = "androgynous";
//Gender constants
#[derive(PartialEq, Debug)]
pub enum Gender {
    Male,
    Female,
    Androgynous,
}

impl Gender {
    pub fn value(&self) -> &'static str {
        match self {
            Gender::Male => MALE,
            Gender::Female => FEMALE,
            Gender::Androgynous => ANDROGYNOUS,
        }
    }

    pub fn detect_gender(middle_name: &str) -> Gender {
        if middle_name.ends_with("ич") {
            return Gender::Male;
        }
        if middle_name.ends_with("на") {
            return Gender::Female;
        }
        Gender::Androgynous
    }

    pub fn equal(&self, gender: &str) -> bool {
        if *self == Gender::Androgynous {
            return true;
        }
        match gender {
            ANDROGYNOUS => true,
            _ => self.value() == gender,
        }
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_male() {
        let actual = Gender::detect_gender("Алексеевич");
        assert_eq!(Gender::Male, actual);
    }

    #[test]
    fn detect_female() {
        let actual = Gender::detect_gender("Алексеевна");
        assert_eq!(Gender::Female, actual);
    }

    #[test]
    fn detect_androgynous() {
        let actual = Gender::detect_gender("Бильжо");
        assert_eq!(Gender::Androgynous, actual);
    }

    #[test]
    fn equal_test() {
        assert!(Gender::Male.equal("male"));
        assert!(Gender::Male.equal("androgynous"));
        assert!(!Gender::Male.equal("female"));

        assert!(!Gender::Female.equal("male"));
        assert!(Gender::Female.equal("androgynous"));
        assert!(Gender::Female.equal("female"));

        assert!(Gender::Androgynous.equal("male"));
        assert!(Gender::Androgynous.equal("androgynous"));
        assert!(Gender::Androgynous.equal("female"));
    }
}
