//{"name":"E. Binary Search","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 3\n1 2 3 4 5 6\n6 5\n3 1 6 5 2 4\n5 1\n3 5 4 2 1\n6 3\n4 3 1 5 2 6\n3 2\n3 2 1\n","output":"0\n1\n3 4\n2\n2 4\n1 5\n2\n4 5\n2 4\n1\n1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBinarySearch"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let x = input.read_int();
    let mut p = input.read_int_vec(n);
    let mut ops = vec![];
    match p.iter().position(|p| *p == x) {
        None => panic!("x not found in perm"),
        Some(p_x) => {
            p.swap(0, p_x);
            ops.push((1, p_x + 1));
        }
    }
    let mut l = 1;
    let mut r = n + 1;
    while r - l > 1 {
        let m = (r + l) / 2;
        if p[m - 1] <= x {
            l = m;
        } else {
            r = m;
        }
    }
    if l != 1 {
        ops.push((1, l));
    }
    out.print_line(ops.len());
    for (i, j) in ops {
        out.print_line((i, j));
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
