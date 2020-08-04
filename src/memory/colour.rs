#[derive(Debug, Clone)]
pub struct Colour {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
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
        assert_eq!(
            Colour {
                red: 0,
                green: 0,
                blue: 0
            },
            BLACK
        );
    }
}
