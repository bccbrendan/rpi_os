use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};
use std::str;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        unimplemented!()
    }
}

fn _print_splash() {
    kprintln!(r##"      ___         ___           ___     "##);
    kprintln!(r##"     /  /\       /  /\         /  /\    "##);
    kprintln!(r##"    /  /::\     /  /::\       /  /:/_   "##);
    kprintln!(r##"   /  /:/\:\   /  /:/\:\     /  /:/ /\  "##);
    kprintln!(r##"  /  /:/~/:/  /  /:/  \:\   /  /:/ /::\ "##);
    kprintln!(r##" /__/:/ /:/  /__/:/ \__\:\ /__/:/ /:/\:\"##);
    kprintln!(r##" \  \:\/:/   \  \:\ /  /:/ \  \:\/:/~/:/"##);
    kprintln!(r##"  \  \::/     \  \:\  /:/   \  \::/ /:/ "##);
    kprintln!(r##"   \  \:\      \  \:\/:/     \__\/ /:/  "##);
    kprintln!(r##"    \  \:\      \  \::/        /__/:/   "##);
    kprintln!(r##"     \__\/       \__\/         \__\/    "##);
}

fn _bell() {
    kprint!("\x07");
}

fn _backspace() {
    // backspace, print space, backspace again
    kprint!("\x08 \x08");
}

fn _read_line<'a>(mut storage: &'a mut [u8]) -> &'a str {
    { // scope to borry storage
        let max_len = storage.len();
        let mut line = StackVec::<u8>::new(&mut storage);
        loop {
            let byte = CONSOLE.lock().read_byte();
            match byte {
                printable @ 32 ... 126 | printable @ 128 ... 255 => {
                    if line.len() < max_len {
                        kprint!("{}", printable as char);
                        line.push(printable).unwrap();
                    } else {
                        _bell();
                    }
                }
                b'\r' | b'\n' => {
                     kprintln!("");
                     break
                }
                8 | 127 => {
                    if line.len() > 0 {
                        _backspace();
                    }
                }
                _ => _bell()
            }
        }
    }
    return str::from_utf8(&storage[0..storage.len()]).unwrap();
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    _print_splash();
    loop {
        kprint!("{}", prefix);
        let mut storage = [0u8; 512];
        let input_line = _read_line(&mut storage);
    }
}
