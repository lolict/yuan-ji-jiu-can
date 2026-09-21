// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.63 NJSG Swarm — 囡囝共生子代理并行开发引擎                   ║
// ║  本体-分身架构 · 名称锁定身份 · 任务分配 · 冲突裁决              ║
// ║                                                                  ║
// ║  核心概念:                                                       ║
// ║  · 本体 (NjsBody) = 唯一真相源, 不可分裂                         ║
// ║  · 分身 (Avatar)  = 本体在特定任务上的投影                        ║
// ║  · 名称锁定 = 分身以名字标识身份, 防止模仿                       ║
// ║  · 规则病毒 = 分身之间的规则冲突 (由裁决器检测)                  ║
// ║  · 协调器 (NjsOrchestrator) = 管理分配/裁决/汇总                  ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::{HashMap, VecDeque};

/// 分身身份: 以名字锁定, 不可伪造
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AvatarId(pub String);

impl AvatarId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl std::fmt::Display for AvatarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 分身角色 (与 MLIL 角色分工对应)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarRole {
    Maori,     // 母体: 承载/编码
    Kndl,      // 壳: 显示/交互
    Kudo,      // 旋翼: 抽取/索引
    Haibara,   // 横展: 统计/表列
    Judge,     // 裁决: 冲突裁定
}

impl AvatarRole {
    pub fn glyph(&self) -> char {
        match self {
            AvatarRole::Maori => '母',
            AvatarRole::Kndl => '壳',
            AvatarRole::Kudo => '旋',
            AvatarRole::Haibara => '横',
            AvatarRole::Judge => '裁',
        }
    }
}

/// 任务本体: 一个分身承担的原子工作单元
#[derive(Clone, Debug)]
pub struct SwarmTask {
    pub id: u64,
    pub description: String,
    pub assigned_to: Option<AvatarId>,
    pub status: TaskStatus,
    pub result: Option<String>,
    pub dependencies: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Blocked, // 等待依赖完成
}

/// 分身状态
#[derive(Clone, Debug)]
pub struct Avatar {
    pub id: AvatarId,
    pub role: AvatarRole,
    pub active_task: Option<u64>,
    pub completed_count: u32,
    pub error_count: u32,
}

impl Avatar {
    pub fn new(name: &str, role: AvatarRole) -> Self {
        Self {
            id: AvatarId::new(name),
            role,
            active_task: None,
            completed_count: 0,
            error_count: 0,
        }
    }

    pub fn is_idle(&self) -> bool {
        self.active_task.is_none()
    }
}

/// 冲突类型: 多分身并行时可能出现的规则/结果不兼容
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SwarmConflict {
    DoubleAssignment { task_id: u64, avatar_a: AvatarId, avatar_b: AvatarId },
    RuleIncompatibility { avatar_a: AvatarId, avatar_b: AvatarId, rule: String },
    DependencyLoop { task_ids: Vec<u64> },
    NameCollision { name: String },
}

impl std::fmt::Display for SwarmConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwarmConflict::DoubleAssignment { task_id, avatar_a, avatar_b } => {
                write!(f, "双分配: 任务{} 同时分配给 {} 和 {}", task_id, avatar_a, avatar_b)
            }
            SwarmConflict::RuleIncompatibility { avatar_a, avatar_b, rule } => {
                write!(f, "规则冲突: {} vs {} [{}]", avatar_a, avatar_b, rule)
            }
            SwarmConflict::DependencyLoop { task_ids } => {
                write!(f, "依赖环: {:?}", task_ids)
            }
            SwarmConflict::NameCollision { name } => {
                write!(f, "名称碰撞: {}", name)
            }
        }
    }
}

/// 裁决结果
#[derive(Clone, Debug)]
pub struct ArbitrationVerdict {
    pub conflict: SwarmConflict,
    pub winner: Option<AvatarId>,
    pub resolution: String,
}

/// NJSG 协调器: 管理分身群 + 任务分配 + 冲突裁决
pub struct NjsOrchestrator {
    pub avatars: HashMap<AvatarId, Avatar>,
    pub tasks: HashMap<u64, SwarmTask>,
    pub pending_queue: VecDeque<u64>,
    pub conflicts: Vec<SwarmConflict>,
    pub arbitrations: Vec<ArbitrationVerdict>,
    pub next_task_id: u64,
}

impl NjsOrchestrator {
    pub fn new() -> Self {
        Self {
            avatars: HashMap::new(),
            tasks: HashMap::new(),
            pending_queue: VecDeque::new(),
            conflicts: Vec::new(),
            arbitrations: Vec::new(),
            next_task_id: 1,
        }
    }

    /// 注册分身 (名称锁定)
    pub fn register_avatar(&mut self, name: &str, role: AvatarRole) -> Result<AvatarId, SwarmConflict> {
        let id = AvatarId::new(name);
        if self.avatars.contains_key(&id) {
            return Err(SwarmConflict::NameCollision { name: name.to_string() });
        }
        let avatar = Avatar::new(name, role);
        self.avatars.insert(id.clone(), avatar);
        Ok(id)
    }

    /// 提交任务
    pub fn submit_task(&mut self, description: &str, dependencies: &[u64]) -> u64 {
        let id = self.next_task_id;
        self.next_task_id += 1;
        let task = SwarmTask {
            id,
            description: description.to_string(),
            assigned_to: None,
            status: TaskStatus::Pending,
            result: None,
            dependencies: dependencies.to_vec(),
        };
        self.tasks.insert(id, task);
        self.pending_queue.push_back(id);
        id
    }

    /// 提交任务 (别名)
    pub fn submit(&mut self, description: &str, dependencies: &[u64]) -> u64 {
        self.submit_task(description, dependencies)
    }

    /// 检测循环依赖
    pub fn detect_dependency_loop(&self, task_id: u64) -> Option<Vec<u64>> {
        let mut visited = Vec::new();
        let mut stack = vec![task_id];
        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                visited.push(current);
                return Some(visited);
            }
            visited.push(current);
            if let Some(task) = self.tasks.get(&current) {
                for dep in &task.dependencies {
                    stack.push(*dep);
                }
            }
        }
        None
    }

    /// 分配任务给空闲分身 (按角色匹配)
    pub fn assign_next(&mut self) -> Option<(u64, AvatarId)> {
        if let Some(task_id) = self.pending_queue.pop_front() {
            // 检查依赖是否已完成
            let deps_satisfied = {
                let task = self.tasks.get(&task_id)?;
                task.dependencies.iter().all(|dep_id| {
                    self.tasks.get(dep_id)
                        .map(|t| matches!(t.status, TaskStatus::Completed))
                        .unwrap_or(false)
                })
            };

            if !deps_satisfied {
                // 依赖未完成, 放回队列末尾
                self.pending_queue.push_back(task_id);
                return None;
            }

            // 找空闲分身 (简单策略: 取第一个空闲的)
            let idle_avatar = self.avatars.iter()
                .find(|(_, avatar)| avatar.is_idle())
                .map(|(id, _)| id.clone());

            if let Some(avatar_id) = idle_avatar {
                // 分配
                if let Some(avatar) = self.avatars.get_mut(&avatar_id) {
                    avatar.active_task = Some(task_id);
                }
                if let Some(task) = self.tasks.get_mut(&task_id) {
                    task.assigned_to = Some(avatar_id.clone());
                    task.status = TaskStatus::Running;
                }
                return Some((task_id, avatar_id));
            } else {
                // 没有空闲分身, 放回队列
                self.pending_queue.push_back(task_id);
            }
        }
        None
    }

    /// 完成一个任务
    pub fn complete_task(&mut self, task_id: u64, result: &str) -> bool {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.status = TaskStatus::Completed;
            task.result = Some(result.to_string());
            // 释放分身
            if let Some(avatar_id) = &task.assigned_to {
                if let Some(avatar) = self.avatars.get_mut(avatar_id) {
                    avatar.active_task = None;
                    avatar.completed_count += 1;
                }
            }
            true
        } else {
            false
        }
    }

    /// 检测冲突: 是否有同一任务被多个分身持有
    pub fn detect_conflicts(&mut self) -> Vec<SwarmConflict> {
        let mut found = Vec::new();

        // 检测双分配
        let mut task_owners: HashMap<u64, Vec<AvatarId>> = HashMap::new();
        for (avatar_id, avatar) in &self.avatars {
            if let Some(task_id) = avatar.active_task {
                task_owners.entry(task_id).or_default().push(avatar_id.clone());
            }
        }
        for (task_id, owners) in task_owners {
            if owners.len() >= 2 {
                found.push(SwarmConflict::DoubleAssignment {
                    task_id,
                    avatar_a: owners[0].clone(),
                    avatar_b: owners[1].clone(),
                });
            }
        }

        self.conflicts.extend(found.clone());
        found
    }

    /// 裁决冲突: 根据角色优先级 (Judge > Maori > Kudo > Kndl > Haibara)
    pub fn arbitrate(&mut self, conflict: SwarmConflict) -> ArbitrationVerdict {
        let verdict = match &conflict {
            SwarmConflict::DoubleAssignment { task_id, avatar_a, avatar_b } => {
                let role_a = self.avatars.get(avatar_a).map(|a| &a.role);
                let role_b = self.avatars.get(avatar_b).map(|a| &a.role);
                let winner = match (role_a, role_b) {
                    (Some(a), Some(b)) => {
                        if role_priority(a) >= role_priority(b) {
                            Some(avatar_a.clone())
                        } else {
                            Some(avatar_b.clone())
                        }
                    }
                    _ => None,
                };
                ArbitrationVerdict {
                    conflict: conflict.clone(),
                    winner: winner.clone(),
                    resolution: format!("任务{}判给{:?}", task_id, winner),
                }
            }
            SwarmConflict::NameCollision { name } => {
                ArbitrationVerdict {
                    conflict: conflict.clone(),
                    winner: None,
                    resolution: format!("名称{}已存在,拒绝注册", name),
                }
            }
            _ => {
                ArbitrationVerdict {
                    conflict: conflict.clone(),
                    winner: None,
                    resolution: "待裁决".to_string(),
                }
            }
        };
        self.arbitrations.push(verdict.clone());
        verdict
    }

    /// 运行所有待分配任务直到完成
    pub fn run_all(&mut self, mock_results: &HashMap<u64, String>) {
        while !self.pending_queue.is_empty() {
            if let Some((task_id, _)) = self.assign_next() {
                if let Some(result) = mock_results.get(&task_id) {
                    self.complete_task(task_id, result);
                } else {
                    self.complete_task(task_id, "done");
                }
            } else {
                break;
            }
        }
    }

    pub fn idle_count(&self) -> usize {
        self.avatars.values().filter(|a| a.is_idle()).count()
    }

    pub fn completed_count(&self) -> usize {
        self.tasks.values().filter(|t| matches!(t.status, TaskStatus::Completed)).count()
    }

    pub fn total_tasks(&self) -> usize {
        self.tasks.len()
    }
}

fn role_priority(role: &AvatarRole) -> u8 {
    match role {
        AvatarRole::Judge => 5,
        AvatarRole::Maori => 4,
        AvatarRole::Kudo => 3,
        AvatarRole::Kndl => 2,
        AvatarRole::Haibara => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orchestrator_new_is_empty() {
        let orch = NjsOrchestrator::new();
        assert_eq!(orch.total_tasks(), 0);
        assert_eq!(orch.idle_count(), 0);
    }

    #[test]
    fn register_avatar_succeeds() {
        let mut orch = NjsOrchestrator::new();
        let id = orch.register_avatar("毛利兰", AvatarRole::Maori).unwrap();
        assert_eq!(id.0, "毛利兰");
    }

    #[test]
    fn register_duplicate_name_fails() {
        let mut orch = NjsOrchestrator::new();
        orch.register_avatar("毛利兰", AvatarRole::Maori).unwrap();
        let err = orch.register_avatar("毛利兰", AvatarRole::Kndl).unwrap_err();
        assert_eq!(err, SwarmConflict::NameCollision { name: "毛利兰".to_string() });
    }

    #[test]
    fn submit_task_gets_id() {
        let mut orch = NjsOrchestrator::new();
        let id = orch.submit_task("测试任务", &[]);
        assert!(id > 0);
        assert_eq!(orch.total_tasks(), 1);
    }

    #[test]
    fn assign_task_to_idle_avatar() {
        let mut orch = NjsOrchestrator::new();
        orch.register_avatar("工藤新一", AvatarRole::Kudo).unwrap();
        let task_id = orch.submit_task("分析数据", &[]);
        let assigned = orch.assign_next();
        assert!(assigned.is_some());
        let (tid, aid) = assigned.unwrap();
        assert_eq!(tid, task_id);
        assert_eq!(aid.0, "工藤新一");
    }

    #[test]
    fn no_idle_avatar_keeps_pending() {
        let mut orch = NjsOrchestrator::new();
        // 不注册任何分身
        orch.submit_task("任务", &[]);
        assert!(orch.assign_next().is_none());
    }

    #[test]
    fn complete_task_releases_avatar() {
        let mut orch = NjsOrchestrator::new();
        orch.register_avatar("灰原哀", AvatarRole::Haibara).unwrap();
        let task_id = orch.submit_task("统计", &[]);
        orch.assign_next();
        assert!(orch.idle_count() != 1); // 已分配, 空闲应减少
        orch.complete_task(task_id, "完成");
        assert_eq!(orch.idle_count(), 1);
        assert_eq!(orch.completed_count(), 1);
    }

    #[test]
    fn detect_double_assignment_conflict() {
        let mut orch = NjsOrchestrator::new();
        let a1 = orch.register_avatar("毛利兰", AvatarRole::Maori).unwrap();
        let a2 = orch.register_avatar("kndl", AvatarRole::Kndl).unwrap();
        let task_id = orch.submit_task("冲突任务", &[]);
        // 手动制造双分配
        orch.avatars.get_mut(&a1).unwrap().active_task = Some(task_id);
        orch.avatars.get_mut(&a2).unwrap().active_task = Some(task_id);
        let conflicts = orch.detect_conflicts();
        assert_eq!(conflicts.len(), 1);
        // 由于 HashMap 迭代顺序不确定, 检查 task_id 和两个 avatar 都匹配即可
        match &conflicts[0] {
            SwarmConflict::DoubleAssignment { task_id: tid, avatar_a, avatar_b } => {
                assert_eq!(*tid, task_id);
                let ids = [avatar_a.0.as_str(), avatar_b.0.as_str()];
                assert!(ids.contains(&"毛利兰"));
                assert!(ids.contains(&"kndl"));
            }
            _ => panic!("期望 DoubleAssignment"),
        }
    }

    #[test]
    fn arbitrate_double_assignment_picks_higher_role() {
        let mut orch = NjsOrchestrator::new();
        let _judge = orch.register_avatar("法官", AvatarRole::Judge).unwrap();
        let _haibara = orch.register_avatar("灰原哀", AvatarRole::Haibara).unwrap();
        let task_id = orch.submit_task("任务", &[]);
        let conflicts = orch.detect_conflicts(); // 没有冲突, 因为还没分配
        assert!(conflicts.is_empty());

        // 创建一个直接冲突
        let conflict = SwarmConflict::DoubleAssignment {
            task_id,
            avatar_a: AvatarId::new("法官"),
            avatar_b: AvatarId::new("灰原哀"),
        };
        let verdict = orch.arbitrate(conflict);
        // Judge优先级 > Haibara, 法官赢
        assert_eq!(verdict.winner, Some(AvatarId::new("法官")));
    }

    #[test]
    fn detect_dependency_loop_simple() {
        let mut orch = NjsOrchestrator::new();
        let t1 = orch.submit("A", &[2]); // A 依赖 B
        let t2 = orch.submit("B", &[1]); // B 依赖 A
        let loop1 = orch.detect_dependency_loop(t1);
        assert!(loop1.is_some(), "应检测到依赖环");
    }

    #[test]
    fn run_all_completes_tasks() {
        let mut orch = NjsOrchestrator::new();
        orch.register_avatar("毛利兰", AvatarRole::Maori).unwrap();
        orch.register_avatar("工藤新一", AvatarRole::Kudo).unwrap();
        orch.submit("任务1", &[]);
        orch.submit("任务2", &[]);
        let mock_results: HashMap<u64, String> = [
            (1, "完成1".to_string()),
            (2, "完成2".to_string()),
        ].into();
        orch.run_all(&mock_results);
        assert_eq!(orch.completed_count(), 2);
        assert_eq!(orch.idle_count(), 2);
    }

    #[test]
    fn avatar_is_idle_initially() {
        let avatar = Avatar::new("毛利兰", AvatarRole::Maori);
        assert!(avatar.is_idle());
        assert_eq!(avatar.completed_count, 0);
    }

    #[test]
    fn role_priority_order() {
        assert!(role_priority(&AvatarRole::Judge) > role_priority(&AvatarRole::Haibara));
        assert!(role_priority(&AvatarRole::Maori) > role_priority(&AvatarRole::Kndl));
        assert!(role_priority(&AvatarRole::Kudo) > role_priority(&AvatarRole::Haibara));
    }

    #[test]
    fn task_with_unmet_dependency_stays_pending() {
        let mut orch = NjsOrchestrator::new();
        orch.register_avatar("毛利兰", AvatarRole::Maori).unwrap();
        // 先提交依赖任务
        let t2 = orch.submit("任务2", &[]);
        // 提交依赖 t2 的任务
        let t1 = orch.submit("任务1", &[t2]);
        // 尝试分配: 任务1依赖未完成的 t2, 应该分配不成功
        let first_assign = orch.assign_next();
        // 任务2 可以被分配
        assert!(first_assign.is_some());
        let (assigned_id, _) = first_assign.unwrap();
        // 被分配的是任务2 (因为任务1依赖未完成的任务2, 被跳回了队列尾)
        assert_eq!(assigned_id, t2);
    }

    #[test]
    fn submit_alias_accepts_description() {
        let mut orch = NjsOrchestrator::new();
        let id = orch.submit("中文描述", &[]);
        assert_eq!(orch.tasks.get(&id).unwrap().description, "中文描述");
    }
}
