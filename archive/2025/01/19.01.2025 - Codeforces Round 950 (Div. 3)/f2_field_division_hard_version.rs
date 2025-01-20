//{"name":"F2. Field Division (hard version)","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/F2","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n2 2 3\n1 1\n1 2\n2 2\n5 5 4\n1 2\n2 2\n3 4\n4 3\n2 5 9\n1 2\n1 5\n1 1\n2 2\n2 4\n2 5\n1 4\n2 3\n1 3\n6 4 4\n6 2\n1 3\n1 4\n1 2\n3 4 5\n2 1\n3 2\n1 4\n1 3\n2 4\n","output":"1\n1 0 1\n11\n0 1 0 4\n1\n0 0 1 1 0 0 0 0 0\n6\n15 0 0 0\n1\n2 3 0 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F2FieldDivisionHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let m = input.read_int();
    let k = input.read();
    let mut a = input
        .read_int_pair_vec(k)
        .into_iter()
        .enumerate()
        .map(|(i, (r, c))| (c - 1, n - r, i as i32))
        .collect::<Vec<_>>();
    a.push((0, n, -1));
    a.sort();
    a.reverse();
    let mut removed_pts = vec![vec![]; k];
    let mut s = vec![(m, -1, -1)];
    for (x, y, i) in a {
        while y <= s.last().cloned().unwrap().1 {
            let (xx, yy, _) = s.pop().unwrap();
            if i != -1 {
                removed_pts[i as usize].push((xx, yy));
            }
        }
        s.push((x, y, i));
    }
    let mut result = vec![0; k];
    let mut area = 0;
    let mut prev_y = n;
    while s.len() > 1 {
        let (x, y, i) = s.pop().unwrap();
        let mut next_x = s.last().cloned().unwrap().0;
        area += y as i64 * (next_x - x) as i64;
        if i != -1 {
            let mut delta_area = -y as i64 * (next_x - x) as i64;
            for (xx, yy) in removed_pts[i as usize].drain(..).rev() {
                if yy >= prev_y {
                    break;
                }
                delta_area += yy as i64 * (next_x - xx) as i64;
                next_x = xx;
            }
            delta_area += prev_y as i64 * (next_x - x) as i64;
            result[i as usize] = delta_area;
        }
        prev_y = y;
    }
    out.print_line(area);
    out.print_iter(result.into_iter());
    out.print_line("");
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
