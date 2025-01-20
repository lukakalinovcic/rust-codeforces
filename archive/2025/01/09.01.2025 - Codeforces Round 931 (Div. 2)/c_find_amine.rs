//{"name":"C. Find a Mine","group":"Codeforces - Codeforces Round 931 (Div. 2)","url":"https://codeforces.com/problemset/problem/1934/C","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n4 4\n\n3\n\n2\n\n2\n\n0\n\n5 5\n\n1\n\n2\n\n3\n","output":"\n? 1 1\n\n? 1 4\n\n? 4 1\n\n? 2 3\n\n! 2 3\n\n? 5 5\n\n? 2 2\n\n? 3 3\n\n! 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFindAMine"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, x: i32, y: i32) -> i32 {
    out.print_line(format!("? {x} {y}"));
    out.flush();
    input.read_int()
}

fn report(out: &mut Output, x: i32, y: i32) {
    out.print(format!("! {x} {y}\n"));
    out.flush();
}

enum Mode<'a> {
    IO(Input<'a>, Output<'a>),
    Given(i32, i32, i32, i32, i32, i32),
}

struct Interactor<'a> {
    mode: Mode<'a>,
    num_queries: i32,
}

impl<'a> Interactor<'a> {
    fn new(mode: Mode<'a>) -> Self {
        Self {
            mode,
            num_queries: 0,
        }
    }

    fn get_input(&mut self) -> (i32, i32) {
        self.num_queries = 0;
        match &mut self.mode {
            Mode::IO(input, _out) => {
                let n = input.read_int();
                let m = input.read_int();
                (n, m)
            }
            Mode::Given(n, m, _, _, _, _) => (*n, *m),
        }
    }

    fn query(&mut self, x: i32, y: i32) -> i32 {
        self.num_queries += 1;
        if self.num_queries > 4 {
            panic!("too many queries");
        }
        match &mut self.mode {
            Mode::IO(input, out) => query(input, out, x, y),
            Mode::Given(n, m, x1, y1, x2, y2) => {
                if x < 1 || x > *n || y < 1 || y > *m {
                    panic!("Coords {x} {y} out of bounds");
                }
                ((x - *x1).abs() + (y - *y1).abs()).min((x - *x2).abs() + (y - *y2).abs())
            }
        }
    }

    fn report(&mut self, x: i32, y: i32) {
        match &mut self.mode {
            Mode::IO(_input, out) => report(out, x, y),
            Mode::Given(_n, _m, x1, y1, x2, y2) => {
                if (x, y) == (*x1, *y1) || (x, y) == (*x2, *y2) {
                    // Correct
                } else {
                    panic!(
                        "Wrong answer for ({} {}), ({} {}), got ({} {})",
                        *x1, *y1, *x2, *y2, x, y
                    );
                }
            }
        }
    }
}

fn solve(interactor: &mut Interactor) {
    let (n, m) = interactor.get_input();
    let d1 = interactor.query(1, 1) + 2;
    let d2a = -interactor.query(n, 1) + n - 1;
    let d2b = interactor.query(1, m) - m + 1;
    let mut cands = vec![];
    for d2 in [d2a, d2b] {
        if (d1 + d2) % 2 == 0 && (d1 - d2) % 2 == 0 {
            let (x, y) = ((d1 + d2) / 2, (d1 - d2) / 2);
            if x >= 1 && x <= n && y >= 1 && y <= m {
                cands.push((x, y));
            }
        }
    }
    match cands.as_slice() {
        [(x1, y1)] => interactor.report(*x1, *y1),
        [(x1, y1), (x2, y2)] => {
            if interactor.query(*x1, *y1) == 0 {
                interactor.report(*x1, *y1);
            } else {
                interactor.report(*x2, *y2);
            }
        }
        _ => panic!("Unexpected cands size"),
    }
}

pub(crate) fn run<'a>(mut input: Input<'a>, output: Output<'a>) -> bool {
    let t = input.read();
    let mut interactor = Interactor::new(Mode::IO(input, output));
    for _ in 0..t {
        solve(&mut interactor);
    }
    true
}

//START MAIN
mod tester;

fn main() {
    if std::env::args().find(|x| x == "io").is_some() {
        let mut sin = std::io::stdin();
        let input = algo_lib::io::input::Input::new(&mut sin);
        let mut stdout = std::io::stdout();
        let output = algo_lib::io::output::Output::new(&mut stdout);
        run(input, output);
    } else {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for test_num in 1.. {
            if test_num % 10000 == 0 {
                eprintln!("test num: {test_num}");
            }
            let n = rng.gen_range(2..100);
            let m = rng.gen_range(2..100);
            let x1 = rng.gen_range(1..=n);
            let y1 = rng.gen_range(1..=m);
            let x2 = rng.gen_range(1..=n);
            let y2 = rng.gen_range(1..=m);
            let mut interactor = Interactor::new(Mode::Given(n, m, x1, y1, x2, y2));
            solve(&mut interactor);
        }
    }
}

//END MAIN
