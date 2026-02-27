use std::marker::PhantomData;
use glam::DVec2;
use log::warn;
use crate::model::{
    body::functions,
    constants::Constants,
    quadrant::Quadrant
};

const EXTERN_NODE_EXP_MSG: &str = "If the Node was external, we expected an index.";

/// A constant value that will check to see if the tree has attempted to infinitely recurse.
const MAX_RECURSION_DEPTH: usize = 100;

/// A trait represented an entity that has a position and a mass.
/// This is as generic as possible for the Barnes-Hut tree, as the only
/// requirements for the tree to create an index over values is for them
/// to have a position and a mass. Any structure that implements this
/// trait can be indexed by this tree.
pub trait HasPositionAndMassTrait {
    fn mass(&self) -> f64;
    fn position(&self) -> DVec2;
}

/// Structure defining a Node.
/// A Node is a single, internal structure maintained by the BHTree.
struct Node<T: HasPositionAndMassTrait> {
    index: Option<usize>,                   // Index stored for each entity
    total_mass: f64,                        // Total mass this Node owns
    center_of_mass: DVec2,                  // Center of mass for this Node
    quadrant: Quadrant,                     // Quadrant owned by this Node
    children: [Option<Box<Node<T>>>; 4],    // Children Nodes underneath this Node
    _marker: PhantomData<T>
}

impl<T: HasPositionAndMassTrait> Node<T> {
    /// Create a new Node.
    fn new(index: Option<usize>, quadrant: Quadrant, total_mass: f64, center_of_mass: DVec2) -> Self {
        Node {
            index: index,
            quadrant: quadrant,
            total_mass: total_mass,
            center_of_mass: center_of_mass,
            children: [None, None, None, None],
            _marker: PhantomData
        }
    }

    /// Returns whether the Node contains an index.
    fn is_empty(&self) -> bool {
        self.index.is_none()
    }

    /// Returns whether the Node is an external Node.
    /// An external Node is one that does not have any children, but does have an index.
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

    /// Insert a child Node into this Node. All inserts start from the
    /// root Node, so we can recursively call this until we hit a valid
    /// location for this new Node to live.
    fn insert(&mut self, index: usize, entities: &[T], depth: usize) {
        if depth > MAX_RECURSION_DEPTH {
            panic!("Infinite recursion detected");
        }

        // Find the index to place the Node into the children array.
        let subquads = self.quadrant.subdivide();

        // Store the new Entity's position and mass.
        let new_position = entities[index].position();
        let new_mass = entities[index].mass();

        for (i, q) in subquads.iter().enumerate() {
            if q.contains(entities[index].position()) {
                match &mut self.children[i] {

                    // If there is no child Node in this index, insert the new Node here.
                    None => {
                        let e = &entities[index];
                        self.children[i] = Some(
                            Box::new(
                                Node::new(Some(index), q.clone(), e.mass(), e.position())
                            )
                        );
                    },

                    // Otherwise, recursively call insert on the child Node.
                    Some(child) => {

                        // Reset the Child's mass values to account for the fact that we will call
                        // update_mass_values() recursively on this Node.
                        child.total_mass = 0.0;
                        child.center_of_mass = DVec2::ZERO;

                        // External Node; take the existing Entity out and push both down into children.
                        if child.is_external() {
                            // Add a check to see if the Entity that is being added has the exact same position as
                            // the child. If this is true, the tree will infinitely recurse. We'll simply return a warning
                            // and skip inserting this Node if that is the case.
                            if entities[child.index.expect(EXTERN_NODE_EXP_MSG)].position().distance(new_position) < 0.01  {
                                return;
                            }

                            let existing_index = child.index.expect(EXTERN_NODE_EXP_MSG);

                            child.index = None;

                            child.insert(existing_index, entities, depth + 1);
                            child.insert(index, entities, depth + 1);
                        }

                        // The Node is not External, so it must be Internal; Pass down to correct child.
                        else {
                            child.insert(index, entities, depth + 1);
                        }
                    }
                }
                break;
            }
        }

        self.update_mass_values(new_position, new_mass);
    }

    /// Updates the center of mass for a Node from the provided Entity's mass and position.
    fn update_mass_values(&mut self, position: DVec2, mass: f64) {
        self.center_of_mass = (self.center_of_mass * self.total_mass + position * mass) / (self.total_mass + mass);
        self.total_mass += mass;
    }

    /// Calculate the forces acting on all Entities from all other Entities indexed in the tree.
    fn calculate_forces(&self, out_forces: &mut Vec<DVec2>, entities: &[T], dt: f64, theta: f64) {
        // Collect all external Node indices.
        let mut external_indices: Vec<usize> = Vec::new();
        self.collect_external_nodes(&mut external_indices);

        // For each external Node, calculate the force that acts on the Entity it contains.
        for index in external_indices {
            self.calculate_force_on_entity(out_forces, index, entities, dt, theta);
        }
    }

    /// Recursively collect all external Nodes (Nodes with an index and no children).
    fn collect_external_nodes(&self, out: &mut Vec<usize>) {
        if self.is_external() {
            out.push(self.index.expect(EXTERN_NODE_EXP_MSG));
        }
        else {
            for child in self.children.iter().flatten() {
                child.collect_external_nodes(out);
            }
        }
    }

    /// Given some target index (that represents a Entity), calculate all the total force acting on it.
    fn calculate_force_on_entity(
        &self, out_forces: &mut Vec<DVec2>, 
        external_index: usize, 
        entities: &[T], 
        dt: f64,
        theta: f64
    ) {
        // This is an external Node (tt contains an index)
        if self.is_external() {

            // Since this Node is external, it MUST have an index associated with it.
            let self_index = self.index.expect(EXTERN_NODE_EXP_MSG);

            // Make sure we aren't trying to calculate the force from an Entity to itself.
            if !(self_index == external_index) {
                // Calculate the direct force between self and target Entities.
                let force = functions::calculate_gravity_between_bodies::<T>(&entities[external_index], &entities[self_index]);
                out_forces[external_index] += force;
            }
            return;
        }

        // This is an internal Node.

        // Get the distance between the target Entity and this center of mass.
        let distance = entities[external_index].position().distance(self.center_of_mass);
    
        // Calculate (s/d) < theta
        if (self.quadrant.width() / distance) < theta {
            let force = functions::calculate_gravity_from_point::<T>(&entities[external_index], self.total_mass, self.center_of_mass);
            out_forces[external_index] += force;
        }
        else {
            for child in self.children.iter().flatten() {
                child.calculate_force_on_entity(out_forces, external_index, entities, dt, theta);
            }
        }
    }

    /// Display the contents of all Nodes, recursively.
    fn print_node(&self, depth: usize, entities: &[T]) {
        let indent = "  ".repeat(depth);
        println!("{}Node: mass={:.2}, com={:.2?}", indent, self.total_mass, self.center_of_mass);
        
        if let Some(index) = self.index {
            println!("{}  Body: pos={:.2?}", indent, entities[index].position());
        }
        
        for (i, child) in self.children.iter().enumerate() {
            if let Some(child_node) = child {
                println!("{}  Child {}:", indent, i);
                child_node.print_node(depth + 1, entities);
            }
        }
    }
}

/// Structure defining a Barnes-Hut quad-tree.
/// More on these tree types for physical body simulation: https://arborjs.org/docs/barnes-hut
pub struct BHTree<'tree, T: HasPositionAndMassTrait> {
    root: Node<T>,
    entities: &'tree [T]
}

impl<'tree, T: HasPositionAndMassTrait> BHTree<'tree, T> {
    /// Return a new BHTree instance.
    pub fn new(entities: &'tree [T], dimensions: (DVec2, DVec2)) -> Self {
        // Calculate the Quadrant to be managed by the root Node.
        let quadrant = Quadrant::new(dimensions.0, dimensions.1);
        let mut root = Node::new(None, quadrant, 0.0, DVec2::new(0.0, 0.0));

        // For each Entity in the bodies list, add these to the Nodes.
        for (i, _) in entities.iter().enumerate() {
            root.insert(i, &entities, 0);
        }

        // Return an instance to a new BHTree that owns the root Node and Body list.
        BHTree { root, entities }
    }

    /// Calculate all forces acting on the Entities indexed by this tree.
    pub fn calculate_forces(&mut self, dt: f64, theta: f64) -> Vec<DVec2> {
        let mut forces = vec![DVec2::ZERO; self.entities.len()];
        self.root.calculate_forces(&mut forces, &self.entities, dt, theta);
        forces
    }

    /// Display the tree contents in the terminal.
    /// (Useful for debugging the tree)
    pub fn display(&self) {
        self.root.print_node(0, &self.entities);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::body::{BodyBuilder, functions};

    #[test]
    fn test_tree_build_and_display() {
        // Initialize some bodies.
        let bodies = vec![
            BodyBuilder::new().position_from_point(10.0, 10.0).mass(100.0).build(),
            BodyBuilder::new().position_from_point(129.0, 10.0).mass(200.0).build(),
            BodyBuilder::new().position_from_point(192.0, 64.0).mass(300.0).build()
        ];

        let dimensions = (DVec2::new(0.0, 0.0), DVec2::new(256.0, 256.0));

        let tree = BHTree::new(&bodies, dimensions);
        tree.display();

        assert!(true);
    }

    #[test]
    fn test_force_calculation() {

        // Initialize some bodies.
        let mut bodies = vec![
            BodyBuilder::new().position_from_point(10.0, 10.0).mass(1e14).build(),
            BodyBuilder::new().position_from_point(129.0, 10.0).mass(2e14).build(),
            BodyBuilder::new().position_from_point(192.0, 64.0).mass(3e14).build()
        ];

        let dimensions = (DVec2::new(0.0, 0.0), DVec2::new(256.0, 256.0));

        const dt: f64 = 0.1;
        const theta: f64 = 0.5;

        for i in 0..100 {
            let mut tree = BHTree::new(&bodies, dimensions);
            // tree.display();

            let forces = tree.calculate_forces(dt, theta);

            for (i, body) in &mut bodies.iter_mut().enumerate() {
                functions::apply_force_to_body(body, forces[i], dt);
            }
        }
        
        assert!(true);
    }
}
