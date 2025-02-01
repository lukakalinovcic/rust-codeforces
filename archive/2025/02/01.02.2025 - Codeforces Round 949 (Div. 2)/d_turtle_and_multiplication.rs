//{"name":"D. Turtle and Multiplication","group":"Codeforces - Codeforces Round 949 (Div. 2)","url":"https://codeforces.com/contest/1981/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n2\n3\n4\n","output":"114514 114514\n1 2 2\n3 3 4 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTurtleAndMultiplication"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

struct PreCalc {
    primes: Vec<i32>,
}

impl PreCalc {
    fn new() -> Self {
        const MAX: usize = 20000;
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

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let n = input.read_size();

    out.print_line(doit(n, &data.primes));
}

fn doit(n: usize, primes: &Vec<i32>) -> Vec<i32> {
    let mut m = 1;
    while euler_path_len(m) < n {
        m += 1;
    }
    let mut g = vec![vec![false; m]; m];
    let mut degree = vec![0; m];
    for i in 0..m {
        for j in 0..m {
            if m % 2 == 0 && i >= 2 && j >= 2 && (i ^ j == 1) {
                continue;
            }
            degree[i] += if i == j { 2 } else { 1 };
            g[i][j] = true;
        }
    }
    let mut result = vec![];
    let mut s = vec![0];
    while !s.is_empty() {
        let i = s.last().cloned().unwrap();
        if degree[i] == 0 {
            result.push(primes[i]);
            s.pop();
        } else {
            for j in 0..m {
                if g[i][j] {
                    g[i][j] = false;
                    g[j][i] = false;
                    degree[i] -= 1;
                    degree[j] -= 1;
                    s.push(j);
                    break;
                }
            }
        }
    }
    result.truncate(n);
    result
}

fn euler_path_len(m: usize) -> usize {
    if m % 2 == 1 {
        1 + m * (m + 1) / 2
    } else {
        2 + m * m / 2
    }
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

fn check(n: usize, a: &Vec<i32>) -> bool {
    let mut seen = std::collections::HashSet::new();
    for i in 0..n {
        if a[i] < 1 || a[i] > 300000 {
            return false;
        }
        if i > 0 {
            let prod = a[i] * a[i - 1];
            if seen.contains(&prod) {
                return false;
            }
            seen.insert(prod);
        }
    }
    true
}

fn main() {
    tester::run_tests();

    let precalc = PreCalc::new();
    for n in 2..=100 {
        let a = doit(n, &precalc.primes);
        if !check(n, &a) {
            eprintln!("Check failed for {n} {a:?}");
            break;
        }
    }
}
//END MAIN
