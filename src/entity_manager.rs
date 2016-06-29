pub use std::collections::HashMap;
pub use uuid::Uuid;
pub type Entity = Uuid;

#[macro_export]
macro_rules! entity_manager {
    ( $($x:ident:$t:ty),* ) => {
        pub struct EntityManager {
        $(
            pub $x: HashMap<Entity, $t>,
        )*
        }

        #[allow(dead_code)]
        impl EntityManager {
            pub fn new() -> Self {
                EntityManager {
                    $(
                        $x: HashMap::new(),
                    )*
                }
            }

            pub fn create_entity(&self) -> Entity {
                Uuid::new_v4()
            }

            pub fn add_component<T: EntityComponent + Sized>(&mut self, entity: Entity, component: T) {
                component.insert(self, entity);
            }

            pub fn remove_component<T: EntityComponent + Sized>(&mut self, entity: Entity, component: T) {
                component.remove(self, entity);
            }

            pub fn remove_entity(&mut self, entity: Entity) {
                $(
                    self.$x.remove(&entity);
                )*
            }
        }

        pub trait EntityComponent: Sized {
            fn insert(self, em: &mut EntityManager, entity: Entity);
            fn remove(self, em: &mut EntityManager, entity: Entity);
        }

        $(
            impl EntityComponent for $t {
                fn insert(self, em: &mut EntityManager, entity: Entity) {
                    &em.$x.insert(entity, self);
                }
                fn remove(self, em: &mut EntityManager, entity: Entity) {
                    &em.$x.remove(&entity);
                }
            }
        )*

    };
}