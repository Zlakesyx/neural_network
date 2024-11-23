use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    INPUT,
    OUTPUT,
    HIDDEN,
}

impl Distribution<NodeType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NodeType {
        match rng.gen_range(0..3) {
            0 => NodeType::INPUT,
            1 => NodeType::OUTPUT,
            2 => NodeType::HIDDEN,
            _ => NodeType::HIDDEN,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeGene {
    pub activation: f32,
    pub innovation_id: i8,
    pub node_type: NodeType,
}

impl NodeGene {
    pub fn new(innovation_id: i8, node_type: NodeType) -> Self {
        let activation: f32 = rand::thread_rng().gen_range(-1.0..1.0);
        return NodeGene {
            activation,
            innovation_id,
            node_type,
        };
    }

    pub fn random(innovation_id: i8) -> Self {
        let activation: f32 = rand::thread_rng().gen_range(-1.0..1.0);
        let node_type: NodeType = rand::random();
        return NodeGene {
            activation,
            innovation_id,
            node_type,
        };
    }
}
