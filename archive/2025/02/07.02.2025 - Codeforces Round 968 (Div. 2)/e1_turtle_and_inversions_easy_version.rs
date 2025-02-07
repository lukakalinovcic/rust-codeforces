//{"name":"E1. Turtle and Inversions (Easy Version)","group":"Codeforces - Codeforces Round 968 (Div. 2)","url":"https://codeforces.com/contest/2003/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2 0\n2 1\n1 2\n5 1\n2 4\n8 2\n1 4\n6 8\n7 2\n1 3\n4 7\n7 3\n1 2\n3 4\n5 6\n","output":"1\n0\n8\n21\n15\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1TurtleAndInversionsEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut num_open = vec![0; n];
    let mut num_closed = vec![0; n];
    for (l, r) in input.read_int_pair_vec(m) {
        let l = l as usize - 1;
        let r = r as usize - 1;
        num_open[l] += 1;
        num_closed[r] += 1;
    }
    let mut curr = vec![[0, -1]];
    let mut sum_open = 0;
    let mut sum_closed = 0;
    for k in 0..n {
        let is_edge_open = sum_open > sum_closed;
        sum_closed += num_closed[k];
        sum_open += num_open[k];
        let mut next = vec![[-1, -1]; k + 2];

        for k_big in 0..=k {
            for prev_big in 0..2 {
                if curr[k_big][prev_big] < 0 {
                    continue;
                }
                // next is small.
                if num_closed[k] == 0 && (!is_edge_open || prev_big == 0) {
                    next[k_big][0] = next[k_big][0].max(curr[k_big][prev_big] + k as i32);
                }
                // next is big.
                if num_open[k] == 0 {
                    next[k_big + 1][1] =
                        next[k_big + 1][1].max(curr[k_big][prev_big] + k_big as i32);
                }
            }
        }

        curr = next;
    }
    let mut result = -1;
    for i in 0..=n {
        result = result.max(curr[i][0].max(curr[i][1]));
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
