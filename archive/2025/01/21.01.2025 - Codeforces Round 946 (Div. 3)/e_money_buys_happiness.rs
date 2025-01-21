//{"name":"E. Money Buys Happiness","group":"Codeforces - Codeforces Round 946 (Div. 3)","url":"https://codeforces.com/contest/1974/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n1 10\n1 5\n2 80\n0 10\n200 100\n3 100\n70 100\n100 200\n150 150\n5 8\n3 1\n5 3\n3 4\n1 5\n5 3\n2 5\n1 5\n2 1\n5 3\n2 5\n2 4\n4 1\n5 1\n3 4\n5 2\n2 1\n1 2\n3 5\n3 2\n3 2\n","output":"0\n10\n200\n15\n1\n9\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMoneyBuysHappiness"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read();
    let x = input.read_long();
    let mut prev = vec![0];
    let mut sum_h = 0;
    for (i, (c, h)) in input.read_int_pair_vec(m).into_iter().enumerate() {
        let money = i as i64 * x;
        let c = c as i64;
        let h = h as usize;
        sum_h += h;
        let mut curr = vec![INF; sum_h + 1];
        for h_prev in 0..prev.len() {
            curr[h_prev] = curr[h_prev].min(prev[h_prev]);
            if money - prev[h_prev] >= c {
                curr[h_prev + h] = curr[h_prev + h].min(prev[h_prev] + c);
            }
        }
        prev = curr;
    }
    for h in (0..prev.len()).rev() {
        if prev[h] != INF {
            out.print_line(h);
            return;
        }
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
