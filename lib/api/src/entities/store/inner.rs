use crate::{
    entities::{
        engine::{AsEngineRef, Engine},
        store::{StoreMut, StoreObjects},
    },
    macros::rt::{gen_rt_ty, match_rt},
    AsStoreMut,
};

#[cfg(feature = "sys")]
use wasmer_vm::TrapHandlerFn;

/// We require the context to have a fixed memory address for its lifetime since
/// various bits of the VM have raw pointers that point back to it. Hence we
/// wrap the actual context in a box.
pub(crate) struct StoreInner {
    pub(crate) objects: StoreObjects,
    pub(crate) store: RuntimeStore,
    pub(crate) on_called: Option<OnCalledHandler>,
}

unsafe impl Send for StoreInner {}
unsafe impl Sync for StoreInner {}

impl std::fmt::Debug for StoreInner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StoreInner")
            .field("objects", &self.objects)
            .field("store", &self.store)
            .field("on_called", &"<...>")
            .finish()
    }
}

/// Call handler for a store.
// TODO: better documentation!
pub type OnCalledHandler = Box<
    dyn FnOnce(
        StoreMut<'_>,
    )
        -> Result<wasmer_types::OnCalledAction, Box<dyn std::error::Error + Send + Sync>>,
>;

gen_rt_ty!(Store @derives derive_more::From, Debug; @path store);

impl RuntimeStore {
    pub(crate) fn engine(&self) -> &Engine {
        match_rt!(on self => s {
            s.engine()
        })
    }

    pub(crate) fn engine_mut(&mut self) -> &mut Engine {
        match_rt!(on self => s {
            s.engine_mut()
        })
    }
}

impl AsEngineRef for RuntimeStore {
    fn as_engine_ref(&self) -> crate::EngineRef<'_> {
        match_rt!(on self => s {
            s.as_engine_ref()
        })
    }
}
