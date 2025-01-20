//{"name":"B. Find the Permutation","group":"Codeforces - Codeforces Round 997 (Div. 2)","url":"https://codeforces.com/contest/2056/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"3\n1\n0\n5\n00101\n00101\n11001\n00001\n11110\n6\n000000\n000000\n000000\n000000\n000000\n000000\n","output":"1\n4 2 1 3 5\n6 5 4 3 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFindThePermutation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = (0..n)
        .map(|_| input.read_line().into_bytes())
        .collect::<Vec<_>>();

    let mut p = vec![0; n];
    for i in 0..n {
        let mut cnt = 0;
        for j in i + 1..n {
            if a[i][j] == b'0' {
                cnt += 1;
            }
        }
        for j in 0..n {
            if p[j] == 0 {
                if cnt == 0 {
                    p[j] = i + 1;
                }
                cnt -= 1;
            }
        }
    }
    out.print_iter(p.into_iter());
    out.print_line("");
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
