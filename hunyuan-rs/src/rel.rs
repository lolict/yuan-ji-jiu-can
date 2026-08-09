// rel.rs — 关系代数子系统
// 以二分法为根基,多分法为衍生,用有向图刻画符号之间的关系网络。
//
// 核心命题:
//   1. 一切关系归结为二分法 (两个节点之间的一条有向边)
//   2. 多分法是二分法的传递闭包 (A→B, B→C ⇒ A→C)
//   3. 分支评判: 一个节点的出度 > 1 即为分支点
//   4. 镜像: 将所有边的方向反转
//   5. 归属: 通过管辖范畴(子图)判定
//   6. 溯源: 沿入边反向追踪直到起点(源点)

/// 关系类型(边标签)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelKind {
    ShengMing,      // 声明/命名 (A 命名了 B)
    YongYou,        // 拥有 (A 拥有 B)
    YinYongBy,      // 被拥有 (反向)
    GongSheng,      // 共生 (双向)
    JiaoHu,         // 交互 (双向单事)
    ShuChu,         // 输出 (A 输出给 B)
    ShuRu,          // 输入 (反向)
    PaiSheng,       // 派生 (A 派生出 B)
    GuiShu,         // 归属 (A 属于 B)
    FanGuiShu,      // 被归属 (反向)
    DuiLi,          // 对立 (A 与 B 对立)
    JiChengBy,      // 继承 (反向)
    BeiCheng,       // 被继承 (反向)
    JieXiBy,        // 劫持 (反向但被动)
}

/// 关系(有向边)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rel {
    pub from: u8,       // 源节点 (0..63)
    pub to: u8,         // 目标节点 (0..63)
    pub kind: RelKind,  // 关系类型
}

impl Rel {
    pub fn new(from: u8, to: u8, kind: RelKind) -> Option<Self> {
        if from < 64 && to < 64 {
            Some(Rel { from, to, kind })
        } else {
            None
        }
    }

    /// 镜像: 反转方向,返回对应的反关系
    pub fn mirror(&self) -> Self {
        let new_kind = match self.kind {
            RelKind::ShuChu => RelKind::ShuRu,
            RelKind::ShuRu => RelKind::ShuChu,
            RelKind::YongYou => RelKind::YinYongBy,
            RelKind::YinYongBy => RelKind::YongYou,
            RelKind::PaiSheng => RelKind::GuiShu,
            RelKind::GuiShu => RelKind::FanGuiShu,
            RelKind::FanGuiShu => RelKind::GuiShu,
            RelKind::JiChengBy => RelKind::BeiCheng,
            RelKind::BeiCheng => RelKind::JiChengBy,
            RelKind::JieXiBy => RelKind::ShuChu,
            _ => self.kind,  // 对称关系镜像不变
        };
        Rel { from: self.to, to: self.from, kind: new_kind }
    }
}

/// 关系图 = 所有边的集合 + 二分法/多分法运算
#[derive(Debug, Clone)]
pub struct RelGraph {
    pub rels: Vec<Rel>,
}

impl RelGraph {
    pub fn new() -> Self { RelGraph { rels: Vec::new() } }

    pub fn add(&mut self, rel: Rel) { self.rels.push(rel); }

    pub fn from_pairs(pairs: &[(u8, u8, RelKind)]) -> Self {
        let mut g = Self::new();
        for &(from, to, kind) in pairs {
            g.add(Rel::new(from, to, kind).unwrap());
        }
        g
    }

    /// 查询从节点 a 出发的所有直接关系 (出)
    pub fn out_edges(&self, a: u8) -> Vec<Rel> {
        self.rels.iter().filter(|r| r.from == a).cloned().collect()
    }

    /// 查询进入节点 a 的所有直接关系 (入)
    pub fn in_edges(&self, a: u8) -> Vec<Rel> {
        self.rels.iter().filter(|r| r.to == a).cloned().collect()
    }

    /// 出度 (分支度,度数 = 有多少条线从该点出去)
    pub fn out_degree(&self, a: u8) -> usize {
        self.out_edges(a).len()
    }

    /// 是否分支点
    pub fn is_branch(&self, a: u8) -> bool {
        self.out_degree(a) > 1
    }

    /// 传递闭包: 二分法 → 多分法 (从 a 能到哪,包括间接)
    pub fn reachable(&self, a: u8) -> Vec<u8> {
        let mut visited = vec![false; 64];
        let mut result = Vec::new();
        let mut stack = vec![a];
        while let Some(cur) = stack.pop() {
            if visited[cur as usize] { continue; }
            visited[cur as usize] = true;
            result.push(cur);
            for rel in self.out_edges(cur) {
                if !visited[rel.to as usize] {
                    stack.push(rel.to);
                }
            }
        }
        result.retain(|&x| x != a);
        result
    }

    /// 溯源: 沿入边反向追溯,返回完整祖先链
    pub fn trace_origin(&self, a: u8) -> Vec<u8> {
        let mut visited = vec![false; 64];
        let mut result = Vec::new();
        let mut stack = vec![a];
        while let Some(cur) = stack.pop() {
            if visited[cur as usize] { continue; }
            visited[cur as usize] = true;
            for rel in self.in_edges(cur) {
                result.push(rel.from);
                if !visited[rel.from as usize] {
                    stack.push(rel.from);
                }
            }
        }
        result
    }

    /// 镜像图: 反转所有边的方向
    pub fn mirror(&self) -> Self {
        RelGraph { rels: self.rels.iter().map(|r| r.mirror()).collect() }
    }

    /// 归关系: a 是否归属于某个范畴(子图的根)
    pub fn is_owned_by(&self, a: u8, owner: u8) -> bool {
        self.reachable(owner).contains(&a)
    }

    /// 共享祖先: 两个节点的最大共同源点(判断分支归属)。
    /// 返回数值最大的共同祖先(在整个整除偏序中这就是 GCD)。
    pub fn common_origin(&self, a: u8, b: u8) -> Option<u8> {
        let ancestors_a: std::collections::HashSet<u8> =
            self.trace_origin(a).into_iter().collect();
        self.trace_origin(b)
            .into_iter()
            .filter(|x| ancestors_a.contains(x))
            .max()
    }
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rel_mirror() {
        let r = Rel::new(0, 1, RelKind::YongYou).unwrap();
        let m = r.mirror();
        assert_eq!(m.from, 1);
        assert_eq!(m.to, 0);
        assert_eq!(m.kind, RelKind::YinYongBy);
    }

    #[test]
    fn out_degree_and_branch() {
        let g = RelGraph::from_pairs(&[
            (0, 1, RelKind::PaiSheng),
            (0, 2, RelKind::PaiSheng),
            (0, 3, RelKind::PaiSheng),
        ]);
        assert_eq!(g.out_degree(0), 3);
        assert!(g.is_branch(0));
    }

    #[test]
    fn transitive_reach() {
        // 二分法: 0→1, 1→2, 2→3
        // 多分法: reachable(0) = {1, 2, 3}
        let g = RelGraph::from_pairs(&[
            (0, 1, RelKind::PaiSheng),
            (1, 2, RelKind::PaiSheng),
            (2, 3, RelKind::PaiSheng),
        ]);
        let reach = g.reachable(0);
        assert!(reach.contains(&1));
        assert!(reach.contains(&2));
        assert!(reach.contains(&3));
        assert!(!reach.contains(&0));
    }

    #[test]
    fn trace_origin_linear() {
        // 线性链: 0→1→2→3
        // trace_origin(3) = [2,1,0] (祖先链)
        let g = RelGraph::from_pairs(&[
            (0, 1, RelKind::PaiSheng),
            (1, 2, RelKind::PaiSheng),
            (2, 3, RelKind::PaiSheng),
        ]);
        let origins = g.trace_origin(3);
        assert!(origins.contains(&2));
        assert!(origins.contains(&1));
        assert!(origins.contains(&0));
    }

    #[test]
    fn mirror_graph() {
        let g = RelGraph::from_pairs(&[
            (0, 1, RelKind::YongYou),
            (2, 1, RelKind::GongSheng),
        ]);
        let m = g.mirror();
        // 原图: 0 拥有 1, 2 共生 1
        // 镜像: 1 被拥有于 0, 1 共生 2
        let r0 = Rel::new(1, 0, RelKind::YinYongBy).unwrap();
        assert!(m.rels.contains(&r0));
    }

    #[test]
    fn common_origin_branch() {
        // 分支结构:
        //   0 派生 1
        //   0 派生 2
        //   1 派生 3
        //   2 派生 3
        // 节点 3 有两个祖先: 1 和 2,再追溯到 0
        // common_origin(1, 2) = Some(0)
        let g = RelGraph::from_pairs(&[
            (0, 1, RelKind::PaiSheng),
            (0, 2, RelKind::PaiSheng),
            (1, 3, RelKind::PaiSheng),
            (2, 3, RelKind::PaiSheng),
        ]);
        let co = g.common_origin(1, 2);
        assert_eq!(co, Some(0));
    }
}
