//{"name":"B. Prefiquence","group":"Codeforces - Codeforces Round 943 (Div. 3)","url":"https://codeforces.com/contest/1968/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5 4\n10011\n1110\n3 3\n100\n110\n1 3\n1\n111\n4 4\n1011\n1111\n3 5\n100\n11010\n3 1\n100\n0\n","output":"2\n2\n1\n1\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPrefiquence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let a = input.read_line().into_bytes();
    let b = input.read_line().into_bytes();
    let mut next_b = vec![[m, m]; m + 1];
    for i in (0..m).rev() {
        if b[i] == b'0' {
            next_b[i][0] = i;
            next_b[i][1] = next_b[i + 1][1];
        } else {
            next_b[i][0] = next_b[i + 1][0];
            next_b[i][1] = i;
        }
    }

    let mut i = 0;
    let mut j = 0;
    while i < n && j < m {
        if a[i] == b'0' {
            j = next_b[j][0];
        } else {
            j = next_b[j][1];
        }
        if j == m {
            break;
        }
        i += 1;
        j += 1;
    }

    out.print_line(i);
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
