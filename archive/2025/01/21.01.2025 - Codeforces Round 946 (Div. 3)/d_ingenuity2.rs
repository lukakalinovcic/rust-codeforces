//{"name":"D. Ingenuity-2","group":"Codeforces - Codeforces Round 946 (Div. 3)","url":"https://codeforces.com/contest/1974/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n6\nNENSNE\n3\nWWW\n6\nNESSWS\n2\nSN\n2\nWE\n4\nSSNN\n4\nWESN\n2\nSS\n4\nEWNN\n4\nWEWE\n","output":"RRHRRH\nNO\nHRRHRH\nNO\nNO\nRHRH\nRRHH\nRH\nRRRH\nRRHH\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DIngenuity2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut s = input.read_line().into_bytes();
    let mut cnt_n = 0;
    let mut cnt_s = 0;
    let mut cnt_e = 0;
    let mut cnt_w = 0;
    for i in 0..n {
        match s[i] {
            b'N' => cnt_n += 1,
            b'S' => cnt_s += 1,
            b'E' => cnt_e += 1,
            b'W' => cnt_w += 1,
            _ => panic!("Unexpected char"),
        }
    }
    let cands_ns = get_candidates(cnt_n, cnt_s);
    let cands_ew = get_candidates(cnt_e, cnt_w);
    for cand_ns in &cands_ns {
        for cand_ew in &cands_ew {
            let (mut robot_n, mut robot_s) = (cand_ns.0, cand_ns.1);
            let (heli_n, heli_s) = (cnt_n - cand_ns.0, cnt_s - cand_ns.1);
            let (mut robot_e, mut robot_w) = (cand_ew.0, cand_ew.1);
            let (heli_e, heli_w) = (cnt_e - cand_ew.0, cnt_w - cand_ew.1);
            if (robot_n, robot_s, robot_e, robot_w) == (0, 0, 0, 0) {
                continue;
            }
            if (heli_n, heli_s, heli_e, heli_w) == (0, 0, 0, 0) {
                continue;
            }
            let h_or_r = |cnt| if cnt <= 0 { b'H' } else { b'R' };
            for i in 0..n {
                let x = match s[i] {
                    b'N' => h_or_r(robot_n),
                    b'S' => h_or_r(robot_s),
                    b'E' => h_or_r(robot_e),
                    b'W' => h_or_r(robot_w),
                    _ => panic!("Unexpected char"),
                };
                match s[i] {
                    b'N' => robot_n -= 1,
                    b'S' => robot_s -= 1,
                    b'E' => robot_e -= 1,
                    b'W' => robot_w -= 1,
                    _ => panic!("Unexpected char"),
                }
                s[i] = x;
            }
            out.print_line(String::from_utf8(s).unwrap());
            return;
        }
    }
    out.print_line("NO");
}

fn get_candidates(cnt_a: i32, cnt_b: i32) -> Vec<(i32, i32)> {
    let mut cands = vec![];
    for robot_a in 0..=cnt_a {
        let heli_a = cnt_a - robot_a;
        if (cnt_b + robot_a - heli_a) % 2 != 0 {
            continue;
        }
        let robot_b = (cnt_b + robot_a - heli_a) / 2;
        let heli_b = cnt_b - robot_b;
        if robot_b >= 0 && robot_b <= cnt_b && heli_b >= 0 && heli_b <= cnt_b {
            cands.push((robot_a, robot_b));
            if cands.len() >= 3 {
                break;
            }
        }
    }
    cands
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
