//{"name":"F. Bitwise Slides","group":"Codeforces - Codeforces Round 1004 (Div. 2)","url":"https://codeforces.com/contest/2067/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n1 7 9\n4\n179 1 1 179\n5\n1 2 3 3 2\n12\n8 2 5 3 9 1 8 12 9 9 9 4\n1\n1000000000\n","output":"3\n9\n39\n123\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBitwiseSlides"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

type PreCalc = ();

const MOD: u32 = 1000000007;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut prefix = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] ^ a[i];
    }
    let mut next = HashMap::new();
    next.insert(prefix[n], n);
    let mut dp0: Vec<ModInt<MOD>> = vec![0.into(); n + 1];
    let mut dp1: Vec<ModInt<MOD>> = vec![0.into(); n + 1];
    dp0[n] = 1.into();
    dp1[n] = 1.into();
    for i in (0..n).rev() {
        dp0[i] = match next.get(&prefix[i]) {
            None => 3.into(),
            Some(&j) => (dp0[j] + dp1[j] * 2.into()) * 3.into(),
        };
        if i > 0 {
            dp1[i] = match next.get(&(prefix[i] ^ a[i - 1])) {
                None => 1.into(),
                Some(&j) => dp0[j] + dp1[j] * 2.into(),
            };
        }
        next.insert(prefix[i], i);
    }
    out.print_line(dp0[0].get());
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
