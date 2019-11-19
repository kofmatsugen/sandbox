use crate::playfab::config::PlayFab;
use amethyst::{
    ecs::{Builder, Entity, WorldExt, WriteStorage},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans},
};
use amethyst_playfab::components::PlayFabApi;
use std::marker::PhantomData;

type LoginReqest = playfab_api::client::login::with_custom_id::Request<PlayFab>;

#[derive(Default)]
pub struct PlayFabCheck<N> {
    api_caller: Option<Entity>,
    _next_state: PhantomData<N>,
}

impl<N> SimpleState for PlayFabCheck<N>
where
    N: 'static + SimpleState + Default,
{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let login = LoginReqest::new(false, "RustReqwest");
        let mut component = PlayFabApi::<LoginReqest, _>::new();
        let _ = component.request_post(login, ()).unwrap();
        self.api_caller = data.world.create_entity().with(component).build().into();
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(_) = self.api_caller.and_then(|e| {
            data.world
                .exec(|mut api: WriteStorage<PlayFabApi<LoginReqest, _>>| {
                    api.get_mut(e).and_then(|api| match api.take_response() {
                        Ok(response) => Some(response),
                        Err(_) => None,
                    })
                })
        }) {
            if let Some(api_caller) = self.api_caller.take() {
                let _ = data.world.delete_entity(api_caller);
            }
            Trans::Push(Box::new(N::default()))
        } else {
            Trans::None
        }
    }
}
