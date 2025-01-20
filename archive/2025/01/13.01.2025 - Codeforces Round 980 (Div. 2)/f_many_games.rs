//{"name":"F. Many Games","group":"Codeforces - Codeforces Round 980 (Div. 2)","url":"https://codeforces.com/contest/2024/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n80 80\n70 100\n50 200\n","output":"112.00000000\n"},{"input":"2\n100 1\n100 1\n","output":"2.00000000\n"},{"input":"4\n1 100\n2 1000\n2 100\n3 1\n","output":"20.00000000\n"},{"input":"5\n34 804\n78 209\n99 191\n61 439\n90 79\n","output":"395.20423800\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FManyGames"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut ws_by_p = vec![vec![]; 101];
    for (p, w) in input.read_int_pair_vec(n) {
        ws_by_p[p as usize].push(w);
    }
    for ws in &mut ws_by_p {
        ws.sort();
        ws.reverse();
    }
    let mut sum_w_for_p100 = 0;
    for w in &ws_by_p[100] {
        let w = *w;
        sum_w_for_p100 += w as i64;
    }
    if sum_w_for_p100 >= 202222 {
        out.print_line(sum_w_for_p100);
        return;
    }
    let m = (202222 - sum_w_for_p100) as usize;
    let mut dp = vec![0.0f64; m as usize + 1];
    dp[0] = 1.0;

    for p in (1..100).rev() {
        let p_f64 = p as f64 / 100.0;
        ws_by_p[p].truncate((100 / (100 - p) + 1) as usize);
        for w in &ws_by_p[p] {
            let w = *w as usize;
            for i in (w..=m).rev() {
                dp[i] = dp[i].max(dp[i - w] * p_f64);
            }
        }
    }
    let mut result: f64 = 0.0;
    for (sum_w, p) in dp.into_iter().enumerate() {
        result = result.max(p * (sum_w as f64 + sum_w_for_p100 as f64));
    }
    out.print_line(format!("{result:.8}"));
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
