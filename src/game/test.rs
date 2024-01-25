#[cfg(test)]
mod test {

    use crate::game::*;

    #[test]
    fn test_puzzle_actions() {
        let mut puzzle = Puzzle::new(
            3,
            vec![
                Tile { number: 0 },
                Tile { number: 1 },
                Tile { number: 3 },
                Tile { number: 4 },
                Tile { number: 5 },
                Tile { number: 6 },
                Tile { number: 7 },
                Tile { number: 2 },
                Tile { number: 8 },
            ],
        );
        assert_eq!(puzzle.zero_index, 0);

        puzzle.perform(Action::Up);
        assert_eq!(puzzle.zero_index, 3);

        puzzle.perform(Action::Down);
        assert_eq!(puzzle.zero_index, 0);

        puzzle.perform(Action::Left);
        assert_eq!(puzzle.zero_index, 1);

        puzzle.perform(Action::Right);
        assert_eq!(puzzle.zero_index, 0);
    }
}
