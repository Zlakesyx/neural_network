use crate::neat::node_gene::NodeGene;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct ConnectionGene {
    pub in_node: NodeGene,
    pub out_node: NodeGene,
    pub weight: f32,
    pub enabled: bool,
    pub innovation_id: i8,
}

impl ConnectionGene {
    pub fn new(in_node: NodeGene, out_node: NodeGene, innovation_id: i8) -> Self {
        let weight = rand::thread_rng().gen_range(-1.0..1.0);
        let enabled = rand::thread_rng().gen_bool(0.5);

        return ConnectionGene {
            in_node,
            out_node,
            weight,
            enabled,
            innovation_id,
        };
    }
}
