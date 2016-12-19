use ::specs::Component;

pub struct ComponentRender {
    pub x: i32,
    pub y: i32,
    pub w: u8,
    pub h: u8,
}

impl Component for ComponentRender {
    // Storage is used to store all data for components of this type
    // VecStorage is meant to be used for components that are in almost every entity
    // HashMapStorage is better for componets that are met rarely
    type Storage = ::specs::VecStorage<ComponentRender>;
}