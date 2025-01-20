//{"name":"G. Natlan Exploring","group":"Codeforces - Codeforces Round 988 (Div. 3)","url":"https://codeforces.com/contest/2037/problem/G","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n2 6 3 4 6\n","output":"5\n"},{"input":"5\n4 196 2662 2197 121\n","output":"2\n"},{"input":"7\n3 6 8 9 11 12 20\n","output":"7\n"},{"input":"2\n2 3\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GNatlanExploring"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const PRECALC_PRIMES_UP_TO: usize = 2000;
const MAX: usize = 1000000;
const MOD: i32 = 998244353;

struct PreCalc {
    primes: Vec<i32>,
}

fn get_primes(mut x: i32, data: &PreCalc) -> Vec<i32> {
    let mut result = vec![];
    for p in data.primes.iter() {
        let p = (*p) as i32;
        if p * p > x {
            break;
        }
        if x % p != 0 {
            continue;
        }
        result.push(p);
        while x % p == 0 {
            x /= p;
        }
    }
    if x > 1 {
        result.push(x);
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut sum_dp = vec![0; MAX];
    for i in 0..n {
        let primes = get_primes(a[i], &data);
        let curr = if i == 0 {
            1
        } else {
            let mut sum = 0;
            let k = primes.len();
            for mask in 1..(1 << k) {
                let mut prod = 1;
                let mut sign = -1;
                for i in 0..k {
                    if ((mask >> i) & 1) == 1 {
                        prod *= primes[i];
                        sign = -sign;
                    }
                }
                sum = (sum + sign * sum_dp[prod as usize]) % MOD;
            }
            sum
        };

        let k = primes.len();
        for mask in 1..(1 << k) {
            let mut prod = 1;
            for i in 0..k {
                if ((mask >> i) & 1) == 1 {
                    prod *= primes[i];
                }
            }
            sum_dp[prod as usize] = (sum_dp[prod as usize] + curr) % MOD;
        }

        if i == n - 1 {
            out.print_line((curr + MOD) % MOD);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut not_prime = vec![false; PRECALC_PRIMES_UP_TO + 1];
    let mut primes = vec![];
    for i in 2..=PRECALC_PRIMES_UP_TO {
        if not_prime[i] {
            continue;
        }
        primes.push(i as i32);
        let mut j = i * i;
        while j <= PRECALC_PRIMES_UP_TO {
            not_prime[j] = true;
            j += i;
        }
    }
    let mut pre_calc = PreCalc { primes };

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
