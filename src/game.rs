use rand::prelude::*;

pub const FIELD_WIDTH: usize = 4;
pub const FIELD_HEIGHT: usize = 4;

enum MoveDirection {
    Left,
    Right,
    Top,
    Bottom,
}

enum GameState {
    WaitingForStart,
    Playing,
    Lose,
    Win,
}

pub struct Game2048 {
    playground: [[i32; FIELD_HEIGHT]; FIELD_WIDTH],
    game_state: GameState,
    rng: ThreadRng,
}

impl Game2048 {
    pub fn new() -> Self {
        Game2048 {
            playground: [[0; FIELD_HEIGHT]; FIELD_WIDTH],
            game_state: GameState::WaitingForStart,
            rng: thread_rng(),
        }
    }
    pub fn get_cell(&self, x: usize, y: usize) -> i32 {
        assert!(x < FIELD_WIDTH);
        assert!(y < FIELD_HEIGHT);

        self.playground[x][y]
    }
    pub fn start(&mut self) {
        assert!(matches!(self.game_state, GameState::WaitingForStart));
        self.game_state = GameState::Playing;
        self.generate_and_place_random_block();
    }
    pub fn update(&mut self, move_direction: MoveDirection) {
        // 1) try to move at least 1 block,
        //    - if cannot then do nothing
        //    - if can, then move every possible block
        //
        // 2) after movings, try to add random 2 or 4 block on empty cell
        //
        // 3) check if there's any possible move, if not then game is over
        assert!(matches!(self.game_state, GameState::Playing));
        if !self.can_move_in_direction(move_direction) {
            return;
        }
        // Make move
        // Generate random block
        self.generate_and_place_random_block();
    }
    fn generate_and_place_random_block(&mut self) {
        assert!(matches!(self.game_state, GameState::Playing));

        let mut available_cells = Vec::<(usize, usize)>::new();

        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                if self.playground[x][y] == 0 {
                    available_cells.push((x, y));
                }
            }
        }

        let (cell_x, cell_y) = available_cells.choose(&mut self.rng).unwrap();
        let values = vec![2, 4];
        let value = values.choose(&mut self.rng).unwrap();

        println!("{cell_x} {cell_y}");

        self.playground[*cell_x][*cell_y] = *value;
    }
    fn can_move_in_direction(&self, move_direction: MoveDirection) -> bool {
        match move_direction {
            MoveDirection::Left => {
                for i in 0..FIELD_HEIGHT {
                    let mut line = Vec::<i32>::new();
                    for x in 0..FIELD_WIDTH {
                        let block = self.playground[x][i];
                        line.push(block);
                    }
                    if Self::can_move_line(&line, false) {
                        return true;
                    }
                }
            }
            MoveDirection::Right => {
                for i in 0..FIELD_HEIGHT {
                    let mut line = Vec::<i32>::new();
                    for x in 0..FIELD_WIDTH {
                        let block = self.playground[x][i];
                        line.push(block);
                    }
                    if Self::can_move_line(&line, true) {
                        return true;
                    }
                }
            }
            MoveDirection::Top => {
                for i in 0..FIELD_WIDTH {
                    let mut line = Vec::<i32>::new();
                    for y in 0..FIELD_HEIGHT {
                        let block = self.playground[i][y];
                        line.push(block);
                    }
                    if Self::can_move_line(&line, false) {
                        return true;
                    }
                }
            }
            MoveDirection::Bottom => {
                for i in 0..FIELD_WIDTH {
                    let mut line = Vec::<i32>::new();
                    for y in 0..FIELD_HEIGHT {
                        let block = self.playground[i][y];
                        line.push(block);
                    }
                    if Self::can_move_line(&line, true) {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn can_move_line(line: &Vec<i32>, forward: bool) -> bool {
        // Here should be a big load of checks

        let mut all_zero_check = false;
        for el in line {
            if *el != 0 {
                all_zero_check = true;
                break;
            }
        }
        if !all_zero_check {
            return false;
        }

        let mut line_full_with_diff_check = true;
        let mut numbers = Vec::<i32>::new();
        for el in line {
            if *el == 0 {
                break;
            }
            if numbers.contains(el) {
                break;
            }
            numbers.push(*el);
            if numbers.len() == line.len() {
                line_full_with_diff_check = false;
            }
        }
        if !line_full_with_diff_check {
            return false;
        }

        true
    }
}
