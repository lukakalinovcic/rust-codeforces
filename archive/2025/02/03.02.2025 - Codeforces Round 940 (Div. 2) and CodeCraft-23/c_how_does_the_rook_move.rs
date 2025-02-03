//{"name":"C. How Does the Rook Move?","group":"Codeforces - Codeforces Round 940 (Div. 2) and CodeCraft-23","url":"https://codeforces.com/contest/1957/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 1\n1 2\n8 1\n7 6\n1000 4\n4 4\n952 343\n222 333\n90 91\n","output":"3\n331\n671968183\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CHowDoesTheRookMove"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

type PreCalc = ();

const MOD: u32 = 1000000007;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut n = input.read_size();
    let k = input.read_size();
    for (r, c) in input.read_int_pair_vec(k) {
        if r == c {
            n -= 1;
        } else {
            n -= 2;
        }
    }
    let mut f2: ModInt<MOD> = 0.into();
    let mut f1: ModInt<MOD> = 1.into();
    for i in 1..=n {
        let f0 = f1 + f2 * (2 * (i - 1)).into();
        f2 = f1;
        f1 = f0;
    }
    out.print_line(f1.unwrap());
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
