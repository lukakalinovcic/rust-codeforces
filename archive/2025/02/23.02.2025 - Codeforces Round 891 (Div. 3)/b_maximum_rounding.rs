//{"name":"B. Maximum Rounding","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n1\n5\n99\n913\n1980\n20444\n20445\n60947\n419860\n40862016542130810467\n","output":"1\n10\n100\n1000\n2000\n20444\n21000\n100000\n420000\n41000000000000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMaximumRounding"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut s = input.read_line().into_bytes();
    let mut carry = 0;
    let mut trailing = s.len();
    for (i, c) in s.iter_mut().enumerate().rev() {
        let x = (*c - b'0') as i32 + carry;
        if x >= 5 {
            carry = 1;
            trailing = i;
        } else {
            carry = 0;
            *c = x as u8 + b'0';
        }
    }
    for i in trailing..s.len() {
        s[i] = b'0';
    }
    if carry == 1 {
        out.print("1");
    }
    out.print_line(String::from_utf8(s).unwrap());
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
