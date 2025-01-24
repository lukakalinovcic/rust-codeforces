//{"name":"D. Cat, Fox and Maximum Array Split","group":"Codeforces - Codeforces Round 945 (Div. 2)","url":"https://codeforces.com/contest/1973/problem/D","interactive":true,"timeLimit":3000,"tests":[{"input":"3\n1 1\n\n1\n2 2\n\n1\n\n3\n\n1\n6 3\n\n7\n\n2\n\n3\n\n6\n\n1\n","output":"! 1\n\n\n? 1 1\n\n? 2 1\n\n! -1\n\n\n? 1 9\n\n? 1 6\n\n? 3 6\n\n? 4 6\n\n! 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCatFoxAndMaximumArraySplit"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, l: i32, x: i32) -> i32 {
    out.print_line(format!("? {l} {x}"));
    out.flush();
    input.read_int()
}

fn report(input: &mut Input, out: &mut Output, x: i32) {
    out.print_line(format!("! {x}"));
    out.flush();
    input.read_int();
}

enum Mode<'a> {
    IO(Input<'a>, Output<'a>),
    Given(Vec<i32>, i32),
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

    fn get_input(&mut self) -> (i32, i32) {
        self.num_queries = 0;
        match &mut self.mode {
            Mode::IO(input, out) => {
                self.n = input.read_int();
                let k = input.read_int();
                (self.n, k)
            }
            Mode::Given(a, k) => {
                self.n = a.len() as i32;
                (a.len() as i32, *k)
            }
        }
    }

    fn query(&mut self, l: i32, x: i32) -> i32 {
        self.num_queries += 1;
        if self.num_queries > 2 * self.n {
            panic!("too many queries");
        }
        match &mut self.mode {
            Mode::IO(input, out) => query(input, out, l, x),
            Mode::Given(a, _k) => {
                let mut m = 0;
                let l = l - 1;
                for r in l..self.n {
                    m = m.max(a[r as usize]);
                    if (r - l + 1) * m == x {
                        eprintln!("q {} {x} {}", l + 1, r + 1);
                        return r + 1;
                    }
                }
                eprintln!("q {} {x} {}", l + 1, self.n + 1);
                self.n + 1
            }
        }
    }

    fn report(&mut self, x: i32) {
        match &mut self.mode {
            Mode::IO(input, out) => report(input, out, x),
            Mode::Given(a, k) => {
                eprintln!("r {x}");
                let n = a.len();
                if x == -1 {
                    let mut m = 0;
                    for a in a.iter() {
                        m = m.max(*a);
                    }
                    for t in 1..=n as i32 / *k {
                        if check(a, t * m) == *k {
                            panic!("Reported -1, partition exists");
                        }
                    }
                } else {
                    if check(a, x) != *k {
                        panic!("Reported {x}: check fails");
                    }
                }
            }
        }
    }
}

fn check(a: &Vec<i32>, x: i32) -> i32 {
    let n = a.len();
    let mut l = 0;
    let mut k = 0;
    while l < n {
        let mut r = l;
        let mut m = 0;
        while r < n {
            m = m.max(a[r]);
            if (r - l + 1) as i32 * m == x {
                break;
            } else if (r - l + 1) as i32 * m > x {
                return -1;
            }
            r += 1;
        }
        if r == n {
            return -1;
        }
        k += 1;
        l = r + 1;
    }
    k
}

fn solve(interactor: &mut Interactor) {
    let (n, k) = interactor.get_input();
    let mut m = 1;
    while interactor.query(1, n * m) != n {
        m += 1;
    }
    for t in 1..=n / k {
        let mut l = 1;
        let mut jumps = 0;
        let mut ok = true;
        while l <= n {
            if jumps == k {
                ok = false;
                break;
            }
            let r = interactor.query(l, t * m);
            if r == n + 1 {
                ok = false;
                break;
            }
            l = r + 1;
            jumps += 1;
        }
        if ok && jumps == k {
            interactor.report(t * m);
            return;
        }
    }
    interactor.report(-1);
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
            let n = rng.gen_range(1..=20);
            // let a = (0..n).map(|_| rng.gen_range(1..=n)).collect::<Vec<_>>();
            // let k = rng.gen_range(1..=n);
            let a = vec![1, 3, 6, 1, 2, 1];
            let k = 3;
            eprintln!("solving {a:?} {k}");
            let mut interactor = Interactor::new(Mode::Given(a, k));
            solve(&mut interactor);
        }
    }
}

//END MAIN
