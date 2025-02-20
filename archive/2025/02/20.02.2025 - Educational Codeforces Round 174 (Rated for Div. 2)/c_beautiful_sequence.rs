//{"name":"C. Beautiful Sequence","group":"Codeforces - Educational Codeforces Round 174 (Rated for Div. 2)","url":"https://codeforces.com/contest/2069/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n7\n3 2 1 2 2 1 3\n4\n3 1 2 2\n3\n1 2 3\n9\n1 2 3 2 1 3 2 2 3\n","output":"3\n0\n1\n22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBeautifulSequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

type PreCalc = ();

const MOD: u32 = 998244353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut curr: [ModInt<MOD>; 3] = [0.into(); 3];
    for a in input.read_int_vec(n) {
        match a {
            1 => curr[0] += 1.into(),
            2 => curr[1] += curr[0] + curr[1],
            3 => curr[2] += curr[1],
            _ => panic!("Unexpected number {a}"),
        }
    }
    out.print_line(curr[2].get());
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
