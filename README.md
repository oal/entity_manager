# entity_manager

A simple entity manager macro for developing entity/component systems in Rust. This crate only contains a macro
for generating an EntityManager struct, and it's up to you to define your components and systems to process
these.

### Example
```rust
// Used inside macro, so these must be imported
use uuid::Uuid;
use std::collections::HashMap;

// Define your component structures
#[derive(Copy, Clone)]
struct Position(f64, f64);
#[derive(Copy, Clone)]
struct Physics;

// Create entity manager with all your component types
entity_manager!(Uuid, {
        position: Position,
        physics: Physics
    }
);

fn update_movement(em: &mut EntityManager) {
    let position = &mut em.position;

    // Loop over all entities with the "physics" component
    for (entity, phys) in em.physics.iter() {
        // Get their "position" component.
        let pos = position.get_mut(entity).unwrap();
        
        // Update position
        *pos = Position(pos.0, pos.1 - 1.0);
    }
}

fn main() {
    let mut em = EntityManager::new();
    let entity = em.create_entity();

    let pos_component = Position(100.0, 100.0);
    em.add_component(entity, pos_component);
    em.add_component(entity, Physics {});

    // Please see src/test.rs for more examples

    loop {
        // Call all your update systems
        update_movement(&mut em);
        // ...
    }
}
```