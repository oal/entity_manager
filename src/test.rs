use uuid::Uuid;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Position(f64, f64);
#[derive(Copy, Clone)]
pub struct Physics;

#[test]
fn simple_entity_manager() {
    entity_manager!(Uuid, {
        position: Position,
        physics: Physics
    });

    let mut em = EntityManager::new();
    let entity = em.create_entity();
    let pos_component = Position(100.0, 100.0);
    em.add_component(entity, pos_component);
    em.add_component(entity, Physics {});

    assert!(em.position.len() == 1);
    assert!(em.physics.len() == 1);

    em.remove_component(entity, pos_component);
    assert!(em.position.len() == 0);

    em.remove_entity(entity);
    assert!(em.physics.len() == 0);
}

#[test]
fn old_insert_entity_manager() {
    entity_manager!(Uuid, {
        position: Position,
        physics: Physics
    });

    let mut em = EntityManager::new();
    let entity = em.create_entity();
    let pos_component = Position(100.0, 100.0);
    em.position.insert(entity, pos_component);
    em.physics.insert(entity, Physics {});

    assert!(em.position.len() == 1);
    assert!(em.physics.len() == 1);

    em.remove_component(entity, pos_component);
    assert!(em.position.len() == 0);

    em.remove_entity(entity);
    assert!(em.physics.len() == 0);
}