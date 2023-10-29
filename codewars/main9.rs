
fn main() {
    test5();
    test8();
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}
impl Direction {
    fn turn_cw(&self) -> Self {
        match &self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }

    fn get_direction_vec(&self) -> (i32, i32) {
        match &self {
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Up => (0, -1),
        }
    }

    fn check_forword(&self, board: &Vec<Vec<i8>>, x: usize, y: usize, end: bool) -> bool {
        let (dx, dy) = self.get_direction_vec();
        let (mut x, mut y) = (x as i32, y as i32);
        x += dx;
        y += dy;
        if (x >= board[0].len() as i32 || x < 0) || (y >= board.len() as i32 || y < 0) {
            return false;
        }
        if end && board[y as usize][x as usize] == 1 {return false}
        x += dx;
        y += dy;
        if !end && !(x >= board[0].len() as i32 || x < 0) && !(y >= board.len() as i32 || y < 0) {
            if board[y as usize][x as usize] == 1 {return false}
        }
        true
    }
}

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut board = vec![vec![0;size];size];

    let (mut x, mut y) = (0, 0);
    let mut dir = Direction::Right;
    loop {
        let mut turned = false;
        board[y][x] = 1;
        if !dir.check_forword(&board, x, y, false) {
            turned = true;
            dir = dir.turn_cw();
        }
        if turned && !dir.check_forword(&board, x, y, false) {
            break;
        }
        let (dx, dy) = dir.get_direction_vec();
        x = (x as i32 + dx) as usize;
        y = (y as i32 + dy) as usize;
        if !dir.turn_cw().check_forword(&board, x, y, true) {
            break;
        }
    }
    // dbg!(&board); // lol
    board
}


















fn test5() {
    assert_eq!(
        spiralize(5),
        [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
    );
}
fn test8() {
    assert_eq!(
        spiralize(8),
        [
            [1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 1, 0, 1],
            [1, 0, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
        ],
    );
}
