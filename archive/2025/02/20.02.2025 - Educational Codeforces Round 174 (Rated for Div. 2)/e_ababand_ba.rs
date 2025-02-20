//{"name":"E. A, B, AB and BA","group":"Codeforces - Educational Codeforces Round 174 (Rated for Div. 2)","url":"https://codeforces.com/contest/2069/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"7\nA\n0 0 10 10\nB\n0 1 0 0\nABA\n0 0 1 1\nABBABAAB\n5 5 0 0\nABABBAABBAAB\n1 1 2 3\nABBBBAB\n0 3 2 0\nBAABBA\n1 3 2 0\n","output":"NO\nYES\nNO\nYES\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EABABAndBA"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line().into_bytes();
    let a_max = input.read_int();
    let b_max = input.read_int();
    let ab_max = input.read_int();
    let ba_max = input.read_int();
    let mut start = 0;
    let mut a_groups = vec![];
    let mut b_groups = vec![];
    let mut a_cnt = 0;
    let mut b_cnt = 0;
    let mut ab_curr = 0;
    let mut ba_curr = 0;
    for i in 0..s.len() {
        if i + 1 == s.len() || s[i] == s[i + 1] {
            let g_len = (i - start + 1) as i32;
            let g_type = if g_len % 2 == 0 { 0 } else { 1 };
            if s[start] == b'A' {
                a_groups.push((g_type, g_len));
                ba_curr += (g_len - 1) / 2;
            } else {
                b_groups.push((g_type, g_len));
                ba_curr += g_len / 2;
            }
            start = i + 1;
        }
        if s[i] == b'A' {
            a_cnt += 1;
        } else {
            b_cnt += 1;
        }
    }
    a_groups.sort();
    b_groups.sort();
    b_groups.reverse();
    let mut max_pairs = ab_curr.min(ab_max) + ba_curr.min(ba_max);
    for (g_type, g_len) in a_groups {
        let ba_cnt = (g_len - 1) / 2;
        for _ in 0..ba_cnt {
            ba_curr -= 1;
            ab_curr += 1;
            max_pairs = max_pairs.max(ab_curr.min(ab_max) + ba_curr.min(ba_max));
        }
        if g_type == 0 {
            ab_curr += 1;
            max_pairs = max_pairs.max(ab_curr.min(ab_max) + ba_curr.min(ba_max));
        }
    }
    for (g_type, g_len) in b_groups {
        let ba_cnt = g_len / 2;
        for i in 0..ba_cnt {
            ba_curr -= 1;
            if i > 0 || g_type == 1 {
                ab_curr += 1;
            }
            max_pairs = max_pairs.max(ab_curr.min(ab_max) + ba_curr.min(ba_max));
        }
    }
    if a_cnt - max_pairs <= a_max && b_cnt - max_pairs <= b_max {
        out.print_line("YES");
    } else {
        out.print_line("NO");
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
