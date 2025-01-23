//{"name":"G1. Division + LCP (easy version)","group":"Codeforces - Codeforces Round 943 (Div. 3)","url":"https://codeforces.com/contest/1968/problem/G1","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3 3 3\naba\n3 3 3\naaa\n7 2 2\nabacaba\n9 4 4\nabababcab\n10 1 1\ncodeforces\n9 3 3\nabafababa\n5 3 3\nzpozp\n","output":"0\n1\n3\n2\n10\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G1DivisionLCPEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn counting_sort_indices(ind: &Vec<usize>, val: &Vec<i32>, max_val: i32) -> Vec<usize> {
    let n = ind.len();
    let m = max_val as usize + 1;
    let mut c = vec![0; m];
    for i in 0..n {
        c[val[ind[i]] as usize] += 1;
    }
    for i in 1..m {
        c[i] += c[i - 1];
    }
    let mut result = vec![0; n];
    for i in (0..n).rev() {
        c[val[ind[i]] as usize] -= 1;
        let pos = c[val[ind[i]] as usize];
        result[pos] = ind[i];
    }
    result
}

fn cyclic_suffix_array(a: &Vec<i32>, max_val: i32) -> Vec<usize> {
    let n = a.len();

    let p = (0..n).collect::<Vec<_>>();
    let mut p = counting_sort_indices(&p, a, max_val);

    let mut c = vec![0; n];
    let mut class = 0;
    for i in 0..n {
        if i > 0 && a[p[i]] != a[p[i - 1]] {
            class += 1;
        }
        c[p[i]] = class;
    }

    let mut h = 0;
    while (1 << h) < n {
        p = p
            .into_iter()
            .map(|x| {
                if x >= (1 << h) {
                    x - (1 << h)
                } else {
                    x + n - (1 << h)
                }
            })
            .collect::<Vec<_>>();
        p = counting_sort_indices(&p, &c, class);

        class = 0;
        let mut new_c = vec![0; n];
        for i in 0..n {
            if i > 0 {
                let pi_a = p[i];
                let pi_b = (pi_a + (1 << h)) % n;
                let pj_a = p[i - 1];
                let pj_b = (pj_a + (1 << h)) % n;
                if (c[pi_a], c[pi_b]) != (c[pj_a], c[pj_b]) {
                    class += 1;
                }
            }
            new_c[p[i]] = class;
        }
        c = new_c;
        h += 1;
    }
    p
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let l = input.read_int();
    let _r = input.read_int();
    let s = input.read_line().into_bytes();
    let mut a = s.iter().map(|x| *x as i32).collect::<Vec<_>>();
    a.push(127);
    let p = cyclic_suffix_array(&a, 128);

    let mut matching = vec![0; n];
    matching[0] = n;
    let zero = p.iter().position(|x| *x == 0).unwrap();
    {
        let mut curr_match = 0;
        for i in 0..zero {
            while p[i] + curr_match < n && s[p[i] + curr_match] == s[curr_match] {
                curr_match += 1;
            }
            matching[p[i]] = curr_match;
        }
    }
    {
        let mut curr_match = 0;
        for i in (zero + 1..n).rev() {
            while p[i] + curr_match < n && s[p[i] + curr_match] == s[curr_match] {
                curr_match += 1;
            }
            matching[p[i]] = curr_match;
        }
    }

    let mut lo = 0;
    let mut hi = n;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        let mut x = 0;
        let mut jumps = 0;
        while x < n {
            if matching[x] >= mid {
                jumps += 1;
                x += mid;
            } else {
                x += 1;
            }
        }
        if jumps >= l {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    out.print_line(lo);
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
