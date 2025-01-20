//{"name":"B. Yet Another Coin Problem","group":"Codeforces - Codeforces Round 931 (Div. 2)","url":"https://codeforces.com/contest/1934/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"14\n1\n2\n3\n5\n7\n11\n12\n14\n16\n17\n18\n20\n98\n402931328\n","output":"1\n2\n1\n3\n2\n2\n2\n3\n2\n3\n2\n2\n8\n26862090\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BYetAnotherCoinProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

struct PreCalc {
    dp: Vec<i32>,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let mut n = input.read_int();
    let mut result = 0;
    if n > 120 {
        let k = (n - 100) / 15;
        result += k;
        n -= k * 15;
    }
    result += data.dp[n as usize];
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut dp = vec![0; 121];
    dp[0] = 0;
    for i in 1..=120 {
        dp[i] = i as i32;
        for x in [3, 6, 10, 15] {
            if i >= x {
                dp[i] = dp[i].min(dp[i - x] + 1);
            }
        }
    }
    let mut pre_calc = PreCalc { dp };

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
