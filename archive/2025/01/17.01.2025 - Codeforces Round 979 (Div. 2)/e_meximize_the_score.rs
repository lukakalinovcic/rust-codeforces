//{"name":"E. MEXimize the Score","group":"Codeforces - Codeforces Round 979 (Div. 2)","url":"https://codeforces.com/contest/2030/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n0 0 1\n4\n0 0 1 1\n5\n0 0 1 2 2\n4\n1 1 1 1\n","output":"11\n26\n53\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMEXimizeTheScore"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

struct PreCalc {
    inv: Vec<ModInt<MOD>>,
}

const MOD: u32 = 998244353;
const MAXN: usize = 200000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let mut c: Vec<usize> = vec![0; n + 1];
    for a in input.read_int_vec(n) {
        c[a as usize] += 1;
    }

    // ways[i][k] => ways to make choices where max(a0, a1, a2, ... ai) = k
    // ways[i][k] = (c[i] choose k) * (ways[i-1][k + 1] + ways[i-1][k + 2] + ... + ways[i-1][c[i-1]]) + (c[i] choose k + c[i] choose (k+1) + ... c[i] choose c[i]) * ways[i-1][k])

    // dp[i][k] => score of all choices where max(a0, a1, a2, ... ai) = k
    // dp[i][k] = k * ways[i][k] + ((c[i] choose k) * (dp[i-1][k + 1] + dp[i-1][k + 2] + ... + dp[i-1][c[i-1]]) + (c[i] choose k + c[i] choose (k+1) + ... c[i] choose c[i]) * dp[i-1][k])

    let mut dp: Vec<ModInt<MOD>> = vec![0.into(); n + 1];
    let mut ways: Vec<ModInt<MOD>> = vec![0.into(); n + 1];
    ways[n] = 1.into();
    let mut prev_cnt = n;
    for i in 0..=n {
        let mut choose: ModInt<MOD> = 1.into();
        let mut choose_suffix_sum: ModInt<MOD> = 1.into();
        let mut prev_dp_suffix_sum: ModInt<MOD> = 0.into();
        let mut prev_ways_suffix_sum: ModInt<MOD> = 0.into();
        for k in c[i] + 1..=prev_cnt {
            prev_ways_suffix_sum += ways[k];
            prev_dp_suffix_sum += dp[k];
            ways[k] = 0.into();
            dp[k] = 0.into();
        }
        for k in (0..=c[i]).rev() {
            let prev_ways = ways[k];
            let prev_dp = dp[k];
            // ways[i][k] = choose * prev_ways_suffix_sum + choose_suffix_sum * ways[i-1][k]
            ways[k] = choose * prev_ways_suffix_sum + choose_suffix_sum * ways[k];
            // dp[i][k] = k * ways[i][k] + choose * prev_dp_suffix_sum + choose_suffix_sum * dp[i-1][k])
            dp[k] = ways[k] * k.into() + choose * prev_dp_suffix_sum + choose_suffix_sum * dp[k];
            prev_ways_suffix_sum += prev_ways;
            prev_dp_suffix_sum += prev_dp;
            choose *= k.into();
            choose *= precalc.inv[c[i] - k + 1];
            choose_suffix_sum += choose;
        }
        prev_cnt = c[i];
    }
    out.print_line(dp[0].get());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc {
        inv: ModInt::<MOD>::gen_inverses(MAXN + 2),
    };

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
