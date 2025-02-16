//{"name":"G. Shuffling Songs","group":"Codeforces - Codeforces Round 937 (Div. 4)","url":"https://codeforces.com/contest/1950/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n1\npop taylorswift\n4\nelectronic themotans\nelectronic carlasdreams\npop themotans\npop irinarimes\n7\nrap eminem\nrap drdre\nrap kanyewest\npop taylorswift\nindierock arcticmonkeys\nindierock arcticmonkeys\npunkrock theoffspring\n4\na b\nc d\ne f\ng h\n","output":"0\n0\n4\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GShufflingSongs"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: i64 = 999998727899999;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut gw = vec![];
    for _ in 0..n {
        let line = input.read_line().into_bytes();
        let mut g: i64 = 0;
        let mut w: i64 = 0;
        let mut seen_space = false;
        for i in 0..line.len() {
            if line[i] == b' ' {
                seen_space = true;
            } else if seen_space {
                w = (w * 1337 + (line[i] as i64)) % MOD;
            } else {
                g = (g * 1337 + (line[i] as i64)) % MOD;
            }
        }
        gw.push((g, w));
    }
    let mut g = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            g[i][j] = gw[i].0 == gw[j].0 || gw[i].1 == gw[j].1;
        }
    }
    out.print_line(n as i32 - longest_path(n, &g));
}

fn longest_path(n: usize, g: &Vec<Vec<bool>>) -> i32 {
    let mut dp = vec![vec![0; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = 1;
    }
    let mut result = 0;
    for mask in 0..1 << n {
        for i in 0..n {
            if (mask >> i) & 1 == 0 {
                continue;
            }
            result = result.max(dp[mask][i]);
            for j in 0..n {
                if !g[i][j] {
                    continue;
                }
                if (mask >> j) & 1 == 1 {
                    continue;
                }
                dp[mask | (1 << j)][j] = dp[mask | (1 << j)][j].max(dp[mask][i] + 1);
            }
        }
    }
    result
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
