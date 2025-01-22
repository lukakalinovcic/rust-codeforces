//{"name":"G1. Permutation Problem (Simple Version)","group":"Codeforces - Codeforces Round 954 (Div. 3)","url":"https://codeforces.com/contest/1986/problem/G1","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n1\n1\n2\n1 2\n3\n2 3 1\n5\n2 4 1 3 5\n12\n8 9 7 12 1 10 6 3 2 4 11 5\n15\n1 2 4 6 8 10 12 14 3 9 15 5 7 11 13\n","output":"0\n1\n1\n3\n9\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G1PermutationProblemSimpleVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const MAX: usize = 100000;

fn gen_divisors(f: &[(i32, i32)], i: usize, mut divisor: i32, out: &mut Vec<i32>) {
    if i == f.len() {
        out.push(divisor);
    } else {
        gen_divisors(f, i + 1, divisor, out);
        for _ in 1..=f[i].1 {
            divisor *= f[i].0;
            gen_divisors(f, i + 1, divisor, out);
        }
    }
}

struct PreCalc {
    divisors: Vec<Vec<i32>>,
}

impl PreCalc {
    fn new(n: usize) -> Self {
        let mut min_factor = vec![(0, 0); n + 1];
        for i in 2..=n {
            if min_factor[i].0 != 0 {
                continue;
            }
            min_factor[i] = (i as i32, 1);
            let mut j = i * i;
            let mut k = i;
            while j <= n {
                if min_factor[j].0 == 0 {
                    min_factor[j] = (i as i32, k as i32);
                }
                k += 1;
                j += i;
            }
        }
        let mut divisors = vec![vec![]; n + 1];
        for i in 1..=n {
            let mut t = i as i32;
            let mut factors = vec![];
            while t != 1 {
                let prime = min_factor[t as usize].0;
                let mut power = 0;
                while prime == min_factor[t as usize].0 {
                    t = min_factor[t as usize].1;
                    power += 1;
                }
                factors.push((prime, power));
            }
            gen_divisors(&factors, 0, 1, &mut divisors[i]);
            divisors[i].sort();
        }

        Self { divisors }
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let p = input.read_int_vec(n);
    let mut by_a = vec![vec![]; n + 1];
    let mut by_b = vec![vec![]; n + 1];
    for i in 0..n {
        let a = p[i];
        let b = (i + 1) as i32;
        let g = gcd(a, b);
        let a = a / g;
        let b = b / g;
        by_a[a as usize].push((b, i));
        by_b[b as usize].push((a, i));
    }
    let mut result = 0;
    let mut active = vec![(vec![], 0); n + 1];
    for b2 in 1..=n {
        for (a2, i2) in &by_b[b2] {
            let a2 = *a2;
            let i2 = *i2;
            for d in &precalc.divisors[a2 as usize] {
                let d = *d;
                if active[d as usize].1 != b2 {
                    active[d as usize] = (vec![i2], b2);
                } else {
                    active[d as usize].0.push(i2);
                }
            }
        }
        for a1 in (b2..=n).step_by(b2) {
            for (b1, i1) in &by_a[a1 as usize] {
                let b1 = *b1;
                let i1 = *i1;
                if active[b1 as usize].1 == b2 {
                    let (Ok(j) | Err(j)) = active[b1 as usize].0.binary_search(&(i1 + 1));
                    let add = (active[b1 as usize].0.len() - j) as i64;
                    result += add;
                }
            }
        }
    }
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new(MAX);

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
