//! This module defines traits to handle abstractions created by the runtimes.

mod impls;

use crate::VMExternToExtern;
use wasmer_types::RawValue;

macro_rules! define_vm_like {
    ($name: ident) => {
        paste::paste! {
        /// The enum for all those VM values of this kind.
        #[repr(C)]
        pub enum [<VM $name>] {
            #[cfg(feature = "sys")]
            Sys(crate::rt::sys::vm::[<VM $name>]),

            #[cfg(feature = "wamr")]
            Wamr(crate::rt::wamr::vm::[<VM $name>]),

            #[cfg(feature = "v8")]
            V8(crate::rt::v8::vm::[<VM $name>]),

            #[cfg(feature = "js")]
            Js(crate::rt::js::vm::[<VM $name>]),

            #[cfg(feature = "jsc")]
            Jsc(crate::rt::jsc::vm::[<VM $name>]),

        }

        impl [<VM $name>] {
            #[cfg(feature = "sys")]
            /// Consume `self` into a `sys` VM kind.
            pub fn into_sys(self) -> crate::rt::sys::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Sys(s) => s,
                    _ => panic!("Not a `sys` value!")
                }
            }

            #[cfg(feature = "sys")]
            /// Convert a reference to [`self`] into a reference to the same `sys` VM kind.
            pub fn as_sys(&self) -> &crate::rt::sys::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Sys(s) => s,
                    _ => panic!("Not a `sys` value!")
                }
            }

            #[cfg(feature = "sys")]
            /// Convert a mutable reference to [`self`] into a mutable reference to the same `sys` VM kind.
            pub fn as_sys_mut(&mut self) -> &mut crate::rt::sys::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Sys(s) => s,
                    _ => panic!("Not a `sys` value!")
                }
            }

            #[cfg(feature = "wamr")]
            /// Consume `self` into a `wamr` VM kind.
            pub fn into_wamr(self) -> crate::rt::wamr::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Wamr(s) => s,
                    _ => panic!("Not a `wamr` value!")
                }
            }

            #[cfg(feature = "wamr")]
            /// Convert a reference to [`self`] into a reference to the same `wamr` VM kind.
            pub fn as_wamr(&self) -> &crate::rt::wamr::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Wamr(s) => s,
                    _ => panic!("Not a `wamr` value!")
                }
            }

            #[cfg(feature = "wamr")]
            /// Convert a mutable reference to [`self`] into a mutable reference to the same `wamr` VM kind.
            pub fn as_wamr_mut(&mut self) -> &mut crate::rt::wamr::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Wamr(s) => s,
                    _ => panic!("Not a `wamr` value!")
                }
            }

            #[cfg(feature = "v8")]
            /// Consume `self` into a `v8` VM kind.
            pub fn into_v8(self) -> crate::rt::v8::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::V8(s) => s,
                    _ => panic!("Not a `v8` value!")
                }
            }

            #[cfg(feature = "v8")]
            /// Convert a reference to [`self`] into a reference to the same `v8` VM kind.
            pub fn as_v8(&self) -> &crate::rt::v8::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::V8(s) => s,
                    _ => panic!("Not a `v8` value!")
                }
            }

            #[cfg(feature = "v8")]
            /// Convert a mutable reference to [`self`] into a mutable reference to the same `v8` VM kind.
            pub fn as_v8_mut(&mut self) -> &mut crate::rt::v8::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::V8(s) => s,
                    _ => panic!("Not a `v8` value!")
                }
            }

            #[cfg(feature = "js")]
            /// Consume `self` into a `js` VM kind.
            pub fn into_js(self) -> crate::rt::js::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Js(s) => s,
                    _ => panic!("Not a `js` value!")
                }
            }

            #[cfg(feature = "js")]
            /// Convert a reference to [`self`] into a reference to the same `js` VM kind.
            pub fn as_js(&self) -> &crate::rt::js::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Js(s) => s,
                    _ => panic!("Not a `js` value!")
                }
            }

            #[cfg(feature = "js")]
            /// Convert a mutable reference to [`self`] into a mutable reference to the same `js` VM kind.
            pub fn as_js_mut(&mut self) -> &mut crate::rt::js::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Js(s) => s,
                    _ => panic!("Not a `js` value!")
                }
            }

            #[cfg(feature = "jsc")]
            /// Consume `self` into a `jsc` VM kind.
            pub fn into_jsc(self) -> crate::rt::jsc::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Jsc(s) => s,
                    _ => panic!("Not a `jsc` value!")
                }
            }

            #[cfg(feature = "jsc")]
            /// Convert a reference to [`self`] into a reference to the same `jsc` VM kind.
            pub fn as_jsc(&self) -> &crate::rt::jsc::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Jsc(s) => s,
                    _ => panic!("Not a `jsc` value!")
                }
            }

            #[cfg(feature = "jsc")]
            /// Convert a mutable reference to [`self`] into a mutable reference to the same `jsc` VM kind.
            pub fn as_jsc_mut(&mut self) -> &mut crate::rt::jsc::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Jsc(s) => s,
                    _ => panic!("Not a `jsc` value!")
                }
            }
        }
        }
    };

    ($name: ident $(, $derive:ident)*) => {
        paste::paste! {
        /// The enum for all those VM values of this kind.
        $(#[derive($derive)])*
        #[repr(C)]
        pub enum [<VM $name>] {
            #[cfg(feature = "sys")]
            Sys(crate::rt::sys::vm::[<VM $name>]),
            #[cfg(feature = "wamr")]
            Wamr(crate::rt::wamr::vm::[<VM $name>]),
            #[cfg(feature = "v8")]
            V8(crate::rt::v8::vm::[<VM $name>]),
            #[cfg(feature = "js")]
            Js(crate::rt::js::vm::[<VM $name>]),
            #[cfg(feature = "jsc")]
            Jsc(crate::rt::jsc::vm::[<VM $name>]),
        }

        impl [<VM $name>] {
            #[cfg(feature = "sys")]
            /// Consume `self` into a `sys` VM kind.
            pub fn into_sys(self) -> crate::rt::sys::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Sys(s) => s,
                    _ => panic!("Not a `sys` value!")
                }
            }

            #[cfg(feature = "sys")]
            /// Convert a reference to [`self`] into a reference to the same `sys` VM kind.
            pub fn as_sys(&self) -> &crate::rt::sys::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Sys(s) => s,
                    _ => panic!("Not a `sys` value!")
                }
            }

            #[cfg(feature = "sys")]
            /// Convert a mutable reference to [`self`] into a mutable reference to the same `sys` VM kind.
            pub fn as_sys_mut(&mut self) -> &mut crate::rt::sys::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Sys(s) => s,
                    _ => panic!("Not a `sys` value!")
                }
            }

            #[cfg(feature = "wamr")]
            /// Consume `self` into a `wamr` VM kind.
            pub fn into_wamr(self) -> crate::rt::wamr::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Wamr(s) => s,
                    _ => panic!("Not a `wamr` value!")
                }
            }

            #[cfg(feature = "wamr")]
            /// Convert a reference to [`self`] into a reference to the same `wamr` VM kind.
            pub fn as_wamr(&self) -> &crate::rt::wamr::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Wamr(s) => s,
                    _ => panic!("Not a `wamr` value!")
                }
            }

            #[cfg(feature = "wamr")]
            /// Convert a mutable reference to [`self`] into a mutable reference to the same `wamr` VM kind.
            pub fn as_wamr_mut(&mut self) -> &mut crate::rt::wamr::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Wamr(s) => s,
                    _ => panic!("Not a `wamr` value!")
                }
            }

            #[cfg(feature = "js")]
            /// Consume `self` into a `js` VM kind.
            pub fn into_js(self) -> crate::rt::js::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Js(s) => s,
                    _ => panic!("Not a `js` value!")
                }
            }

            #[cfg(feature = "js")]
            /// Convert a reference to [`self`] into a reference to the same `js` VM kind.
            pub fn as_js(&self) -> &crate::rt::js::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Js(s) => s,
                    _ => panic!("Not a `js` value!")
                }
            }

            #[cfg(feature = "js")]
            /// Convert a mutable reference to [`self`] into a mutable reference to the same `js` VM kind.
            pub fn as_js_mut(&mut self) -> &mut crate::rt::js::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Js(s) => s,
                    _ => panic!("Not a `js` value!")
                }
            }

            #[cfg(feature = "jsc")]
            /// Consume `self` into a `jsc` VM kind.
            pub fn into_jsc(self) -> crate::rt::jsc::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Jsc(s) => s,
                    _ => panic!("Not a `jsc` value!")
                }
            }

            #[cfg(feature = "jsc")]
            /// Convert a reference to [`self`] into a reference to the same `jsc` VM kind.
            pub fn as_jsc(&self) -> &crate::rt::jsc::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Jsc(s) => s,
                    _ => panic!("Not a `jsc` value!")
                }
            }

            #[cfg(feature = "jsc")]
            /// Convert a mutable reference to [`self`] into a mutable reference to the same `jsc` VM kind.
            pub fn as_jsc_mut(&mut self) -> &mut crate::rt::jsc::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::Jsc(s) => s,
                    _ => panic!("Not a `jsc` value!")
                }
            }


            #[cfg(feature = "v8")]
            /// Consume `self` into a `v8` VM kind.
            pub fn into_v8(self) -> crate::rt::v8::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::V8(s) => s,
                    _ => panic!("Not a `v8` value!")
                }
            }

            #[cfg(feature = "v8")]
            /// Convert a reference to [`self`] into a reference to the same `v8` VM kind.
            pub fn as_v8(&self) -> &crate::rt::v8::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::V8(s) => s,
                    _ => panic!("Not a `v8` value!")
                }
            }

            #[cfg(feature = "v8")]
            /// Convert a mutable reference to [`self`] into a mutable reference to the same `v8` VM kind.
            pub fn as_v8_mut(&mut self) -> &mut crate::rt::v8::vm::[<VM $name>] {
                match self {
                    [<VM $name>]::V8(s) => s,
                    _ => panic!("Not a `v8` value!")
                }
            }
        }
        }
    };
}

define_vm_like!(Extern);
define_vm_like!(ExternFunction);
define_vm_like!(ExternGlobal);
define_vm_like!(ExternMemory);
define_vm_like!(ExternTable);
//define_vm_like!(ExternObj, Debug);
define_vm_like!(FunctionCallback);
define_vm_like!(FunctionBody);
define_vm_like!(FunctionEnvironment, Debug);
define_vm_like!(Instance, Debug);
define_vm_like!(Trampoline);

//define_vm_like!(Config);
define_vm_like!(Function, Debug);
define_vm_like!(Global, Debug);
define_vm_like!(Memory, Debug);
define_vm_like!(SharedMemory);
define_vm_like!(Table, Debug);

define_vm_like!(ExternRef);
define_vm_like!(FuncRef);
