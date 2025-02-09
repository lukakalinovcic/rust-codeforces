//{"name":"C. Password Cracking","group":"Codeforces - Codeforces Round 973 (Div. 2)","url":"https://codeforces.com/contest/2013/problem/C","interactive":true,"timeLimit":2000,"tests":[{"input":"4\n3\n\n0\n\n0\n\n1\n\n4\n\n4\n\n2\n","output":"\n? 00\n\n? 000\n\n? 010\n\n! 010\n\n! 1100\n\n! 0110\n\n! 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPasswordCracking"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, t: &[u8]) -> i32 {
    out.print_line(format!("? {}", String::from_utf8(t.to_vec()).unwrap()));
    out.flush();
    input.read_int()
}

fn report(out: &mut Output, t: &[u8]) {
    out.print_line(format!("! {}", String::from_utf8(t.to_vec()).unwrap()));
    out.flush();
}

enum Mode<'a> {
    IO(Input<'a>, Output<'a>),
    Given(String),
}

struct Interactor<'a> {
    mode: Mode<'a>,
    num_queries: i32,
    n: usize,
}

impl<'a> Interactor<'a> {
    fn new(mode: Mode<'a>) -> Self {
        Self {
            mode,
            num_queries: 0,
            n: 0,
        }
    }

    fn get_input(&mut self) -> usize {
        self.num_queries = 0;
        match &mut self.mode {
            Mode::IO(input, _) => {
                self.n = input.read_size();
                self.n
            }
            Mode::Given(s) => {
                self.n = s.len();
                self.n
            }
        }
    }

    fn query(&mut self, t: &[u8]) -> bool {
        self.num_queries += 1;
        if self.num_queries > 2 * self.n as i32 {
            panic!("too many queries");
        }
        match &mut self.mode {
            Mode::IO(input, out) => query(input, out, t) == 1,
            Mode::Given(s) => {
                let t = String::from_utf8(t.to_vec()).unwrap();
                s.contains(&t)
            }
        }
    }

    fn report(&mut self, t: &[u8]) {
        match &mut self.mode {
            Mode::IO(_input, out) => report(out, t),
            Mode::Given(s) => {
                let t = String::from_utf8(t.to_vec()).unwrap();
                if t == *s {
                    // OK.
                } else {
                    panic!("Wrong answer! got {t}, wanted {s}");
                }
            }
        }
    }
}

fn solve(interactor: &mut Interactor) {
    let n = interactor.get_input();
    let mut t = vec![b'0'];
    if !interactor.query(&t) {
        interactor.report(&vec![b'1'; n]);
        return;
    }
    while t.len() != n {
        t.push(b'0');
        if interactor.query(&t) {
            continue;
        }
        t.pop();
        t.push(b'1');
        if interactor.query(&t) {
            continue;
        }
        t.pop();
        break;
    }
    while t.len() != n {
        t.insert(0, b'0');
        if interactor.query(&t) {
            continue;
        }
        t.remove(0);
        t.insert(0, b'1');
    }
    interactor.report(&t);
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
            if test_num % 10000 == 0 {
                eprintln!("test num: {test_num}");
            }
            let n = rng.gen_range(2..=100);
            let s = (0..n)
                .map(|_| rng.gen_range(b'0'..=b'1'))
                .collect::<Vec<_>>();
            let mut interactor = Interactor::new(Mode::Given(String::from_utf8(s).unwrap()));
            solve(&mut interactor);
        }
    }
}
//END MAIN
