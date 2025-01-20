//{"name":"E. Matrix Transformation","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1\n12\n13\n2 2\n10 10\n42 42\n21 21\n21 21\n2 2\n74 10\n42 106\n21 85\n85 21\n2 4\n1 2 3 4\n5 6 7 8\n3 2 3 4\n1 0 1 0\n","output":"Yes\nYes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMatrixTransformation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn read_matrix(input: &mut Input, n: usize, m: usize) -> Vec<Vec<u32>> {
    (0..n)
        .map(|_| (0..m).map(|_| input.read_unsigned()).collect())
        .collect()
}

fn extract_bit(a: &Vec<Vec<u32>>, bit: u32) -> Vec<Vec<u8>> {
    a.iter()
        .map(|row| row.iter().map(|cell| ((cell >> bit) & 1) as u8).collect())
        .collect()
}

fn check(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>, n: usize, m: usize) -> bool {
    let mut alive_row = vec![true; n];
    let mut alive_col = vec![true; m];
    loop {
        let mut found = false;
        for row in 0..n {
            if !alive_row[row] {
                continue;
            }
            let mut ok = true;
            for col in 0..m {
                if !alive_col[col] {
                    continue;
                }
                if b[row][col] != 0 {
                    ok = false;
                    break;
                }
            }
            if ok {
                alive_row[row] = false;
                found = true;
            }
        }
        for col in 0..m {
            if !alive_col[col] {
                continue;
            }
            let mut ok = true;
            for row in 0..n {
                if !alive_row[row] {
                    continue;
                }
                if b[row][col] != 1 {
                    ok = false;
                    break;
                }
            }
            if ok {
                alive_col[col] = false;
                found = true;
            }
        }
        if !found {
            break;
        }
    }
    let mut ok = true;
    for row in 0..n {
        if !alive_row[row] {
            continue;
        }
        for col in 0..m {
            if !alive_col[col] {
                continue;
            }
            ok = ok & (a[row][col] == b[row][col]);
        }
    }
    ok
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, m) = (input.read_int() as usize, input.read_int() as usize);
    let a = read_matrix(input, n, m);
    let b = read_matrix(input, n, m);
    let mut ok = true;
    for bit in 0..30 {
        let a = extract_bit(&a, bit);
        let b = extract_bit(&b, bit);
        ok = ok & check(&a, &b, n, m);
    }
    if ok {
        out.print_line("Yes");
    } else {
        out.print_line("No");
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
