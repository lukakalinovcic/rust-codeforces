//{"name":"D. Strong Vertices","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n3 1 2 4\n4 3 2 1\n5\n1 2 4 1 2\n5 2 3 3 1\n2\n1 2\n2 1\n3\n0 2 1\n1 3 2\n3\n5 7 4\n-2 -3 -6\n","output":"1\n4\n2\n3 5\n1\n2\n3\n1 2 3\n2\n2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DStrongVertices"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let mut c = a
        .into_iter()
        .zip(b.into_iter())
        .enumerate()
        .map(|(i, (a, b))| (a - b, i))
        .collect::<Vec<_>>();
    c.sort();
    let mut results = vec![];
    for i in 0..c.len() {
        if c[i].0 == c[c.len() - 1].0 {
            results.push(c[i].1 + 1);
        }
    }
    out.print_line(results.len());
    out.print_line(results);
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
