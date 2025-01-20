//{"name":"B. Farmer John's Card Game","group":"Codeforces - Codeforces Round 998 (Div. 3)","url":"https://codeforces.com/contest/2060/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 3\n0 4 2\n1 5 3\n1 1\n0\n2 2\n1 2\n0 3\n4 1\n1\n2\n0\n3\n","output":"1 2\n1\n-1\n3 1 2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFarmerJohnsCardGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let a = (0..n).map(|_| input.read_int_vec(m)).collect::<Vec<_>>();
    let mut p = vec![0; n as usize];
    for i in 0..n {
        let k = a[i][0] % n as i32;
        for j in 1..m {
            if a[i][j] % n as i32 != k {
                out.print_line(-1);
                return;
            }
        }
        p[k as usize] = i + 1;
    }
    out.print_iter(p.into_iter());
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
