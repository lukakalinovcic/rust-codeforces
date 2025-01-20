//{"name":"E. Insane Problem","group":"Codeforces - Codeforces Round 993 (Div. 4)","url":"https://codeforces.com/contest/2044/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 2 6 2 12\n2 1 1000000000 1 1000000000\n3 5 7 15 63\n1000000000 1 5 6 1000000000\n15 17 78 2596 20914861\n","output":"12\n1999999987\n6\n1\n197\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EInsaneProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_long();
    let (l1, r1) = (input.read_long(), input.read_long());
    let (l2, r2) = (input.read_long(), input.read_long());
    let mut k_n = 1;
    let mut result = 0;
    for _n in 0.. {
        if k_n > r2 {
            break;
        }
        let l3 = (l2 + k_n - 1) / k_n;
        let r3 = r2 / k_n;
        let l = l1.max(l3);
        let r = r1.min(r3);
        result += (r - l + 1).max(0);
        k_n *= k;
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
