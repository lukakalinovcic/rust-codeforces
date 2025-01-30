//{"name":"A. Find K Distinct Points with Fixed Center","group":"Codeforces - Codeforces Round 965 (Div. 2)","url":"https://codeforces.com/contest/1998/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n10 10 1\n0 0 3\n-5 -8 8\n4 -5 3\n","output":"10 10\n-1 -1\n5 -1\n-4 2\n-6 -7\n-5 -7\n-4 -7\n-4 -8\n-4 -9\n-5 -9\n-6 -9\n-6 -8\n1000 -1000\n-996 995\n8 -10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AFindKDistinctPointsWithFixedCenter"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let xc = input.read_int();
    let yc = input.read_int();
    let k = input.read_int();
    let mut p = vec![];
    if k % 2 == 1 {
        p.push((xc, yc));
    }
    let start = k % 2;
    for i in start..k {
        let d = (i - start + 2) / 2;
        if i % 2 == 1 {
            p.push((xc - d, yc - d));
        } else {
            p.push((xc + d, yc + d));
        }
    }
    for p in p {
        out.print_line(p)
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
