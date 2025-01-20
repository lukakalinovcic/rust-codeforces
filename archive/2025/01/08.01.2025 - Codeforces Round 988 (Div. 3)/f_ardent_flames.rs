//{"name":"F. Ardent Flames","group":"Codeforces - Codeforces Round 988 (Div. 3)","url":"https://codeforces.com/contest/2037/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"6\n5 5 3\n7 7 7 7 7\n1 2 3 4 5\n9 5 9\n2 4 6 8 10 8 6 4 2\n1 2 3 4 5 6 7 8 9\n2 10 2\n1 1\n1 20\n2 10 1\n69696969 420420420\n1 20\n2 10 2\n10 15\n1 19\n2 2 2\n1000000000 1\n1 3\n","output":"2\n2\n-1\n6969697\n15\n1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FArdentFlames"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 1000010000;

fn max_cover(h: &Vec<i32>, x: &Vec<i32>, max_damage: i32, hits: i32) -> i32 {
    let n = h.len();
    let mut events = vec![];
    for i in 0..n {
        let need_damage_per_hit = (h[i] + hits - 1) / hits;
        let range = (max_damage - need_damage_per_hit + 1).max(0);
        if range > 0 {
            events.push((x[i] - range + 1, 1));
            events.push((x[i] + range, -1));
        }
    }
    events.sort();
    let mut result = 0;
    let mut curr = 0;
    for (_, delta) in events {
        curr += delta;
        result = result.max(curr);
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let max_damage = input.read_int();
    let target = input.read_int();
    let h = input.read_int_vec(n);
    let x = input.read_int_vec(n);

    let mut lo = 1;
    let mut hi = INF;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if max_cover(&h, &x, max_damage, mid) < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo == INF {
        out.print_line(-1);
    } else {
        out.print_line(lo);
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
