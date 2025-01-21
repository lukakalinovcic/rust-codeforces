//{"name":"C. Inhabitant of the Deep Sea","group":"Codeforces - Codeforces Round 938 (Div. 3)","url":"https://codeforces.com/contest/1955/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n4 5\n1 2 4 3\n4 6\n1 2 4 3\n5 20\n2 7 1 8 2\n2 2\n3 2\n2 15\n1 5\n2 7\n5 2\n","output":"2\n3\n5\n0\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CInhabitantOfTheDeepSea"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const MAX: usize = 1000000;

struct PreCalc {
    cnt_a: Vec<i32>,
    cnt_b: Vec<i32>,
}

impl PreCalc {
    fn new() -> Self {
        Self {
            cnt_a: vec![0; MAX + 1],
            cnt_b: vec![0; MAX + 1],
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut k = input.read_long();
    let mut a = input.read_long_vec(n);
    let mut i: i32 = 0;
    let mut j: i32 = n as i32 - 1;
    while i < j && k > 0 {
        let t = a[i as usize].min(a[j as usize]).min(k / 2) - 1;
        if t > 0 {
            k -= 2 * t;
            a[i as usize] -= t;
            a[j as usize] -= t;
        }
        if k == 0 {
            break;
        }
        a[i as usize] -= 1;
        k -= 1;
        if a[i as usize] == 0 {
            i += 1;
        }

        if k == 0 {
            break;
        }
        a[j as usize] -= 1;
        k -= 1;
        if a[j as usize] == 0 {
            j -= 1;
        }
    }
    if i == j {
        a[i as usize] -= k;
        if a[i as usize] <= 0 {
            i += 1;
        }
    }
    out.print_line(n as i32 - j + i - 1);
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
