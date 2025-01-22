//{"name":"G1. Ruler (easy version)","group":"Codeforces - Codeforces Round 964 (Div. 4)","url":"https://codeforces.com/contest/1999/problem/G1","interactive":true,"timeLimit":1000,"tests":[{"input":"2\n\n18\n\n25\n\n\n9999\n","output":"? 3 5\n\n? 4 4\n\n! 4\n? 99 100\n\n! 100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G1RulerEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, x: i32, y: i32) -> i32 {
    out.print_line(format!("? {x} {y}"));
    out.flush();
    input.read_int()
}

fn report(out: &mut Output, x: i32) {
    out.print_line(format!("! {x}"));
    out.flush();
}

enum Mode<'a> {
    IO(Input<'a>, Output<'a>),
    Given(i32),
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

    fn start(&mut self) {
        self.num_queries = 0;
    }

    fn query(&mut self, x: i32, y: i32) -> i32 {
        self.num_queries += 1;
        if self.num_queries > 10 {
            panic!("too many queries");
        }
        match &mut self.mode {
            Mode::IO(input, out) => query(input, out, x, y),
            Mode::Given(a) => {
                let xx = if x < *a { x } else { x - 1 };
                let yy = if y < *a { y } else { y - 1 };
                xx * yy
            }
        }
    }

    fn report(&mut self, x: i32) {
        match &mut self.mode {
            Mode::IO(_input, out) => report(out, x),
            Mode::Given(a) => {
                if x == *a {
                    // Correct
                } else {
                    panic!("Wrong answer for {} got ({})", *a, x);
                }
            }
        }
    }
}

fn solve(interactor: &mut Interactor) {
    interactor.start();
    let mut lo = 2;
    let mut hi = 999;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if interactor.query(mid, mid) == mid * mid {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    interactor.report(lo);
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
            if test_num % 1000000 == 0 {
                eprintln!("test num: {test_num}");
            }
            let x = rng.gen_range(2..=999);
            let mut interactor = Interactor::new(Mode::Given(x));
            solve(&mut interactor);
        }
    }
}

//END MAIN
