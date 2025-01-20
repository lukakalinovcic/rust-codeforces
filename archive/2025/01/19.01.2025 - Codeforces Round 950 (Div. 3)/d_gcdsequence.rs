//{"name":"D. GCD-sequence","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"12\n6\n20 6 12 3 48 36\n4\n12 6 3 4\n3\n10 12 3\n5\n32 16 8 4 2\n5\n100 50 2 10 20\n4\n2 4 8 1\n10\n7 4 6 2 4 5 1 4 2 8\n7\n5 9 6 8 5 9 2\n6\n11 14 8 12 9 3\n9\n5 7 3 10 6 3 12 6 3\n3\n4 2 4\n8\n1 6 11 12 6 12 3 6\n","output":"YES\nNO\nYES\nNO\nYES\nYES\nNO\nYES\nYES\nYES\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DGCDSequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut b = vec![0; n];
    let mut prefix = vec![true; n];
    for i in 1..n {
        b[i] = gcd(a[i], a[i - 1]);
        prefix[i] = prefix[i - 1] & (b[i] >= b[i - 1]);
    }

    let mut result = prefix[n - 2];
    let mut suffix = true;
    for i in (1..n - 1).rev() {
        let new_element = gcd(a[i + 1], a[i - 1]);
        let mut element_ok = new_element >= b[i - 1];
        if i + 2 < n {
            element_ok &= b[i + 2] >= new_element;
        }
        result |= prefix[i - 1] && element_ok && suffix;
        if i + 2 < n {
            suffix &= b[i + 2] >= b[i + 1];
        }
    }
    result |= suffix;
    if result {
        out.print_line("YES");
    } else {
        out.print_line("NO");
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
