//{"name":"D. Shift + Esc","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/D","interactive":false,"timeLimit":2500,"tests":[{"input":"5\n3 3 100\n3 4 9\n5 2 4\n0 101 101\n3 4 1\n10 0 0 10\n0 0 10 0\n10 10 0 10\n1 1 3\n4\n3 2 3\n1 2\n3 6\n5 4\n10 10 14\n58 49 25 12 89 69 8 49 71 23\n45 27 65 59 36 100 73 23 5 84\n82 91 54 92 53 15 43 46 11 65\n61 69 71 87 67 72 51 42 55 80\n1 64 8 54 61 70 47 100 84 50\n86 93 43 51 47 35 56 20 33 61\n100 59 5 68 15 55 69 8 8 60\n33 61 20 79 69 51 23 24 56 28\n67 76 3 69 58 79 75 10 65 63\n6 64 73 79 17 62 55 53 61 58\n","output":"113\n6\n4\n13\n618\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DShiftEsc"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, m) = (input.read_int() as usize, input.read_int() as usize);
    let k = input.read_long();
    let mut prev = vec![INF; m];
    prev[0] = 0;
    for _ in 0..n {
        let row = input.read_long_vec(m);
        let row = [row.clone(), row].concat();

        let mut next = vec![INF; m];
        for sh in 0..m {
            let mut prev_cell = INF;
            for c in 0..m {
                let curr_cell = row[c + sh] + prev_cell.min(prev[c]);
                next[c] = next[c].min(curr_cell + k * sh as i64);
                prev_cell = curr_cell;
            }
        }
        prev = next;
    }
    out.print_line(prev[m - 1]);
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
