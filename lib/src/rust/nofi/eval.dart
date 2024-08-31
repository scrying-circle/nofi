// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.3.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `encode_png`, `get_absolute_path_from_relative`, `get_default`
// These types are ignored because they are not used by any `pub` functions: `Args`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `augment_args_for_update`, `augment_args`, `clone`, `command_for_update`, `command`, `fmt`, `from_arg_matches_mut`, `from_arg_matches`, `group_id`, `update_from_arg_matches_mut`, `update_from_arg_matches`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<RustApplication>>
abstract class RustApplication implements RustOpaqueInterface {
  Future<(String, List<String>)> autocompleteExpression();

  Future<void> copyEvalTree();

  Future<void> copyImage();

  Future<(Uint8List, int)> evaluateExpression({required String expression});

  Future<String> fetchEvalTree({required String ansi});

  Future<int> getNumberOfSuggestions();

  factory RustApplication() =>
      RustLib.instance.api.crateNofiEvalRustApplicationNew();
}
