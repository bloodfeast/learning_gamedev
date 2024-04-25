use crate::behaviors::model::{Behavior, BehaviorTreeTrait, EnemyBehaviors, Node, NodeTrait};

fn get_normal_behaviors() -> EnemyBehaviors {
    EnemyBehaviors {
        root: Node::new(0, Behavior::Idle, None, Some((1, 2))),
        nodes: vec![
            Node::new(1, Behavior::MoveToRandom, Some(0), Some((3, 4))),
            Node::new(2, Behavior::MoveToPlayer, Some(0), Some((5, 6))),
            Node::new(3, Behavior::AttackRandom, Some(1), Some((7, 8))),
            Node::new(4, Behavior::MoveToPlayer, Some(1), Some((9, 10))),
            Node::new(5, Behavior::Dodge, Some(2), None),
            Node::new(6, Behavior::AttackPlayer, Some(2), None),
            Node::new(7, Behavior::MoveToPlayer, Some(3), None),
            Node::new(8, Behavior::AttackPlayer, Some(3), None),
            Node::new(9, Behavior::Dodge, Some(4), None),
            Node::new(10, Behavior::AttackPlayer, Some(4), None),
        ],
    }
}

pub struct NormalEnemyBehaviorTree {
    root: Node,
    nodes: Vec<Node>,
    current_node: u32,
}

impl BehaviorTreeTrait for NormalEnemyBehaviorTree {
    fn new() -> NormalEnemyBehaviorTree {
        NormalEnemyBehaviorTree {
            root: get_normal_behaviors().root.clone(),
            nodes: get_normal_behaviors().nodes.clone(),
            current_node: 0,
        }
    }

    fn get_root(&self) -> &Node {
        &self.root
    }

    fn get_next_behaviors(&self, node_id: u32) -> Option<(u32, u32)> {
        for node in &self.nodes {
            if node.get_id() == node_id {
                return node.get_children();
            }
        }
        None
    }

    fn get_node(&self, node_id: u32) -> Option<&Node> {
        for node in &self.nodes {
            if node.get_id() == node_id {
                return Some(node);
            }
        }
        None
    }

    fn get_node_name(&self, node_id: u32) -> Option<Behavior> {
        for node in &self.nodes {
            if node.get_id() == node_id {
                return Some(node.get_name());
            }
        }
        None
    }

    fn get_node_parent(&self, node_id: u32) -> Option<u32> {
        for node in &self.nodes {
            if node.get_id() == node_id {
                return node.get_parent();
            }
        }
        None
    }

    fn get_node_children(&self, node_id: u32) -> Option<(u32, u32)> {
        if node_id == 0 {
            return Some((1, 2));
        }
        for node in &self.nodes {
            if node.get_id() == node_id {
                return node.get_children();
            }
        }
        None
    }

    fn get_current_node(&self) -> u32 {
        self.current_node
    }
}
