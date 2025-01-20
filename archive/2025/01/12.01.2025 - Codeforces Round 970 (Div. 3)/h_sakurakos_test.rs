//{"name":"H. Sakurako's Test","group":"Codeforces - Codeforces Round 970 (Div. 3)","url":"https://codeforces.com/contest/2008/problem/H","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5 5\n1 2 3 4 5\n1\n2\n3\n4\n5\n6 3\n1 2 6 4 1 3\n2\n1\n5\n","output":"0 1 1 1 2\n1 0 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HSakurakosTest"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn cnt_smaller(prefix_cnt: &Vec<i32>, n: i32, x: i32, modulo: i32) -> i32 {
    let mut result = prefix_cnt[x as usize];
    for block in 1.. {
        let block_start = block * modulo;
        if block_start > n {
            break;
        }
        result += prefix_cnt[(block_start + x) as usize] - prefix_cnt[(block_start - 1) as usize];
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let a = input.read_int_vec(n);
    let mut cnt = vec![0; 2 * n + 1];
    for x in a {
        cnt[x as usize] += 1;
    }
    let mut prefix_cnt = cnt;
    for i in 0..2 * n {
        prefix_cnt[i + 1] += prefix_cnt[i];
    }

    let mut result_cache = vec![None; n + 1];
    let mut results = vec![];
    let n = n as i32;
    for _ in 0..q {
        let modulo = input.read_int();
        if let Some(r) = result_cache[modulo as usize] {
            results.push(r);
            continue;
        }

        let mut lo = 0;
        let mut hi = modulo - 1;
        while lo != hi {
            let mid = (lo + hi) / 2;
            if 2 * cnt_smaller(&prefix_cnt, n, mid, modulo) > n {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        result_cache[modulo as usize] = Some(lo);
        results.push(lo);
    }
    out.print_iter(results.into_iter());
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
