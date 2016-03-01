use std::collections::HashSet;
use std::io;
use std::io::BufRead;

mod password;

use password::Password;

#[derive(Debug)]
enum Failure {
    Input,
    Validation,
}

fn main() {
    // Each password represents a given string and its putative distance to the "correct" string,
    // which information we presume will be provided accurately as a result of Robco's ever so
    // helpful password hinting function--you know, the one that tells you the number of letters
    // you guessed correctly. Between that and the fact that the "encrypted" password file is
    // available to be viewed by any user on the system, and the fact that only the usernames and
    // other profile information are actually encrypted, we can pretty easily hack any Robco
    // terminal in existence. This tool assists in that task by accepting a list of passwords (of
    // the form <word> [<distance>]) on standard in and then printing all those words which are
    // valid candidates for all witnesses, thereby narrowing down the user's options considerably.
    match read_passwords() {
        Err(e) => panic!("{:?}", e),
        Ok(pairs) => {
            // Here we have a list of lists containing all valid words for each word with a known
            // distance. From these, we will print only those words appearing in all lists.
            let valid_words: Vec<HashSet<&str>> = pairs.iter()
                .filter_map(|pair| match pair.distance() {
                    None => None,
                    Some(distance) => Some(pairs.iter()
                        .filter(|other| distance == other.closeness_to(&pair))
                        .map(|pair| pair.word())
                        .collect())
                }).collect();

            match valid_words.first() {
                None => println!("At least one word must have a known distance"),
                Some(first) => {
                    let shared_words = first.iter().filter(|&word|
                        valid_words.iter().skip(1).all(|set| set.contains(word))
                    );

                    for word in shared_words {
                        println!("{}", word);
                    }
                }
            }
        }
    }
}

fn read_passwords() -> Result<Vec<Password>, Failure> {
    let handle = io::stdin();

    // This is another one of those cases where a return statement mollifies the borrow checker
    // but a simple expression does not. I'm surprised these are still cropping up; I had thought
    // they were fixed. It is possible that this is a regression.
    return handle.lock().lines()
        .map(|line| line
            .map_err(|_| Failure::Input)
            .and_then(|line| line.parse().map_err(|_| Failure::Validation))
        ).collect();
}
