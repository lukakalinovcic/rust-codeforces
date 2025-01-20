//{"name":"G. Ultra-Meow","group":"Codeforces - Codeforces Round 957 (Div. 3)","url":"https://codeforces.com/contest/1992/problem/G","interactive":false,"timeLimit":2500,"tests":[{"input":"5\n2\n3\n4999\n5\n1\n","output":"12\n31\n354226409\n184\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GUltraMeow"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: i64 = 1000000007;

fn mul(a: i64, b: i64) -> i64 {
    (a * b) % MOD
}

fn pow(a: i64, n: i64) -> i64 {
    if n == 0 {
        1
    } else if n % 2 == 1 {
        mul(a, pow(a, n - 1))
    } else {
        let x = pow(a, n / 2);
        mul(x, x)
    }
}

fn inverse(a: i64) -> i64 {
    pow(a, MOD - 2)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let mut fact = vec![1; n + 1];
    let mut inv_fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = mul(fact[i - 1], i as i64);
        inv_fact[i] = inverse(fact[i]);
    }
    let choose = |n: usize, k: usize| {
        if k < 0 || k > n {
            0
        } else {
            mul(fact[n], mul(inv_fact[k], inv_fact[n - k]))
        }
    };

    let mut result = 0;
    for sz in 0..=n {
        for mex in sz + 1..=n.min(2 * sz + 1) {
            let pick_lo = mex - sz - 1;
            let pick_hi = sz - pick_lo;
            let ways = mul(choose(mex - 1, pick_lo), choose(n - mex, pick_hi));
            result += mul(mex as i64, ways);
            result %= MOD;
        }
        if 2 * sz + 1 > n {
            let mex = 2 * sz + 1;
            let ways = choose(n, sz);
            result += mul(mex as i64, ways);
            result %= MOD;
        }
    }
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
