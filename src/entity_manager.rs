#[macro_export]
macro_rules! entity_manager {
    ($entity:ty, {$($field:ident:$component:ty),*} ) => {
        pub type Entity = $entity;
        pub struct EntityManager {
        $(
            pub $field: HashMap<Entity, $component>,
        )*
        }

        #[allow(dead_code)]
        impl EntityManager {
            pub fn new() -> Self {
                EntityManager {
                    $(
                        $field: HashMap::new(),
                    )*
                }
            }

            pub fn create_entity(&self) -> Entity {
                Entity::new_v4()
            }

            pub fn add_component<T: EntityComponent + Sized>(&mut self, entity: Entity, component: T) {
                component.insert(self, entity);
            }

            pub fn remove_component<T: EntityComponent + Sized>(&mut self, entity: Entity, component: T) {
                component.remove(self, entity);
            }

            pub fn remove_entity(&mut self, entity: Entity) {
                $(
                    self.$field.remove(&entity);
                )*
            }
        }

        pub trait EntityComponent: Sized {
            fn insert(self, em: &mut EntityManager, entity: Entity);
            fn remove(self, em: &mut EntityManager, entity: Entity);
        }

        $(
            impl EntityComponent for $component {
                fn insert(self, em: &mut EntityManager, entity: Entity) {
                    &em.$field.insert(entity, self);
                }
                fn remove(self, em: &mut EntityManager, entity: Entity) {
                    &em.$field.remove(&entity);
                }
            }
        )*

    };
}