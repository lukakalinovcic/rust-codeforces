//{"name":"F. Turtle and Three Sequences","group":"Codeforces - Codeforces Round 968 (Div. 2)","url":"https://codeforces.com/contest/2003/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"4 2\n2 3 4 2\n1 3 3 2\n1 4 2 3\n","output":"5\n"},{"input":"7 3\n1 4 5 2 3 6 7\n1 2 2 1 1 3 2\n1 5 6 7 3 2 4\n","output":"13\n"},{"input":"5 3\n1 2 3 4 5\n1 1 2 1 2\n5 4 3 2 1\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FTurtleAndThreeSequences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MAX_CAND_LEN: usize = 3;
const MAX_CANDS: usize = 8;

#[derive(Clone, Copy, Debug, Default)]
struct Cand {
    sum_c: i32,
    b: [i16; MAX_CAND_LEN],
}

#[derive(Clone, Copy, Default)]
struct FixedVecCand {
    n_cands: u8,
    cands: [Cand; MAX_CANDS],
}

impl FixedVecCand {
    fn push(&mut self, c: Cand) {
        self.cands[self.n_cands as usize] = c;
        self.n_cands += 1;
    }

    fn as_slice(&self) -> &[Cand] {
        &self.cands[0..self.n_cands as usize]
    }

    fn to_vec(&self) -> Vec<Cand> {
        self.cands[0..self.n_cands as usize].to_vec()
    }
}

fn get_cands(a: &[i32], b: &[i16], c: &[i32]) -> Vec<[FixedVecCand; MAX_CAND_LEN]> {
    let n = a.len();
    let mut cands = vec![[FixedVecCand::default(); MAX_CAND_LEN]; n];
    let mut new_cands = vec![];
    for i in 0..n {
        let mut bb = [0; MAX_CAND_LEN];
        bb[0] = b[i];
        cands[i][0].push(Cand { sum_c: c[i], b: bb });

        for k in 2..=MAX_CAND_LEN.min(i + 1) {
            for j in 0..i {
                if a[j] > a[i] {
                    continue;
                }
                for cand in cands[j][k - 2].as_slice() {
                    if cand.b.contains(&b[i]) {
                        continue;
                    }
                    let mut new_cand = cand.clone();
                    new_cand.sum_c += c[i];
                    new_cand.b[k - 1] = b[i];
                    new_cands.push(new_cand);
                }
            }
            new_cands.sort_by_key(|cand| -cand.sum_c);
            if new_cands.is_empty() {
                continue;
            }
            if k == 2 {
                let cand1 = new_cands[0];
                let mut cand2 = None;
                let mut cand3 = None;
                let mut cand4 = None;
                for cand in new_cands.drain(..).skip(1) {
                    if cand.b[0] != cand1.b[0] {
                        match cand2 {
                            None => cand2 = Some(cand),
                            Some(cand2) => {
                                if cand.b[0] != cand2.b[0] {
                                    match cand3 {
                                        None => cand3 = Some(cand),
                                        Some(cand3) => {
                                            if cand4.is_none() && cand.b[0] != cand3.b[0] {
                                                cand4 = Some(cand);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                cands[i][k - 1].push(cand1);
                for cand in [cand2, cand3, cand4] {
                    if let Some(cand) = cand {
                        cands[i][k - 1].push(cand);
                    }
                }
            } else {
                let (a, b) = (new_cands[0].b[0], new_cands[0].b[1]);
                let cand_ab = new_cands[0];
                let mut cand_cd = None;
                let mut cand_a1 = None;
                let mut cand_a2 = None;
                let mut cand_b1 = None;
                let mut cand_b2 = None;
                let mut cand_c1 = None;
                let mut cand_d1 = None;
                for cand in new_cands.drain(..).skip(1) {
                    let (x, y) = (cand.b[0], cand.b[1]);
                    if (x, y) == (a, b) || (x, y) == (b, a) {
                        continue;
                    } else if x == a || y == a {
                        let z = if x == a { y } else { x };
                        match cand_a1 {
                            None => cand_a1 = Some(cand),
                            Some(cand_a1) => {
                                if cand_a2.is_none() && !cand_a1.b.contains(&z) {
                                    cand_a2 = Some(cand);
                                }
                            }
                        }
                    } else if x == b || y == b {
                        let z = if x == b { y } else { x };
                        match cand_b1 {
                            None => cand_b1 = Some(cand),
                            Some(cand_b1) => {
                                if cand_b2.is_none() && !cand_b1.b.contains(&z) {
                                    cand_b2 = Some(cand);
                                }
                            }
                        }
                    } else {
                        match cand_cd {
                            None => {
                                cand_cd = Some(cand);
                            }
                            Some(cand_cd) => {
                                let (c, d) = (cand_cd.b[0], cand_cd.b[1]);
                                if (x, y) == (c, d) || (x, y) == (d, c) {
                                    continue;
                                } else if x == c || y == c {
                                    if cand_c1.is_none() {
                                        cand_c1 = Some(cand);
                                    }
                                } else if x == d || y == d {
                                    if cand_d1.is_none() {
                                        cand_d1 = Some(cand);
                                    }
                                } else {
                                    if cand_c1.is_none() {
                                        cand_c1 = Some(cand);
                                    } else if cand_d1.is_none() {
                                        cand_d1 = Some(cand);
                                    }
                                }
                            }
                        }
                    }
                }
                cands[i][k - 1].push(cand_ab);
                for cand in [
                    cand_cd, cand_a1, cand_a2, cand_b1, cand_b2, cand_c1, cand_d1,
                ] {
                    if let Some(cand) = cand {
                        cands[i][k - 1].push(cand);
                    }
                }
            }
        }
    }
    cands
}

fn merge_cands(ka: usize, kb: usize, a: &[Cand], b: &[Cand], c: i32) -> i32 {
    let mut result = -1;
    for cand_a in a {
        for cand_b in b {
            let mut ok = true;
            for i in 0..ka - 1 {
                for j in 0..kb - 1 {
                    if cand_a.b[i] == cand_b.b[j] {
                        ok = false;
                    }
                }
            }
            if ok {
                result = result.max(cand_a.sum_c + cand_b.sum_c - c);
            }
        }
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let c = input.read_int_vec(n);

    out.print_line(doit(m, a, b, c));
}

fn doit(m: usize, a: Vec<i32>, b: Vec<i32>, c: Vec<i32>) -> i32 {
    let n = a.len();
    let mut a = a;
    let mut b = b.into_iter().map(|b| b as i16).collect::<Vec<_>>();
    let mut c = c;
    let cands = get_cands(&a, &b, &c);
    a.reverse();
    b.reverse();
    c.reverse();
    for i in 0..n {
        a[i] = -a[i];
    }
    let rev_cands = get_cands(&a, &b, &c);
    let k1 = (m + 2) / 2;
    let k2 = (m + 1) / 2;
    let mut result = -1;
    for i in 0..n {
        result = result.max(merge_cands(
            k1,
            k2,
            &cands[i][k1 - 1].to_vec(),
            &rev_cands[n - i - 1][k2 - 1].to_vec(),
            c[n - i - 1],
        ));
    }
    result
}

pub static TEST_TYPE: TestType = TestType::Single;
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

fn brute(m: usize, a: Vec<i32>, b: Vec<i32>, c: Vec<i32>) -> i32 {
    let mut b_picks = vec![];
    rec(0, m, &a, &b, &c, 0, 0, &mut b_picks)
}

fn rec(
    i: usize,
    m: usize,
    a: &[i32],
    b: &[i32],
    c: &[i32],
    sum_c: i32,
    prev_a: i32,
    b_picks: &mut Vec<i32>,
) -> i32 {
    if m == 0 {
        return 0;
    }
    if i == a.len() {
        return -1;
    }
    let mut result = rec(i + 1, m, a, b, c, sum_c, prev_a, b_picks);
    if a[i] >= prev_a && !b_picks.contains(&b[i]) {
        b_picks.push(b[i]);
        let t = rec(i + 1, m - 1, a, b, c, sum_c + c[i], a[i], b_picks);
        if t != -1 {
            result = result.max(c[i] + t);
        }
        b_picks.pop();
    }
    result
}

fn main() {
    tester::run_tests();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 50000 == 0 {
            eprintln!("test num: {test_num}");
        }
        // let n = 3000;
        // let m = 5;
        let n = rng.gen_range(2..=25);
        let m = rng.gen_range(1..=5);
        let a = (0..n).map(|_| rng.gen_range(1..=20)).collect::<Vec<_>>();
        let b = (0..n).map(|_| rng.gen_range(1..=10)).collect::<Vec<_>>();
        let c = (0..n).map(|_| rng.gen_range(1..=100)).collect::<Vec<_>>();
        let t0 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let x = doit(m, a.clone(), b.clone(), c.clone());
        let t1 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        // eprintln!("{:?}", t1 - t0);
        let y = brute(m, a.clone(), b.clone(), c.clone());

        if x != y {
            eprintln!("Mismatch at {m} {a:?} {b:?} {c:?}  {x:?} != {y:?}");
            break;
        }
    }
}
//END MAIN
