use builder_pattern::Builder;
use std::marker::PhantomData;

#[derive(Builder)]
pub struct ClosureFold<
    K,
    V,
    R,
    FAdd,
    FRemove,
    FUpdate = for<'a> fn(R, &'a K, &'a V, &'a V) -> R,
    FInitial = for<'a> fn(R, &::im_rc::OrdMap<K, V>) -> R,
> where
    FAdd: for<'a> FnMut(R, &'a K, &'a V) -> R + 'static,
    FRemove: for<'a> FnMut(R, &'a K, &'a V) -> R + 'static,
    FUpdate: for<'a> FnMut(R, &'a K, &'a V, &'a V) -> R + 'static,
    FInitial: FnMut(R, &::im_rc::OrdMap<K, V>) -> R + 'static,
{
    pub add: FAdd,
    pub remove: FRemove,
    #[default(None)]
    pub update: Option<FUpdate>,
    #[default(None)]
    pub specialized_initial: Option<FInitial>,
    #[default(false)]
    pub revert_to_init_when_empty: bool,
    #[default(PhantomData)]
    #[hidden]
    pub phantom: PhantomData<(K, V, R)>,
}

fn main() {
    ClosureFold::new().add(|r, k, v| r).remove(|r, k, v| r);
}
