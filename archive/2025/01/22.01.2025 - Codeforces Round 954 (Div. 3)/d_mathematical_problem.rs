//{"name":"D. Mathematical Problem","group":"Codeforces - Codeforces Round 954 (Div. 3)","url":"https://codeforces.com/contest/1986/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"18\n2\n10\n2\n74\n2\n00\n2\n01\n3\n901\n3\n101\n5\n23311\n6\n987009\n7\n1111111\n20\n99999999999999999999\n20\n00000000000000000000\n4\n0212\n18\n057235283621345395\n4\n1112\n20\n19811678487321784121\n4\n1121\n4\n2221\n3\n011\n","output":"10\n74\n0\n1\n9\n1\n19\n0\n11\n261\n0\n0\n0\n12\n93\n12\n24\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMathematicalProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let s = input
        .read_line()
        .into_bytes()
        .into_iter()
        .map(|ch| (ch - b'0') as i32)
        .collect::<Vec<_>>();
    let mut result = 1000000000;
    for i in 1..n {
        let mut a = vec![];
        for j in 0..i - 1 {
            a.push(s[j]);
        }
        a.push(s[i - 1] * 10 + s[i]);
        for j in i + 1..n {
            a.push(s[j]);
        }
        result = result.min(doit(&a));
    }
    out.print_line(result);
}

fn doit(a: &[i32]) -> i32 {
    let mut zeros = 0;
    let mut ones = 0;
    let mut sum = 0;
    for a in a {
        let a = *a;
        if a == 0 {
            zeros += 1;
        }
        if a == 1 {
            ones += 1;
        }
        sum += a;
    }
    if zeros > 0 {
        0
    } else if ones == sum {
        1
    } else {
        sum - ones
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
