//{"name":"G. Mischievous Shooter","group":"Codeforces - Codeforces Round 920 (Div. 3)","url":"https://codeforces.com/contest/1921/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 3 1\n.#.\n###\n.#.\n2 5 3\n###..\n...##\n4 4 2\n..##\n###.\n#..#\n####\n2 1 3\n#\n#\n","output":"3\n4\n5\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GMischievousShooter"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _m = input.read_size();
    let k = input.read_size();
    let mut a = (0..n).map(|_| input.read_line().into_bytes()).collect();
    let mut result = 0;
    for _ in 0..4 {
        result = result.max(doit(&a, k));
        a = rot90(a);
    }
    out.print_line(result);
}

fn rot90(a: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = a.len();
    let m = a[0].len();
    let mut b = vec![vec![0; n]; m];
    for r in 0..n {
        for c in 0..m {
            b[c][n - r - 1] = a[r][c];
        }
    }
    b
}

fn val(a: &Vec<Vec<u8>>, r: usize, c: usize) -> i32 {
    if a[r][c] == b'#' {
        1
    } else {
        0
    }
}

fn doit(a: &Vec<Vec<u8>>, k: usize) -> i32 {
    let n = a.len();
    let m = a[0].len();
    let k = k + 1;
    let mut tri = vec![0; m];
    let mut diag = vec![0; n + m];
    let mut result = 0;
    for r in 0..n {
        let mut row = 0;
        for c in 0..m {
            row += val(a, r, c);
            diag[r + c] += val(a, r, c);
            tri[c] += row;
            if r + c >= k {
                tri[c] -= diag[r + c - k];
            }
            result = result.max(tri[c]);
            if c >= k {
                row -= val(a, r, c - k);
            }
            if r >= k {
                diag[r + c - k] -= val(a, r - k, c);
            }
        }
    }
    result
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
