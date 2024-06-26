// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.40.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import '../lib.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'metadata.freezed.dart';

            // These functions are ignored because they are not marked as `pub`: `extract_metadata`


            

            
                // Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Metadata>>
                abstract class Metadata implements RustOpaqueInterface {
                     DateTime  accessed();


 String?  author();


 FileFormat  format();


 DateTime  modified();


 SpecificMetadata?  specific();


 String?  title();



                    
                }
                

@freezed
                sealed class SpecificMetadata with _$SpecificMetadata  {
                    const SpecificMetadata._();

                     const factory SpecificMetadata.archive() = SpecificMetadata_Archive;
 const factory SpecificMetadata.audio() = SpecificMetadata_Audio;
 const factory SpecificMetadata.compressed() = SpecificMetadata_Compressed;
 const factory SpecificMetadata.database() = SpecificMetadata_Database;
 const factory SpecificMetadata.diagram() = SpecificMetadata_Diagram;
 const factory SpecificMetadata.disk() = SpecificMetadata_Disk;
 const factory SpecificMetadata.document() = SpecificMetadata_Document;
 const factory SpecificMetadata.ebook() = SpecificMetadata_Ebook;
 const factory SpecificMetadata.executable() = SpecificMetadata_Executable;
 const factory SpecificMetadata.font() = SpecificMetadata_Font;
 const factory SpecificMetadata.formula() = SpecificMetadata_Formula;
 const factory SpecificMetadata.geospatial() = SpecificMetadata_Geospatial;
 const factory SpecificMetadata.image({   required int width ,  required int height ,  required double pixelAspect , }) = SpecificMetadata_Image;
 const factory SpecificMetadata.metadata() = SpecificMetadata_Metadata;
 const factory SpecificMetadata.model() = SpecificMetadata_Model;
 const factory SpecificMetadata.other() = SpecificMetadata_Other;
 const factory SpecificMetadata.package() = SpecificMetadata_Package;
 const factory SpecificMetadata.playlist() = SpecificMetadata_Playlist;
 const factory SpecificMetadata.presentation() = SpecificMetadata_Presentation;
 const factory SpecificMetadata.rom() = SpecificMetadata_Rom;
 const factory SpecificMetadata.spreadsheet() = SpecificMetadata_Spreadsheet;
 const factory SpecificMetadata.subtitle() = SpecificMetadata_Subtitle;
 const factory SpecificMetadata.video({   required int width ,  required int height ,  required double pixelAspect ,  required BigInt numberOfFrames ,  required double framesPerSecond , }) = SpecificMetadata_Video;

                    
                }
            