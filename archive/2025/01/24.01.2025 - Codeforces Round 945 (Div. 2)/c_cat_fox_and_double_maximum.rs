//{"name":"C. Cat, Fox and Double Maximum","group":"Codeforces - Codeforces Round 945 (Div. 2)","url":"https://codeforces.com/contest/1973/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n1 2 3 4\n4\n4 3 1 2\n6\n6 5 1 4 2 3\n8\n1 2 4 5 7 6 8 3\n","output":"2 4 1 3\n3 1 4 2\n2 5 1 4 3 6\n5 4 8 2 7 1 6 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCatFoxAndDoubleMaximum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let p = input.read_int_vec(n);

    let mut one_pos = 0;
    for i in 0..n {
        if p[i] == 1 {
            one_pos = i;
        }
    }
    let mut q = vec![0; n];
    let mut taken = vec![false; n + 1];
    for i in 1..n - 1 {
        if i % 2 != one_pos % 2 {
            q[i] = n as i32 - p[i] + 2;
            taken[q[i] as usize] = true;
        }
    }
    for i in 0..n {
        if q[i] == 0 {
            q[i] = n as i32 - p[i] + 1;
            while q[i] > 0 && taken[q[i] as usize] {
                q[i] -= 1;
            }
            assert_ne!(q[i], 0);
        }
    }
    out.print_iter(q.into_iter());
    out.print_line("");
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
