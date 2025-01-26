//{"name":"F. Kostyanych's Theorem","group":"Codeforces - Codeforces Round 951 (Div. 2)","url":"https://codeforces.com/contest/1979/problem/F","interactive":true,"timeLimit":3000,"tests":[{"input":"3\n4\n\n0 0\n\n1 4\n\n2 3\n\n4\n\n1 0\n\n4 2\n\n2\n\n1 0\n","output":"\n? 3\n\n? 2\n\n? 1\n\n! 4 3 1 2\n\n? 3\n\n? 0\n\n! 4 1 2 3\n\n? 0\n\n! 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKostyanychsTheorem"}}}

use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, d: i32) -> (i32, i32) {
    out.print_line(format!("? {d}"));
    out.flush();
    (input.read_int(), input.read_int())
}

fn report(out: &mut Output, x: Vec<i32>) {
    out.print("!");
    for x in x {
        out.print(" ");
        out.print(x);
    }
    out.print_line("");
    out.flush();
}

enum Mode<'a> {
    IO(Input<'a>, Output<'a>),
    Given(),
}

struct Interactor<'a> {
    mode: Mode<'a>,
    num_queries: i32,
    n: i32,
}

impl<'a> Interactor<'a> {
    fn new(mode: Mode<'a>) -> Self {
        Self {
            mode,
            num_queries: 0,
            n: 0,
        }
    }

    fn get_input(&mut self) -> i32 {
        self.num_queries = 0;
        match &mut self.mode {
            Mode::IO(input, _) => {
                self.n = input.read_int();
                self.n
            }
            Mode::Given() => unimplemented!(),
        }
    }

    fn query(&mut self, d: i32) -> (i32, i32) {
        self.num_queries += 1;
        if self.num_queries > self.n {
            panic!("too many queries");
        }
        match &mut self.mode {
            Mode::IO(input, out) => query(input, out, d),
            Mode::Given() => unimplemented!(),
        }
    }

    fn report(&mut self, x: Vec<i32>) {
        match &mut self.mode {
            Mode::IO(_input, out) => report(out, x),
            Mode::Given() => unimplemented!(),
        }
    }
}

fn solve(interactor: &mut Interactor) {
    let mut n = interactor.get_input();
    let mut steps = vec![];

    while n > 2 {
        let (i, j) = interactor.query(n - 2);
        if j == 0 {
            let (j, _) = interactor.query(0);
            steps.push((2, i, j));
            n -= 2;
        } else {
            steps.push((1, i, j));
            n -= 1;
        }
    }
    let mut path = VecDeque::new();
    while n > 0 {
        path.push_back(interactor.query(0).0);
        n -= 1;
    }
    while !steps.is_empty() {
        let (t, i, j) = steps.pop().unwrap();
        if t == 2 {
            path.push_back(i);
            path.push_back(j);
        } else {
            if *path.back().unwrap() == j {
                path.push_front(i);
            } else {
                path.push_back(i);
            }
        }
    }
    interactor.report(path.into());
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);
    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);
    run(input, output);
}

//END MAIN
