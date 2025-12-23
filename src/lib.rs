use std::marker::PhantomData;

use bevy::{
    app::App,
    ecs::{
        bundle::Bundle,
        event::{EntityEvent, Event},
        system::{EntityCommands, IntoObserverSystem},
    },
};

pub trait EntityObserverRegistration: Send + Sync {
    fn register_observer(&self, entity_commands: &mut EntityCommands);
}

pub trait GlobalObserverRegistration: Send + Sync {
    fn register_observer(&self, app: &mut App);
}

pub struct FunctionObserverRegistration<F: Send + Sync, Marker, E: Event, B: Bundle>
where
    F: IntoObserverSystem<E, B, Marker> + Copy,
{
    f: F,
    _pd: PhantomData<dyn Fn((Marker, B, E)) + Send + Sync>,
}
pub trait IntoEntityObserverRegistration<R: EntityObserverRegistration> {
    fn into_registration(self) -> R;
}
pub trait IntoGlobalObserverRegistration<R: GlobalObserverRegistration> {
    fn into_registration(self) -> R;
}

impl<F: Send + Sync, E: EntityEvent, B: Bundle, Marker>
    IntoEntityObserverRegistration<FunctionObserverRegistration<F, Marker, E, B>> for F
where
    FunctionObserverRegistration<F, Marker, E, B>: EntityObserverRegistration,
    F: IntoObserverSystem<E, B, Marker> + Copy,
{
    fn into_registration(self) -> FunctionObserverRegistration<F, Marker, E, B> {
        FunctionObserverRegistration {
            f: self,
            _pd: Default::default(),
        }
    }
}

impl<F: Send + Sync, Marker, E: Event, B: Bundle> GlobalObserverRegistration
    for FunctionObserverRegistration<F, Marker, E, B>
where
    F: IntoObserverSystem<E, B, Marker> + Copy,
{
    fn register_observer(&self, app: &mut App) {
        app.add_observer(self.f);
    }
}

impl<F: Send + Sync, Marker, E: EntityEvent, B: Bundle> EntityObserverRegistration
    for FunctionObserverRegistration<F, Marker, E, B>
where
    F: IntoObserverSystem<E, B, Marker> + Copy,
{
    fn register_observer(&self, app: &mut EntityCommands) {
        app.observe(self.f);
    }
}
