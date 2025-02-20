//{"name":"D. Palindrome Shuffle","group":"Codeforces - Educational Codeforces Round 174 (Rated for Div. 2)","url":"https://codeforces.com/contest/2069/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\nbaba\ncc\nddaa\nacbacddacbca\n","output":"2\n0\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPalindromeShuffle"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _precalc: &mut PreCalc) {
    let s = input.read_line().into_bytes();
    let n = s.len();
    let mut cnt = vec![[0; 26]; n + 1];
    for i in 0..n {
        for j in 0..26 {
            cnt[i + 1][j] = cnt[i][j];
        }
        cnt[i + 1][(s[i] - b'a') as usize] += 1;
    }
    let chars_superset = |i: usize, j: usize, len_i: usize, len_j: usize| {
        for k in 0..26 {
            let cnt_i = cnt[i + len_i][k] - cnt[i][k];
            let cnt_j = cnt[j + len_j][k] - cnt[j][k];
            if cnt_i < cnt_j {
                return false;
            }
        }
        true
    };
    let mut l = 0;
    let mut r = n - 1;
    while l < r && s[l] == s[r] {
        l += 1;
        r -= 1;
    }
    if l > r {
        out.print_line(0);
        return;
    }
    let mut l_mid = n / 2 - 1;
    let mut r_mid = n / 2;
    while s[l_mid] == s[r_mid] {
        l_mid -= 1;
        r_mid += 1;
    }
    if chars_superset(l, r_mid, l_mid - l + 1, r - r_mid + 1) {
        out.print_line(l_mid - l + 1);
        return;
    }

    let total = r - l + 1;
    for len in total / 2..total {
        let rem = total - len;
        if chars_superset(l, r - rem + 1, len, rem) || chars_superset(r - len + 1, l, len, rem) {
            out.print_line(len);
            return;
        }
    }
    out.print_line(String::from_utf8(s).unwrap());
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
