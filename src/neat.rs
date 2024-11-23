/*
*
* ##########
* # Genome #
* ##########
*   
*   What is it?
*       - Genome is the created network which combines all NodeGene and
*         ConnectionGene. It maintains all knowlege of the network and handles
*         many operations on the network including mutation, cross-over,
*         fitness, etc. 
*
*
* ############
* # NodeGene #
* ############
*   
*   What is it?
*       - Nodes are the Neurons of the network. Each node has a calculated
*         activation based off all the input connection and its activation 
*         function. Nodes can be Input, Output, or Hidden nodes and are
*         connected together with ConnectionGene.
*
* 
* ##################
* # ConnectionGene #
* ##################
*   
*   What is it?
*       - Connections are the connections between nodes. They have values
*         associated with them called weights. Each weight of a Connection Gene
*         is the input for a Node's activation calculation. An innovation number
*         is assigned to a ConnectionGene upon creation based on the connection's
*         input Node and output Node. 
*
*
*
* Genome:
*   - connection_genes:
*       innovation_number: int
*       
*       
*
*   - node_genes:
*
*
*
*/


pub mod connection_gene;
pub mod genome;
pub mod node_gene;
mod innovation;


// Need a way to track global innovation numbers and handle operations on of all genomes
