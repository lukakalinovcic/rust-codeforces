//{"name":"E. Haystacks","group":"Codeforces - Codeforces Round 996 (Div. 2)","url":"https://codeforces.com/contest/2055/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n2\n3 5\n2 4\n2\n10 1\n1 10\n3\n1 3\n4 3\n1 1\n3\n5 4\n2 4\n1 10\n6\n2 1\n3 3\n5 4\n1 5\n1 6\n1 8\n5\n3 2\n1 2\n1 1\n1 3\n6 5\n2\n5 10\n7 12\n","output":"8\n-1\n8\n9\n14\n15\n19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EHaystacks"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut sum_a: i64 = 0;
    let mut sum_b: i64 = 0;
    let mut order = vec![];
    for _ in 0..n {
        let a = input.read_long();
        let b = input.read_long();
        sum_a += a;
        sum_b += b;
        if a <= b {
            order.push(((0, a), (a, b)));
        } else {
            order.push(((1, -b), (a, b)));
        }
    }
    order.sort();
    let mut prefix_max = vec![-INF; n + 1];
    let mut prefix_sum_a = 0;
    let mut prefix_sum_b = 0;
    for (i, (_, (a, b))) in order.iter().enumerate() {
        let (a, b) = (*a, *b);
        prefix_sum_a += a;
        prefix_max[i + 1] = prefix_max[i].max(prefix_sum_a - prefix_sum_b);
        prefix_sum_b += b;
    }
    let mut suffix_max = -INF;
    let mut min_extra_moves = None;
    for (i, (_, (a, b))) in order.iter().enumerate().rev() {
        let (a, b) = (*a, *b);
        prefix_sum_a -= a;
        prefix_sum_b -= b;
        if sum_a <= sum_b - b {
            let extra_moves = prefix_max[i].max(suffix_max + prefix_sum_a - prefix_sum_b);
            min_extra_moves = match min_extra_moves {
                None => Some(extra_moves),
                Some(min_extra_moves) => Some(min_extra_moves.min(extra_moves)),
            };
        }
        suffix_max = (suffix_max + a - b).max(a);
    }
    match min_extra_moves {
        None => out.print_line(-1),
        Some(min_extra_moves) => out.print_line(sum_a + min_extra_moves),
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
