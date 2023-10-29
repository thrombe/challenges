
fn main() {
    small_fixed_tests()
}

fn check_pipe(pipe_map: &[&str]) -> bool {
    // let pipes = vec!['┗', '┓', '┏', '┛', '━', '┃', '┣', '┫', '┳', '┻', '╋'];
    let up_pipes = vec!['┗', '┛', '┃', '┣', '┫', '┻', '╋'];
    let down_pipes = vec!['┓', '┏', '┃', '┣', '┫', '┳', '╋'];
    let right_pipes = vec!['┗', '┏', '━', '┣', '┳', '┻', '╋'];
    let left_pipes = vec!['┓', '┛', '━', '┫', '┳', '┻', '╋'];

    let pipe_map = {
        let mut pml: Vec<Vec<char>> = vec![];
        for pipe_line in pipe_map {
            pml.push(pipe_line.chars().collect());
        }
        pml
    };
    let width = pipe_map[0].len();
    let height = pipe_map.len();
    let mut been_here_map = vec![vec![false;width];height];
    // top/bottom
    for x in 0..width {
        let pp = pipe_map[0][x];
        if up_pipes.contains(&pp) {
            if !re(&pipe_map, &mut been_here_map, x, 0, true) {return false}
        }
        let pp = pipe_map[height-1][x];
        if down_pipes.contains(&pp) {
            if !re(&pipe_map, &mut been_here_map, x, height-1, true) {return false}
        }
    }    
    for y in 0..height {
        let pp = pipe_map[y][0];
        if left_pipes.contains(&pp) {
            if !re(&pipe_map, &mut been_here_map, 0, y, true) {return false}
        }
        let pp = pipe_map[y][width-1];
        if right_pipes.contains(&pp) {
            if !re(&pipe_map, &mut been_here_map, width-1, y, true) {return false}
        }
    }    

    
    true
}

fn re(pipe_map: &Vec<Vec<char>>, been_here_map: &mut Vec<Vec<bool>>, x: usize, y: usize, recurse: bool) -> bool {
    let width = pipe_map[0].len();
    let height = pipe_map.len();
    let up_pipes = vec!['┗', '┛', '┃', '┣', '┫', '┻', '╋'];
    let down_pipes = vec!['┓', '┏', '┃', '┣', '┫', '┳', '╋'];
    let right_pipes = vec!['┗', '┏', '━', '┣', '┳', '┻', '╋'];
    let left_pipes = vec!['┓', '┛', '━', '┫', '┳', '┻', '╋'];
    let pp = pipe_map[y][x];

    if been_here_map[y][x] {return true}
    been_here_map[y][x] = true;

    if left_pipes.contains(&pp) {
        if x == 0 {
        } else if right_pipes.contains(&pipe_map[y][x-1]) {
            if !re(pipe_map, been_here_map, x-1, y, recurse) {return false}
        } else {
            return false
        }
    }
    if right_pipes.contains(&pp) {
        if x == width-1 {
        } else if left_pipes.contains(&pipe_map[y][x+1]) {
            if !re(pipe_map, been_here_map, x+1, y, recurse) {return false}
        } else {
            return false
        }
    }
    if up_pipes.contains(&pp) {
        if y == 0 {
        } else if down_pipes.contains(&pipe_map[y-1][x]) {
            if !re(pipe_map, been_here_map, x, y-1, recurse) {return false}
        } else {
            return false
        }
    }
    if down_pipes.contains(&pp) {
        if y == height-1 {
        } else if up_pipes.contains(&pipe_map[y+1][x]) {
            if !re(pipe_map, been_here_map, x, y+1, recurse) {return false}
        } else {
            return false
        }
    }

    true
}







fn run_test(pmap: &[&str], answer: bool) {
    let test_result = check_pipe(pmap);
    assert!(
        test_result == answer,
        "Output: {}; expected value: {}; for input:\n{}\n",
        test_result,
        answer,
        pmap.join("\n")
    );
}

fn small_fixed_tests() {
    for (pmap, answer) in &TEST_CASES {
        run_test(pmap, *answer);
    }
}
const TEST_CASES: [([&str; 3], bool); 7] = [
    (["╋━━┓", "┃..┃", "┛..┣"], true),
    (["...┏", "┃..┃", "┛..┣"], false),
    (["...┏", "...┃", "┛..┣"], false),
    (["...┏", "...┃", "┓..┣"], true),
    (["╋", "╋", "╋"], true),
    (["╋....", "┃..┛.", "┃...."], false),
    (["....", ".┛┛.", "...."], true),
];