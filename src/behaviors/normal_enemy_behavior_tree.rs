use crate::behaviors::model::Node;
use crate::behaviors::model::NodeTrait;

pub struct StdEnemyBehaviorTree {
    root: Node,
    nodes: Vec<Node>,
    current_node: u32,
}

impl StdEnemyBehaviorTree {
    pub fn new() -> StdEnemyBehaviorTree {
        StdEnemyBehaviorTree {
            root: Node::new(0, "Root".to_string(), None, Some((1, 2))),
            nodes: vec![
                Node::new(1, "MoveToPlayer".to_string(), Some(0), Some((3, 4))),
                Node::new(2, "AttackPlayer".to_string(), Some(0), Some((5, 6))),
                Node::new(3, "CalculatePathToPlayer".to_string(), Some(1), None),
                Node::new(4, "MoveAlongPath".to_string(), Some(1), None),
                Node::new(5, "CalculateAttack".to_string(), Some(2), None),
                Node::new(6, "Attack".to_string(), Some(2), None),
            ],
            current_node: 0,
        }
    }

    pub fn get_root(&self) -> &Node {
        &self.root
    }

    pub fn get_next_behaviors(&self, node_id: u32) -> Option<(u32, u32)> {
        for node in &self.nodes {
            if node.get_id() == node_id {
                return node.get_children();
            }
        }
        None
    }

    pub fn get_node(&self, node_id: u32) -> Option<&Node> {
        for node in &self.nodes {
            if node.get_id() == node_id {
                return Some(node);
            }
        }
        None
    }

    pub fn get_node_name(&self, node_id: u32) -> Option<String> {
        for node in &self.nodes {
            if node.get_id() == node_id {
                return Some(node.get_name());
            }
        }
        None
    }

    pub fn get_node_parent(&self, node_id: u32) -> Option<u32> {
        for node in &self.nodes {
            if node.get_id() == node_id {
                return node.get_parent();
            }
        }
        None
    }

    pub fn get_node_children(&self, node_id: u32) -> Option<(u32, u32)> {
        for node in &self.nodes {
            if node.get_id() == node_id {
                return node.get_children();
            }
        }
        None
    }

    pub fn get_current_node(&self) -> u32 {
        self.current_node
    }

    pub fn set_current_node(&mut self, node_id: u32) {
        self.current_node = node_id;
    }
}
