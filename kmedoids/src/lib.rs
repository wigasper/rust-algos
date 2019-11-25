extern crate rand;

use rand::Rng;
use std::collections::HashMap;


fn cost(medoids: &Vec<String>, node_labels: &HashMap<String, usize>,
        label_indices: &HashMap<String, usize>, dist_matrix: &Vec<Vec<f64>>) -> f64 {
    // get total cost
    let mut total_cost = 0.0;
    for (key, val) in node_labels.iter() {
        // Labels are defined from medoid vector indices, so can just get the medoid
        // for any node this way
        let medoid = &medoids[*val];

        // get indices
        let medoid_idx = label_indices.get(medoid).unwrap();
        let node_idx = label_indices.get(key).unwrap();
        
        total_cost += dist_matrix[*node_idx][*medoid_idx];
    }
    total_cost
//    for medoid in medoids {
//        let current_cluster = new_node_labels.get(medoid).unwrap();
//        for (key, val) in new_node_labels.iter() {
//            if val == current_cluster {
//                let medoid_idx = label_indices.get(medoid).unwrap();
//                let node_idx = label_indices.get(key).unwrap();
//                total_cost += dist_matrix[*node_idx][*medoid_idx];
//            }
//        }
//    }
}

fn assign_identities(medoids: &Vec<String>, //node_labels: &HashMap<String, usize>,
                     nodes: &Vec<String>, label_indices: &HashMap<String, usize>,
                     dist_matrix: &Vec<Vec<f64>>) -> HashMap<String, usize> {
    // Assign each medoid the label of its index in the vector
    let mut new_node_labels: HashMap<String, usize> = HashMap::new();
    for (index, medoid) in medoids.iter().enumerate() {
        new_node_labels.insert(medoid.to_string(), index);
    }

    // For each non-medoid node, assign it the label of the closest medoid
    for node in nodes.iter() {
        if !medoids.contains(&node) {
            let mut closest_medoid = medoids[0].to_string();
            let node_idx = label_indices.get(node).unwrap();
            let medoid_idx = label_indices.get(&closest_medoid).unwrap();
            let mut min_dist = dist_matrix[*node_idx][*medoid_idx];
            
            // Check the dist of the node from each medoid
            for medoid in medoids.iter() {
                let node_idx = label_indices.get(node).unwrap();
                let medoid_idx = label_indices.get(medoid).unwrap();
                let current_dist = dist_matrix[*node_idx][*medoid_idx];
                if current_dist < min_dist {
                    min_dist = current_dist;
                    closest_medoid = medoid.to_string();
                }
            }
            new_node_labels.insert(node.to_string(), *new_node_labels.get(&closest_medoid).unwrap());
        }
    }

    new_node_labels
}

#[derive(Default)]
pub struct KMedoids {
    medoids: Vec<String>,
    node_labels: HashMap<String, usize>,
    label_indices: HashMap<String, usize>,
    nodes: Vec<String>,
    dist_matrix: Vec<Vec<f64>>,
    k: u64,
    //dists: HashMap<(String, String), f64>,
}

impl KMedoids {
    fn new() -> Self {
        Default::default()
    }

    fn swap_cost(&self, medoids: &Vec<String>, medoid: &String, node: &String) -> f64 {
        let mut temp_medoids = medoids.to_vec();
        let swap_index = temp_medoids.iter().position(|r| r == medoid).unwrap();
        temp_medoids.remove(swap_index);
        temp_medoids.insert(swap_index, node.to_string());
        
        let new_labels = assign_identities(&temp_medoids, &self.nodes, &self.label_indices,
                                           &self.dist_matrix);
        let new_cost = cost(&temp_medoids, &new_labels, &self.label_indices, &self.dist_matrix);

        new_cost
    }

    // Init function to deal with input if needed
    fn init(&mut self, labeled_matrix: &Vec<Vec<String>>) {
        for (index, col) in labeled_matrix[0].iter().enumerate() {
            if index != 0 {
                self.label_indices.insert(col.to_string(), index - 1);
                self.nodes.push(col.to_string());
            }
        }

        for (r_idx, row) in labeled_matrix.iter().enumerate() {
            if r_idx != 0 {
                let mut new_row = Vec::new();
                for (c_idx, col) in row.iter().enumerate() {
                    if c_idx != 0 {
                        let val: f64 = col.parse().unwrap_or_else(|why| {
                            panic!("Could not parse '{}': {}", col, why
                            );
                        });
                        new_row.push(val);
                    }
                }
                self.dist_matrix.push(new_row);
            }
        }
    }

    fn fit(&mut self, k: u64) {
        self.k = k;
        let mut keys = Vec::new();
        let mut rng = rand::thread_rng();
        
        // use nodes here
        for key in self.label_indices.keys() {
            keys.push(key)
        }
        
        // use nodes here instead of keys
        // initialize medoids, randomly choose k nodes
        //let mut init_medoids = Vec::new();
        for _ in 0..k {
            let rand_node = keys[rng.gen_range(0, keys.len())];
            self.medoids.push(rand_node.to_string());
        }
        
        let labels = assign_identities(&self.medoids, //&self.node_labels, 
                                                   &self.nodes, &self.label_indices, &self.dist_matrix);
        let mut current_cost = cost(&self.medoids, &labels, &self.label_indices, &self.dist_matrix);
        
        let mut num_iters: u64 = 0;
        loop {
            let prior_medoids = self.medoids.to_vec();
            for medoid in prior_medoids.iter() {
                // Default best swap
                let mut best_swap = self.nodes[0].to_string();
                let mut best_swap_cost = self.swap_cost(&prior_medoids, &medoid, &best_swap);
                // Get swap costs for the medoid and every node
                for node in self.nodes.iter() {
                    if !self.medoids.contains(node) {
                        let this_swap_cost = self.swap_cost(&prior_medoids, &medoid, &node);
                        
                        // If the swap cost is good, set vars
                        if this_swap_cost < best_swap_cost {
                            best_swap_cost = this_swap_cost;
                            best_swap = node.to_string();
                        }
                    //if !new_medoids.contains(node) {
                        // new_medoids must reflect swaps from prior rounds, cannot just copy
                        // prior_medoids in order to get correct labels and cost
                        // let mut new_medoids = self.medoids.to_vec();
                       
                        // Remove medoid of interest, replace with node
                        //let swap_index = new_medoids.iter().position(|r| r == medoid).unwrap()
                        //new_medoids.remove(swap_index);
                        //new_medoids.insert(swap_index, node.to_string());
                        
                        // Get new labels and cost for medoid/node swap
                        //let new_labels = assign_identities(&new_medoids, &self.nodes, &self.label_indices,
                        //                                   &self.dist_matrix);
                        //let new_cost = cost(&new_medoids, &new_labels, &self.label_indices, &self.dist_matrix);
                        
                        // If the swap results in a lower cost, update everything, and then go back 
                        // to fit_loop start in order to refresh everything and avoid a mutable
                        // borrow of an immutable.
                        //if new_cost < current_cost {
                        //    let swap_index = self.medoids.iter().position(|r| r == medoid).unwrap();
                        //    self.medoids.remove(swap_index);
                        //    self.medoids.insert(swap_index, node.to_string());
                        //    current_cost = new_cost;
                        //    continue 'fit_loop;
                       // }
                    }
                }
                // Make the best swap if it is less than the current_cost
                if best_swap_cost < current_cost {
                    let swap_index = self.medoids.iter().position(|r| r == medoid).unwrap();
                    self.medoids.remove(swap_index);
                    self.medoids.insert(swap_index, best_swap.to_string());
                    current_cost = best_swap_cost;
                }
            }
            num_iters += 1;
            if prior_medoids == self.medoids {
                break;
            }
        }
        self.node_labels = assign_identities(&self.medoids, &self.nodes, &self.label_indices, &self.dist_matrix);
    }

    fn print_labels(&mut self) {
        for label in 0..self.k {
            let mut relevant_nodes = Vec::new();
            for (key, val) in self.node_labels.iter() {
                if *val == label as usize {
                    relevant_nodes.push(key.to_string());
                }
            }
            print!("Cluster {}: ", label);
            for node in relevant_nodes {
                print!("{} ", node);
            }
            print!{"\n"};
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_init() {
        let mut model = KMedoids::new();
        let mut labeled_matrix: Vec<Vec<String>> = Vec::new();
        labeled_matrix.push(vec!["na".to_string(), "a".to_string(), "b".to_string()]);
        labeled_matrix.push(vec!["a".to_string(), "0".to_string(), "1".to_string()]);
        labeled_matrix.push(vec!["b".to_string(), "1".to_string(), "0".to_string()]);
        model.init(&labeled_matrix);
        let val_0: &usize = &0;
        let val_1: &usize = &1;
        // test label indices correctly recorded
        assert_eq!(model.label_indices.get(&"a".to_string()), Some(val_0));
        assert_eq!(model.label_indices.get(&"b".to_string()), Some(val_1));
        // test unlabeled_matrix vals
        assert_eq!(model.dist_matrix[0][0], 0.0);
        assert_eq!(model.dist_matrix[0][1], 1.0);
        assert_eq!(model.dist_matrix[1][0], 1.0);
        assert_eq!(model.dist_matrix[1][1], 0.0);


    }

    #[test]
    fn test_assign_identities() {
        let medoids = vec!["c".to_string(), "e".to_string()];
        //let node_labels: HashMap<String, usize>= HashMap::new();
        let nodes = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()];
        let mut label_indices: HashMap<String, usize> = HashMap::new();
        label_indices.insert("a".to_string(), 0);
        label_indices.insert("b".to_string(), 1);
        label_indices.insert("c".to_string(), 2);
        label_indices.insert("d".to_string(), 3);
        label_indices.insert("e".to_string(), 4);
        let mut dist_matrix: Vec<Vec<f64>> = Vec::new();
        dist_matrix.push(vec![0.0, 1.0, 2.0, 3.0, 4.0]);
        dist_matrix.push(vec![1.0, 0.0, 1.0, 2.0, 3.0]);
        dist_matrix.push(vec![2.0, 1.0, 0.0, 4.0, 2.0]);
        dist_matrix.push(vec![3.0, 2.0, 4.0, 0.0, 1.0]);
        dist_matrix.push(vec![4.0, 3.0, 2.0, 1.0, 0.0]);
        let new_labels = assign_identities(&medoids, &nodes, &label_indices, &dist_matrix);
       
        let val_0: &usize = &0;
        let val_1: &usize = &1;
        assert_eq!(new_labels.get(&"a".to_string()), Some(val_0));
        assert_eq!(new_labels.get(&"b".to_string()), Some(val_0));
        assert_eq!(new_labels.get(&"c".to_string()), Some(val_0));
        assert_eq!(new_labels.get(&"d".to_string()), Some(val_1));
        assert_eq!(new_labels.get(&"e".to_string()), Some(val_1));
    }

    #[test]
    fn test_cost() {
        let medoids = vec!["c".to_string(), "e".to_string()];
        //let node_labels: HashMap<String, usize>= HashMap::new();
        let nodes = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()];
        let mut label_indices: HashMap<String, usize> = HashMap::new();
        label_indices.insert("a".to_string(), 0);
        label_indices.insert("b".to_string(), 1);
        label_indices.insert("c".to_string(), 2);
        label_indices.insert("d".to_string(), 3);
        label_indices.insert("e".to_string(), 4);
        let mut dist_matrix: Vec<Vec<f64>> = Vec::new();
        dist_matrix.push(vec![0.0, 1.0, 2.0, 3.0, 4.0]);
        dist_matrix.push(vec![1.0, 0.0, 1.0, 2.0, 3.0]);
        dist_matrix.push(vec![2.0, 1.0, 0.0, 4.0, 2.0]);
        dist_matrix.push(vec![3.0, 2.0, 4.0, 0.0, 1.0]);
        dist_matrix.push(vec![4.0, 3.0, 2.0, 1.0, 0.0]);
        let new_labels = assign_identities(&medoids, &nodes, &label_indices, &dist_matrix);
       
        let cost0 = cost(&medoids, &new_labels, &label_indices, &dist_matrix);
        assert_eq!(cost0, 4.0);
        
        let medoids = vec!["e".to_string(), "c".to_string()];
        //let node_labels: HashMap<String, usize>= HashMap::new();
        let nodes = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()];
        let mut label_indices: HashMap<String, usize> = HashMap::new();
        label_indices.insert("a".to_string(), 0);
        label_indices.insert("b".to_string(), 1);
        label_indices.insert("c".to_string(), 2);
        label_indices.insert("d".to_string(), 3);
        label_indices.insert("e".to_string(), 4);
        let mut dist_matrix: Vec<Vec<f64>> = Vec::new();
        dist_matrix.push(vec![0.0, 1.0, 2.0, 3.0, 4.0]);
        dist_matrix.push(vec![1.0, 0.0, 1.0, 2.0, 3.0]);
        dist_matrix.push(vec![2.0, 1.0, 0.0, 4.0, 2.0]);
        dist_matrix.push(vec![3.0, 2.0, 4.0, 0.0, 1.0]);
        dist_matrix.push(vec![4.0, 3.0, 2.0, 1.0, 0.0]);
        let new_labels = assign_identities(&medoids, &nodes, &label_indices, &dist_matrix);
       
        let cost1 = cost(&medoids, &new_labels, &label_indices, &dist_matrix);
        assert_eq!(cost1, 4.0);
    }

    #[test]
    fn test_fit_0() {
        let mut lab_matrix: Vec<Vec<String>> = Vec::new();
        lab_matrix.push(vec!["na".to_string(),"a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string()]);
        lab_matrix.push(vec!["a".to_string(), "0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]);
        lab_matrix.push(vec!["b".to_string(), "1".to_string(), "0".to_string(), "1".to_string(), "2".to_string(), "3".to_string()]);
        lab_matrix.push(vec!["c".to_string(), "2".to_string(), "1".to_string(), "0".to_string(), "4".to_string(), "2".to_string()]);
        lab_matrix.push(vec!["d".to_string(), "3".to_string(), "2".to_string(), "4".to_string(), "0".to_string(), "1".to_string()]);
        lab_matrix.push(vec!["e".to_string(), "4".to_string(), "3".to_string(), "2".to_string(), "1".to_string(), "0".to_string()]);
        
        let mut model = KMedoids::new();
        model.init(&lab_matrix);
        model.fit(2);
        model.print_labels();
    }

    #[test]
    fn test_fit_1() {
        let mut lab_matrix: Vec<Vec<String>> = Vec::new();
        lab_matrix.push(vec!["na".to_string(),"a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string()]);
        lab_matrix.push(vec!["a".to_string(), "0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]);
        lab_matrix.push(vec!["b".to_string(), "1".to_string(), "0".to_string(), "1".to_string(), "2".to_string(), "3".to_string()]);
        lab_matrix.push(vec!["c".to_string(), "2".to_string(), "1".to_string(), "0".to_string(), "4".to_string(), "2".to_string()]);
        lab_matrix.push(vec!["d".to_string(), "3".to_string(), "2".to_string(), "4".to_string(), "0".to_string(), "1".to_string()]);
        lab_matrix.push(vec!["e".to_string(), "4".to_string(), "3".to_string(), "2".to_string(), "1".to_string(), "0".to_string()]);
        
        let mut model = KMedoids::new();
        model.init(&lab_matrix);
        model.fit(3);
        model.print_labels();
    }

    #[test]
    fn test_fit_2() {
        let mut lab_matrix: Vec<Vec<String>> = Vec::new();
        lab_matrix.push(vec!["na".to_string(),"a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string(), "f".to_string()]);
        lab_matrix.push(vec!["a".to_string(), "0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()]);
        lab_matrix.push(vec!["b".to_string(), "1".to_string(), "0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]);
        lab_matrix.push(vec!["c".to_string(), "2".to_string(), "1".to_string(), "0".to_string(), "4".to_string(), "2".to_string(), "3".to_string()]);
        lab_matrix.push(vec!["d".to_string(), "3".to_string(), "2".to_string(), "4".to_string(), "0".to_string(), "1".to_string(), "2".to_string()]);
        lab_matrix.push(vec!["e".to_string(), "4".to_string(), "3".to_string(), "2".to_string(), "1".to_string(), "0".to_string(), "1".to_string()]);
        lab_matrix.push(vec!["f".to_string(), "5".to_string(), "4".to_string(), "3".to_string(), "2".to_string(), "1".to_string(), "0".to_string()]);
        
        
        let mut model = KMedoids::new();
        model.init(&lab_matrix);
        model.fit(2);
        model.print_labels();
    }

}
