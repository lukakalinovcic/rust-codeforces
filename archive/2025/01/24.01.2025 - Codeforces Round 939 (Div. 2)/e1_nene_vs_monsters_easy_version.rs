//{"name":"E1. Nene vs. Monsters (Easy Version)","group":"Codeforces - Codeforces Round 939 (Div. 2)","url":"https://codeforces.com/contest/1956/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n2 5 3\n2\n0 0\n4\n1 5 7 2\n4\n4 2 1 2\n13\n1 1 4 5 1 4 1 9 1 9 8 1 0\n","output":"1\n1\n0\n\n1\n1\n2\n1 3\n6\n1 3 6 8 10 12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1NeneVsMonstersEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input.read_int_vec(n);

    let mut i = 0;
    loop {
        let j = if i + 1 < n { i + 1 } else { 0 };
        let x = a[i];
        let y = a[j];
        a[j] = (y - x).max(0);
        i = j;
        if a[j] == 0 {
            break;
        }
    }
    loop {
        let mut groups = vec![];
        let mut curr_group = 0;
        let mut max_group = 0;
        for _ in 0..n {
            let j = if i + 1 < n { i + 1 } else { 0 };
            let x = a[i];
            let y = a[j];
            a[j] = (y - x).max(0);
            i = j;
            if a[j] == 0 {
                if curr_group > 0 {
                    max_group = max_group.max(curr_group);
                    groups.push(curr_group);
                }
                curr_group = 0;
            } else {
                curr_group += 1;
            }
        }
        if max_group <= 2 {
            break;
        }
    }

    let mut result = vec![];
    let mut curr_group: Vec<(usize, i32)> = vec![];
    for _ in 0..n {
        let j = if i + 1 < n { i + 1 } else { 0 };
        let x = a[i];
        let y = a[j];
        a[j] = (y - x).max(0);
        i = j;
        if a[j] == 0 {
            if curr_group.len() > 0 {
                result.push(curr_group[0].0 + 1);
            }
            curr_group.clear();
        } else {
            curr_group.push((j, a[j]));
        }
    }
    result.sort();
    out.print_line(result.len());
    out.print_iter(result.into_iter());
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
