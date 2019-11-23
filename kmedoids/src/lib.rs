extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn tester1() -> u8 {
    4
}


#[derive(Default)]
pub struct KMedoids {
    medoids: Vec<String>,
    node_labels: HashMap<String, usize>,
    label_indices: HashMap<String, usize>,
    nodes: Vec<String>,
    dist_matrix: Vec<Vec<f64>>,
    //dists: HashMap<(String, String), f64>,
}

impl KMedoids {
    fn new() -> Self {
        Default::default()
    }

    // init function to deal with input if needed
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
    
    fn assign_identities(&mut self) {
        for (index, medoid) in self.medoids.iter().enumerate() {
            self.node_labels.insert(medoid.to_string(), index);
        }
        for node in self.nodes.iter() {
            if !self.medoids.contains(&node) {
                //let node_index = match self.label_indices.get(&node) {
                //    Some(&index) => node_index,
                //    _ => println!("Key error"),
                //}
                let mut closest_medoid = &self.medoids[0];
                let node_idx = self.label_indices.get(node).unwrap();
                let medoid_idx = self.label_indices.get(closest_medoid).unwrap();
                let mut min_dist = self.dist_matrix[*node_idx][*medoid_idx];
 
                for medoid in self.medoids.iter() {
                    let node_idx = self.label_indices.get(node).unwrap();
                    let medoid_idx = self.label_indices.get(medoid).unwrap();
                    let current_dist = self.dist_matrix[*node_idx][*medoid_idx];
                    if current_dist < min_dist{
                        min_dist = current_dist;
                        closest_medoid = medoid;
                    }
                }
                self.node_labels.insert(node.to_string(), *self.node_labels.get(closest_medoid).unwrap());
            }
        }
    }

    fn fit(&mut self, k: u64) {
        let mut keys = Vec::new();
        let mut rng = rand::thread_rng();

        for key in self.label_indices.keys() {
            keys.push(key)
        }
    
        // initialize medoids, randomly choose k nodes
        //let mut init_medoids = Vec::new();
        for _ in 0..k {
            let rand_node = keys[rng.gen_range(0, keys.len())];
            self.medoids.push(rand_node.to_string());
        }
        
        self.assign_identities();
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
    fn test2() {
        assert_eq!(tester1(), 4);
    }
}
