//{"name":"F. Penchick and Even Medians","group":"Codeforces - Codeforces Round 987 (Div. 2)","url":"https://codeforces.com/contest/2031/problem/F","interactive":true,"timeLimit":3000,"tests":[{"input":"2\n6\n\n3 4\n\n3 4\n\n2 3\n\n10\n\n3 4\n\n6 7\n","output":"\n? 6 1 2 3 4 5 6\n\n? 4 3 6 1 5\n\n? 4 3 6 2 5\n\n! 3 6\n\n? 6 1 3 7 8 9 10\n\n? 8 1 2 3 4 5 6 7 8\n\n! 6 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPenchickAndEvenMedians"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, a: &Vec<i32>) -> (i32, i32) {
    out.print("? ");
    out.print(a.len());
    out.print(" ");
    out.print_iter(a.iter());
    out.print("\n");
    out.flush();
    let a = input.read_int();
    let b = input.read_int();
    (a, b)
}

fn report(out: &mut Output, a: i32, b: i32) {
    out.print(format!("! {a} {b}\n"));
    out.flush();
}

enum Mode<'a> {
    IO(Input<'a>, Output<'a>),
    Given(Vec<i32>),
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

    fn get_n(&mut self) -> i32 {
        self.num_queries = 0;
        match &mut self.mode {
            Mode::IO(input, _out) => input.read_int(),
            Mode::Given(perm) => perm.len() as i32,
        }
    }

    fn query(&mut self, mut a: Vec<i32>) -> (i32, i32) {
        self.num_queries += 1;
        if self.num_queries > 80 {
            panic!("too many queries");
        }
        let k = a.len();
        if k % 2 == 1 {
            panic!("odd query");
        }
        if k < 4 {
            panic!("k too low");
        }
        a.sort();
        a.dedup();
        if a.len() != k {
            panic!("a was not unique");
        }
        match &mut self.mode {
            Mode::IO(input, out) => query(input, out, &a),
            Mode::Given(perm) => {
                let mut x = a
                    .into_iter()
                    .map(|a| perm[a as usize - 1])
                    .collect::<Vec<_>>();
                x.sort();
                (x[k / 2 - 1], x[k / 2])
            }
        }
    }

    fn report(&mut self, i: i32, j: i32) {
        match &mut self.mode {
            Mode::IO(_input, out) => report(out, i, j),
            Mode::Given(perm) => {
                let n = perm.len() as i32;
                let a = n / 2;
                let b = n / 2 + 1;
                let x = perm[i as usize - 1];
                let y = perm[j as usize - 1];
                if (x, y) == (a, b) || (x, y) == (b, a) {
                    // Correct
                } else {
                    panic!("Wrong answer for {perm:?}, got {i} {j}");
                }
            }
        }
    }
}

fn solve(interactor: &mut Interactor) {
    let n = interactor.get_n() as i32;
    let (a, b) = (n / 2, n / 2 + 1);
    let mut lo = vec![];
    let mut hi = vec![];
    let mut cands = vec![];
    let mut smaller = 0;
    let mut bigger = 0;
    let mut top = vec![];
    let mut bottom = vec![];
    let mut top_bottom = vec![];
    for i in (1..=n).step_by(2) {
        let (x, y) = interactor.query((1..=n).filter(|x| *x != i && *x != i + 1).collect());
        if (x, y) == (a, b) {
            top_bottom.push((i, i + 1));
        } else if x == a {
            cands.push(i);
            cands.push(i + 1);
            smaller += 1;
        } else if y == b {
            cands.push(i);
            cands.push(i + 1);
            bigger += 1;
        } else if y == a {
            hi.push((i, i + 1));
        } else if x == b {
            lo.push((i, i + 1));
        } else {
            interactor.report(i, i + 1);
            return;
        }
        if !lo.is_empty() && !hi.is_empty() {
            let (c1, c2) = lo.pop().unwrap();
            let (c3, c4) = hi.pop().unwrap();
            let (x, y) = interactor.query(vec![c1, c2, c3, c4]);
            if x == a {
                cands.push(c1);
                cands.push(c2);
                smaller += 1;
            } else {
                bottom.push(c1);
                bottom.push(c2);
            }
            if y == b {
                cands.push(c3);
                cands.push(c4);
                bigger += 1;
            } else {
                top.push(c3);
                top.push(c4);
            }
        }
    }
    for (x, y) in top_bottom {
        if smaller < 2 && bigger < 2 {
            cands.push(x);
            cands.push(y);
            smaller += 1;
            bigger += 1;
        }
    }
    for x in bottom {
        if smaller < 2 {
            cands.push(x);
            smaller += 1;
        }
    }
    for x in top {
        if bigger < 2 {
            cands.push(x);
            bigger += 1;
        }
    }
    for i in 0..cands.len() {
        for j in 0..i {
            let (x, y) = interactor.query(
                cands
                    .clone()
                    .into_iter()
                    .filter(|x| *x != cands[i] && *x != cands[j])
                    .collect(),
            );
            if x != a && x != b && y != a && y != b {
                interactor.report(cands[i], cands[j]);
                return;
            }
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
        for test_num in 1.. {
            if test_num % 10000 == 0 {
                eprintln!("test num: {test_num}");
            }
            let n = 100;
            let mut perm: Vec<i32> = (1..=n).collect();
            for i in 0..n {
                let j = rng.gen_range(0..=i);
                perm.swap(i as usize, j as usize);
            }
            let mut interactor = Interactor::new(Mode::Given(perm));
            solve(&mut interactor);
        }
    }
}

//END MAIN
