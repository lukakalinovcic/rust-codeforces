//{"name":"D. Alice's Adventures in Cards","group":"Codeforces - Codeforces Round 986 (Div. 2)","url":"https://codeforces.com/contest/2028/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3\n1 3 2\n2 1 3\n1 2 3\n4\n2 3 1 4\n1 2 3 4\n1 4 2 3\n","output":"YES\n2\nk 2\nq 3\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAlicesAdventuresInCards"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = vec![vec![0], input.read_int_vec(n)].concat();
    let b = vec![vec![0], input.read_int_vec(n)].concat();
    let c = vec![vec![0], input.read_int_vec(n)].concat();
    let mut ma = (a[1], 1);
    let mut mb = (b[1], 1);
    let mut mc = (c[1], 1);
    let mut how = vec![None; n + 1];
    for i in 2..=n {
        if ma.0 > a[i] {
            how[i] = Some(('q', ma.1));
        }
        if mb.0 > b[i] {
            how[i] = Some(('k', mb.1));
        }
        if mc.0 > c[i] {
            how[i] = Some(('j', mc.1));
        }
        if how[i].is_some() {
            if a[i] > ma.0 {
                ma = (a[i], i);
            }
            if b[i] > mb.0 {
                mb = (b[i], i);
            }
            if c[i] > mc.0 {
                mc = (c[i], i);
            }
        }
    }
    let mut ops = vec![];
    let mut curr = n;
    while curr != 1 {
        match how[curr] {
            None => {
                out.print_line("NO");
                return;
            }
            Some((ch, prev)) => {
                ops.push((ch, curr));
                curr = prev;
            }
        }
    }
    ops.reverse();
    out.print_line("YES");
    out.print_line(ops.len());
    for (ch, i) in ops {
        out.print_line(format!("{ch} {i}"));
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
