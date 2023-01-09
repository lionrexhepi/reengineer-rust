use std::{collections::HashMap, hash::Hash};

pub enum Block {
    Air(AirState),
}

type StateMap<T> = HashMap<HashMap<&'static str, dyn StateValue>, T>;

fn build_tree<TResult, TCreate>(
    props: HashMap<&'static str, Vec<dyn StateValue>>,
    factory: TCreate,
) -> StateMap<TResult>
where
    TCreate: Fn,
{
    let mut state_dict = StateMap::new();



    state_dict
}

trait BlockState {
    fn get_tree(&self) -> StateMap<Self> ;
}

trait StateValue: Sized {}
