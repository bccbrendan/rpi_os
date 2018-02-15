// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
   pub fn to_string(&self) -> String {
        match (&self.string, self.number) {
            (&Some(ref s), Some(n)) => format!("{} {}", s, n),
            (&Some(ref s), None) => s.to_string(),
            (&None, Some(n)) => n.to_string(),
            _ => "".to_string()
        }
    }

    pub fn string<S: Into<String>>(&mut self, s: S) -> &mut Builder {
        self.string = Some(s.into());
        self
    }
    pub fn number(&mut self, n: usize) -> &mut Builder {
        self.number = Some(n);
        self
    }
}

impl Default for Builder {
    fn default() -> Builder {
       Builder { string: None, number: None }
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
