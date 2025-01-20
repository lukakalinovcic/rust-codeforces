//{"name":"G. Bugged Sort","group":"Codeforces - Codeforces Round 998 (Div. 3)","url":"https://codeforces.com/contest/2060/problem/G","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n3\n2 1 3\n4 6 5\n3\n2 1 5\n4 3 6\n4\n1 6 4 3\n5 2 8 7\n4\n5 3 7 1\n8 6 4 2\n7\n5 1 9 12 3 13 7\n2 4 11 14 6 10 8\n","output":"NO\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GBuggedSort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let mut pos = vec![0; 2 * n + 1];
    for i in 0..n {
        pos[a[i] as usize] = i;
        pos[b[i] as usize] = i;
    }

    let mut total_flips = 0;
    let mut group_size = 0;
    let mut have_odd_group = false;
    let mut seen = vec![false; n];
    let mut prev = (0, 1);
    for i in 1..=2 * n {
        let p = pos[i];
        if seen[p] {
            continue;
        }
        seen[p] = true;
        if a[p] > prev.0 && b[p] > prev.1 && a[p] > prev.1 && b[p] > prev.0 {
            // new group.
            if group_size % 2 == 1 {
                have_odd_group = true;
            }
            group_size = 0;
            prev = (a[p], b[p]);
        } else if a[p] > prev.0 && b[p] > prev.1 {
            prev = (a[p], b[p]);
        } else if a[p] > prev.1 && b[p] > prev.0 {
            prev = (b[p], a[p]);
            total_flips += 1;
        } else {
            out.print_line("NO");
            return;
        }
        group_size += 1;
    }
    if group_size % 2 == 1 {
        have_odd_group = true;
    }
    if total_flips % 2 == 1 && !have_odd_group {
        out.print_line("NO");
    } else {
        out.print_line("YES");
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
