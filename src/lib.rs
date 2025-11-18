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

pub struct FunctionObserverRegistration<'a, F: Send + Sync, Marker, E: Event, B: Bundle>
where
    &'a F: IntoObserverSystem<E, B, Marker>,
{
    f: &'a F,
    _pd: PhantomData<dyn Fn((Marker, B, E)) + Send + Sync>,
}
pub trait IntoEntityObserverRegistration<R: EntityObserverRegistration> {
    fn into_registration(self) -> R;
}
pub trait IntoGlobalObserverRegistration<R: GlobalObserverRegistration> {
    fn into_registration(self) -> R;
}

impl<'a, F: Send + Sync, E: EntityEvent, B: Bundle, Marker>
    IntoEntityObserverRegistration<FunctionObserverRegistration<'a, F, Marker, E, B>> for &'a F
where
    FunctionObserverRegistration<'a, F, Marker, E, B>: EntityObserverRegistration,
    &'a F: IntoObserverSystem<E, B, Marker>,
{
    fn into_registration(self) -> FunctionObserverRegistration<'a, F, Marker, E, B> {
        FunctionObserverRegistration {
            f: self,
            _pd: Default::default(),
        }
    }
}

impl<'a, F: Send + Sync, Marker, E: Event, B: Bundle> GlobalObserverRegistration
    for FunctionObserverRegistration<'a, F, Marker, E, B>
where
    &'a F: IntoObserverSystem<E, B, Marker>,
{
    fn register_observer(&self, app: &mut App) {
        app.add_observer(self.f);
    }
}

impl<'a, F: Send + Sync, Marker, E: EntityEvent, B: Bundle> EntityObserverRegistration
    for FunctionObserverRegistration<'a, F, Marker, E, B>
where
    &'a F: IntoObserverSystem<E, B, Marker>,
{
    fn register_observer(&self, app: &mut EntityCommands) {
        app.observe(self.f);
    }
}

