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

    #[test]
    fn test_puzzle_is_unsolved() {
        let puzzle = Puzzle::new(
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
        assert!(!puzzle.is_win(), "Unsolved puzzle is marked as solved");
    }

    #[test]
    fn test_puzzle_is_solved() {
        let puzzle = Puzzle::new(
            3,
            vec![
                Tile { number: 1 },
                Tile { number: 2 },
                Tile { number: 3 },
                Tile { number: 4 },
                Tile { number: 5 },
                Tile { number: 6 },
                Tile { number: 7 },
                Tile { number: 8 },
                Tile { number: 0 },
            ],
        );
        assert!(puzzle.is_win(), "Solved puzzle is marked as unsolved");
    }
}
