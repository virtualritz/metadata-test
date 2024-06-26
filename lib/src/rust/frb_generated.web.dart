// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.40.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field


// Static analysis wrongly picks the IO variant, thus ignore this
// ignore_for_file: argument_type_not_assignable

import 'api/file_info.dart';
import 'api/metadata.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'lib.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';




                abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
                  RustLibApiImplPlatform({
                    required super.handler,
                    required super.wire,
                    required super.generalizedFrbRustBinding,
                    required super.portManager,
                  });

                  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_FileFormatPtr => wire.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat;

CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_FileInfoPtr => wire.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo;

CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_MetadataPtr => wire.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata;

CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_PathBufPtr => wire.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf;



                  @protected AnyhowException dco_decode_AnyhowException(dynamic raw);

@protected FileFormat dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(dynamic raw);

@protected FileInfo dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(dynamic raw);

@protected Metadata dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(dynamic raw);

@protected PathBuf dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(dynamic raw);

@protected FileInfo dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(dynamic raw);

@protected Metadata dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(dynamic raw);

@protected DateTime dco_decode_Chrono_Utc(dynamic raw);

@protected FileFormat dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(dynamic raw);

@protected FileInfo dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(dynamic raw);

@protected Metadata dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(dynamic raw);

@protected PathBuf dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(dynamic raw);

@protected String dco_decode_String(dynamic raw);

@protected SpecificMetadata dco_decode_box_autoadd_specific_metadata(dynamic raw);

@protected double dco_decode_f_32(dynamic raw);

@protected PlatformInt64 dco_decode_i_64(dynamic raw);

@protected Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

@protected String? dco_decode_opt_String(dynamic raw);

@protected SpecificMetadata? dco_decode_opt_box_autoadd_specific_metadata(dynamic raw);

@protected SpecificMetadata dco_decode_specific_metadata(dynamic raw);

@protected int dco_decode_u_32(dynamic raw);

@protected int dco_decode_u_8(dynamic raw);

@protected void dco_decode_unit(dynamic raw);

@protected BigInt dco_decode_usize(dynamic raw);

@protected AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer);

@protected FileFormat sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(SseDeserializer deserializer);

@protected FileInfo sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(SseDeserializer deserializer);

@protected Metadata sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(SseDeserializer deserializer);

@protected PathBuf sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(SseDeserializer deserializer);

@protected FileInfo sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(SseDeserializer deserializer);

@protected Metadata sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(SseDeserializer deserializer);

@protected DateTime sse_decode_Chrono_Utc(SseDeserializer deserializer);

@protected FileFormat sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(SseDeserializer deserializer);

@protected FileInfo sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(SseDeserializer deserializer);

@protected Metadata sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(SseDeserializer deserializer);

@protected PathBuf sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(SseDeserializer deserializer);

@protected String sse_decode_String(SseDeserializer deserializer);

@protected SpecificMetadata sse_decode_box_autoadd_specific_metadata(SseDeserializer deserializer);

@protected double sse_decode_f_32(SseDeserializer deserializer);

@protected PlatformInt64 sse_decode_i_64(SseDeserializer deserializer);

@protected Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

@protected String? sse_decode_opt_String(SseDeserializer deserializer);

@protected SpecificMetadata? sse_decode_opt_box_autoadd_specific_metadata(SseDeserializer deserializer);

@protected SpecificMetadata sse_decode_specific_metadata(SseDeserializer deserializer);

@protected int sse_decode_u_32(SseDeserializer deserializer);

@protected int sse_decode_u_8(SseDeserializer deserializer);

@protected void sse_decode_unit(SseDeserializer deserializer);

@protected BigInt sse_decode_usize(SseDeserializer deserializer);

@protected int sse_decode_i_32(SseDeserializer deserializer);

@protected bool sse_decode_bool(SseDeserializer deserializer);

@protected void sse_encode_AnyhowException(AnyhowException self, SseSerializer serializer);

@protected void sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(FileFormat self, SseSerializer serializer);

@protected void sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(FileInfo self, SseSerializer serializer);

@protected void sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(Metadata self, SseSerializer serializer);

@protected void sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(PathBuf self, SseSerializer serializer);

@protected void sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(FileInfo self, SseSerializer serializer);

@protected void sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(Metadata self, SseSerializer serializer);

@protected void sse_encode_Chrono_Utc(DateTime self, SseSerializer serializer);

@protected void sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(FileFormat self, SseSerializer serializer);

@protected void sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(FileInfo self, SseSerializer serializer);

@protected void sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(Metadata self, SseSerializer serializer);

@protected void sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(PathBuf self, SseSerializer serializer);

@protected void sse_encode_String(String self, SseSerializer serializer);

@protected void sse_encode_box_autoadd_specific_metadata(SpecificMetadata self, SseSerializer serializer);

@protected void sse_encode_f_32(double self, SseSerializer serializer);

@protected void sse_encode_i_64(PlatformInt64 self, SseSerializer serializer);

@protected void sse_encode_list_prim_u_8_strict(Uint8List self, SseSerializer serializer);

@protected void sse_encode_opt_String(String? self, SseSerializer serializer);

@protected void sse_encode_opt_box_autoadd_specific_metadata(SpecificMetadata? self, SseSerializer serializer);

@protected void sse_encode_specific_metadata(SpecificMetadata self, SseSerializer serializer);

@protected void sse_encode_u_32(int self, SseSerializer serializer);

@protected void sse_encode_u_8(int self, SseSerializer serializer);

@protected void sse_encode_unit(void self, SseSerializer serializer);

@protected void sse_encode_usize(BigInt self, SseSerializer serializer);

@protected void sse_encode_i_32(int self, SseSerializer serializer);

@protected void sse_encode_bool(bool self, SseSerializer serializer);
                }
                


// Section: wire_class

class RustLibWire implements BaseWire {
            RustLibWire.fromExternalLibrary(ExternalLibrary lib);

            void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(int ptr) => wasmModule.rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(ptr);

void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(int ptr) => wasmModule.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(ptr);

void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(int ptr) => wasmModule.rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(ptr);

void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(int ptr) => wasmModule.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(ptr);

void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(int ptr) => wasmModule.rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(ptr);

void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(int ptr) => wasmModule.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(ptr);

void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(int ptr) => wasmModule.rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(ptr);

void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(int ptr) => wasmModule.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(ptr);
        }
        @JS('wasm_bindgen') external RustLibWasmModule get wasmModule;

        @JS() @anonymous extension type RustLibWasmModule._(JSObject _) implements JSObject {
            external void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(int ptr);

external void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileFormat(int ptr);

external void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(int ptr);

external void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerFileInfo(int ptr);

external void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(int ptr);

external void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerMetadata(int ptr);

external void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(int ptr);

external void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPathBuf(int ptr);
        }
        