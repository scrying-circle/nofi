// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.3.0.

#![allow(
    non_camel_case_types,
    unused,
    non_snake_case,
    clippy::needless_return,
    clippy::redundant_closure_call,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::unused_unit,
    clippy::double_parens,
    clippy::let_and_return,
    clippy::too_many_arguments,
    clippy::match_single_binding,
    clippy::clone_on_copy,
    clippy::let_unit_value,
    clippy::deref_addrof,
    clippy::explicit_auto_deref,
    clippy::borrow_deref_ref,
    clippy::needless_borrow
)]

// Section: imports

use crate::nofi::eval::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::{transform_result_dco, Lifetimeable, Lockable};
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate!(
    default_stream_sink_codec = SseCodec,
    default_rust_opaque = RustOpaqueMoi,
    default_rust_auto_opaque = RustAutoOpaqueMoi,
);
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "2.3.0";
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = -29392853;

// Section: executor

flutter_rust_bridge::frb_generated_default_handler!();

// Section: wire_funcs

fn wire__crate__nofi__eval__RustApplication_autocomplete_expression_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "RustApplication_autocomplete_expression",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>,
            >>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, false,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref()),
                            _ => unreachable!(),
                        }
                    }
                    let api_that_guard = api_that_guard.unwrap();
                    let output_ok = Result::<_, ()>::Ok(
                        crate::nofi::eval::RustApplication::autocomplete_expression(
                            &*api_that_guard,
                        ),
                    )?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__nofi__eval__RustApplication_copy_eval_tree_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "RustApplication_copy_eval_tree",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>,
            >>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, false,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref()),
                            _ => unreachable!(),
                        }
                    }
                    let api_that_guard = api_that_guard.unwrap();
                    let output_ok = Result::<_, ()>::Ok({
                        crate::nofi::eval::RustApplication::copy_eval_tree(&*api_that_guard);
                    })?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__nofi__eval__RustApplication_copy_image_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "RustApplication_copy_image",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>,
            >>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, false,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref()),
                            _ => unreachable!(),
                        }
                    }
                    let api_that_guard = api_that_guard.unwrap();
                    let output_ok = Result::<_, ()>::Ok({
                        crate::nofi::eval::RustApplication::copy_image(&*api_that_guard);
                    })?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__nofi__eval__RustApplication_evaluate_expression_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "RustApplication_evaluate_expression",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>,
            >>::sse_decode(&mut deserializer);
            let api_expression = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, true,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref_mut()),
                            _ => unreachable!(),
                        }
                    }
                    let mut api_that_guard = api_that_guard.unwrap();
                    let output_ok = Result::<_, ()>::Ok(
                        crate::nofi::eval::RustApplication::evaluate_expression(
                            &mut *api_that_guard,
                            &api_expression,
                        ),
                    )?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__nofi__eval__RustApplication_fetch_eval_tree_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "RustApplication_fetch_eval_tree",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>,
            >>::sse_decode(&mut deserializer);
            let api_ansi = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, false,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref()),
                            _ => unreachable!(),
                        }
                    }
                    let api_that_guard = api_that_guard.unwrap();
                    let output_ok =
                        Result::<_, ()>::Ok(crate::nofi::eval::RustApplication::fetch_eval_tree(
                            &*api_that_guard,
                            &api_ansi,
                        ))?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__nofi__eval__RustApplication_get_number_of_suggestions_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "RustApplication_get_number_of_suggestions",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>,
            >>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, false,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref()),
                            _ => unreachable!(),
                        }
                    }
                    let api_that_guard = api_that_guard.unwrap();
                    let output_ok = Result::<_, ()>::Ok(
                        crate::nofi::eval::RustApplication::get_number_of_suggestions(
                            &*api_that_guard,
                        ),
                    )?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__nofi__eval__RustApplication_new_impl(
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync::<flutter_rust_bridge::for_generated::SseCodec, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "RustApplication_new",
            port: None,
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Sync,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            deserializer.end();
            transform_result_sse::<_, ()>((move || {
                let output_ok = Result::<_, ()>::Ok(crate::nofi::eval::RustApplication::new())?;
                Ok(output_ok)
            })())
        },
    )
}
fn wire__crate__nofi__eval__spell_type_bg_path_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "spell_type_bg_path",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <crate::nofi::eval::SpellType>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Result::<_, ()>::Ok({
                        crate::nofi::eval::SpellType::bg_path(&api_that);
                    })?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__nofi__eval__spell_type_from_int_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "spell_type_from_int",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_i = <usize>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok =
                        Result::<_, ()>::Ok(crate::nofi::eval::SpellType::from_int(api_i))?;
                    Ok(output_ok)
                })())
            }
        },
    )
}

// Section: related_funcs

flutter_rust_bridge::frb_generated_moi_arc_impl_value!(
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>
);

// Section: dart2rust

impl SseDecode for RustApplication {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <RustOpaqueMoi<
            flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>,
        >>::sse_decode(deserializer);
        return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(inner);
    }
}

impl SseDecode
    for RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>>
{
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <usize>::sse_decode(deserializer);
        return decode_rust_opaque_moi(inner);
    }
}

impl SseDecode for String {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<u8>>::sse_decode(deserializer);
        return String::from_utf8(inner).unwrap();
    }
}

impl SseDecode for i32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for Vec<String> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<String>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Vec<u8> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<u8>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for (Vec<u8>, u32) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <Vec<u8>>::sse_decode(deserializer);
        let mut var_field1 = <u32>::sse_decode(deserializer);
        return (var_field0, var_field1);
    }
}

impl SseDecode for (String, Vec<String>) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <String>::sse_decode(deserializer);
        let mut var_field1 = <Vec<String>>::sse_decode(deserializer);
        return (var_field0, var_field1);
    }
}

impl SseDecode for crate::nofi::eval::SpellType {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => crate::nofi::eval::SpellType::Projectile,
            1 => crate::nofi::eval::SpellType::Modifier,
            2 => crate::nofi::eval::SpellType::StaticProjectile,
            3 => crate::nofi::eval::SpellType::Material,
            4 => crate::nofi::eval::SpellType::Utility,
            5 => crate::nofi::eval::SpellType::Other,
            6 => crate::nofi::eval::SpellType::Passive,
            7 => crate::nofi::eval::SpellType::Multicast,
            _ => unreachable!("Invalid variant for SpellType: {}", inner),
        };
    }
}

impl SseDecode for u32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for u8 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap()
    }
}

impl SseDecode for () {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {}
}

impl SseDecode for usize {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u64::<NativeEndian>().unwrap() as _
    }
}

impl SseDecode for bool {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap() != 0
    }
}

fn pde_ffi_dispatcher_primary_impl(
    func_id: i32,
    port: flutter_rust_bridge::for_generated::MessagePort,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) {
    // Codec=Pde (Serialization + dispatch), see doc to use other codecs
    match func_id {
        1 => wire__crate__nofi__eval__RustApplication_autocomplete_expression_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        2 => wire__crate__nofi__eval__RustApplication_copy_eval_tree_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        3 => wire__crate__nofi__eval__RustApplication_copy_image_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        4 => wire__crate__nofi__eval__RustApplication_evaluate_expression_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        5 => wire__crate__nofi__eval__RustApplication_fetch_eval_tree_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        6 => wire__crate__nofi__eval__RustApplication_get_number_of_suggestions_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        8 => wire__crate__nofi__eval__spell_type_bg_path_impl(port, ptr, rust_vec_len, data_len),
        9 => wire__crate__nofi__eval__spell_type_from_int_impl(port, ptr, rust_vec_len, data_len),
        _ => unreachable!(),
    }
}

fn pde_ffi_dispatcher_sync_impl(
    func_id: i32,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
    // Codec=Pde (Serialization + dispatch), see doc to use other codecs
    match func_id {
        7 => wire__crate__nofi__eval__RustApplication_new_impl(ptr, rust_vec_len, data_len),
        _ => unreachable!(),
    }
}

// Section: rust2dart

// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for FrbWrapper<RustApplication> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, MoiArc<_>>(self.0)
            .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for FrbWrapper<RustApplication> {}

impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<RustApplication>> for RustApplication {
    fn into_into_dart(self) -> FrbWrapper<RustApplication> {
        self.into()
    }
}

// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::nofi::eval::SpellType {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self {
            Self::Projectile => 0.into_dart(),
            Self::Modifier => 1.into_dart(),
            Self::StaticProjectile => 2.into_dart(),
            Self::Material => 3.into_dart(),
            Self::Utility => 4.into_dart(),
            Self::Other => 5.into_dart(),
            Self::Passive => 6.into_dart(),
            Self::Multicast => 7.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for crate::nofi::eval::SpellType {}
impl flutter_rust_bridge::IntoIntoDart<crate::nofi::eval::SpellType>
    for crate::nofi::eval::SpellType
{
    fn into_into_dart(self) -> crate::nofi::eval::SpellType {
        self
    }
}

impl SseEncode for RustApplication {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>>>::sse_encode(flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, MoiArc<_>>(self), serializer);
    }
}

impl SseEncode
    for RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>>
{
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        let (ptr, size) = self.sse_encode_raw();
        <usize>::sse_encode(ptr, serializer);
        <i32>::sse_encode(size, serializer);
    }
}

impl SseEncode for String {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.into_bytes(), serializer);
    }
}

impl SseEncode for i32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for Vec<String> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <String>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<u8> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <u8>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for (Vec<u8>, u32) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.0, serializer);
        <u32>::sse_encode(self.1, serializer);
    }
}

impl SseEncode for (String, Vec<String>) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.0, serializer);
        <Vec<String>>::sse_encode(self.1, serializer);
    }
}

impl SseEncode for crate::nofi::eval::SpellType {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                crate::nofi::eval::SpellType::Projectile => 0,
                crate::nofi::eval::SpellType::Modifier => 1,
                crate::nofi::eval::SpellType::StaticProjectile => 2,
                crate::nofi::eval::SpellType::Material => 3,
                crate::nofi::eval::SpellType::Utility => 4,
                crate::nofi::eval::SpellType::Other => 5,
                crate::nofi::eval::SpellType::Passive => 6,
                crate::nofi::eval::SpellType::Multicast => 7,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for u32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for u8 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self).unwrap();
    }
}

impl SseEncode for () {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {}
}

impl SseEncode for usize {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer
            .cursor
            .write_u64::<NativeEndian>(self as _)
            .unwrap();
    }
}

impl SseEncode for bool {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self as _).unwrap();
    }
}

#[cfg(not(target_family = "wasm"))]
mod io {
    // This file is automatically generated, so please do not edit it.
    // Generated by `flutter_rust_bridge`@ 2.3.0.

    // Section: imports

    use super::*;
    use crate::nofi::eval::*;
    use flutter_rust_bridge::for_generated::byteorder::{
        NativeEndian, ReadBytesExt, WriteBytesExt,
    };
    use flutter_rust_bridge::for_generated::{transform_result_dco, Lifetimeable, Lockable};
    use flutter_rust_bridge::{Handler, IntoIntoDart};

    // Section: boilerplate

    flutter_rust_bridge::frb_generated_boilerplate_io!();

    #[no_mangle]
    pub extern "C" fn frbgen_nofi2_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustApplication(
        ptr: *const std::ffi::c_void,
    ) {
        MoiArc::<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>>::increment_strong_count(ptr as _);
    }

    #[no_mangle]
    pub extern "C" fn frbgen_nofi2_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustApplication(
        ptr: *const std::ffi::c_void,
    ) {
        MoiArc::<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>>::decrement_strong_count(ptr as _);
    }
}
#[cfg(not(target_family = "wasm"))]
pub use io::*;

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
mod web {
    // This file is automatically generated, so please do not edit it.
    // Generated by `flutter_rust_bridge`@ 2.3.0.

    // Section: imports

    use super::*;
    use crate::nofi::eval::*;
    use flutter_rust_bridge::for_generated::byteorder::{
        NativeEndian, ReadBytesExt, WriteBytesExt,
    };
    use flutter_rust_bridge::for_generated::wasm_bindgen;
    use flutter_rust_bridge::for_generated::wasm_bindgen::prelude::*;
    use flutter_rust_bridge::for_generated::{transform_result_dco, Lifetimeable, Lockable};
    use flutter_rust_bridge::{Handler, IntoIntoDart};

    // Section: boilerplate

    flutter_rust_bridge::frb_generated_boilerplate_web!();

    #[wasm_bindgen]
    pub fn rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustApplication(
        ptr: *const std::ffi::c_void,
    ) {
        MoiArc::<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>>::increment_strong_count(ptr as _);
    }

    #[wasm_bindgen]
    pub fn rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustApplication(
        ptr: *const std::ffi::c_void,
    ) {
        MoiArc::<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>>::decrement_strong_count(ptr as _);
    }
}
#[cfg(target_family = "wasm")]
pub use web::*;
