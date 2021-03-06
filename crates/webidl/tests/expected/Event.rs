#[allow(bad_style)]
pub struct Event {
    obj: ::wasm_bindgen::JsValue,
}
impl ::wasm_bindgen::describe::WasmDescribe for Event {
    fn describe() {
        ::wasm_bindgen::JsValue::describe();
    }
}
impl ::wasm_bindgen::convert::IntoWasmAbi for Event {
    type Abi = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn into_abi(self, extra: &mut ::wasm_bindgen::convert::Stack) -> Self::Abi {
        self.obj.into_abi(extra)
    }
}
impl ::wasm_bindgen::convert::FromWasmAbi for Event {
    type Abi = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi, extra: &mut ::wasm_bindgen::convert::Stack) -> Self {
        Event {
            obj: ::wasm_bindgen::JsValue::from_abi(js, extra),
        }
    }
}
impl<'a> ::wasm_bindgen::convert::IntoWasmAbi for &'a Event {
    type Abi = <&'a ::wasm_bindgen::JsValue as ::wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn into_abi(self, extra: &mut ::wasm_bindgen::convert::Stack) -> Self::Abi {
        (&self.obj).into_abi(extra)
    }
}
impl ::wasm_bindgen::convert::RefFromWasmAbi for Event {
    type Abi = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::RefFromWasmAbi>::Abi;
    type Anchor = ::wasm_bindgen::__rt::core::mem::ManuallyDrop<Event>;
    unsafe fn ref_from_abi(
        js: Self::Abi,
        extra: &mut ::wasm_bindgen::convert::Stack,
    ) -> Self::Anchor {
        let tmp =
            <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                js, extra,
            );
        ::wasm_bindgen::__rt::core::mem::ManuallyDrop::new(Event {
            obj: ::wasm_bindgen::__rt::core::mem::ManuallyDrop::into_inner(tmp),
        })
    }
}
impl From<::wasm_bindgen::JsValue> for Event {
    fn from(obj: ::wasm_bindgen::JsValue) -> Event {
        Event { obj }
    }
}
impl From<Event> for ::wasm_bindgen::JsValue {
    fn from(obj: Event) -> ::wasm_bindgen::JsValue {
        obj.obj
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __wbindgen_describe___wbg_f_stopPropagation_stopPropagation_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(0);
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn stopPropagation(&self) {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __wbg_f_stopPropagation_stopPropagation_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __wbg_f_stopPropagation_stopPropagation_Event(self_)
            };
            ()
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn stopPropagation(&self) {
        panic!(
            "cannot call wasm-bindgen imported functions on \
             non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __wbindgen_describe___wbg_f_stopImmediatePropagation_stopImmediatePropagation_Event(
) {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(0);
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn stopImmediatePropagation(&self) {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __wbg_f_stopImmediatePropagation_stopImmediatePropagation_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __wbg_f_stopImmediatePropagation_stopImmediatePropagation_Event(self_)
            };
            ()
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn stopImmediatePropagation(&self) {
        panic!(
            "cannot call wasm-bindgen imported functions on \
             non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __wbindgen_describe___wbg_f_preventDefault_preventDefault_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    inform(0);
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn preventDefault(&self) {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __wbg_f_preventDefault_preventDefault_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                __wbg_f_preventDefault_preventDefault_Event(self_)
            };
            ()
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn preventDefault(&self) {
        panic!(
            "cannot call wasm-bindgen imported functions on \
             non-wasm targets"
        );
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __wbindgen_describe___wbg_f_initEvent_initEvent_Event() {
    use wasm_bindgen::describe::*;
    inform(FUNCTION);
    inform(4u32);
    <&Event as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    inform(0);
}
impl Event {
    #[allow(bad_style)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub extern "C" fn initEvent(&self, type_: &str, bubbles: bool, cancelable: bool) {
        ::wasm_bindgen::__rt::link_this_library();
        #[wasm_import_module = "__wbindgen_placeholder__"]
        extern "C" {
            fn __wbg_f_initEvent_initEvent_Event(
                self_: <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as ::wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        unsafe {
            let _ret = {
                let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                let self_ =
                    <&Event as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self, &mut __stack);
                let type_ =
                    <&str as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_, &mut __stack);
                let bubbles =
                    <bool as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles, &mut __stack);
                let cancelable = <bool as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                    cancelable,
                    &mut __stack,
                );
                __wbg_f_initEvent_initEvent_Event(self_, type_, bubbles, cancelable)
            };
            ()
        }
    }
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    pub extern "C" fn initEvent(&self, type_: &str, bubbles: bool, cancelable: bool) {
        panic!(
            "cannot call wasm-bindgen imported functions on \
             non-wasm targets"
        );
    }
}
#[allow(non_upper_case_globals)]
#[wasm_custom_section = "__wasm_bindgen_unstable"]
const __WASM_BINDGEN_GENERATED_wasm_bindgen_webidl_0_1_0_0 : [ u8 ; 180usize ] = * b"\xB0\0\0\0{\"exports\":[],\"enums\":[],\"imports\":[{\"module\":null,\"version\":null,\"js_namespace\":null,\"kind\":{\"kind\":\"type\"}}],\"structs\":[],\"version\":\"0.2.11 (3879f6f42)\",\"schema_version\":\"4\"}" ;
