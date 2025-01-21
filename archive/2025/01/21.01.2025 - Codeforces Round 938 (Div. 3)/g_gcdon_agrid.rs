//{"name":"G. GCD on a grid","group":"Codeforces - Codeforces Round 938 (Div. 3)","url":"https://codeforces.com/contest/1955/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n2 3\n30 20 30\n15 25 40\n3 3\n12 4 9\n3 12 2\n8 3 12\n2 4\n2 4 6 8\n1 3 6 9\n","output":"10\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GGCDOnAGrid"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const MAX: usize = 1000000;

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

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut a = vec![vec![0; m]; n];
    let mut prev = vec![vec![]; m];
    for i in 0..n {
        let mut curr = vec![vec![]; m];
        for j in 0..m {
            a[i][j] = input.read_int();
            let k1 = precalc.divisors[a[i][j] as usize].len();
            curr[j] = vec![false; k1];
            if i == 0 && j == 0 {
                curr[0] = vec![true; k1];
            }
            if j > 0 {
                let k2 = precalc.divisors[a[i][j - 1] as usize].len();
                let mut i1 = 0;
                let mut i2 = 0;
                while i1 < k1 && i2 < k2 {
                    let d1 = precalc.divisors[a[i][j] as usize][i1];
                    let d2 = precalc.divisors[a[i][j - 1] as usize][i2];
                    if d1 < d2 {
                        i1 += 1;
                    } else if d1 > d2 {
                        i2 += 1;
                    } else {
                        curr[j][i1] |= curr[j - 1][i2];
                        i1 += 1;
                        i2 += 1;
                    }
                }
            }
            if i > 0 {
                let k2 = precalc.divisors[a[i - 1][j] as usize].len();
                let mut i1 = 0;
                let mut i2 = 0;
                while i1 < k1 && i2 < k2 {
                    let d1 = precalc.divisors[a[i][j] as usize][i1];
                    let d2 = precalc.divisors[a[i - 1][j] as usize][i2];
                    if d1 < d2 {
                        i1 += 1;
                    } else if d1 > d2 {
                        i2 += 1;
                    } else {
                        curr[j][i1] |= prev[j][i2];
                        i1 += 1;
                        i2 += 1;
                    }
                }
            }
        }
        prev = curr;
    }
    let k = precalc.divisors[a[n - 1][m - 1] as usize].len();
    for i in (0..k).rev() {
        if prev[m - 1][i] {
            out.print_line(precalc.divisors[a[n - 1][m - 1] as usize][i]);
            return;
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new(MAX + 1);

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
