use uefi::Char16;

pub const UNSIGNED_SIZE: usize = 0;
pub const UNSIGNED_16: u16 = 0;
pub const INTEGER_16: i16 = 0;
pub const CHARACTER: char = '0';
pub const CHARACTER_16: Char16 = unsafe { Char16::from_u16_unchecked('0' as u16) };
