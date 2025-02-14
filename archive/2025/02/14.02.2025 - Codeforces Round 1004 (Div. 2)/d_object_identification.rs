//{"name":"D. Object Identification","group":"Codeforces - Codeforces Round 1004 (Div. 2)","url":"https://codeforces.com/contest/2067/problem/D","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n3\n2 2 3\n\n1\n\n0\n\n5\n5 1 4 2 3\n\n4\n\n4\n","output":"? 2 3\n\n? 1 2\n\n! A\n\n? 1 5\n\n? 5 1\n\n! B\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DObjectIdentification"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, i: i32, j: i32) -> i32 {
    out.print_line(format!("? {i} {j}"));
    out.flush();
    input.read_int()
}

fn report(out: &mut Output, ch: char) {
    out.print_line(format!("! {ch}"));
    out.flush();
}

enum Mode<'a> {
    IO(Input<'a>, Output<'a>),
    Given(Vec<i32>, Vec<i32>, char),
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

    fn get_input(&mut self) -> (usize, Vec<i32>) {
        self.num_queries = 0;
        match &mut self.mode {
            Mode::IO(input, _) => {
                let n = input.read_size();
                let x = input.read_int_vec(n);
                (n, x)
            }
            Mode::Given(x, _, _) => {
                let n = x.len();
                (n, x.clone())
            }
        }
    }

    fn query(&mut self, i: i32, j: i32) -> i32 {
        self.num_queries += 1;
        if self.num_queries > 2 {
            panic!("too many queries");
        }
        match &mut self.mode {
            Mode::IO(input, out) => query(input, out, i, j),
            Mode::Given(x, y, ch) => {
                let i = i as usize - 1;
                let j = j as usize - 1;
                if *ch == 'A' {
                    let n = x.len();
                    let mut dist = vec![-1; n];
                    let mut q = vec![i as i32];
                    let mut i = 0;
                    while i < q.len() {
                        let u = q[i];
                        i += 1;
                        for k in 0..n {
                            if x[k] - 1 == u {
                                let v = y[k] - 1;
                                if dist[v as usize] == -1 {
                                    dist[v as usize] = dist[u as usize] + 1;
                                    q.push(v);
                                }
                            }
                        }
                    }
                    if dist[j] == -1 {
                        0
                    } else {
                        dist[j]
                    }
                } else {
                    (x[i] - x[j]).abs() + (y[i] - y[j]).abs()
                }
            }
        }
    }

    fn report(&mut self, c: char) {
        match &mut self.mode {
            Mode::IO(_input, out) => report(out, c),
            Mode::Given(x, y, ch) => {
                if c != *ch {
                    panic!("Wrong answer for {x:?} {y:?} {ch}. Got {c}");
                }
            }
        }
    }
}

fn solve(interactor: &mut Interactor) {
    let (n, x) = interactor.get_input();
    let mut seen = vec![false; n + 1];
    let n = n as i32;
    let (mut i, mut mini) = (-1, n + 1);
    let (mut j, mut maxi) = (0, 0);
    for (k, x) in x.into_iter().enumerate() {
        if x < mini {
            (i, mini) = (k as i32, x);
        }
        if x > maxi {
            (j, maxi) = (k as i32, x);
        }
        seen[x as usize] = true;
    }
    let mut missing = 1;
    while missing <= n && seen[missing as usize] {
        missing += 1;
    }
    if missing <= n {
        let (i, j) = if missing == 1 {
            (missing, n)
        } else {
            (missing, 1)
        };
        if interactor.query(i, j) == 0 {
            interactor.report('A');
        } else {
            interactor.report('B');
        }
    } else {
        let d1 = interactor.query(i + 1, j + 1);
        let d2 = interactor.query(j + 1, i + 1);
        if d1 == d2 && d1 >= maxi - mini {
            interactor.report('B');
        } else {
            interactor.report('A');
        }
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
        for test_num in 0.. {
            if test_num % 1000 == 0 {
                eprintln!("test num: {test_num}");
            }
            let n = rng.gen_range(3..=10);
            let (x, y) = loop {
                let x = (0..n)
                    .map(|_| rng.gen_range(1..=n as i32))
                    .collect::<Vec<i32>>();
                let y = (0..n)
                    .map(|_| rng.gen_range(1..=n as i32))
                    .collect::<Vec<i32>>();
                let mut ok = true;
                for i in 0..n {
                    if x[i] == y[i] {
                        ok = false;
                    }
                    for j in 0..i {
                        if (x[i], y[i]) == (x[j], y[j]) {
                            ok = false;
                        }
                    }
                }
                if ok {
                    break (x, y);
                }
            };
            let ch = if rng.gen_bool(0.5) { 'A' } else { 'B' };
            let mut interactor = Interactor::new(Mode::Given(x, y, ch));
            solve(&mut interactor);
        }
    }
}
//END MAIN
