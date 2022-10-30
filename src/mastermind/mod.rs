mod hint;

use hint::Hint;

#[derive(PartialEq, Debug)]
pub struct Mastermind {
    secret: [u32; 4],
    resolved: bool,
    guesses: u32,
    max_guesses: u32,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    MaxGuesses,
    InvalidSecret,
}

impl Mastermind {
    pub fn new(secret: [u32; 4], max_guesses: u32) -> Result<Mastermind, Error> {
        if !Self::check_secret(secret) {
            return Err(Error::InvalidSecret);
        }

        Ok(Mastermind {
            secret,
            resolved: false,
            guesses: 0,
            max_guesses,
        })
    }

    pub fn get_secret(&self) -> [u32; 4] {
        self.secret
    }

    pub fn guess(&mut self, guess: [u32; 4]) -> Result<Hint, Error> {
        if self.guesses >= self.max_guesses {
            return Err(Error::MaxGuesses);
        }
        if guess == self.secret {
            self.resolved = true;
        }
        self.guesses += 1;
        Ok(self.compare_guess_and_secret(guess))
    }

    pub fn is_resolved(&self) -> bool {
        self.resolved
    }

    // private functions and methods

    fn check_secret(secret: [u32; 4]) -> bool {
        for i in 0..secret.len() {
            for j in i + 1..secret.len() {
                if secret[i] == secret[j] {
                    return false;
                }
            }
        }
        return true;
    }

    fn compare_guess_and_secret(&self, guess: [u32; 4]) -> Hint {
        let mut in_right_position = 0;
        let mut in_wrong_position = 0;
        for (guess_index, guess_item) in guess.iter().enumerate() {
            for (secret_index, secret_item) in self.secret.iter().enumerate() {
                if guess_item == secret_item && guess_index == secret_index {
                    in_right_position += 1;
                } else if guess_item == secret_item && guess_index != secret_index {
                    in_wrong_position += 1;
                }
            }
        }
        Hint::new(in_right_position, in_wrong_position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_new_game() {
        let game = Mastermind::new([1, 2, 3, 4], 1).unwrap();

        assert_eq!([1, 2, 3, 4], game.get_secret())
    }

    #[test]
    fn cannot_create_a_game_with_an_invalid_secret() {
        assert_eq!(Err(Error::InvalidSecret), Mastermind::new([1, 1, 1, 1], 1));
        assert_eq!(Err(Error::InvalidSecret), Mastermind::new([1, 2, 3, 1], 1));
        assert_eq!(Err(Error::InvalidSecret), Mastermind::new([1, 2, 3, 1], 1));
    }

    #[test]
    #[allow(dead_code)]
    fn solves_a_game_in_one_guess() {
        let mut game = Mastermind::new([1, 2, 3, 4], 1).unwrap();

        let _ = game.guess([1, 2, 3, 4]);

        assert!(game.is_resolved())
    }

    #[test]
    fn gives_a_hint_of_one_number_in_right_position() {
        let mut game = Mastermind::new([1, 5, 6, 7], 1).unwrap();

        let hint = game.guess([1, 2, 3, 4]);

        assert_eq!(Ok(Hint::new(1, 0)), hint);
    }

    #[test]
    fn gives_a_hint_of_one_number_in_wrong_position() {
        let mut game = Mastermind::new([5, 2, 6, 7], 1).unwrap();

        let hint = game.guess([2, 1, 3, 4]);

        assert_eq!(Ok(Hint::new(0, 1)), hint);
    }

    #[test]
    fn gives_a_hint_with_one_in_right_position_and_one_in_wrong_position() {
        let mut game = Mastermind::new([5, 2, 6, 7], 1).unwrap();

        let hint = game.guess([2, 1, 6, 4]);

        assert_eq!(Ok(Hint::new(1, 1)), hint);
    }

    #[test]
    fn stops_after_maximum_guess() {
        let mut game = Mastermind::new([4, 5, 6, 7], 2).unwrap();

        let _ = game.guess([1, 2, 3, 4]);
        let _ = game.guess([1, 2, 3, 4]);
        let hint = game.guess([1, 2, 3, 4]);

        assert_eq!(Err(Error::MaxGuesses), hint)
    }
}
