//{"name":"D. For Wizards, the Exam Is Easy, but I Couldn't Handle It","group":"Codeforces - Codeforces Round 1006 (Div. 3)","url":"https://codeforces.com/contest/2072/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n7\n1 4 3 2 5 3 3\n6\n1 4 3 2 5 3\n8\n7 6 5 8 4 3 2 1\n10\n1 1 1 5 1 1 5 6 7 8\n2\n1337 69\n4\n2 1 2 1\n3\n998 244 353\n3\n1 2 1\n9\n1 1 2 3 5 8 13 21 34\n","output":"2 7\n2 4\n1 8\n4 6\n1 2\n1 4\n1 3\n2 3\n5 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DForWizardsTheExamIsEasyButICouldntHandleIt"}}}

use std::cmp::Ordering;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let mut best = 0;
    let mut result = (1, 1);
    for i in 0..n {
        let mut delta = 0;
        for j in i + 1..n {
            match a[i].cmp(&a[j]) {
                Ordering::Less => delta += 1,
                Ordering::Greater => delta -= 1,
                Ordering::Equal => {}
            }
            if delta < best {
                best = delta;
                result = (i + 1, j + 1);
            }
        }
    }
    out.print_line(result)
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
