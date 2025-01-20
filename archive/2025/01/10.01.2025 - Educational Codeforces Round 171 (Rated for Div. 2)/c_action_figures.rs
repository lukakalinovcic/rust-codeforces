//{"name":"C. Action Figures","group":"Codeforces - Educational Codeforces Round 171 (Rated for Div. 2)","url":"https://codeforces.com/contest/2026/problem/C","interactive":false,"timeLimit":2500,"tests":[{"input":"4\n1\n1\n6\n101101\n7\n1110001\n5\n11111\n","output":"1\n8\n18\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CActionFigures"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: i32 = input.read_int();
    let s = input.read_line().into_bytes();
    let mut result: i64 = 0;
    let mut z = n;
    let mut o = 0;
    eprintln!("{}", String::from_utf8(s.clone()).unwrap());
    for i in (0..n).rev() {
        if s[i as usize] == b'0' {
            if i <= z {
                result += (i as i64) + 1;
            }
        } else {
            z = z.min(i - 1);
            while z >= 0 && s[z as usize] == b'1' {
                z -= 1;
            }
            if z >= 0 {
                result += (z as i64) + 1;
                z -= 1;
                continue;
            }
            while o < i && s[o as usize] == b'0' {
                o += 1;
            }
            if o < i {
                result += (o as i64) + 1;
                o += 1;
                continue;
            }
            if i >= o {
                result += (i as i64) + 1;
            }
        }
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
