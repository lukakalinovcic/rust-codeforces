//{"name":"F. Nim","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/F","interactive":false,"timeLimit":6000,"tests":[{"input":"9 5\n0 1 2 1 3 4 5 6 0\n1 5\n2 5\n3 5\n4 5\n1 9\n","output":"4 1\n2 1\n0 1\n-1\n8 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FNim"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve_seq(a: &[usize], total: u32) -> Option<(u32, u32)> {
    let mut prev = [(100, 1); 64];
    for i in 0..a.len() {
        let mut curr = prev;
        curr[a[i]] = (1, 1);
        for x in 0..64 {
            let (kept, ways) = prev[x ^ a[i]];
            let kept = kept + 1;
            if curr[x].0 > kept {
                curr[x] = (kept, ways);
            } else if curr[x].0 == kept {
                curr[x] = (kept, curr[x].1 + ways);
            }
        }
        prev = curr;
    }
    if prev[0].0 < 100 {
        return Some((total - prev[0].0, prev[0].1));
    }
    None
}

fn solve_cnts(cnts: &[u32; 51], total: u32) -> Option<(u32, u32)> {
    if cnts[0] > 0 {
        return Some((total - 1, cnts[0]));
    }
    let mut ways_two = 0;
    for a in 1..=50 {
        if cnts[a] >= 2 {
            ways_two += (cnts[a] as u64) * (cnts[a] as u64 - 1) / 2;
        }
    }
    if ways_two > 0 {
        return Some((total - 2, (ways_two % 998244353) as u32));
    }
    None
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, q) = (input.read_int() as usize, input.read_int() as usize);
    let mut prefix_cnts = vec![[0 as u32; 51]];
    prefix_cnts.reserve(n);
    for i in 0..n {
        let a = input.read_int() as usize;
        let mut next_cnts = prefix_cnts[i];
        next_cnts[a] += 1;
        prefix_cnts.push(next_cnts);
    }
    for _ in 0..q {
        let (l, r) = (input.read_int() as usize, input.read_int() as usize);
        let mut cnts = prefix_cnts[r];
        for a in 0..=50 {
            cnts[a] -= prefix_cnts[l - 1][a];
        }
        let total = (r - l + 1) as u32;
        let mut result = solve_cnts(&cnts, total);
        if result.is_none() {
            let mut a = vec![];
            for i in 0..=50 {
                if cnts[i] > 0 {
                    a.push(i);
                }
            }
            result = solve_seq(&a, total);
        }
        match result {
            Some((a, b)) => {
                out.print_iter([a, b].into_iter());
                out.put(b'\n');
            }
            None => out.print_line(-1),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
