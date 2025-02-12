//{"name":"B. Parity and Sum","group":"Codeforces - Codeforces Round 963 (Div. 2)","url":"https://codeforces.com/contest/1993/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n5\n1 3 5 7 9\n4\n4 4 4 4\n3\n2 3 4\n4\n3 2 2 8\n6\n4 3 6 1 2 1\n6\n3 6 1 2 1 2\n5\n999999996 999999997 999999998 999999999 1000000000\n","output":"0\n0\n2\n4\n3\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BParityAndSum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut odd = vec![];
    let mut even = vec![];
    for a in input.read_long_vec(n) {
        if a % 2 == 0 {
            even.push(a);
        } else {
            odd.push(a);
        }
    }
    if even.is_empty() || odd.is_empty() {
        out.print_line(0);
        return;
    }
    odd.sort();
    even.sort();
    let max_odd = odd[odd.len() - 1];
    let mut s = 0;
    let mut ok = true;
    for i in 0..even.len() {
        if even[i] > max_odd && max_odd + s < even[i] {
            ok = false;
        }
        s += even[i];
    }
    if ok {
        out.print_line(even.len())
    } else {
        out.print_line(even.len() + 1);
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
