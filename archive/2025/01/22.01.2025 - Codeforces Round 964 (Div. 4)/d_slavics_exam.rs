//{"name":"D. Slavic's Exam","group":"Codeforces - Codeforces Round 964 (Div. 4)","url":"https://codeforces.com/contest/1999/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n?????\nxbx\nab??e\nabcde\nayy?x\na\nab??e\ndac\npaiu\nmom\n","output":"YES\nxabax\nYES\nabcde\nYES\nayyyx\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSlavicsExam"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut s = input.read_line().into_bytes();
    let t = input.read_line().into_bytes();
    let mut i = 0;
    for j in 0..t.len() {
        while i < s.len() && s[i] != t[j] && s[i] != b'?' {
            i += 1;
        }
        if i == s.len() {
            out.print_line("NO");
            return;
        } else {
            s[i] = t[j];
            i += 1;
        }
    }
    while i < s.len() {
        if s[i] == b'?' {
            s[i] = b'a';
        }
        i += 1;
    }
    out.print_line("YES");
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
