use std::str::FromStr;

pub enum Password {
    Candidate(String),
    Witness(String, usize),
}

impl Password {
    pub fn candidate<T: Into<String>>(word: T) -> Password {
        Password::Candidate(word.into())
    }

    pub fn witness<T: Into<String>>(word: T, distance: usize) -> Password {
        Password::Witness(word.into(), distance)
    }

    pub fn word(&self) -> &str {
        match self {
            &Password::Candidate(ref word)
            | &Password::Witness(ref word, _) => word
        }
    }

    pub fn distance(&self) -> Option<usize> {
        match self {
            &Password::Candidate(_) => None,
            &Password::Witness(_, distance) => Some(distance),
        }
    }

    // I was originally using `strsim` and either Hamming or Levenshtein for this comparison, but
    // the reality is that I don't *need* an external library, because this comparison is just too
    // simple to justify it. Using `zip` for this does not handle the case where one password may
    // be longer or shorter than the other, but that's irrelevant for all known production verions
    // of Robco's operating system, because all passwords were nominally required to be the same
    // length, system-wide. Some "high security" variations permit different user access levels
    // to have different required password lengths, but any given user could be granted only one
    // access level, so this is still irrelevant in practice because users of different access
    // levels are actually stored in different password files. Anyway, if you want admin access,
    // why bother trying to crack a luser-level password?
    pub fn closeness_to(&self, other: &Password) -> usize {
        self.word().chars().zip(other.word().chars())
            .filter(|&(a, b)| a == b)
            .count()
    }
}

pub enum PasswordParseError {
    NoInput,
    BadDistance,
}

impl FromStr for Password {
    type Err = PasswordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // This parse implementation will happily ignore any segment after the first two, because
        // why would anyone want to send us any more than two segments? I'm sure it's fine. At
        // least it doesn't require us to create something totally off the wall, like a vector of
        // strings or something. Keep it simple, amirite?
        let mut segments = s.split(' ');
        match segments.next() {
            None => Err(PasswordParseError::NoInput),
            Some(word) => match segments.next() {
                None => Ok(Password::candidate(word)),
                Some(distance) => distance.parse()
                    .map_err(|_| PasswordParseError::BadDistance)
                    .map(|distance| Password::witness(word, distance))
            }
        }
    }
}
