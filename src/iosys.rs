// marisa stole the precious input output system

use core::fmt::Write;
use uefi::Status;
use uefi::proto::console::text::Key;
use uefi::CStr16;
use uefi::Char16;
use core::slice;
use uefi::proto::console::text::ScanCode;

use crate::chars;
use crate::empty;


pub enum SimpleIO<'a> {
    Char(Char16),
    Code(ScanCode),
    Text(&'a str),
    Num(i16),
    ArrayChar([Char16; 1024]),
    ArrayInt([i32; 1024]),
    ArrayExp([i32; 1024]),
}

pub fn out(output_text: SimpleIO, last_position: usize) -> i16 {
    // uefi::system::with_stdout(|stdout| { 
    //     write!(stdout, "{output_text}").unwrap();  
    // })
    let mut newline: i16 = 0;
    match output_text {
        SimpleIO::Char(c) => {
            newline = uefi::system::with_stdout(|stdout| {
            if c == chars::RETURN_KEY {
                write!(stdout, "\n").unwrap();
                1
            } else if c == chars::BACKSPACE_KEY {
                let mut delete_mode: i16 = 3;
                // self.out(SimpleIO::Num(last_position as i16), empty::UNSIGNED_SIZE);
                if last_position > 0 {
                    out(SimpleIO::Text("\u{8}"), empty::UNSIGNED_SIZE);
                    // write!(stdout, "\u{8}").unwrap();
                    delete_mode = 2;
                }
                // self.out(SimpleIO::Num(delete_mode), empty::UNSIGNED_SIZE);
                delete_mode
            } else if c == chars::SPACE_KEY {
                write!(stdout, "").unwrap();
                3
            } else {
                write!(stdout, "{}", c).unwrap();
                0
            }
            });


        },
        SimpleIO::Code(d) => {
            uefi::system::with_stdout(|stdout| {
            write!(stdout, "{:#?}", d).unwrap();
            });
        }
        SimpleIO::Text(t) => {
            uefi::system::with_stdout(|stdout| {
                write!(stdout, "{}", t).unwrap();
            });
        }
        SimpleIO::Num(n) => {
                uefi::system::with_stdout(|stdout| {
                    write!(stdout, "{}", n).unwrap();
                });
        }
        SimpleIO::ArrayChar(a) => {
            uefi::system::with_stdout(|stdout| {
            let mut b = a;
            b[last_position] = chars::NULL_KEY;
            // let slice_with_nul: &[u16] = &a[..=last_position + 1];
            
            let slice_with_nul: &[u16] = unsafe {
                slice::from_raw_parts(
                    a.as_ptr() as *const u16,
                    last_position + 1, 
                    )
                };

            let cstr: &CStr16 = unsafe { CStr16::from_u16_with_nul_unchecked(slice_with_nul)
                                };

            stdout.output_string(cstr).unwrap();
        });
        },
        SimpleIO::ArrayInt(x) => {
            uefi::system::with_stdout(|stdout| {
            for n in 0..1024 {
                let b: &i32 = &x[n];
                if b == &-1  {
                    write!(stdout, "Breakpoint at {}", n).unwrap();
                    break;
                }
                // Print numbers as decimal values
                write!(stdout, "{} ", b).unwrap();
            }
            });
        },
        SimpleIO::ArrayExp(x) => {
            uefi::system::with_stdout(|stdout| {
            for n in 0..1024 {
                let b: &i32 = &x[n];
                if b == &-10  {
                    write!(stdout, "Breakpoint at {}", n).unwrap();
                    break;
                }
                // Print numbers as decimal values
                write!(stdout, "{} ", b).unwrap();
            }
            });
        },
}
newline
}

pub fn inkey() -> Result<Option<Key>, Status> {
    uefi::system::with_stdin(|stdin| {
        match stdin.read_key() {
            Ok(key) => Ok(key), 
            Err(_) => Err(Status::ABORTED), 
        }
    })
}


pub fn outdbg(to_debug: SimpleIO, _last_position: usize) -> i16 {
    match to_debug {
        SimpleIO::Char(c) => {
            uefi::system::with_stdout(|stdout| {
            write!(stdout, "{:?}", c).unwrap();
            });
        },
        SimpleIO::Code(d) => {
            uefi::system::with_stdout(|stdout| {
            write!(stdout, "{:?}", d).unwrap();
            });
        } 
        SimpleIO::Text(t) => {
            uefi::system::with_stdout(|stdout| {
                write!(stdout, "{:?}", t).unwrap();
            });
        }
        SimpleIO::Num(n) => {
            uefi::system::with_stdout(|stdout| {
                write!(stdout, "{:?}", n).unwrap();
            });
        }
        SimpleIO::ArrayChar(a) => {
            uefi::system::with_stdout(|stdout| {
            write!(stdout, "{:?}", a).unwrap();
            });
        },
        SimpleIO::ArrayInt(a) => {
            uefi::system::with_stdout(|stdout| {
            write!(stdout, "{:?}", a).unwrap();
            });
        },
        SimpleIO::ArrayExp(a) => {
            uefi::system::with_stdout(|stdout| {
            write!(stdout, "{:?}", a).unwrap();
            });
        },
}
0

}



pub fn ArrayExpOut(x: SimpleIO, y: SimpleIO) {
    uefi::system::with_stdout(|stdout| {
        match x {
            SimpleIO::ArrayExp(xx) => {
                match y {
                    SimpleIO::ArrayInt(yy) => {
                        for n in 0..1024 {
                        let b: &i32 = &xx[n];
                        if b == &-10  {
                            write!(stdout, "Breakpoint at {}", n).unwrap();
                            break;
                        } else if b >= &0 {
                            write!(stdout, "{}", yy[*b as usize]).unwrap();
                        }
                        write!(stdout, "{} ", b).unwrap();
                        // Print numbers as decimal values
                        }
                    },
                    _ => todo!(),
                }
            },
            _ => todo!(),
        }
            
            });
    }