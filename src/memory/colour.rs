#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Colour {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Colour { red, green, blue }
    }
}

pub const BLACK: Colour = Colour {
    red: 0,
    green: 0,
    blue: 0,
};

impl PartialEq for Colour {
    fn eq(&self, other: &Colour) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fetch_black_colour() {
        assert_eq!(Colour::new(0, 0, 0), BLACK);
    }

    #[test]
    fn test_match_fail() {
        assert_ne!(Colour::new(0, 0, 255), Colour::new(0, 255, 0));
    }

    #[test]
    fn creates_colour() {
        let colour = Colour::new(255, 128, 0);
        let expected = Colour {
            red: 255,
            green: 128,
            blue: 0,
        };
        assert_eq!(colour, expected);
    }
}
