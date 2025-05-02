use bevy_rapier3d::prelude::*;

macro_rules! group {
    ($name:ident, $index:expr) => {
        paste::paste! {
            pub const $name: Group = Group::[<GROUP_ $index>];
        }
    };
}
pub struct NamedGroup;

impl NamedGroup {
    group!(TANK, 1);
    group!(BULLET, 2);
    group!(TERRAIN, 3);
    group!(BULLET_VELOCITY, 4);

    pub fn physical() -> Group {
        NamedGroup::TANK | NamedGroup::BULLET | NamedGroup::TERRAIN
    }
}
