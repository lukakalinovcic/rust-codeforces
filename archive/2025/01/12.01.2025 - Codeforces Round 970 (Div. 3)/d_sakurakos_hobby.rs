//{"name":"D. Sakurako's Hobby","group":"Codeforces - Codeforces Round 970 (Div. 3)","url":"https://codeforces.com/contest/2008/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1\n1\n0\n5\n1 2 4 5 3\n10101\n5\n5 4 1 3 2\n10011\n6\n3 5 6 1 2 4\n010000\n6\n1 2 3 4 5 6\n100110\n","output":"1\n0 1 1 1 1\n2 2 2 2 2\n4 1 4 4 1 4\n0 1 1 0 0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSakurakosHobby"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let p = input.read_int_vec(n);
    let s = input.read_line().into_bytes();
    let mut seen = vec![false; n];
    let mut result = vec![0; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut cycle = vec![];
        let mut x = i;
        let mut cnt = 0;
        while !seen[x] {
            seen[x] = true;
            cycle.push(x);
            x = p[x] as usize - 1;
            if s[x] == b'0' {
                cnt += 1
            }
        }
        for x in cycle {
            result[x] = cnt;
        }
    }
    out.print_iter(result.into_iter());
    out.print("\n");
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
