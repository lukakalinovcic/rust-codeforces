//{"name":"F. Peanuts","group":"Codeforces - Codeforces Round 983 (Div. 2)","url":"https://codeforces.com/contest/2032/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n3\n1 2 3\n4\n1 2 3 1\n5\n1 1 1 1 1\n2\n1 1\n10\n1 2 3 4 5 6 7 8 9 10\n","output":"1\n4\n16\n0\n205\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPeanuts"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: i32 = 998244353;

fn add(a: i32, b: i32) -> i32 {
    (a + b) % MOD
}
fn sub(a: i32, b: i32) -> i32 {
    (a + MOD - b) % MOD
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut suffix_xor = 0;
    let mut winning_with_xor: HashMap<i32, i32> = HashMap::new();
    let mut losing_with_xor: HashMap<i32, i32> = HashMap::new();
    losing_with_xor.insert(0, 1);
    let mut total_winning = 0;
    let mut total_losing = 1;
    let mut one_run_winning = [0, 0];
    let mut prev_winning = 0;
    for i in (0..n).rev() {
        if a[i] == 1 {
            one_run_winning[i % 2] = add(one_run_winning[i % 2], prev_winning);
        } else {
            one_run_winning = [0, 0];
        }
        suffix_xor ^= a[i];
        let xor_zero_losing = losing_with_xor.entry(suffix_xor).or_insert(0);
        let xor_zero_winning = winning_with_xor.entry(suffix_xor).or_insert(0);
        let xor_non_zero_losing = sub(total_losing, *xor_zero_losing);
        let xor_non_zero_winning = sub(total_winning, *xor_zero_winning);
        let odd_one_run_winning = one_run_winning[i % 2];
        let even_one_run_winning = one_run_winning[(i % 2) ^ 1];
        let mut curr_losing = 0;
        let mut curr_winning = 0;
        // xor != 0 && losing : we can take last
        curr_winning = add(curr_winning, xor_non_zero_losing);
        // xor != 0 && winning : we can force opponent to take last unless odd one run
        curr_winning = add(curr_winning, sub(xor_non_zero_winning, odd_one_run_winning));
        curr_losing = add(curr_losing, odd_one_run_winning);
        // xor == 0 && losing : we can't take last
        curr_losing = add(curr_losing, *xor_zero_losing);
        // xor == 0 && winning : we can force opponent to take last unless even one run
        curr_losing = add(curr_losing, sub(*xor_zero_winning, even_one_run_winning));
        curr_winning = add(curr_winning, even_one_run_winning);

        *xor_zero_losing = add(*xor_zero_losing, curr_losing);
        *xor_zero_winning = add(*xor_zero_winning, curr_winning);
        total_losing = add(total_losing, curr_losing);
        total_winning = add(total_winning, curr_winning);
        prev_winning = curr_winning;
    }
    out.print_line(prev_winning);
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
