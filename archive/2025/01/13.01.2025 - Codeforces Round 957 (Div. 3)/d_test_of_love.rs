//{"name":"D. Test of Love","group":"Codeforces - Codeforces Round 957 (Div. 3)","url":"https://codeforces.com/contest/1992/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6 2 0\nLWLLLW\n6 1 1\nLWLLLL\n6 1 1\nLWLLWL\n6 2 15\nLWLLCC\n6 10 0\nCCCCCC\n6 6 1\nWCCCCW\n","output":"YES\nYES\nNO\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTestOfLove"}}}

use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 1000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let k = input.read();
    let s = input.read_line().into_bytes();
    let is_water = |i| i >= 1 && i <= n && s[i - 1] == b'W';
    let can_jump = |i| i <= n + 1 && (i == 0 || i == n + 1 || s[i - 1] != b'C');

    let mut pq = VecDeque::new();
    let mut dist = vec![INF; n as usize + 2];
    dist[0] = 0;
    pq.push_back((0, 0));
    while !pq.is_empty() {
        let (d, i) = pq.pop_front().unwrap();
        if d > dist[i] {
            continue;
        }
        if i == n + 1 {
            break;
        }
        if is_water(i) {
            if can_jump(i + 1) && d + 1 < dist[i + 1] {
                dist[i + 1] = d + 1;
                pq.push_back((d + 1, i + 1));
            }
        } else {
            for j in 1..=m {
                if can_jump(i + j) && d < dist[i + j] {
                    dist[i + j] = d;
                    pq.push_front((d, i + j));
                }
            }
        }
    }
    if dist[n + 1] <= k {
        out.print_line("YES");
    } else {
        out.print_line("NO");
    }
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
