//{"name":"C2. Skibidus and Fanum Tax (hard version)","group":"Codeforces - Codeforces Round 1003 (Div. 4)","url":"https://codeforces.com/contest/2065/problem/C2","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 3\n5\n9 1 1000000000\n3 2\n1 4 3\n3 4\n4 3\n2 4 6 5\n6 1 8\n5 2\n6 4 5 4 5\n4 1000\n3 1\n9 8 7\n8\n","output":"YES\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C2SkibidusAndFanumTaxHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let a = input.read_int_vec(n);
    let mut b = input.read_int_vec(m);
    b.sort();
    let mut prev = a[0].min(b[0] - a[0]);
    for i in 1..n {
        let mut m = None;
        if a[i] >= prev {
            m = Some(a[i]);
        }
        let (Ok(j) | Err(j)) = b.binary_search(&(prev + a[i]));
        if j < b.len() {
            m = match m {
                None => Some(b[j] - a[i]),
                Some(m) => Some(m.min(b[j] - a[i])),
            }
        }
        prev = match m {
            None => {
                out.print_line("NO");
                return;
            }
            Some(m) => m,
        };
    }
    out.print_line("YES");
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
