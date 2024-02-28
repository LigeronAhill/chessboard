use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address {
    pub row: u8,
    pub col: u8,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseAddressError;

impl Address {
    pub fn parse(s: &str) -> Result<Self, ParseAddressError> {
        Address::from_str(s)
    }
}
impl FromStr for Address {
    type Err = ParseAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let &[col, row] = s.as_bytes() else {
            return Err(ParseAddressError);
        };

        let col = match col {
            b'a'..=b'h' => col - b'a',
            b'A'..=b'H' => col - b'A',
            _ => return Err(ParseAddressError),
        };

        let row = match row {
            b'1'..=b'8' => row - b'1',
            _ => return Err(ParseAddressError),
        };

        Ok(Self { col, row })
    }
}
