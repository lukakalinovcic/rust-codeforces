//{"name":"B. Two Large Bags","group":"Codeforces - Codeforces Round 1004 (Div. 2)","url":"https://codeforces.com/contest/2067/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n2\n1 1\n2\n2 1\n4\n1 1 4 4\n4\n3 4 3 3\n4\n2 3 4 4\n6\n3 3 4 5 3 3\n6\n2 2 2 4 4 4\n8\n1 1 1 1 1 1 1 4\n10\n9 9 9 10 10 10 10 10 10 10\n","output":"Yes\nNo\nYes\nYes\nNo\nYes\nNo\nYes\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTwoLargeBags"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut cnt = vec![0; n + 1];
    for a in input.read_int_vec(n) {
        cnt[a as usize] += 1;
    }
    let mut carry = 0;
    for i in 1..=n {
        let c = cnt[i] + carry;
        if c == 1 {
            out.print_line("No");
            return;
        }
        carry = (c - 2).max(0);
    }
    if carry % 2 == 1 {
        out.print_line("No");
    } else {
        out.print_line("Yes");
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
