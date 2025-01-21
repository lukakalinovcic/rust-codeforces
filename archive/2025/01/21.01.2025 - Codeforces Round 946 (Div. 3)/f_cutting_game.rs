//{"name":"F. Cutting Game","group":"Codeforces - Codeforces Round 946 (Div. 3)","url":"https://codeforces.com/contest/1974/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n4 4 3 2\n4 1\n3 3\n2 4\nD 2\nR 1\n4 4 3 3\n4 1\n3 2\n2 3\nD 1\nL 1\nU 2\n3 5 3 2\n1 3\n2 2\n3 3\nR 2\nR 2\n6 4 4 2\n1 4\n2 3\n5 3\n1 1\nR 1\nU 1\n9 3 2 1\n6 1\n3 3\nD 8\n10 10 2 5\n7 5\n9 1\nR 1\nL 2\nD 1\nU 4\nD 1\n","output":"2 1\n2 0\n0 3\n1 1\n2 0\n0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FCuttingGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut r1 = 1;
    let mut r2 = input.read_int();
    let mut c1 = 1;
    let mut c2 = input.read_int();
    let n = input.read();
    let m = input.read();
    let p = input.read_int_pair_vec(n);
    let mut order_by_row = p
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, (r, _c))| (r, i))
        .collect::<Vec<_>>();
    let mut order_by_col = p
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, (_r, c))| (c, i))
        .collect::<Vec<_>>();
    let mut dead = vec![false; n];
    order_by_row.sort();
    order_by_col.sort();
    let (mut ri, mut rj) = (0, n as i32 - 1);
    let (mut ci, mut cj) = (0, n as i32 - 1);

    let mut score = [0, 0];
    for i in 0..m {
        let ch = input.read_char();
        let k = input.read_int();
        match ch {
            'U' => {
                r1 += k;
                while ri <= rj && order_by_row[ri as usize].0 < r1 {
                    let j = order_by_row[ri as usize].1;
                    if !dead[j] {
                        dead[j] = true;
                        score[i % 2] += 1;
                    }
                    ri += 1;
                }
            }
            'D' => {
                r2 -= k;
                while ri <= rj && order_by_row[rj as usize].0 > r2 {
                    let j = order_by_row[rj as usize].1;
                    if !dead[j] {
                        dead[j] = true;
                        score[i % 2] += 1;
                    }
                    rj -= 1;
                }
            }
            'L' => {
                c1 += k;
                while ci <= cj && order_by_col[ci as usize].0 < c1 {
                    let j = order_by_col[ci as usize].1;
                    if !dead[j] {
                        dead[j] = true;
                        score[i % 2] += 1;
                    }
                    ci += 1;
                }
            }
            'R' => {
                c2 -= k;
                while ci <= cj && order_by_col[cj as usize].0 > c2 {
                    let j = order_by_col[cj as usize].1;
                    if !dead[j] {
                        dead[j] = true;
                        score[i % 2] += 1;
                    }
                    cj -= 1;
                }
            }
            _ => panic!("Unexpected char"),
        }
    }
    out.print_line(score);
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
