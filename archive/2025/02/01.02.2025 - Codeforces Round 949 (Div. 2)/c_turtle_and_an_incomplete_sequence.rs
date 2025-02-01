//{"name":"C. Turtle and an Incomplete Sequence","group":"Codeforces - Codeforces Round 949 (Div. 2)","url":"https://codeforces.com/contest/1981/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"9\n8\n-1 -1 -1 2 -1 -1 1 -1\n4\n-1 -1 -1 -1\n6\n3 -1 -1 -1 9 -1\n4\n-1 5 -1 6\n4\n2 -1 -1 3\n4\n1 2 3 4\n2\n4 2\n5\n-1 3 -1 3 6\n13\n-1 -1 3 -1 -1 -1 -1 7 -1 -1 3 -1 -1\n","output":"4 9 4 2 4 2 1 2\n7 3 6 13\n3 1 2 4 9 18\n-1\n-1\n-1\n4 2\n6 3 1 3 6\n3 1 3 1 3 7 3 7 3 1 3 1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTurtleAndAnIncompleteSequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input.read_int_vec(n);
    let mut prev = n;
    let mut ok = true;
    for i in 0..n {
        if a[i] == -1 {
            continue;
        }
        if prev == n {
            fix_prefix(&mut a, i);
        } else {
            ok &= fix_between(&mut a, prev, i);
        }
        prev = i;
    }
    if prev == n {
        a[0] = 1;
        prev = 0;
    }
    fix_suffix(&mut a, prev);
    if ok {
        out.print_line(a);
    } else {
        out.print_line(-1);
    }
}

fn fix_prefix(a: &mut [i32], i: usize) {
    for i in (0..i).rev() {
        a[i] = next(a[i + 1]);
    }
}

fn fix_suffix(a: &mut [i32], i: usize) {
    for i in i + 1..a.len() {
        a[i] = next(a[i - 1]);
    }
}

fn fix_between(a: &mut [i32], mut i: usize, mut j: usize) -> bool {
    while (j - i) >= 2 {
        if a[i] > a[j] {
            a[i + 1] = next(a[i]);
            i += 1;
        } else {
            a[j - 1] = next(a[j]);
            j -= 1;
        }
    }
    a[i] == a[j] / 2 || a[j] == a[i] / 2
}

fn next(x: i32) -> i32 {
    if x == 1 {
        2
    } else {
        x / 2
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
