//{"name":"E. Tensor","group":"Codeforces - Codeforces Round 948 (Div. 2)","url":"https://codeforces.com/contest/1977/problem/E","interactive":true,"timeLimit":3000,"tests":[{"input":"2\n4\n\nYES\n\nYES\n\nYES\n\nNO\n\nNO\n\nNO\n\n5\n","output":"? 1 2\n\n? 2 3\n\n? 1 3\n\n? 1 4\n\n? 2 4\n\n? 3 4\n\n! 0 0 0 1\n\n! 1 1 0 1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETensor"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, i: i32, j: i32) -> bool {
    out.print_line(format!("? {i} {j}"));
    out.flush();
    let s = input.read_line();
    s == "YES"
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

    fn query(&mut self, i: i32, j: i32) -> bool {
        self.num_queries += 1;
        if self.num_queries > 2 * self.n {
            panic!("too many queries");
        }
        match &mut self.mode {
            Mode::IO(input, out) => query(input, out, i, j),
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
    let n = interactor.get_input();

    let mut a = vec![1];
    let mut b = vec![];
    let mut i = 2;
    while i <= n && interactor.query(*a.last().unwrap(), i) {
        a.push(i);
        i += 1;
    }
    if i <= n {
        b.push(i);
        i += 1;
    }
    let mut c = vec![];
    while i <= n {
        if c.is_empty() {
            let aa = interactor.query(*a.last().unwrap(), i);
            let bb = interactor.query(*b.last().unwrap(), i);
            if aa && bb {
                c.push(i);
            } else if aa {
                a.push(i);
            } else if bb {
                b.push(i);
            } else {
                panic!("Unexpected");
            }
        } else {
            let cc = interactor.query(*c.last().unwrap(), i);
            if cc {
                c.push(i);
            } else {
                let aa = interactor.query(*a.last().unwrap(), i);
                if aa {
                    a.push(i);
                    b.append(&mut c);
                } else {
                    b.push(i);
                    a.append(&mut c);
                }
            }
        }
        i += 1;
    }

    let mut result = vec![0; n as usize];
    for b in b {
        result[b as usize - 1] = 1;
    }
    interactor.report(result);
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
