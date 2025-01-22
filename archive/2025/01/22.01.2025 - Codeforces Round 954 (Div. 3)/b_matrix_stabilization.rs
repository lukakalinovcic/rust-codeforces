//{"name":"B. Matrix Stabilization","group":"Codeforces - Codeforces Round 954 (Div. 3)","url":"https://codeforces.com/contest/1986/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 2\n3 1\n2 1\n1\n1\n2 2\n1 2\n3 4\n2 3\n7 4 5\n1 8 10\n5 4\n92 74 31 74\n74 92 17 7\n31 17 92 3\n74 7 3 92\n7 31 1 1\n3 3\n1000000000 1 1000000000\n1 1000000000 1\n1000000000 1 1000000000\n","output":"1 1\n1\n1\n1 2\n3 3\n4 4 5\n1 8 8\n74 74 31 31\n74 74 17 7\n31 17 17 3\n31 7 3 3\n7 7 1 1\n1 1 1\n1 1 1\n1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMatrixStabilization"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut a = vec![vec![0; m]; n];
    for r in 0..n {
        for c in 0..m {
            a[r][c] = input.read_int();
        }
    }
    for r in 0..n as i32 {
        for c in 0..m as i32 {
            let mut mx = 0;
            for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let rr = r + dr;
                let cc = c + dc;
                if rr >= 0 && rr < n as i32 && cc >= 0 && cc < m as i32 {
                    mx = mx.max(a[rr as usize][cc as usize]);
                }
            }
            if mx < a[r as usize][c as usize] {
                a[r as usize][c as usize] = mx;
            }
        }
    }
    for r in 0..n {
        out.print_iter(a[r].iter());
        out.print_line("");
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
