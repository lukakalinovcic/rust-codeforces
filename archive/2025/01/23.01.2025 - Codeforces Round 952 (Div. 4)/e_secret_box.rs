//{"name":"E. Secret Box","group":"Codeforces - Codeforces Round 952 (Div. 4)","url":"https://codeforces.com/contest/1985/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n3 3 3 8\n3 3 3 18\n5 1 1 1\n2 2 2 7\n3 4 2 12\n4 3 1 6\n1800 1800 1800 4913000000\n","output":"8\n2\n5\n0\n4\n4\n1030301\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESecretBox"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_long();
    let y = input.read_long();
    let z = input.read_long();
    let k = input.read_long();
    let mut mx = 0;
    for xx in 1..=x {
        if k % xx != 0 {
            continue;
        }
        let k = k / xx;
        for yy in 1..=y {
            if k % yy != 0 {
                continue;
            }
            let zz = k / yy;
            if zz >= 1 && zz <= z {
                mx = mx.max((x - xx + 1) * (y - yy + 1) * (z - zz + 1));
            }
        }
    }
    out.print_line(mx);
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
