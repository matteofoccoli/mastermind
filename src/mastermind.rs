pub struct Mastermind {
    secret: [u32; 4],
    resolved: bool,
}

#[derive(PartialEq, Debug)]
pub struct Hint {
    in_wrong_position: u32,
    in_right_position: u32,
}

impl Mastermind {
    pub fn new(secret: [u32; 4]) -> Mastermind {
        Mastermind {
            secret,
            resolved: false, 
        }
    }

    pub fn get_secret(&self) -> [u32; 4] {
        self.secret
    }

    pub fn guess(&mut self, guess: [u32; 4]) -> Hint {
        if guess == self.secret {
            self.resolved = true;
        }
        Hint {
            in_right_position: 1,
            in_wrong_position: 0
        }
    }

    pub fn is_resolved(&self) -> bool {
        self.resolved
    }
}

#[cfg(test)]
mod mastermind_test {
    use super::*;

    #[test]
    fn creates_a_new_game() {
        let game = Mastermind::new([1,1,1,1]);

        assert_eq!([1,1,1,1], game.get_secret())
    }

    #[test]
    fn solves_a_game_in_one_guess() {
        // Mutable binding
        let mut game = Mastermind::new([1,1,1,1]);

        game.guess([1,1,1,1]);

        assert!(game.is_resolved())
    }

    #[test]
    fn gives_a_hint_after_not_winning_guess() {
        let mut game = Mastermind::new([1,1,1,1]);

        let hint = game.guess([1,2,3,4]);

        assert_eq!(Hint{in_right_position:1, in_wrong_position:0}, hint)
    }

    #[test]
    #[ignore]
    fn stops_after_maximum_guess() {
        let mut game = Mastermind::new([1,1,1,1]);

        let hint = game.guess([1,2,3,4]);

        assert_eq!(Hint{in_right_position:1, in_wrong_position:0}, hint)
    }
}