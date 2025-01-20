//{"name":"C. MEX Cycle","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n5 1 3\n4 2 4\n6 3 5\n7 3 6\n3 2 3\n5 1 5\n6 2 5\n","output":"0 2 1 0 1\n1 2 1 0\n1 2 0 1 2 0\n0 1 2 0 1 0 1\n2 0 1\n1 0 2 1 0\n0 1 2 0 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMEXCycle"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, x, y) = (input.read_int(), input.read_int(), input.read_int());
    let result: Vec<_> = if n % 2 == 0 && (x - y).abs() % 2 == 1 {
        (1..=n).map(|i| i % 2).collect()
    } else {
        (1..=n)
            .map(|i| if i == x { 2 } else { ((i + n - x) % n) % 2 })
            .collect()
    };
    out.print_iter(result.into_iter());
    out.put(b'\n');
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
