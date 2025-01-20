//{"name":"B. Crafting","group":"Codeforces - Codeforces Round 996 (Div. 2)","url":"https://codeforces.com/contest/2055/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n0 5 5 1\n1 4 4 0\n3\n1 1 3\n2 2 1\n2\n1 10\n3 3\n","output":"YES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCrafting"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let mut max_ops = 1000000000;
    let mut required_ops: i64 = 0;
    let mut num_requires = 0;
    for i in 0..n {
        if a[i] >= b[i] {
            max_ops = max_ops.min(a[i] - b[i]);
        } else if b[i] > a[i] {
            required_ops += (b[i] - a[i]) as i64;
            num_requires += 1;
        }
    }
    if num_requires <= 1 && required_ops <= max_ops as i64 {
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
