//{"name":"C. Gerrymandering","group":"Codeforces - Codeforces Round 978 (Div. 2)","url":"https://codeforces.com/contest/2022/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\nAAA\nAJJ\n6\nJAJAJJ\nJJAJAJ\n6\nAJJJAJ\nAJJAAA\n9\nAJJJJAJAJ\nJAAJJJJJA\n","output":"2\n2\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGerrymandering"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn win(a: u8, b: u8, c: u8) -> i32 {
    let votes: i32 = [a, b, c]
        .into_iter()
        .map(|x| if x == b'A' { 1 } else { 0 })
        .sum();
    if votes >= 2 {
        1
    } else {
        0
    }
}

fn max_assign(a: &mut i32, b: i32) {
    *a = (*a).max(b);
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let a = (0..2)
        .map(|_| input.read_line().into_bytes())
        .collect::<Vec<_>>();
    let mut dp = vec![[[0; 3]; 3]; n + 1];
    for s0 in 0..3 {
        for s1 in 0..3 {
            dp[0][s0][s1] = if s0 == 0 && s1 == 0 { 0 } else { -1000000000 };
        }
    }
    for i in 0..n {
        for s0 in 0..3 {
            for s1 in 0..3 {
                let curr = dp[i][s0][s1];
                if s0 == 0 && s1 == 0 {
                    if i + 1 < n {
                        // *
                        // **
                        max_assign(
                            &mut dp[i + 1][0][1],
                            curr + win(a[0][i], a[1][i], a[1][i + 1]),
                        );
                    }
                    if i + 1 < n {
                        // **
                        // *
                        max_assign(
                            &mut dp[i + 1][1][0],
                            curr + win(a[0][i], a[0][i + 1], a[1][i]),
                        );
                    }
                    if i + 2 < n {
                        // ***
                        // ***
                        max_assign(
                            &mut dp[i + 1][2][2],
                            curr + win(a[0][i], a[0][i + 1], a[0][i + 2])
                                + win(a[1][i], a[1][i + 1], a[1][i + 2]),
                        );
                    }
                } else if s0 == 0 {
                    if s1 == 1 && i + 1 < n {
                        // **
                        // x*
                        max_assign(
                            &mut dp[i + 1][1][1],
                            curr + win(a[0][i], a[0][i + 1], a[1][i + 1]),
                        );
                    }
                    if i + 2 < n {
                        // ***
                        // x?
                        max_assign(
                            &mut dp[i + 1][2][s1 - 1],
                            curr + win(a[0][i], a[0][i + 1], a[0][i + 2]),
                        );
                    }
                } else if s1 == 0 {
                    if s0 == 1 && i + 1 < n {
                        // x*
                        // **
                        max_assign(
                            &mut dp[i + 1][1][1],
                            curr + win(a[0][i + 1], a[1][i], a[1][i + 1]),
                        );
                    }
                    if i + 2 < n {
                        // x?
                        // ***
                        max_assign(
                            &mut dp[i + 1][s0 - 1][2],
                            curr + win(a[1][i], a[1][i + 1], a[1][i + 2]),
                        );
                    }
                } else {
                    max_assign(&mut dp[i + 1][s0 - 1][s1 - 1], curr);
                }
            }
        }
    }
    out.print_line(dp[n][0][0]);
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
