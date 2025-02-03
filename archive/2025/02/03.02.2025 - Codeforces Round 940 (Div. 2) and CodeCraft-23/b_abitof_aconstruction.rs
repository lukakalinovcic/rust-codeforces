//{"name":"B. A BIT of a Construction","group":"Codeforces - Codeforces Round 940 (Div. 2) and CodeCraft-23","url":"https://codeforces.com/contest/1957/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 5\n2 3\n2 5\n6 51\n","output":"5\n1 2\n5 0\n3 1 1 32 2 12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BABITOfAConstruction"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int() as u32;
    let mut result = vec![0; n];
    if n == 1 {
        result[0] = k;
    } else {
        let mut t = k.next_power_of_two() - 1;
        if t > k {
            t = t / 2;
        }
        result[0] = t;
        result[1] = k - t;
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
