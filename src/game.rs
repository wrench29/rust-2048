use rand::prelude::*;

pub const FIELD_WIDTH: usize = 4;
pub const FIELD_HEIGHT: usize = 4;

pub enum MoveDirection {
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
        assert!(matches!(self.game_state, GameState::Playing));

        self.move_in_direction(move_direction);

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

    fn move_in_direction(&mut self, move_direction: MoveDirection) {
        match move_direction {
            MoveDirection::Left => {
                for i in 0..FIELD_HEIGHT {
                    let mut old_line = Vec::<i32>::new();
                    for x in 0..FIELD_WIDTH {
                        let block = self.playground[x][i];
                        old_line.push(block);
                    }
                    let new_line = Game2048::shift(&old_line, false);
                    if new_line.len() == 0 {
                        continue;
                    }
                    for x in 0..FIELD_WIDTH {
                        self.playground[x][i] = new_line[x];
                    }
                }
            }
            MoveDirection::Right => {
                for i in 0..FIELD_HEIGHT {
                    let mut old_line = Vec::<i32>::new();
                    for x in 0..FIELD_WIDTH {
                        let block = self.playground[x][i];
                        old_line.push(block);
                    }
                    let new_line = Game2048::shift(&old_line, true);
                    if new_line.len() == 0 {
                        continue;
                    }
                    for x in 0..FIELD_WIDTH {
                        self.playground[x][i] = new_line[x];
                    }
                }
            }
            MoveDirection::Top => {
                for i in 0..FIELD_WIDTH {
                    let mut old_line = Vec::<i32>::new();
                    for y in 0..FIELD_HEIGHT {
                        let block = self.playground[i][y];
                        old_line.push(block);
                    }
                    let new_line = Game2048::shift(&old_line, false);
                    if new_line.len() == 0 {
                        continue;
                    }
                    for y in 0..FIELD_HEIGHT {
                        self.playground[i][y] = new_line[y];
                    }
                }
            }
            MoveDirection::Bottom => {
                for i in 0..FIELD_WIDTH {
                    let mut old_line = Vec::<i32>::new();
                    for y in 0..FIELD_HEIGHT {
                        let block = self.playground[i][y];
                        old_line.push(block);
                    }
                    let new_line = Game2048::shift(&old_line, true);
                    if new_line.len() == 0 {
                        continue;
                    }
                    for y in 0..FIELD_HEIGHT {
                        self.playground[i][y] = new_line[y];
                    }
                }
            }
        }
    }

    fn shift(input: &Vec<i32>, forward: bool) -> Vec<i32> {
        let mut indices_of_tiles = Vec::<i32>::new();
        let tiles_length = input.len();

        let mut output = Vec::<i32>::new();
        for _ in 0..tiles_length {
            output.push(0);
        }

        for i in 0..tiles_length {
            if input[i] != 0 {
                indices_of_tiles.push(i as i32);
            }
        }

        if indices_of_tiles.len() == 0 {
            return vec![];
        }

        let mut pairs_indices = Vec::<(i32, i32)>::new();

        if forward {
            let mut i = indices_of_tiles.len() - 1;
            while i > 0 {
                let left_index = indices_of_tiles[i] as usize;
                let right_index = indices_of_tiles[i - 1] as usize;
                if input[left_index] == input[right_index] {
                    pairs_indices.push((left_index as i32, right_index as i32));
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        } else {
            let mut i = 0;
            while i < indices_of_tiles.len() - 1 {
                let left_index = indices_of_tiles[i] as usize;
                let right_index = indices_of_tiles[i + 1] as usize;
                if input[left_index] == input[right_index] {
                    pairs_indices.push((left_index as i32, right_index as i32));
                    i += 1;
                }
                i += 1;
            }
        }

        let mut temp_vector = Vec::<i32>::new();
        for i in 0..tiles_length {
            temp_vector.push(input[i]);
        }

        if forward {
            for (left_index, right_index) in pairs_indices {
                temp_vector[right_index as usize] = input[right_index as usize] * 2;
                temp_vector[left_index as usize] = 0;
            }
        } else {
            for (left_index, right_index) in pairs_indices {
                temp_vector[left_index as usize] = input[left_index as usize] * 2;
                temp_vector[right_index as usize] = 0;
            }
        }

        if forward {
            let mut temp_index = tiles_length - 1;
            let mut origin_index = tiles_length - 1;
            loop {
                if temp_vector[temp_index] == 0 {
                    if temp_index == 0 {
                        break;
                    }
                    temp_index -= 1;
                    continue;
                }
                output[origin_index] = temp_vector[temp_index];
                if temp_index == 0 {
                    break;
                }
                temp_index -= 1;
                origin_index -= 1;
            }
        } else {
            let mut temp_index = 0;
            let mut origin_index = 0;
            loop {
                if temp_index == input.len() {
                    break;
                }
                if temp_vector[temp_index] == 0 {
                    temp_index += 1;
                    continue;
                }
                output[origin_index] = temp_vector[temp_index];

                temp_index += 1;
                origin_index += 1;
            }
        }

        output
    }
}
