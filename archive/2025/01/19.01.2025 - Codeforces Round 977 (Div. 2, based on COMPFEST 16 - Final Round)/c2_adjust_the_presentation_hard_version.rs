//{"name":"C2. Adjust The Presentation (Hard Version)","group":"Codeforces - Codeforces Round 977 (Div. 2, based on COMPFEST 16 - Final Round)","url":"https://codeforces.com/contest/2021/problem/C2","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n4 2 2\n1 2 3 4\n1 1\n1 2\n1 1\n3 6 2\n1 2 3\n1 1 2 3 3 2\n3 3\n2 2\n4 6 2\n3 1 4 2\n3 1 1 2 3 4\n3 4\n4 2\n","output":"YA\nTIDAK\nYA\nYA\nTIDAK\nYA\nTIDAK\nYA\nYA\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C2AdjustThePresentationHardVersion"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn check(
    is_ok: &mut Vec<i32>,
    a: &Vec<i32>,
    pos: &Vec<BTreeSet<usize>>,
    i: usize,
    n: usize,
    m: usize,
) -> i32 {
    if i < 1 || i >= n {
        return 0;
    }
    let lt = pos[a[i - 1] as usize].first().cloned().unwrap_or(m);
    let rt = pos[a[i] as usize].first().cloned().unwrap_or(m);
    let old_ok = is_ok[i];
    let new_ok = if lt <= rt { 1 } else { 0 };
    is_ok[i] = new_ok;
    new_ok - old_ok
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m: usize = input.read();
    let q: usize = input.read();
    let a = input.read_int_vec(n);
    let mut pos_a = vec![0; n + 1];
    for i in 0..n {
        pos_a[a[i] as usize] = i;
    }
    let mut b = input.read_int_vec(m);
    let mut pos = vec![BTreeSet::new(); n + 1];
    for i in 0..m {
        pos[b[i] as usize].insert(i);
    }
    let mut is_ok = vec![0; n];
    let mut num_ok = 0;
    for i in 1..n {
        num_ok += check(&mut is_ok, &a, &pos, i, n, m);
    }
    if num_ok == n as i32 - 1 {
        out.print_line("YA");
    } else {
        out.print_line("TIDAK");
    }
    for _ in 0..q {
        let i: usize = input.read::<usize>() - 1;
        let old_val = b[i];
        let new_val = input.read_int();
        pos[old_val as usize].remove(&i);
        pos[new_val as usize].insert(i);
        b[i] = new_val;
        num_ok += check(&mut is_ok, &a, &pos, pos_a[old_val as usize], n, m);
        num_ok += check(&mut is_ok, &a, &pos, pos_a[old_val as usize] + 1, n, m);
        num_ok += check(&mut is_ok, &a, &pos, pos_a[new_val as usize], n, m);
        num_ok += check(&mut is_ok, &a, &pos, pos_a[new_val as usize] + 1, n, m);
        if num_ok == n as i32 - 1 {
            out.print_line("YA");
        } else {
            out.print_line("TIDAK");
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
