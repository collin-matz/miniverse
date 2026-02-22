use std::marker::PhantomData;
use glam::DVec2;
use crate::model::{
    constants::Constants,
    types::{Body2D, Quadrant}
};

/// Structure defining a Node.
/// A Node is a single, internal structure maintained by the BHTree.
struct Node<T: Body2D> {
    index: Option<usize>,
    total_mass: f64,
    center_of_mass: DVec2,
    region_width: f64,
    quadrants: [Quadrant; 4],
    children: [Option<Box<Node<T>>>; 4],
    _marker: PhantomData<T>
}

impl<T: Body2D> Node<T> {
    /// Create a new Node with a Body index and a Quadrant it owns.
    fn new(index: Option<usize>, quadrant: Quadrant, bodies: &Vec<T>) -> Self {
        Node {
            index: index,
            quadrants: quadrant.subdivide(),
            region_width: quadrant.width(),
            total_mass: index.map_or(0.0, |i| bodies[i].mass()),
            center_of_mass: index.map_or(DVec2::ZERO, |i| bodies[i].position()),
            children: [None, None, None, None],
            _marker: PhantomData
        }
    }

    /// Returns whether the Node contains a Body index.
    fn is_empty(&self) -> bool {
        self.index.is_none()
    }

    /// Returns whether the Node is a leaf Node.
    fn is_external(&self) -> bool {
        // Check to see that all children are empty.
        let no_children = self.children.iter().all(|child| {
            match child {
                Some(_) => false,
                None => true
            }
        });

        !self.is_empty() && no_children
    }

    /// Returns whether this is an internal Node.
    fn is_internal(&self) -> bool {
        !self.is_external()
    }

    /// Insert a child Node into this Node. All inserts start from the
    /// root Node, so we can recursively call this until we hit a valid
    /// location for this new Node to live.
    fn insert(&mut self, index: usize, bodies: &Vec<T>) {
        // Find the index to place the Node into the children array.
        for (i, q) in self.quadrants.iter().enumerate() {
            if q.contains(bodies[index].position()) {

                // If the Node is None, insert the Body index here. Otherwise, we'll pass a
                // Box to a filled Node and handle it in the insert_into_child function.
                match &mut self.children[i] {
                    Some(child) => {
                        child.insert_into_child(index, bodies);
                    },
                    None => {
                        // This Node is completely empty, so we just set a new Node.
                        self.children[i] = Some(Box::new(Node::new(Some(index), q.clone(), bodies)));
                    }
                }
                break;
            }
        }
    }

    /// A recursive helper for the insert function.
    fn insert_into_child(&mut self, index: usize, bodies: &Vec<T>) {
        // Store this Body's position and mass.
        let body_pos = bodies[index].position();
        let body_mass = bodies[index].mass();

        // This Node is empty, so we can just store the Body.
        if self.is_empty() {
            self.index = Some(index);
        }
        else if self.is_external() {
            // Already has a body; take the existing body out and push both down into children.
            let existing = self.index.unwrap();
            self.insert(existing, bodies);
            self.insert(index, bodies);
        }
        else {
            // Internal Node; just pass down to correct child.
            self.insert(index, bodies);
        }
        self.update_center_of_mass(body_pos, body_mass);
    }

    /// Updates the center of mass for a Node from the provided Body's mass and position.
    fn update_center_of_mass(&mut self, body_pos: DVec2, body_mass: f64) {
        self.center_of_mass = (self.center_of_mass * self.total_mass + body_pos * body_mass) / (self.total_mass + body_mass);
        self.total_mass += body_mass;
    }

    // /// Calculate the forces acting on all Bodies from all other Bodies in the tree.
    // fn calculate_all_forces(&self, bodies: &Vec<T>) {
    //     // Collect all external Nodes.
    //     let mut exeternal_nodes: Vec<&Node<T>> = Vec::new();
    //     self.collect_external_nodes(&mut exeternal_nodes);

    //     // For each external Node, calculate the force that acts on the Body it contains.
    //     for node in exeternal_nodes {
    //         self.calculate_force_on_body(node, bodies);
    //     }
    // }

    // /// Recursively collect all external Nodes (Nodes with a Body and no children).
    // fn collect_external_nodes<'a>(&'a self, out: &mut Vec<&'a Node<T>>) {
    //     if self.is_external() {
    //         out.push(self);
    //     }
    //     else {
    //         for child in self.children.iter().flatten() {
    //             child.collect_external_nodes(out);
    //         }
    //     }
    // }

    // fn calculate_force_on_body(&self, target: &mut Node<T>, bodies: &Vec<T>) {
    //     if let Some(self_index) = self.index {
    //         if self.is_external() {
    //             if let Some(target_index) = target.index {
    //                 if !(self_index == target_index) {
    //                     // Calculate the direct force between self and target
    //                     bodies[target_index].apply_force_from(&bodies[self_index]);
    //                 }
    //             }
    //             return;
    //         }

    //         if let Some(target_index) = target.index {
    //             let distance = bodies[target_index].distance(&bodies[self_index]);
                
    //             // Calculate (s/d) < theta
    //             if (self.region_width / distance) < Constants::THETA {
    //                 let [self_body, target_body] = bodies.get_many_mut([self_index, target_index]);
                    

    //                 target_body.apply_force_from(self_body);
    //             }
    //             else {
    //                 for child in self.children.iter().flatten() {
    //                     child.calculate_force_on_body(target, bodies);
    //                 }
    //             }
    //         }
    //     }
    // }

    /// Display the contents of all Nodes, recursively.
    fn print_node(&self, depth: usize, bodies: &Vec<T>) {
        let indent = "  ".repeat(depth);
        println!("{}Node: mass={:.2}, com={:.2?}", indent, self.total_mass, self.center_of_mass);
        
        if let Some(index) = self.index {
            println!("{}  Body: pos={:.2?}", indent, bodies[index].position());
        }
        
        for (i, child) in self.children.iter().enumerate() {
            if let Some(child_node) = child {
                println!("{}  Child {}:", indent, i);
                child_node.print_node(depth + 1, bodies);
            }
        }
    }
}

/// Structure defining a Barnes-Hut quad-tree.
/// More on these tree types for physical body simulation: https://arborjs.org/docs/barnes-hut
pub struct BHTree<T: Body2D> {
    root: Node<T>,
    bodies: Vec<T>
}

impl<T: Body2D> BHTree<T> {
    /// Return a new BHTree instance.
    pub fn new(bodies: Vec<T>, dimensions: (DVec2, DVec2)) -> Self {
        // Calculate the Quadrant to be managed by the root Node.
        let quadrant = Quadrant::new(dimensions.0, dimensions.1);
        let mut root = Node::new(None, quadrant, &bodies);

        // For each Body in the bodies list, add these to the Nodes.
        for (i, body) in bodies.iter().enumerate() {
            root.update_center_of_mass(body.position(), body.mass());
            root.insert(i, &bodies);
        }

        // Return an instance to a new BHTree that owns the root Node and Body list.
        BHTree { root, bodies }
    }

    // /// Calculate the forces exerted on all bodies in the tree.
    // pub fn calculate_forces(&mut self) {
    //     let mut processed_nodes: Vec<&Node<T>> = vec![];
    //     self.root.calculate_all_forces();
    // }

    /// Display the tree contents in the terminal.
    /// (Useful for debugging the tree)
    pub fn display(&self) {
        self.root.print_node(0, &self.bodies);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::body::BodyBuilder;

    #[test]
    fn test_tree_build() {
        // Initialize some bodies.
        let bodies = vec![
            BodyBuilder::new().position(DVec2::new(10.0, 10.0)).mass(100.0).build(),
            BodyBuilder::new().position(DVec2::new(129.0, 10.0)).mass(200.0).build(),
            BodyBuilder::new().position(DVec2::new(192.0, 64.0)).mass(300.0).build()
        ];

        let dimensions = (DVec2::new(0.0, 0.0), DVec2::new(256.0, 256.0));

        let tree = BHTree::new(bodies, dimensions);
        tree.display();

        assert!(true);
    }
}
