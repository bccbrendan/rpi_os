// FIXME: Make me pass! Diff budget: 25 lines.

enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl std::fmt::Debug for Duration {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &MilliSeconds(ms) => write!(f, "{}ms", ms),
            &Seconds(s) => write!(f, "{}ms", s as u64 * 1000),
            &Minutes(m) => write!(f, "{}ms", m as u64 * 1000 * 60),
        }
    }
}

impl std::cmp::PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        match (self, other) {
            (&MilliSeconds(us), &MilliSeconds(them)) => us == them,
            (&MilliSeconds(us), &Seconds(them)) => us == (them as u64) * 1000,
            (&MilliSeconds(us), &Minutes(them)) => us == (them as u64) * 1000 * 60,
            (&Seconds(us), &MilliSeconds(them)) => us as u64 * 1000 == them,
            (&Seconds(us), &Seconds(them)) => us == them,
            (&Seconds(us), &Minutes(them)) => us == them as u32 * 60,
            (&Minutes(us), &MilliSeconds(them)) => us as u64 * 60 * 1000 == them,
            (&Minutes(us), &Seconds(them)) => us as u32 * 60 == them,
            (&Minutes(us), &Minutes(them)) => us == them,
       }
    }
}

use Duration::MilliSeconds;
use Duration::Seconds;
use Duration::Minutes;

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
