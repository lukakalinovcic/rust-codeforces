//{"name":"B. Cost of the Array","group":"Codeforces - Codeforces Round 1002 (Div. 2)","url":"https://codeforces.com/contest/2059/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 2\n1 1 1\n8 8\n1 1 2 2 3 3 4 4\n5 4\n1 1 1 2 2\n5 4\n1 1 1000000000 2 2\n","output":"2\n5\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCostOfTheArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);
    if k == n {
        for i in 1..=k / 2 {
            if a[2 * i - 1] != i as i32 {
                out.print_line(i);
                return;
            }
        }
        out.print_line(k / 2 + 1);
    } else {
        let l = n - k;
        for i in 1..=1 + l {
            if a[i] != 1 {
                out.print_line(1);
                return;
            }
        }
        out.print_line(2);
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
