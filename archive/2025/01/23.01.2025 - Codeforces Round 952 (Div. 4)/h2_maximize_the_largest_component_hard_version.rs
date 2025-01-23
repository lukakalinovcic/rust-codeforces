//{"name":"H2. Maximize the Largest Component (Hard Version)","group":"Codeforces - Codeforces Round 952 (Div. 4)","url":"https://codeforces.com/contest/1985/problem/H2","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 1\n.\n4 2\n..\n#.\n#.\n.#\n3 5\n.#.#.\n..#..\n.#.#.\n5 5\n#...#\n....#\n#...#\n.....\n...##\n6 6\n.#..#.\n#..#..\n.#...#\n#.#.#.\n.#.##.\n###..#\n6 8\n..#....#\n.####.#.\n###.#..#\n.##.#.##\n.#.##.##\n#..##.#.\n","output":"1\n7\n11\n16\n22\n36\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"H2MaximizeTheLargestComponentHardVersion"}}}

use std::collections::BTreeSet;

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

    let mut row_events = vec![];
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

                let component_size = component.len() as i32;
                for r in minr..=maxr {
                    result_row[r] += component_size;
                }
                for c in minc..=maxc {
                    result_col[c] += component_size;
                }
                row_events.push((minr, component_size, (minc, maxc), r * n + c));
                row_events.push((maxr + 1, -component_size, (minc, maxc), r * n + c));
            }
        }
    }
    row_events.sort();
    let mut next_row_event = 0;
    let mut col_events = BTreeSet::new();
    let mut mx = 0;
    for r in 0..n {
        while next_row_event < row_events.len() {
            let (rr, delta, (minc, maxc), component_id) = row_events[next_row_event];
            if rr != r {
                break;
            }
            next_row_event += 1;
            if delta > 0 {
                col_events.insert((minc, delta, component_id));
                col_events.insert((maxc + 1, -delta, component_id));
            } else {
                col_events.remove(&(minc, -delta, component_id));
                col_events.remove(&(maxc + 1, delta, component_id));
            }
        }
        let col_events = col_events.iter().cloned().collect::<Vec<_>>();
        let mut next_col_event = 0;
        let mut overlap_size = 0;
        for c in 0..m {
            while next_col_event < col_events.len() {
                let (cc, delta, _) = col_events[next_col_event];
                if cc != c {
                    break;
                }
                next_col_event += 1;
                overlap_size += delta;
            }

            let mut result = result_row[r] + result_col[c] - overlap_size;
            if a[r][c] == b'.' {
                result -= 1;
            }
            mx = mx.max(result);
        }
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
