//{"name":"E. Card Game","group":"Codeforces - Educational Codeforces Round 170 (Rated for Div. 2)","url":"https://codeforces.com/contest/2025/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"1 4\n","output":"2\n"},{"input":"2 2\n","output":"2\n"},{"input":"3 6\n","output":"1690\n"},{"input":"5 4\n","output":"568\n"},{"input":"500 500\n","output":"84693741\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECardGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

type PreCalc = ();

const MOD: u32 = 998244353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();

    let mut dp = vec![vec![ModInt::<MOD>::from(0); m + 1]; m + 1];
    dp[0][0] = 1.into();
    for i in 0..m {
        let mut dp_next = vec![vec![ModInt::<MOD>::from(0); m + 1]; m + 1];
        for saved in 0..=i {
            for trump in 0..=i {
                let curr_dp = dp[saved][trump];
                // first player's card
                dp_next[saved + 1][trump] += curr_dp;
                // second player's card
                if saved > 0 {
                    dp_next[saved - 1][trump] += curr_dp;
                } else {
                    dp_next[0][trump + 1] += curr_dp;
                }
            }
        }
        dp = dp_next;
    }
    let mut dp_trump = vec![ModInt::<MOD>::from(0); m + 1];
    dp_trump[0] = 1.into();
    for _ in 0..n - 1 {
        for j in (0..=m).rev() {
            let mut ways: ModInt<MOD> = 0.into();
            for k in 0..=j {
                ways += dp_trump[j - k] * dp[0][k];
            }
            dp_trump[j] = ways;
        }
    }
    let mut result: ModInt<MOD> = 0.into();
    for trump in 0..=m {
        result += dp_trump[trump] * dp[trump][0];
    }
    out.print_line(result.unwrap());
}

pub static TEST_TYPE: TestType = TestType::Single;
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
