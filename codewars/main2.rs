
fn main() {
    println!("{}", solve_n_queens(8, (3, 0)).unwrap());
}

pub fn solve_n_queens(n: usize, mandatory_coords: (usize, usize)) -> Option<String> {
    let mcx = mandatory_coords.0 as i64;
    let mcy = mandatory_coords.1 as i64;
    
    let mut xl = vec![false;n];
    let mut yl = vec![false;n];
    let mut fdl = vec![false;2*n-1];
    let mut bdl = vec![false;2*n-1];
    
    let n = n as i64;

    let mut qv = vec![];
    qv.push(Queen::new(mcx, mcy, n));
    xl[mcx as usize] = true;
    yl[mcy as usize] = true;
    fdl[qv[0].fd() as usize] = true;
    bdl[qv[0].bd() as usize] = true;

    if n == 1 {return form_board(qv, n as usize)}


    // dbg!(&xl, &yl, &fdl, &bdl);

    if bt(n, &mut qv, &mut xl, &mut yl, &mut fdl, &mut bdl, 0, 0) {
        // dbg!(&xl, &yl, &fdl, &bdl, &qv);
        return form_board(qv, n as usize)
    }
    None
}

fn bt(n: i64, qv: &mut Vec<Queen>, xl: &mut Vec<bool>, yl: &mut Vec<bool>, fdl: &mut Vec<bool>, bdl: &mut Vec<bool>, ex: i64, yae: i64) -> bool {
    for y in yae..n {
        if yl[y as usize] {continue}
        for x in 0..n {
            if xl[x as usize] {continue}
            let q = Queen::new(x, y, n);
            if fdl[q.fd() as usize] {continue}
            if bdl[q.bd() as usize] {continue}
            xl[x as usize] = true;
            yl[y as usize] = true;
            fdl[q.fd() as usize] = true;
            bdl[q.bd() as usize] = true;
            qv.push(q);
            if qv.len() == n as usize {
                return true;
            }
            if bt(n, qv, xl, yl, fdl, bdl, x,y) {return true}
        }
    }
    let q = qv.pop().unwrap();
    xl[ex as usize] = false;
    yl[yae as usize] = false;
    fdl[q.fd() as usize] = false;
    bdl[q.bd() as usize] = false;

    false
}

#[derive(Clone, Debug)]
struct Queen {
    x: i64,
    y: i64,
    n: i64
}

impl Queen {
    fn new(x: i64, y: i64, n: i64) -> Self {
        Self {x, y, n}
    }

    fn fd(&self) -> i64 {
        self.x+self.y
    }
    fn bd(&self) -> i64 {
        self.n-1 + self.x-self.y
    }

    // fn check_these() -> [(i64, i64);8] {
    //     [(-1, -2), (-2, -1), (2, 1), (1, 2), (-1, 2), (2, -1), (-2, 1), (1, -2)]
    // }
}

fn form_board(ql: Vec<Queen>, n: usize) -> Option<String> {
    let mut sl = vec![vec![b"."[0];n];n];
    for q in ql {
        sl[q.y as usize][q.x as usize] = b"Q"[0];
    }
    let mut s = vec![];
    for l in sl {
        s.extend(l.iter());
        s.push(b"\n"[0]);
    }
    String::from_utf8(s).ok()
}