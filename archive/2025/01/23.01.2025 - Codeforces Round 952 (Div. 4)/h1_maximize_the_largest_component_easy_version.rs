//{"name":"H1. Maximize the Largest Component (Easy Version)","group":"Codeforces - Codeforces Round 952 (Div. 4)","url":"https://codeforces.com/contest/1985/problem/H1","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 1\n.\n4 2\n..\n#.\n#.\n.#\n3 5\n.#.#.\n..#..\n.#.#.\n5 5\n#...#\n....#\n#...#\n.....\n...##\n6 6\n.#..#.\n#..#..\n.#...#\n#.#.#.\n.#.##.\n###..#\n6 8\n..#....#\n.####.#.\n###.#..#\n.##.#.##\n.#.##.##\n#..##.#.\n","output":"1\n6\n9\n11\n15\n30\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"H1MaximizeTheLargestComponentEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut a = (0..n)
        .map(|_| input.read_line().into_bytes())
        .collect::<Vec<_>>();

    let mut result_row = vec![0; n];
    let mut result_col = vec![0; m];
    for r in 0..n {
        for c in 0..m {
            if a[r][c] == b'.' {
                result_row[r] += 1;
                result_col[c] += 1;
            } else if a[r][c] == b'#' {
                a[r][c] = b'!';
                let mut component = vec![(r, c)];
                let mut i = 0;
                let mut minr = r;
                let mut maxr = r;
                let mut minc = c;
                let mut maxc = c;

                while i < component.len() {
                    let (r, c) = component[i];
                    i += 1;
                    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let rr = r as i32 + dr;
                        let cc = c as i32 + dc;
                        let (rr, cc) = if rr >= 0 && rr < n as i32 && cc >= 0 && cc < m as i32 {
                            (rr as usize, cc as usize)
                        } else {
                            continue;
                        };
                        minr = minr.min(rr);
                        maxr = maxr.max(rr);
                        minc = minc.min(cc);
                        maxc = maxc.max(cc);
                        if a[rr][cc] == b'#' {
                            a[rr][cc] = b'!';
                            component.push((rr, cc));
                        }
                    }
                }

                for r in minr..=maxr {
                    result_row[r] += component.len() as i32;
                }
                for c in minc..=maxc {
                    result_col[c] += component.len() as i32;
                }
            }
        }
    }
    let mut mx = 0;
    for result_r in result_row {
        mx = mx.max(result_r);
    }
    for result_c in result_col {
        mx = mx.max(result_c);
    }
    out.print_line(mx);
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
