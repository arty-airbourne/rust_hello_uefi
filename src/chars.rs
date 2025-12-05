use uefi::Char16;

pub const RETURN_KEY: Char16 = unsafe { Char16::from_u16_unchecked('\r' as u16) };
pub const SPACE_KEY: Char16 = unsafe { Char16::from_u16_unchecked(' ' as u16)};
pub const BACKSPACE_KEY: Char16 = unsafe { Char16::from_u16_unchecked('\u{8}' as u16)};
pub const NULL_KEY: Char16 = unsafe { Char16::from_u16_unchecked('\0' as u16)};

pub const NUMBER_ONE: Char16 = unsafe { Char16::from_u16_unchecked('1' as u16)};
pub const NUMBER_TWO: Char16 = unsafe {  Char16::from_u16_unchecked('2' as u16)};
pub const NUMBER_THREE: Char16 = unsafe { Char16::from_u16_unchecked('3' as u16)};
pub const NUMBER_FOUR: Char16 = unsafe { Char16::from_u16_unchecked('4' as u16)};
pub const NUMBER_FIVE: Char16 = unsafe { Char16::from_u16_unchecked('5' as u16)};
pub const NUMBER_SIX: Char16 = unsafe { Char16::from_u16_unchecked('6' as u16)};
pub const NUMBER_SEVEN: Char16 = unsafe { Char16::from_u16_unchecked('7' as u16)};
pub const NUMBER_EIGHT: Char16 = unsafe { Char16::from_u16_unchecked('8' as u16)};
pub const NUMBER_NINE: Char16 = unsafe { Char16::from_u16_unchecked('9' as u16)};
pub const NUMBER_ZERO: Char16 = unsafe { Char16::from_u16_unchecked('0' as u16)};


pub const ADDITION: Char16 = unsafe { Char16::from_u16_unchecked('+' as u16)};
pub const NEGATE: Char16 = unsafe { Char16::from_u16_unchecked('-' as u16)};
pub const DIVISION: Char16 = unsafe { Char16::from_u16_unchecked('/' as u16)};
pub const MULTIPLICATION: Char16 = unsafe { Char16::from_u16_unchecked('*' as u16)};
pub const OPEN_BRACKET: Char16 = unsafe { Char16::from_u16_unchecked('(' as u16)};
pub const CLOSE_BRACKET: Char16 = unsafe { Char16::from_u16_unchecked(')' as u16)};
pub const EXPONENTIAL: Char16 = unsafe { Char16::from_u16_unchecked('^' as u16)};
pub const SQROOT: Char16 = unsafe { Char16::from_u16_unchecked('$' as u16)};
pub const EQUALS: Char16 = unsafe { Char16::from_u16_unchecked('=' as u16)};



pub fn from(input_char: char) -> Char16 {
    unsafe {
        Char16::from_u16_unchecked(input_char as u16)
    }
}