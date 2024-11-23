use std::collections::BTreeMap;

use rand::{seq::SliceRandom, Rng};

use crate::neat::{
    connection_gene::ConnectionGene,
    innovation,
    node_gene::{NodeGene, NodeType},
};

#[derive(Clone, Debug)]
pub struct Genome {
    pub connection_genes: BTreeMap<i8, ConnectionGene>, // {innovation_id: ConnectionGene}
    pub node_genes: BTreeMap<i8, NodeGene>,             // {innovation_id: NodeGene}
}

// TODO maybe add a Gene trait with specific implementations for node and connection
impl Genome {
    pub fn new(node_size: i8, connection_size: i8) -> Self {
        let connection_genes = BTreeMap::<i8, ConnectionGene>::new();
        let node_genes = BTreeMap::<i8, NodeGene>::new();
        let mut genome = Genome {
            connection_genes,
            node_genes,
        };

        for _ in 0..node_size {
            genome.add_node_gene();
        }

        for _ in 0..connection_size {
            genome.add_connection_gene();
        }

        genome
    }

    pub fn add_node_gene(&mut self) {
        // TODO All input nodes for now
        //let node = NodeGene::new(innovation::next_node(), NodeType::HIDDEN);
        let node = NodeGene::random(innovation::next_node());
        self.node_genes.insert(node.innovation_id, node);
    }

    pub fn add_connection_gene(&mut self) {
        let mut in_node_innovation: i8 = 0;
        let mut out_node_innovation: i8 = 0;
        let mut in_node = None;
        let mut out_node = None;
        // TODO attempt to gather list of NodeGenes for selecting random Node
        //let node_genes: Result<Vec<NodeGene>, _> = self.node_genes.clone().into_values().try_into();

        loop {
            println!("Starting loop");
            println!("{}", in_node_innovation);
            println!("{}", out_node_innovation);
            // TODO maybe choose random node from filtered selection instead of having it
            // completely randomized? Who owns the total collection of ALL innovation
            // nodes/connections?
            in_node_innovation = rand::thread_rng().gen_range(1..=innovation::current_node());
            out_node_innovation = rand::thread_rng().gen_range(1..=innovation::current_node());
            in_node = self.node_genes.get(&in_node_innovation);
            out_node = self.node_genes.get(&out_node_innovation);
            let mut valid = true;

            // TODO not accurate. If nodes do not exist within genome but exists globally, should
            // be added
            if in_node == None || out_node == None {
                println!("None");
                continue;
            }

            // In nodes cannot be output and out nodes cannot be input
            if in_node.unwrap().node_type == NodeType::OUTPUT
                || out_node.unwrap().node_type == NodeType::INPUT
            {
                println!("in/out error");
                continue;
            }

            for conn in self.connection_genes.clone().into_values() {
                // Exact connection exists
                if in_node_innovation == conn.in_node.innovation_id
                    && out_node_innovation == conn.out_node.innovation_id
                {
                    valid = false;
                    println!("conn exist");
                    break;
                }
            }

            if valid {
                break;
            }
        }

        let connection_gene = ConnectionGene::new(
            in_node.unwrap().clone(),
            out_node.unwrap().clone(),
            innovation::next_connection(),
        );

        self.connection_genes
            .insert(connection_gene.innovation_id, connection_gene);
    }

    pub fn feed_forward(&self) {

        /*
        let mut sorted_nodes = self.sort_nodes();

        for (layer, nodes) in sorted_nodes {
        for connection in self.connection_genes.clone() {
        if connection.in_node.layer == layer {
        let result: f32 = connection.in_node.activation * connection.weight;
        }
        }
        }
        */
    }

    pub fn mutate() {}

    pub fn calculate_distance() {}

    pub fn fitness() {}

    pub fn crossover() {}
}

#[test]
fn create_genome() {
    let genome = Genome::new(5, 4);
    println!("{:#?}", genome);

    let genome = Genome::new(3, 3);
    println!("{:#?}", genome);
}
