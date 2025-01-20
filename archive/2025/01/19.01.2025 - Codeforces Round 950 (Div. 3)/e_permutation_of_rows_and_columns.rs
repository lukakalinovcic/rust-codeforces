//{"name":"E. Permutation of Rows and Columns","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n1 1\n1\n1\n2 2\n1 2\n3 4\n4 3\n2 1\n2 2\n1 2\n3 4\n4 3\n1 2\n3 4\n1 5 9 6\n12 10 4 8\n7 11 3 2\n1 5 9 6\n12 10 4 8\n7 11 3 2\n3 3\n1 5 9\n6 4 2\n3 8 7\n9 5 1\n2 4 6\n7 8 3\n2 3\n1 2 6\n5 4 3\n6 1 2\n3 4 5\n1 5\n5 1 2 3 4\n4 2 5 1 3\n","output":"YES\nYES\nNO\nYES\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPermutationOfRowsAndColumns"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn read_matrix(input: &mut Input, n: usize, m: usize) -> (Vec<usize>, Vec<usize>) {
    let mut row = vec![0; n * m + 1];
    let mut col = vec![0; n * m + 1];
    for r in 0..n {
        for c in 0..m {
            let x: usize = input.read();
            row[x] = r;
            col[x] = c;
        }
    }
    (row, col)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let (a_row, a_col) = read_matrix(input, n, m);
    let (b_row, b_col) = read_matrix(input, n, m);

    let mut a_row_with_1 = vec![0; m];
    let mut a_col_with_1 = vec![0; n];
    for i in 1..=n * m {
        if a_row[i] == a_row[1] {
            a_row_with_1[a_col[i]] = i;
        }
        if a_col[i] == a_col[1] {
            a_col_with_1[a_row[i]] = i;
        }
    }
    for i in 1..=n * m {
        if b_col[i] != b_col[a_row_with_1[a_col[i]]] {
            out.print_line("NO");
            return;
        }
        if b_row[i] != b_row[a_col_with_1[a_row[i]]] {
            out.print_line("NO");
            return;
        }
    }
    out.print_line("YES");
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
