use crate::ds::rmq::RMQ;

pub struct LCA {
    parent: Vec<i32>,
    entry_time: Vec<i32>,
    rmq: RMQ<(i32, i32)>,
}

struct DfsContext {
    parent: Vec<i32>,
    entry_time: Vec<i32>,
    traversal: Vec<(i32, i32)>,
}

impl LCA {
    pub fn new(adj: &Vec<Vec<i32>>, root: i32) -> Self {
        let n = adj.len();
        let mut ctx = DfsContext {
            parent: vec![0; n],
            entry_time: vec![0; n],
            traversal: vec![],
        };

        LCA::dfs(adj, root, root, 0, &mut ctx);
        Self {
            parent: ctx.parent,
            entry_time: ctx.entry_time,
            rmq: RMQ::new(ctx.traversal),
        }
    }

    fn dfs(adj: &Vec<Vec<i32>>, u: i32, p: i32, depth: i32, ctx: &mut DfsContext) {
        ctx.parent[u as usize] = p;
        ctx.entry_time[u as usize] = ctx.traversal.len() as i32;
        ctx.traversal.push((depth, u));
        for &v in &adj[u as usize] {
            if v != p {
                LCA::dfs(adj, v, u, depth + 1, ctx);
                ctx.traversal.push((depth, u));
            }
        }
    }

    pub fn query(&self, u: i32, v: i32) -> i32 {
        let (tu, tv) = (self.entry_time[u as usize], self.entry_time[v as usize]);
        let (tu, tv) = if tu <= tv { (tu, tv) } else { (tv, tu) };
        self.rmq.query(tu as usize, tv as usize).1
    }

    pub fn parent(&self, u: i32) -> Option<i32> {
        let p = self.parent[u as usize];
        if p == u {
            None
        } else {
            Some(p)
        }
    }
}
