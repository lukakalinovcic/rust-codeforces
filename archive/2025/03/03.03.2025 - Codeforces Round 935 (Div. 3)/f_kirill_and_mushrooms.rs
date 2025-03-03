//{"name":"F. Kirill and Mushrooms","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3\n9 8 14\n3 2 1\n5\n1 2 3 4 5\n1 2 3 4 5\n6\n1 2 3 4 5 6\n6 5 4 3 2 1\n5\n1 4 6 10 10\n2 1 4 5 3\n4\n2 2 5 5\n4 2 3 1\n5\n1 2 9 10 10\n1 4 2 3 5\n","output":"16 2\n9 3\n8 2\n20 2\n5 1\n20 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKirillAndMushrooms"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let v = input.read_int_vec(n);
    let p = input.read_int_vec(n);
    let mut a = v
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<Vec<_>>();
    a.sort();
    a.reverse();
    let mut dead = vec![false; n];
    let mut on_stack = vec![false; n];
    let mut s = vec![];
    let mut non_zero = 0;
    let mut result = (0, 0);
    let mut i = 0;
    for k in 1..=n {
        while i < n && non_zero < k {
            let u = a[i].1;
            i += 1;
            if !dead[u] {
                s.push(u);
                on_stack[u] = true;
                non_zero += 1;
            }
        }
        while !s.is_empty() {
            let u = *s.last().unwrap();
            if dead[u] {
                s.pop();
            } else {
                break;
            }
        }
        let mini = if s.is_empty() {
            0
        } else {
            v[(*s.last().unwrap()) as usize]
        };
        let power = mini as i64 * non_zero as i64;
        if power > result.0 {
            result = (power, k);
        }

        dead[p[k - 1] as usize - 1] = true;
        if on_stack[p[k - 1] as usize - 1] {
            non_zero -= 1;
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
