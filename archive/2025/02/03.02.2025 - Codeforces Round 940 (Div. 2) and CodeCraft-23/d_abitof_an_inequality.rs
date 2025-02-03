//{"name":"D. A BIT of an Inequality","group":"Codeforces - Codeforces Round 940 (Div. 2) and CodeCraft-23","url":"https://codeforces.com/contest/1957/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n6 2 4\n1\n3\n5\n7 3 7 2 1\n","output":"4\n0\n16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DABITOfAnInequality"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_unsigned_vec(n);
    let mut prefix = vec![[0; 30]; n + 1];
    let mut xor = 0;
    for i in 0..n {
        xor ^= a[i];
        for j in 0..30 {
            prefix[i + 1][j] = prefix[i][j];
            if (xor >> j) & 1 == 1 {
                prefix[i + 1][j] += 1;
            }
        }
    }
    let mut suffix = [0; 30];
    let mut result = 0;
    for i in (0..n).rev() {
        for j in 0..30 {
            if (xor >> j) & 1 == 1 {
                suffix[j] += 1;
            }
        }
        xor ^= a[i];
        let j = ((a[i] + 1).next_power_of_two().trailing_zeros() - 1) as usize;
        let cnt11 = suffix[j] as i64 * prefix[i][j] as i64;
        let cnt00 = (n - i - suffix[j]) as i64 * (i + 1 - prefix[i][j]) as i64;
        result += cnt00 + cnt11;
    }
    out.print_line(result);
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
