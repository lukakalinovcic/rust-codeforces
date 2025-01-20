//{"name":"D. Penchick and Desert Rabbit","group":"Codeforces - Codeforces Round 987 (Div. 2)","url":"https://codeforces.com/contest/2031/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n4\n2 3 1 4\n5\n5 4 3 2 1\n4\n2 1 1 3\n4\n1 1 3 1\n8\n2 4 1 6 3 8 5 7\n","output":"3 3 3 4\n5 5 5 5 5\n2 2 2 3\n1 1 3 3\n8 8 8 8 8 8 8 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPenchickAndDesertRabbit"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let h = input.read_int_vec(n);
    let mut upper: Vec<(usize, i32, i32)> = vec![];
    for (i, a) in h.iter().enumerate() {
        let a = *a;
        if upper.is_empty() || upper.last().unwrap().1 < a {
            upper.push((i, a, a));
        }
    }
    let mut result = vec![0; n];
    for i in (0..n).rev() {
        result[i] = upper.last().unwrap().2;
        if upper.last().unwrap().0 == i {
            upper.pop();
        }
        let (Ok(mut j) | Err(mut j)) = upper.binary_search_by(|(_, a, _)| a.cmp(&(h[i] + 1)));
        while j < upper.len() {
            if upper[j].2 >= result[i] {
                break;
            }
            upper[j].2 = result[i];
            j += 1;
        }
    }
    out.print_iter(result.into_iter());
    out.put(b'\n');
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
