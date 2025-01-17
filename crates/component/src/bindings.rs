#[allow(dead_code)]
pub mod smndtrl {
    #[allow(dead_code)]
    pub mod experiments {
        #[allow(dead_code, clippy::all)]
        pub mod data {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Counter {
                handle: _rt::Resource<Counter>,
            }
            impl Counter {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Counter {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "smndtrl:experiments/data")]
                        extern "C" {
                            #[link_name = "[resource-drop]counter"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Clone)]
            pub enum Error {
                Generic,
                Other(_rt::String),
            }
            impl ::core::fmt::Debug for Error {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Error::Generic => f.debug_tuple("Error::Generic").finish(),
                        Error::Other(e) => {
                            f.debug_tuple("Error::Other").field(e).finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for Error {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for Error {}
            impl Counter {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(value: u8) -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "smndtrl:experiments/data")]
                        extern "C" {
                            #[link_name = "[constructor]counter"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(_rt::as_i32(&value));
                        Counter::from_handle(ret as u32)
                    }
                }
            }
            impl Counter {
                #[allow(unused_unsafe, clippy::all)]
                /// add: func(amount: u8) -> result<counter, error>;
                /// dec: func(amount: u8) -> result<counter, error>;
                pub fn add(&self, amount: u8) -> u8 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "smndtrl:experiments/data")]
                        extern "C" {
                            #[link_name = "[method]counter.add"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&amount),
                        );
                        ret as u8
                    }
                }
            }
            impl Counter {
                #[allow(unused_unsafe, clippy::all)]
                pub fn dec(&self, amount: u8) -> u8 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "smndtrl:experiments/data")]
                        extern "C" {
                            #[link_name = "[method]counter.dec"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&amount),
                        );
                        ret as u8
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod smndtrl {
        #[allow(dead_code)]
        pub mod experiments {
            #[allow(dead_code, clippy::all)]
            pub mod component {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Counter = super::super::super::super::smndtrl::experiments::data::Counter;
                pub type Error = super::super::super::super::smndtrl::experiments::data::Error;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_modify_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::modify();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr1.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            use super::super::super::super::smndtrl::experiments::data::Error as V3;
                            match e {
                                V3::Generic => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                }
                                V3::Other(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec2 = (e.into_bytes()).into_boxed_slice();
                                    let ptr2 = vec2.as_ptr().cast::<u8>();
                                    let len2 = vec2.len();
                                    ::core::mem::forget(vec2);
                                    *ptr1.add(12).cast::<usize>() = len2;
                                    *ptr1.add(8).cast::<*mut u8>() = ptr2.cast_mut();
                                }
                            }
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_modify<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = i32::from(*arg0.add(4).cast::<u8>());
                            match l1 {
                                0 => {}
                                _ => {
                                    let l2 = *arg0.add(8).cast::<*mut u8>();
                                    let l3 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l2, l3, 1);
                                }
                            }
                        }
                    }
                }
                pub trait Guest {
                    fn modify() -> Result<Counter, Error>;
                }
                #[doc(hidden)]
                macro_rules! __export_smndtrl_experiments_component_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "smndtrl:experiments/component#modify"] unsafe extern "C" fn
                        export_modify() -> * mut u8 { $($path_to_types)*::
                        _export_modify_cabi::<$ty > () } #[export_name =
                        "cabi_post_smndtrl:experiments/component#modify"] unsafe extern
                        "C" fn _post_return_modify(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_modify::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_smndtrl_experiments_component_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::string::String;
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_counter_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::smndtrl::experiments::component::__export_smndtrl_experiments_component_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::smndtrl::experiments::component);
    };
}
#[doc(inline)]
pub(crate) use __export_counter_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:smndtrl:experiments:counter:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 469] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xd7\x02\x01A\x02\x01\
A\x06\x01B\x0a\x04\0\x07counter\x03\x01\x01q\x02\x07generic\0\0\x05other\x01s\0\x04\
\0\x05error\x03\0\x01\x01i\0\x01@\x01\x05value}\0\x03\x04\0\x14[constructor]coun\
ter\x01\x04\x01h\0\x01@\x02\x04self\x05\x06amount}\0}\x04\0\x13[method]counter.a\
dd\x01\x06\x04\0\x13[method]counter.dec\x01\x06\x03\x01\x18smndtrl:experiments/d\
ata\x05\0\x02\x03\0\0\x07counter\x02\x03\0\0\x05error\x01B\x08\x02\x03\x02\x01\x01\
\x04\0\x07counter\x03\0\0\x02\x03\x02\x01\x02\x04\0\x05error\x03\0\x02\x01i\x01\x01\
j\x01\x04\x01\x03\x01@\0\0\x05\x04\0\x06modify\x01\x06\x04\x01\x1dsmndtrl:experi\
ments/component\x05\x03\x04\x01\x1bsmndtrl:experiments/counter\x04\0\x0b\x0d\x01\
\0\x07counter\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x07\
0.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
