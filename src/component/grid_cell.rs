use std::marker::PhantomData;

use bevy::{ecs::component::Component, math::UVec2};

#[derive(Component, Debug, Default)]
pub struct GridCell<Marker: Component, const N: usize = 4> {
    pub inner: UVec2,
    marker: PhantomData<Marker>,
}

impl<Marker: Component, const N: usize> GridCell<Marker, N> {
    pub(crate) fn new(inner: UVec2) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }
}

impl<Marker: Component, const N: usize> std::ops::Deref for GridCell<Marker, N> {
    type Target = UVec2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
