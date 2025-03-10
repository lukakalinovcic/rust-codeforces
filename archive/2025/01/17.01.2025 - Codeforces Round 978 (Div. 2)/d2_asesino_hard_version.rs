//{"name":"D2. Asesino (Hard Version)","group":"Codeforces - Codeforces Round 978 (Div. 2)","url":"https://codeforces.com/contest/2022/problem/D2","interactive":true,"timeLimit":2500,"tests":[{"input":"2\n7\n\n1\n\n0\n\n0\n\n1\n\n1\n\n0\n\n0\n\n4\n\n0\n\n1\n\n1\n\n1\n","output":"\n? 1 3\n\n? 7 6\n\n? 2 5\n\n? 6 2\n\n? 4 5\n\n? 4 6\n\n? 1 4\n\n! 4\n\n? 1 2\n\n? 2 3\n\n? 3 4\n\n? 4 1\n\n! 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2AsesinoHardVersion"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, i: i32, j: i32) -> i32 {
    out.print_line(format!("? {i} {j}"));
    out.flush();
    input.read_int()
}

fn report(out: &mut Output, a: i32) {
    out.print(format!("! {a}\n"));
    out.flush();
}

#[derive(Debug)]
enum Role {
    Knight,
    Knave,
    Impostor,
}

enum Mode<'a> {
    IO(Input<'a>, Output<'a>),
    Given(Vec<Role>),
}

struct Interactor<'a> {
    mode: Mode<'a>,
    n: i32,
    num_queries: i32,
    cache: HashMap<(i32, i32), i32>,
}

impl<'a> Interactor<'a> {
    fn new(mode: Mode<'a>) -> Self {
        Self {
            mode,
            n: 0,
            num_queries: 0,
            cache: HashMap::new(),
        }
    }

    fn get_input(&mut self) -> i32 {
        self.num_queries = 0;
        self.cache.clear();
        match &mut self.mode {
            Mode::IO(input, _out) => {
                self.n = input.read_int();
            }
            Mode::Given(roles) => {
                self.n = roles.len() as i32;
            }
        }
        self.n
    }

    fn query(&mut self, i: i32, j: i32) -> i32 {
        if let Some(answer) = self.cache.get(&(i, j)) {
            return *answer;
        }
        self.num_queries += 1;
        if self.num_queries > self.n + 69 {
            panic!("too many queries");
        }
        let answer = match &mut self.mode {
            Mode::IO(input, out) => query(input, out, i, j),
            Mode::Given(roles) => match (&roles[i as usize - 1], &roles[j as usize - 1]) {
                (Role::Knight, Role::Knight) => 1,
                (Role::Knight, Role::Knave) => 0,
                (Role::Knight, Role::Impostor) => 1,
                (Role::Knave, Role::Knight) => 0,
                (Role::Knave, Role::Knave) => 1,
                (Role::Knave, Role::Impostor) => 0,
                (Role::Impostor, Role::Knight) => 0,
                (Role::Impostor, Role::Knave) => 1,
                (Role::Impostor, Role::Impostor) => panic!("Multiple impostors!"),
            },
        };
        self.cache.insert((i, j), answer);
        answer
    }

    fn report(&mut self, x: i32) {
        match &mut self.mode {
            Mode::IO(_input, out) => report(out, x),
            Mode::Given(roles) => {
                if let Role::Impostor = roles[x as usize - 1] {
                    // Correct
                    // eprintln!("{} => {}", self.n, self.num_queries);
                } else {
                    panic!("Wrong answer for {roles:?}, got {x}");
                }
            }
        }
    }
}

fn check_two(interactor: &mut Interactor, a: i32, b: i32) -> bool {
    interactor.query(a, b) != interactor.query(b, a)
}

fn solve(interactor: &mut Interactor) {
    let mut n = interactor.get_input();

    while n > 2 {
        if n == 5 {
            if interactor.query(n, n - 1)
                ^ interactor.query(n - 1, n - 2)
                ^ interactor.query(n - 2, n)
                == 0
            {
                if check_two(interactor, n, n - 1) {
                    if check_two(interactor, n, n - 2) {
                        interactor.report(n);
                    } else {
                        interactor.report(n - 1);
                    }
                } else {
                    interactor.report(n - 2);
                }
                return;
            }
            n -= 3;
        } else {
            if check_two(interactor, n, n - 1) {
                if check_two(interactor, n, n - 2) {
                    interactor.report(n);
                } else {
                    interactor.report(n - 1);
                }
                return;
            }
            n -= 2;
        }
    }
    if n == 2 {
        if check_two(interactor, 2, 3) {
            interactor.report(2);
        } else {
            interactor.report(1);
        }
    } else {
        interactor.report(1);
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
            if test_num % 1000 == 1 {
                eprintln!("test num: {test_num}");
            }
            let n = rng.gen_range(3..=100000);
            let mut roles: Vec<Role> = (0..n)
                .map(|_| {
                    if rng.gen_range(1..100) < 50 {
                        Role::Knight
                    } else {
                        Role::Knave
                    }
                })
                .collect();
            let target = rng.gen_range(0..n);
            roles[target] = Role::Impostor;
            let mut interactor = Interactor::new(Mode::Given(roles));
            solve(&mut interactor);
        }
    }
}

//END MAIN
