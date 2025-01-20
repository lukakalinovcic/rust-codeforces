//{"name":"A. MEX Destruction","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n4\n0 1 2 3\n6\n0 0 0 0 0 0\n5\n1 0 1 0 1\n5\n3 1 4 1 5\n4\n3 2 1 0\n7\n9 100 0 89 12 2 3\n4\n0 3 9 0\n7\n0 7 0 2 0 7 0\n1\n0\n2\n0 1\n","output":"1\n0\n2\n1\n1\n2\n1\n2\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMEXDestruction"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int() as usize;
    let a: Vec<_> = (0..n).map(|_| input.read_int()).collect();
    let mut i = 0;
    while i < a.len() && a[i] == 0 {
        i += 1;
    }
    let mut j = a.len() - 1;
    while j > i && a[j] == 0 {
        j -= 1;
    }
    if i > j {
        out.print_line(0);
    } else if a[i..=j].contains(&0) {
        out.print_line(2);
    } else {
        out.print_line(1);
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
