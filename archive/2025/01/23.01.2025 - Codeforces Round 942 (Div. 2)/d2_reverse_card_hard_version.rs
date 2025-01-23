//{"name":"D2. Reverse Card (Hard Version)","group":"Codeforces - Codeforces Round 942 (Div. 2)","url":"https://codeforces.com/contest/1972/problem/D2","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 1\n2 3\n3 5\n10 8\n100 1233\n1000000 1145141\n","output":"0\n1\n1\n6\n423\n5933961\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2ReverseCardHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const MAX: usize = 2000000;

struct PreCalc {
    primes: Vec<i32>,
}

impl PreCalc {
    fn new() -> Self {
        let mut not_prime = vec![false; MAX];
        let mut primes = vec![];
        for i in 2..MAX {
            if not_prime[i] {
                continue;
            }
            primes.push(i as i32);
            let mut j = i * i;
            while j < MAX {
                not_prime[j] = true;
                j += i;
            }
        }
        Self { primes }
    }
}

fn rec(primes: &Vec<i32>, i: usize, b: i64, a_plus_b: i64, g: i64, n: i64, m: i64, out: &mut i32) {
    if i == primes.len() || b * primes[i] as i64 > m {
        let a = a_plus_b - b;
        if a >= 1 && a <= n {
            *out += 1;
        }
        return;
    }
    let mut b = b;
    let mut g = g;
    let mut a_plus_b = a_plus_b;
    for z in 0.. {
        rec(primes, i + 1, b, a_plus_b, g, n, m, out);
        {
            let mut b = b;
            loop {
                b *= primes[i] as i64;
                if b > m {
                    break;
                }
                rec(primes, i + 1, b, a_plus_b, g, n, m, out);
            }
        }
        {
            let mut a_plus_b = a_plus_b;
            for _ in 1..=z {
                a_plus_b *= primes[i] as i64;
                if a_plus_b > n + m {
                    break;
                }
                rec(primes, i + 1, b, a_plus_b, g, n, m, out);
            }
        }

        b *= primes[i] as i64;
        if b > m {
            break;
        }
        g *= primes[i] as i64;
        a_plus_b *= primes[i] as i64;
        if a_plus_b > n + m {
            break;
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read_long();
    let m = input.read_long();
    let mut result = 0;
    rec(&precalc.primes, 0, 1, 1, 1, n, m, &mut result);
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
