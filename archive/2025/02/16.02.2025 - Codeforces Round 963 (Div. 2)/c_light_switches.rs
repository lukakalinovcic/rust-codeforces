//{"name":"C. Light Switches","group":"Codeforces - Codeforces Round 963 (Div. 2)","url":"https://codeforces.com/contest/1993/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n4 4\n2 3 4 5\n4 3\n2 3 4 5\n4 3\n3 4 8 9\n3 3\n6 2 1\n1 1\n1\n7 5\n14 34 6 25 46 7 17\n6 5\n40 80 99 60 90 50\n6 5\n64 40 50 68 70 10\n2 1\n1 1000000000\n","output":"5\n-1\n10\n8\n1\n47\n100\n-1\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLightSwitches"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let mut a = input.read_int_vec(n);
    a.sort();
    let mut lo = 0;
    let mut hi = k;
    for i in 0..n {
        let x = (a[i] - a[0]) % (2 * k);
        if x < k {
            lo = lo.max(x);
        } else {
            hi = hi.min((x + k) % (2 * k));
        }
    }
    if lo < hi {
        let x = (a[n - 1] - a[0]) % (2 * k);
        if lo <= x && x <= hi {
            out.print_line(a[n - 1]);
        } else if x < lo {
            out.print_line(a[n - 1] + (lo - x));
        } else {
            out.print_line(a[n - 1] + (lo - x + 2 * k));
        }
    } else {
        out.print_line(-1);
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
