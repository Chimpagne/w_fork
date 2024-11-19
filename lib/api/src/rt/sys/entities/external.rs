use std::any::Any;
use wasmer_vm::{StoreHandle, VMExternRef};

use crate::{
    store::{AsStoreMut, AsStoreRef},
    vm::VMExternObj,
};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ExternRef {
    handle: StoreHandle<wasmer_vm::VMExternObj>,
}

impl ExternRef {
    pub fn new<T>(store: &mut impl AsStoreMut, value: T) -> Self
    where
        T: Any + Send + Sync + 'static + Sized,
    {
        Self {
            handle: crate::rt::sys::store::StoreHandle::new(
                store.objects_mut().as_sys_mut(),
                wasmer_vm::VMExternObj::new(value),
            ),
        }
    }

    pub fn downcast<'a, T>(&self, store: &'a impl AsStoreRef) -> Option<&'a T>
    where
        T: Any + Send + Sync + 'static + Sized,
    {
        self.handle
            .get(store.as_store_ref().objects().as_sys())
            .as_ref()
            .downcast_ref::<T>()
    }

    pub(crate) fn vm_externref(&self) -> VMExternRef {
        wasmer_vm::VMExternRef(self.handle.internal_handle())
    }

    pub(crate) unsafe fn from_vm_externref(
        store: &mut impl AsStoreMut,
        vm_externref: VMExternRef,
    ) -> Self {
        Self {
            handle: StoreHandle::from_internal(store.objects_mut().id(), vm_externref.0),
        }
    }

    pub fn is_from_store(&self, store: &impl AsStoreRef) -> bool {
        self.handle.store_id() == store.as_store_ref().objects().id()
    }
}
