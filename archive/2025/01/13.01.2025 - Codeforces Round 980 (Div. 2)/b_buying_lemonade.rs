//{"name":"B. Buying Lemonade","group":"Codeforces - Codeforces Round 980 (Div. 2)","url":"https://codeforces.com/contest/2024/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2 1\n1 1\n2 2\n1 2\n3 4\n2 1 3\n10 50\n1 1 3 8 8 9 12 13 27 27\n2 1000000000\n1000000000 500000000\n","output":"1\n2\n5\n53\n1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBuyingLemonade"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let mut m = input.read_long();
    let mut a = input.read_long_vec(n as usize);
    a.sort();
    let mut prev = 0;
    let mut result = 0;
    let mut j = 0;
    for i in 0..n {
        let curr = a[i as usize];
        let available = (curr - prev) * (n - i);
        if available > 0 {
            if available < m {
                result += (curr - prev) * (n - i) + (i - j);
                m -= available;
            } else {
                let k = (m - 1) / (n - j);
                result += k * (n - j);
                m -= k * (n - j);
                result += (i - j) + m;
                break;
            }
        }
        if curr != prev {
            while a[j as usize] == prev {
                j += 1;
            }
        }
        prev = curr;
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
