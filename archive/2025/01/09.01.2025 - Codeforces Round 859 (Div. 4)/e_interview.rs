//{"name":"E. Interview","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/E","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n5\n1 2 3 4 5\n\n11\n\n6\n\n3\n\n7\n1 2 3 5 3 4 2\n\n12\n\n6\n","output":"? 4 1 2 3 4\n\n? 2 2 3\n\n? 1 2\n\n! 2\n\n? 4 2 3 5 6\n\n? 2 1 4\n\n! 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EInterview"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, a: &Vec<i32>) -> i32 {
    out.print("? ");
    out.print(a.len());
    out.print(" ");
    out.print_iter(a.iter());
    out.print("\n");
    out.flush();
    input.read_int()
}

fn report(out: &mut Output, a: i32) {
    out.print(format!("! {a}\n"));
    out.flush();
}

enum Mode<'a> {
    IO(Input<'a>, Output<'a>),
    Given(usize, Vec<i32>),
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

    fn get_input(&mut self) -> Vec<i32> {
        self.num_queries = 0;
        match &mut self.mode {
            Mode::IO(input, _out) => {
                let n = input.read();
                input.read_int_vec(n)
            }
            Mode::Given(_target, a) => a.clone(),
        }
    }

    fn query(&mut self, mut p: Vec<i32>) -> i32 {
        self.num_queries += 1;
        if self.num_queries > 30 {
            panic!("too many queries");
        }
        let k = p.len();
        p.sort();
        p.dedup();
        if p.len() != k {
            panic!("a was not unique");
        }
        match &mut self.mode {
            Mode::IO(input, out) => query(input, out, &p),
            Mode::Given(target, a) => p
                .into_iter()
                .map(|p| {
                    let i = p as usize - 1;
                    a[i] + if i == *target { 1 } else { 0 }
                })
                .sum(),
        }
    }

    fn report(&mut self, p: i32) {
        match &mut self.mode {
            Mode::IO(_input, out) => report(out, p),
            Mode::Given(target, a) => {
                let i = p as usize - 1;
                if i == *target {
                    // Correct
                } else {
                    panic!("Wrong answer for {target}, {a:?}, got {i}");
                }
            }
        }
    }
}

fn solve(interactor: &mut Interactor) {
    let a = interactor.get_input();
    let n = a.len();
    let mut prefix = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
    }
    let mut lo = 1;
    let mut hi = n as i32;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if interactor.query((1..=mid).collect()) == prefix[mid as usize] + 1 {
            hi = mid;
        } else {
            lo = mid + 1;
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
            if test_num % 10000 == 0 {
                eprintln!("test num: {test_num}");
            }
            let n = 1 + rng.gen_range(0..10);
            let a: Vec<i32> = (0..n).map(|_| rng.gen_range(1..10)).collect();
            let target = rng.gen_range(0..n);
            let mut interactor = Interactor::new(Mode::Given(target as usize, a));
            solve(&mut interactor);
        }
    }
}

//END MAIN
