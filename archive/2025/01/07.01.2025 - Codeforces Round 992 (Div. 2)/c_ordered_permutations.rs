//{"name":"C. Ordered Permutations","group":"Codeforces - Codeforces Round 992 (Div. 2)","url":"https://codeforces.com/contest/2040/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 2\n3 3\n4 11\n4 6\n6 39\n7 34\n","output":"1 3 2\n2 3 1\n-1\n2 4 3 1\n-1\n2 3 4 5 7 6 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COrderedPermutations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut k = input.read_long();
    let mut result = vec![0; n];
    let mut front = 0;
    let mut back = n - 1;
    for i in 1..n {
        let p = n - i - 1;
        if p > 60 || k <= (1 << p) {
            result[front] = i;
            front += 1;
        } else {
            k -= 1 << p;
            result[back] = i;
            back -= 1;
        }
    }
    result[front] = n;
    if k == 1 {
        out.print_iter(result.into_iter());
        out.put(b'\n');
    } else {
        out.print_line(-1);
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
