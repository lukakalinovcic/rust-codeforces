//{"name":"B. Rectangle Filling","group":"Codeforces - Codeforces Round 941 (Div. 2)","url":"https://codeforces.com/contest/1966/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n2 1\nW\nB\n6 6\nWWWWBW\nWBWWWW\nBBBWWW\nBWWWBB\nWWBWBB\nBBBWBW\n1 1\nW\n2 2\nBB\nBB\n3 4\nBWBW\nWBWB\nBWBW\n4 2\nBB\nBB\nWW\nWW\n4 4\nWWBW\nBBWB\nWWBB\nBBBB\n1 5\nWBBWB\n","output":"NO\nYES\nYES\nYES\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRectangleFilling"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let a = (0..n)
        .map(|_| input.read_line().into_bytes())
        .collect::<Vec<_>>();
    if doit(n, m, a) {
        out.print_line("YES");
    } else {
        out.print_line("NO");
    }
}

fn doit(n: usize, m: usize, a: Vec<Vec<u8>>) -> bool {
    if a[0][0] == a[n - 1][m - 1] || a[0][m - 1] == a[n - 1][0] {
        return true;
    }
    for r in 1..n - 1 {
        if a[r][0] != a[0][0] && a[r][0] != a[n - 1][0] {
            return true;
        }
        if a[r][m - 1] != a[0][m - 1] && a[r][m - 1] != a[n - 1][m - 1] {
            return true;
        }
    }
    for c in 1..m - 1 {
        if a[0][c] != a[0][0] && a[0][c] != a[0][m - 1] {
            return true;
        }
        if a[n - 1][c] != a[n - 1][0] && a[n - 1][c] != a[n - 1][m - 1] {
            return true;
        }
    }
    false
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
