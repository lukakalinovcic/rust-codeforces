//{"name":"D2. Turtle and a MEX Problem (Hard Version)","group":"Codeforces - Codeforces Round 968 (Div. 2)","url":"https://codeforces.com/contest/2003/problem/D2","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 4\n2 0 2\n3 2 3 3\n4 7 0 1 5\n3 4\n5 0 2 0 4 11\n1 1\n5 1 3 0 3 3\n2 50\n2 1 2\n2 1 2\n1 1\n7 1 2 4 1 4 9 5\n4 114514\n2 2 2\n5 7 3 6 0 3\n3 0 1 1\n5 0 9 2 1 5\n5 1919810\n1 2\n2 324003 0\n3 1416324 2 1460728\n4 1312631 2 0 1415195\n5 1223554 192248 2 1492515 725556\n","output":"16\n18\n1281\n4\n6556785365\n1842836177961\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2TurtleAndAMEXProblemHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read_size();
    let mut xy = vec![];
    let mut maxy = 0;
    for _ in 0..n {
        let k = input.read();
        let (x, y) = inspect(input.read_int_vec(k));
        maxy = maxy.max(y as usize);
        xy.push((x, y));
    }
    xy.sort();
    xy.reverse();
    let mut top = vec![0; maxy + 1];
    let mut cnt_x = vec![0; maxy + 1];
    let mut i = 0;
    let mut alt_start_top = 0;
    for x in (0..=maxy).rev() {
        top[x] = x;
        while i < xy.len() && xy[i].0 as usize == x {
            let y = xy[i].1 as usize;
            top[x] = top[x].max(top[y]);
            cnt_x[x] += 1;
            i += 1;
        }
        if cnt_x[x] >= 2 {
            alt_start_top = alt_start_top.max(top[x]);
        } else if cnt_x[x] >= 1 {
            alt_start_top = alt_start_top.max(x);
        }
    }

    let mut result = 0;
    for x in 0..=m.min(maxy) {
        result += top[x].max(alt_start_top) as i64;
    }
    if m > maxy {
        result += (maxy + 1 + m) as i64 * (m - maxy) as i64 / 2;
    }
    out.print_line(result);
}

fn inspect(a: Vec<i32>) -> (i32, i32) {
    let mut a = a;
    a.sort();
    let mut first = None;
    let mut second = None;
    let mut i = 0;
    for x in 0..a.len() as i32 + 2 {
        let mut missing = true;
        while i < a.len() && a[i] == x {
            i += 1;
            missing = false;
        }
        if missing {
            if first.is_none() {
                first = Some(x);
            } else if second.is_none() {
                second = Some(x);
            }
        }
    }
    (first.unwrap(), second.unwrap())
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
