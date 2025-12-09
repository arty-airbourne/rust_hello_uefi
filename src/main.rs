
// The 10 operations by code
// -1 = +
// -2 = -
// -3 = /
// -4 = *
// -5 = (
// -6 = )
// -7 = ^
// -8 = $ (Sqrt)
// -9 = =

#![no_main]
#![no_std]


use uefi::Status;
use uefi::proto::console::text::ScanCode;
use uefi::proto::console::text::Key;
use uefi::Char16;
use uefi::entry;
use::uefi::println;

use core::panic::PanicInfo;

mod iosys;
mod empty;
mod chars;

struct CharConv;

struct Expressions {
    index: i16,
    array: [i32; 1024],
}

struct IsNumber {
    latch: bool,
    array: [i32; 1024],
    index: usize,
    start: bool,
}


// Key Check Constants


// leave delete key and arrow keys as not done yet

impl Expressions {
    fn verify(&mut self, character: Char16) -> i32 {

        let mut matched_char: i32 = match character {
            chars::ADDITION => {-1},
            chars::NEGATE => {-2},
            chars::DIVISION => {-3},
            chars::MULTIPLICATION => {-4},
            chars::OPEN_BRACKET => {-5},
            chars::CLOSE_BRACKET => {-6},
            chars::EXPONENTIAL => {-7},
            chars::SQROOT => {-8},
            chars::EQUALS => {-9},
            _ => todo!(),
        };
        self.index = self.index + 1;
        self.array[self.index as usize] = matched_char;
        matched_char

    }
}

impl CharConv {
    fn from(&self, input_char: char) -> Char16 {
        unsafe {
            Char16::from_u16_unchecked(input_char as u16)
        }
    }
}

impl IsNumber {
    fn verify(&self, verity: Char16) -> bool {
        let one: Char16 = unsafe { Char16::from_u16_unchecked('1' as u16)};
        let two: Char16 = unsafe {  Char16::from_u16_unchecked('2' as u16)};
        let three: Char16 = unsafe { Char16::from_u16_unchecked('3' as u16)};
        let four: Char16 = unsafe { Char16::from_u16_unchecked('4' as u16)};
        let five: Char16 = unsafe { Char16::from_u16_unchecked('5' as u16)};
        let six: Char16 = unsafe { Char16::from_u16_unchecked('6' as u16)};
        let seven: Char16 = unsafe { Char16::from_u16_unchecked('7' as u16)};
        let eight: Char16 = unsafe { Char16::from_u16_unchecked('8' as u16)};
        let nine: Char16 = unsafe { Char16::from_u16_unchecked('9' as u16)};
        let zero: Char16 = unsafe { Char16::from_u16_unchecked('0' as u16)};
        if verity == one || verity == two || verity == three || verity == four || verity == four || verity == five || verity == six || verity == seven || verity == eight || verity == nine || verity == zero {
            true
        } else {
            false
        }
    }
    fn convert(&self, convertee: Char16) ->  i32 {
        
        
        if convertee == chars::NUMBER_ZERO {
            0 as i32
        } else if convertee == chars::NUMBER_ONE {
            1 as i32
        } else if convertee == chars::NUMBER_TWO {
            2 as i32
        } else if convertee == chars::NUMBER_THREE {
            3 as i32
        } else if convertee == chars::NUMBER_FOUR {
            4 as i32
        } else if convertee == chars::NUMBER_FIVE {
            5 as i32
        } else if convertee == chars::NUMBER_SIX {
            6 as i32
        } else if convertee == chars::NUMBER_SEVEN {
            7 as i32
        } else if convertee == chars::NUMBER_EIGHT {
            8 as i32
        } else if convertee == chars::NUMBER_NINE {
            9 as i32
        } else {
            panic!("Not A Number, Jeff!");
        }
    }
}


fn calc(inputted_command: [Char16; 1024]) -> [i32; 1024] {
    let mut length_found: bool = false;
    let mut length_of_input: usize = 0;

    while length_found != true {
        if inputted_command[length_of_input] == chars::NULL_KEY {
            length_found = true;
        } else {
            length_found = false; 
            length_of_input = length_of_input + 1;
        }
    }
    let mut is_number: IsNumber = IsNumber{
        latch: false,
        array: [-1; 1024],
        index: 0,
        start: true,
    };
    let mut expressions: Expressions = Expressions {
        index: 0,
        array: [-10 as i32; 1024], // we support 10 operation display things as described by comments at top
    };
    
    for i in 0..length_of_input {
        //iosys::out(iosys::SimpleIO::Char(inputted_command[i]), empty::UNSIGNED_SIZE);
        //iosys::outdbg(iosys::SimpleIO::Num(i as i16), empty::UNSIGNED_SIZE); // debug
        
        // start gather of numbers
        if is_number.verify(inputted_command[i]) {
            //iosys::outdbg(iosys::SimpleIO::Char(inputted_command[i]), empty::UNSIGNED_SIZE); // debug
            if is_number.latch {
                is_number.array[is_number.index] = (is_number.array[is_number.index] * 10) + is_number.convert(inputted_command[i])
            } else {
                if is_number.start == false {
                    is_number.index = is_number.index + 1;
                    expressions.index = expressions.index + 1;
                    expressions.array[expressions.index as usize] = is_number.index as i32;
                } else {
                    is_number.start = false;
                    expressions.array[expressions.index as usize] = is_number.index as i32;
                }
                is_number.array[is_number.index] = is_number.convert(inputted_command[i]);
                is_number.latch = true;
            }
        } else if (expressions.verify(inputted_command[i]) / -1) > 0 {
            is_number.latch = false;
        }

        //iosys::out(iosys::SimpleIO::ArrayInt(is_number.array), empty::UNSIGNED_SIZE);
        //iosys::ArrayExpOut(iosys::SimpleIO::ArrayExp(expressions.array), iosys::SimpleIO::ArrayInt(is_number.array));
        iosys::out(iosys::SimpleIO::ArrayExp(expressions.array), empty::UNSIGNED_SIZE);
        iosys::out(iosys::SimpleIO::Text("\t"), empty::UNSIGNED_SIZE);
        iosys::out(iosys::SimpleIO::Text("\n"), empty::UNSIGNED_SIZE);
    } 
    is_number.array
    // NUMBER SEPARATION WORKS !! although look into comboing with +-/* <- doing that now past me 
}


#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();


    iosys::out(iosys::SimpleIO::Text("Press ESCAPE to exit...\n"), empty::UNSIGNED_SIZE);
    iosys::out(iosys::SimpleIO::Text("$SHELL > "), empty::UNSIGNED_SIZE);

    let mut position_counter: i16 = 0;
    let mut total_input: [Char16; 1024] = [chars::NULL_KEY; 1024];
    let mut end_program: bool = false;

    loop {
        let result = iosys::inkey();
        match result {
            Ok(Some(Key::Printable(p))) => {
                let mut check: i16;
                check = iosys::out(iosys::SimpleIO::Char(p), position_counter as usize);
                if check == 1 { // RETURN KEY

                    // iosys::out(iosys::SimpleIO::ArrayChar((total_input)), (position_counter) as usize); 
                    // iosys::out(iosys::SimpleIO::Text("\nSHELL $> "), empty::UNSIGNED_SIZE);
                    calc(total_input);
                    // iosys::out(iosys::SimpleIO::ArrayInt(calc(total_input)), empty::UNSIGNED_SIZE);
                    iosys::out(iosys::SimpleIO::Text("\nSHELL $> "), empty::UNSIGNED_SIZE);

                    position_counter = 0;
                    total_input = [chars::NULL_KEY; 1024];


                } else if check == 0 { // NORMAL KEY


                    if position_counter < 1024 {
                        total_input[position_counter as usize] = p;
                    } else {
                        panic!("A manual panic was triggered because we do not implement such a long amount yet.");
                    }
                    position_counter += 1;


                } else if check == 2 { // REMOVE A KEY
                    position_counter -= 1;
                    total_input[position_counter as usize] = chars::NULL_KEY;

                } else if check == 3 { // DO NOT LOG THIS KEY
                }
            },
            Ok(Some(Key::Special(p))) => { // OTHER MODIFIER KEY IE F1
                // iosys::out("You Pressed ");
                // iosys::outdbg(iosys::SimpleIO::Code(p));
                // iosys::out("\n");

                if p == ScanCode::ESCAPE {
                   end_program = true;
                }
            }, 
            Ok(None) => {
                
            },
            Err(_) => {
                iosys::out(iosys::SimpleIO::Text("Error Unknown"), empty::UNSIGNED_SIZE);
            }
        }
        if end_program {
            iosys::out(iosys::SimpleIO::Char(chars::from('\n')), empty::UNSIGNED_SIZE);
            break
        }
    }

    Status::SUCCESS
} 


#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("[PANIC]: {}", info);
    loop {} 
}