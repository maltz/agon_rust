// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct MetaGraphDef {
    // message fields
    pub meta_info_def: ::protobuf::SingularPtrField<MetaGraphDef_MetaInfoDef>,
    pub graph_def: ::protobuf::SingularPtrField<super::graph::GraphDef>,
    pub saver_def: ::protobuf::SingularPtrField<super::saver::SaverDef>,
    pub collection_def: ::std::collections::HashMap<::std::string::String, CollectionDef>,
    pub signature_def: ::std::collections::HashMap<::std::string::String, SignatureDef>,
    pub asset_file_def: ::protobuf::RepeatedField<AssetFileDef>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MetaGraphDef {}

impl MetaGraphDef {
    pub fn new() -> MetaGraphDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MetaGraphDef {
        static mut instance: ::protobuf::lazy::Lazy<MetaGraphDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MetaGraphDef,
        };
        unsafe {
            instance.get(MetaGraphDef::new)
        }
    }

    // .tensorflow.MetaGraphDef.MetaInfoDef meta_info_def = 1;

    pub fn clear_meta_info_def(&mut self) {
        self.meta_info_def.clear();
    }

    pub fn has_meta_info_def(&self) -> bool {
        self.meta_info_def.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meta_info_def(&mut self, v: MetaGraphDef_MetaInfoDef) {
        self.meta_info_def = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meta_info_def(&mut self) -> &mut MetaGraphDef_MetaInfoDef {
        if self.meta_info_def.is_none() {
            self.meta_info_def.set_default();
        }
        self.meta_info_def.as_mut().unwrap()
    }

    // Take field
    pub fn take_meta_info_def(&mut self) -> MetaGraphDef_MetaInfoDef {
        self.meta_info_def.take().unwrap_or_else(|| MetaGraphDef_MetaInfoDef::new())
    }

    pub fn get_meta_info_def(&self) -> &MetaGraphDef_MetaInfoDef {
        self.meta_info_def.as_ref().unwrap_or_else(|| MetaGraphDef_MetaInfoDef::default_instance())
    }

    fn get_meta_info_def_for_reflect(&self) -> &::protobuf::SingularPtrField<MetaGraphDef_MetaInfoDef> {
        &self.meta_info_def
    }

    fn mut_meta_info_def_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MetaGraphDef_MetaInfoDef> {
        &mut self.meta_info_def
    }

    // .tensorflow.GraphDef graph_def = 2;

    pub fn clear_graph_def(&mut self) {
        self.graph_def.clear();
    }

    pub fn has_graph_def(&self) -> bool {
        self.graph_def.is_some()
    }

    // Param is passed by value, moved
    pub fn set_graph_def(&mut self, v: super::graph::GraphDef) {
        self.graph_def = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_graph_def(&mut self) -> &mut super::graph::GraphDef {
        if self.graph_def.is_none() {
            self.graph_def.set_default();
        }
        self.graph_def.as_mut().unwrap()
    }

    // Take field
    pub fn take_graph_def(&mut self) -> super::graph::GraphDef {
        self.graph_def.take().unwrap_or_else(|| super::graph::GraphDef::new())
    }

    pub fn get_graph_def(&self) -> &super::graph::GraphDef {
        self.graph_def.as_ref().unwrap_or_else(|| super::graph::GraphDef::default_instance())
    }

    fn get_graph_def_for_reflect(&self) -> &::protobuf::SingularPtrField<super::graph::GraphDef> {
        &self.graph_def
    }

    fn mut_graph_def_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::graph::GraphDef> {
        &mut self.graph_def
    }

    // .tensorflow.SaverDef saver_def = 3;

    pub fn clear_saver_def(&mut self) {
        self.saver_def.clear();
    }

    pub fn has_saver_def(&self) -> bool {
        self.saver_def.is_some()
    }

    // Param is passed by value, moved
    pub fn set_saver_def(&mut self, v: super::saver::SaverDef) {
        self.saver_def = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_saver_def(&mut self) -> &mut super::saver::SaverDef {
        if self.saver_def.is_none() {
            self.saver_def.set_default();
        }
        self.saver_def.as_mut().unwrap()
    }

    // Take field
    pub fn take_saver_def(&mut self) -> super::saver::SaverDef {
        self.saver_def.take().unwrap_or_else(|| super::saver::SaverDef::new())
    }

    pub fn get_saver_def(&self) -> &super::saver::SaverDef {
        self.saver_def.as_ref().unwrap_or_else(|| super::saver::SaverDef::default_instance())
    }

    fn get_saver_def_for_reflect(&self) -> &::protobuf::SingularPtrField<super::saver::SaverDef> {
        &self.saver_def
    }

    fn mut_saver_def_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::saver::SaverDef> {
        &mut self.saver_def
    }

    // repeated .tensorflow.MetaGraphDef.CollectionDefEntry collection_def = 4;

    pub fn clear_collection_def(&mut self) {
        self.collection_def.clear();
    }

    // Param is passed by value, moved
    pub fn set_collection_def(&mut self, v: ::std::collections::HashMap<::std::string::String, CollectionDef>) {
        self.collection_def = v;
    }

    // Mutable pointer to the field.
    pub fn mut_collection_def(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, CollectionDef> {
        &mut self.collection_def
    }

    // Take field
    pub fn take_collection_def(&mut self) -> ::std::collections::HashMap<::std::string::String, CollectionDef> {
        ::std::mem::replace(&mut self.collection_def, ::std::collections::HashMap::new())
    }

    pub fn get_collection_def(&self) -> &::std::collections::HashMap<::std::string::String, CollectionDef> {
        &self.collection_def
    }

    fn get_collection_def_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, CollectionDef> {
        &self.collection_def
    }

    fn mut_collection_def_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, CollectionDef> {
        &mut self.collection_def
    }

    // repeated .tensorflow.MetaGraphDef.SignatureDefEntry signature_def = 5;

    pub fn clear_signature_def(&mut self) {
        self.signature_def.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature_def(&mut self, v: ::std::collections::HashMap<::std::string::String, SignatureDef>) {
        self.signature_def = v;
    }

    // Mutable pointer to the field.
    pub fn mut_signature_def(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, SignatureDef> {
        &mut self.signature_def
    }

    // Take field
    pub fn take_signature_def(&mut self) -> ::std::collections::HashMap<::std::string::String, SignatureDef> {
        ::std::mem::replace(&mut self.signature_def, ::std::collections::HashMap::new())
    }

    pub fn get_signature_def(&self) -> &::std::collections::HashMap<::std::string::String, SignatureDef> {
        &self.signature_def
    }

    fn get_signature_def_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, SignatureDef> {
        &self.signature_def
    }

    fn mut_signature_def_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, SignatureDef> {
        &mut self.signature_def
    }

    // repeated .tensorflow.AssetFileDef asset_file_def = 6;

    pub fn clear_asset_file_def(&mut self) {
        self.asset_file_def.clear();
    }

    // Param is passed by value, moved
    pub fn set_asset_file_def(&mut self, v: ::protobuf::RepeatedField<AssetFileDef>) {
        self.asset_file_def = v;
    }

    // Mutable pointer to the field.
    pub fn mut_asset_file_def(&mut self) -> &mut ::protobuf::RepeatedField<AssetFileDef> {
        &mut self.asset_file_def
    }

    // Take field
    pub fn take_asset_file_def(&mut self) -> ::protobuf::RepeatedField<AssetFileDef> {
        ::std::mem::replace(&mut self.asset_file_def, ::protobuf::RepeatedField::new())
    }

    pub fn get_asset_file_def(&self) -> &[AssetFileDef] {
        &self.asset_file_def
    }

    fn get_asset_file_def_for_reflect(&self) -> &::protobuf::RepeatedField<AssetFileDef> {
        &self.asset_file_def
    }

    fn mut_asset_file_def_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AssetFileDef> {
        &mut self.asset_file_def
    }
}

impl ::protobuf::Message for MetaGraphDef {
    fn is_initialized(&self) -> bool {
        for v in &self.meta_info_def {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.graph_def {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.saver_def {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.asset_file_def {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meta_info_def)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.graph_def)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.saver_def)?;
                },
                4 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<CollectionDef>>(wire_type, is, &mut self.collection_def)?;
                },
                5 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<SignatureDef>>(wire_type, is, &mut self.signature_def)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.asset_file_def)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.meta_info_def.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.graph_def.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.saver_def.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<CollectionDef>>(4, &self.collection_def);
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<SignatureDef>>(5, &self.signature_def);
        for value in &self.asset_file_def {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.meta_info_def.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.graph_def.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.saver_def.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<CollectionDef>>(4, &self.collection_def, os)?;
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<SignatureDef>>(5, &self.signature_def, os)?;
        for v in &self.asset_file_def {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MetaGraphDef {
    fn new() -> MetaGraphDef {
        MetaGraphDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<MetaGraphDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MetaGraphDef_MetaInfoDef>>(
                    "meta_info_def",
                    MetaGraphDef::get_meta_info_def_for_reflect,
                    MetaGraphDef::mut_meta_info_def_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::graph::GraphDef>>(
                    "graph_def",
                    MetaGraphDef::get_graph_def_for_reflect,
                    MetaGraphDef::mut_graph_def_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::saver::SaverDef>>(
                    "saver_def",
                    MetaGraphDef::get_saver_def_for_reflect,
                    MetaGraphDef::mut_saver_def_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<CollectionDef>>(
                    "collection_def",
                    MetaGraphDef::get_collection_def_for_reflect,
                    MetaGraphDef::mut_collection_def_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<SignatureDef>>(
                    "signature_def",
                    MetaGraphDef::get_signature_def_for_reflect,
                    MetaGraphDef::mut_signature_def_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AssetFileDef>>(
                    "asset_file_def",
                    MetaGraphDef::get_asset_file_def_for_reflect,
                    MetaGraphDef::mut_asset_file_def_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MetaGraphDef>(
                    "MetaGraphDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MetaGraphDef {
    fn clear(&mut self) {
        self.clear_meta_info_def();
        self.clear_graph_def();
        self.clear_saver_def();
        self.clear_collection_def();
        self.clear_signature_def();
        self.clear_asset_file_def();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetaGraphDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetaGraphDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MetaGraphDef_MetaInfoDef {
    // message fields
    pub meta_graph_version: ::std::string::String,
    pub stripped_op_list: ::protobuf::SingularPtrField<super::op_def::OpList>,
    pub any_info: ::protobuf::SingularPtrField<::protobuf::well_known_types::Any>,
    pub tags: ::protobuf::RepeatedField<::std::string::String>,
    pub tensorflow_version: ::std::string::String,
    pub tensorflow_git_version: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MetaGraphDef_MetaInfoDef {}

impl MetaGraphDef_MetaInfoDef {
    pub fn new() -> MetaGraphDef_MetaInfoDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MetaGraphDef_MetaInfoDef {
        static mut instance: ::protobuf::lazy::Lazy<MetaGraphDef_MetaInfoDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MetaGraphDef_MetaInfoDef,
        };
        unsafe {
            instance.get(MetaGraphDef_MetaInfoDef::new)
        }
    }

    // string meta_graph_version = 1;

    pub fn clear_meta_graph_version(&mut self) {
        self.meta_graph_version.clear();
    }

    // Param is passed by value, moved
    pub fn set_meta_graph_version(&mut self, v: ::std::string::String) {
        self.meta_graph_version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meta_graph_version(&mut self) -> &mut ::std::string::String {
        &mut self.meta_graph_version
    }

    // Take field
    pub fn take_meta_graph_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.meta_graph_version, ::std::string::String::new())
    }

    pub fn get_meta_graph_version(&self) -> &str {
        &self.meta_graph_version
    }

    fn get_meta_graph_version_for_reflect(&self) -> &::std::string::String {
        &self.meta_graph_version
    }

    fn mut_meta_graph_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.meta_graph_version
    }

    // .tensorflow.OpList stripped_op_list = 2;

    pub fn clear_stripped_op_list(&mut self) {
        self.stripped_op_list.clear();
    }

    pub fn has_stripped_op_list(&self) -> bool {
        self.stripped_op_list.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stripped_op_list(&mut self, v: super::op_def::OpList) {
        self.stripped_op_list = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stripped_op_list(&mut self) -> &mut super::op_def::OpList {
        if self.stripped_op_list.is_none() {
            self.stripped_op_list.set_default();
        }
        self.stripped_op_list.as_mut().unwrap()
    }

    // Take field
    pub fn take_stripped_op_list(&mut self) -> super::op_def::OpList {
        self.stripped_op_list.take().unwrap_or_else(|| super::op_def::OpList::new())
    }

    pub fn get_stripped_op_list(&self) -> &super::op_def::OpList {
        self.stripped_op_list.as_ref().unwrap_or_else(|| super::op_def::OpList::default_instance())
    }

    fn get_stripped_op_list_for_reflect(&self) -> &::protobuf::SingularPtrField<super::op_def::OpList> {
        &self.stripped_op_list
    }

    fn mut_stripped_op_list_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::op_def::OpList> {
        &mut self.stripped_op_list
    }

    // .google.protobuf.Any any_info = 3;

    pub fn clear_any_info(&mut self) {
        self.any_info.clear();
    }

    pub fn has_any_info(&self) -> bool {
        self.any_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_any_info(&mut self, v: ::protobuf::well_known_types::Any) {
        self.any_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_any_info(&mut self) -> &mut ::protobuf::well_known_types::Any {
        if self.any_info.is_none() {
            self.any_info.set_default();
        }
        self.any_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_any_info(&mut self) -> ::protobuf::well_known_types::Any {
        self.any_info.take().unwrap_or_else(|| ::protobuf::well_known_types::Any::new())
    }

    pub fn get_any_info(&self) -> &::protobuf::well_known_types::Any {
        self.any_info.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Any::default_instance())
    }

    fn get_any_info_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Any> {
        &self.any_info
    }

    fn mut_any_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Any> {
        &mut self.any_info
    }

    // repeated string tags = 4;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[::std::string::String] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // string tensorflow_version = 5;

    pub fn clear_tensorflow_version(&mut self) {
        self.tensorflow_version.clear();
    }

    // Param is passed by value, moved
    pub fn set_tensorflow_version(&mut self, v: ::std::string::String) {
        self.tensorflow_version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensorflow_version(&mut self) -> &mut ::std::string::String {
        &mut self.tensorflow_version
    }

    // Take field
    pub fn take_tensorflow_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tensorflow_version, ::std::string::String::new())
    }

    pub fn get_tensorflow_version(&self) -> &str {
        &self.tensorflow_version
    }

    fn get_tensorflow_version_for_reflect(&self) -> &::std::string::String {
        &self.tensorflow_version
    }

    fn mut_tensorflow_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tensorflow_version
    }

    // string tensorflow_git_version = 6;

    pub fn clear_tensorflow_git_version(&mut self) {
        self.tensorflow_git_version.clear();
    }

    // Param is passed by value, moved
    pub fn set_tensorflow_git_version(&mut self, v: ::std::string::String) {
        self.tensorflow_git_version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensorflow_git_version(&mut self) -> &mut ::std::string::String {
        &mut self.tensorflow_git_version
    }

    // Take field
    pub fn take_tensorflow_git_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tensorflow_git_version, ::std::string::String::new())
    }

    pub fn get_tensorflow_git_version(&self) -> &str {
        &self.tensorflow_git_version
    }

    fn get_tensorflow_git_version_for_reflect(&self) -> &::std::string::String {
        &self.tensorflow_git_version
    }

    fn mut_tensorflow_git_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tensorflow_git_version
    }
}

impl ::protobuf::Message for MetaGraphDef_MetaInfoDef {
    fn is_initialized(&self) -> bool {
        for v in &self.stripped_op_list {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.any_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.meta_graph_version)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stripped_op_list)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.any_info)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tags)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tensorflow_version)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tensorflow_git_version)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.meta_graph_version.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.meta_graph_version);
        }
        if let Some(ref v) = self.stripped_op_list.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.any_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.tags {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if !self.tensorflow_version.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.tensorflow_version);
        }
        if !self.tensorflow_git_version.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.tensorflow_git_version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.meta_graph_version.is_empty() {
            os.write_string(1, &self.meta_graph_version)?;
        }
        if let Some(ref v) = self.stripped_op_list.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.any_info.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.tags {
            os.write_string(4, &v)?;
        };
        if !self.tensorflow_version.is_empty() {
            os.write_string(5, &self.tensorflow_version)?;
        }
        if !self.tensorflow_git_version.is_empty() {
            os.write_string(6, &self.tensorflow_git_version)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MetaGraphDef_MetaInfoDef {
    fn new() -> MetaGraphDef_MetaInfoDef {
        MetaGraphDef_MetaInfoDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<MetaGraphDef_MetaInfoDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "meta_graph_version",
                    MetaGraphDef_MetaInfoDef::get_meta_graph_version_for_reflect,
                    MetaGraphDef_MetaInfoDef::mut_meta_graph_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::op_def::OpList>>(
                    "stripped_op_list",
                    MetaGraphDef_MetaInfoDef::get_stripped_op_list_for_reflect,
                    MetaGraphDef_MetaInfoDef::mut_stripped_op_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                    "any_info",
                    MetaGraphDef_MetaInfoDef::get_any_info_for_reflect,
                    MetaGraphDef_MetaInfoDef::mut_any_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tags",
                    MetaGraphDef_MetaInfoDef::get_tags_for_reflect,
                    MetaGraphDef_MetaInfoDef::mut_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tensorflow_version",
                    MetaGraphDef_MetaInfoDef::get_tensorflow_version_for_reflect,
                    MetaGraphDef_MetaInfoDef::mut_tensorflow_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tensorflow_git_version",
                    MetaGraphDef_MetaInfoDef::get_tensorflow_git_version_for_reflect,
                    MetaGraphDef_MetaInfoDef::mut_tensorflow_git_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MetaGraphDef_MetaInfoDef>(
                    "MetaGraphDef_MetaInfoDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MetaGraphDef_MetaInfoDef {
    fn clear(&mut self) {
        self.clear_meta_graph_version();
        self.clear_stripped_op_list();
        self.clear_any_info();
        self.clear_tags();
        self.clear_tensorflow_version();
        self.clear_tensorflow_git_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetaGraphDef_MetaInfoDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetaGraphDef_MetaInfoDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CollectionDef {
    // message oneof groups
    kind: ::std::option::Option<CollectionDef_oneof_kind>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CollectionDef {}

#[derive(Clone,PartialEq)]
pub enum CollectionDef_oneof_kind {
    node_list(CollectionDef_NodeList),
    bytes_list(CollectionDef_BytesList),
    int64_list(CollectionDef_Int64List),
    float_list(CollectionDef_FloatList),
    any_list(CollectionDef_AnyList),
}

impl CollectionDef {
    pub fn new() -> CollectionDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CollectionDef {
        static mut instance: ::protobuf::lazy::Lazy<CollectionDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CollectionDef,
        };
        unsafe {
            instance.get(CollectionDef::new)
        }
    }

    // .tensorflow.CollectionDef.NodeList node_list = 1;

    pub fn clear_node_list(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_node_list(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::node_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_node_list(&mut self, v: CollectionDef_NodeList) {
        self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::node_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_node_list(&mut self) -> &mut CollectionDef_NodeList {
        if let ::std::option::Option::Some(CollectionDef_oneof_kind::node_list(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::node_list(CollectionDef_NodeList::new()));
        }
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::node_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_node_list(&mut self) -> CollectionDef_NodeList {
        if self.has_node_list() {
            match self.kind.take() {
                ::std::option::Option::Some(CollectionDef_oneof_kind::node_list(v)) => v,
                _ => panic!(),
            }
        } else {
            CollectionDef_NodeList::new()
        }
    }

    pub fn get_node_list(&self) -> &CollectionDef_NodeList {
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::node_list(ref v)) => v,
            _ => CollectionDef_NodeList::default_instance(),
        }
    }

    // .tensorflow.CollectionDef.BytesList bytes_list = 2;

    pub fn clear_bytes_list(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_bytes_list(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::bytes_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bytes_list(&mut self, v: CollectionDef_BytesList) {
        self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::bytes_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_bytes_list(&mut self) -> &mut CollectionDef_BytesList {
        if let ::std::option::Option::Some(CollectionDef_oneof_kind::bytes_list(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::bytes_list(CollectionDef_BytesList::new()));
        }
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::bytes_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bytes_list(&mut self) -> CollectionDef_BytesList {
        if self.has_bytes_list() {
            match self.kind.take() {
                ::std::option::Option::Some(CollectionDef_oneof_kind::bytes_list(v)) => v,
                _ => panic!(),
            }
        } else {
            CollectionDef_BytesList::new()
        }
    }

    pub fn get_bytes_list(&self) -> &CollectionDef_BytesList {
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::bytes_list(ref v)) => v,
            _ => CollectionDef_BytesList::default_instance(),
        }
    }

    // .tensorflow.CollectionDef.Int64List int64_list = 3;

    pub fn clear_int64_list(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_int64_list(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::int64_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int64_list(&mut self, v: CollectionDef_Int64List) {
        self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::int64_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_int64_list(&mut self) -> &mut CollectionDef_Int64List {
        if let ::std::option::Option::Some(CollectionDef_oneof_kind::int64_list(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::int64_list(CollectionDef_Int64List::new()));
        }
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::int64_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_int64_list(&mut self) -> CollectionDef_Int64List {
        if self.has_int64_list() {
            match self.kind.take() {
                ::std::option::Option::Some(CollectionDef_oneof_kind::int64_list(v)) => v,
                _ => panic!(),
            }
        } else {
            CollectionDef_Int64List::new()
        }
    }

    pub fn get_int64_list(&self) -> &CollectionDef_Int64List {
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::int64_list(ref v)) => v,
            _ => CollectionDef_Int64List::default_instance(),
        }
    }

    // .tensorflow.CollectionDef.FloatList float_list = 4;

    pub fn clear_float_list(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_float_list(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::float_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float_list(&mut self, v: CollectionDef_FloatList) {
        self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::float_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_float_list(&mut self) -> &mut CollectionDef_FloatList {
        if let ::std::option::Option::Some(CollectionDef_oneof_kind::float_list(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::float_list(CollectionDef_FloatList::new()));
        }
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::float_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_float_list(&mut self) -> CollectionDef_FloatList {
        if self.has_float_list() {
            match self.kind.take() {
                ::std::option::Option::Some(CollectionDef_oneof_kind::float_list(v)) => v,
                _ => panic!(),
            }
        } else {
            CollectionDef_FloatList::new()
        }
    }

    pub fn get_float_list(&self) -> &CollectionDef_FloatList {
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::float_list(ref v)) => v,
            _ => CollectionDef_FloatList::default_instance(),
        }
    }

    // .tensorflow.CollectionDef.AnyList any_list = 5;

    pub fn clear_any_list(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_any_list(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::any_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_any_list(&mut self, v: CollectionDef_AnyList) {
        self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::any_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_any_list(&mut self) -> &mut CollectionDef_AnyList {
        if let ::std::option::Option::Some(CollectionDef_oneof_kind::any_list(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::any_list(CollectionDef_AnyList::new()));
        }
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::any_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_any_list(&mut self) -> CollectionDef_AnyList {
        if self.has_any_list() {
            match self.kind.take() {
                ::std::option::Option::Some(CollectionDef_oneof_kind::any_list(v)) => v,
                _ => panic!(),
            }
        } else {
            CollectionDef_AnyList::new()
        }
    }

    pub fn get_any_list(&self) -> &CollectionDef_AnyList {
        match self.kind {
            ::std::option::Option::Some(CollectionDef_oneof_kind::any_list(ref v)) => v,
            _ => CollectionDef_AnyList::default_instance(),
        }
    }
}

impl ::protobuf::Message for CollectionDef {
    fn is_initialized(&self) -> bool {
        if let Some(CollectionDef_oneof_kind::node_list(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(CollectionDef_oneof_kind::bytes_list(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(CollectionDef_oneof_kind::int64_list(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(CollectionDef_oneof_kind::float_list(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(CollectionDef_oneof_kind::any_list(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::node_list(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::bytes_list(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::int64_list(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::float_list(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(CollectionDef_oneof_kind::any_list(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &CollectionDef_oneof_kind::node_list(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &CollectionDef_oneof_kind::bytes_list(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &CollectionDef_oneof_kind::int64_list(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &CollectionDef_oneof_kind::float_list(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &CollectionDef_oneof_kind::any_list(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &CollectionDef_oneof_kind::node_list(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &CollectionDef_oneof_kind::bytes_list(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &CollectionDef_oneof_kind::int64_list(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &CollectionDef_oneof_kind::float_list(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &CollectionDef_oneof_kind::any_list(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CollectionDef {
    fn new() -> CollectionDef {
        CollectionDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<CollectionDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CollectionDef_NodeList>(
                    "node_list",
                    CollectionDef::has_node_list,
                    CollectionDef::get_node_list,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CollectionDef_BytesList>(
                    "bytes_list",
                    CollectionDef::has_bytes_list,
                    CollectionDef::get_bytes_list,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CollectionDef_Int64List>(
                    "int64_list",
                    CollectionDef::has_int64_list,
                    CollectionDef::get_int64_list,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CollectionDef_FloatList>(
                    "float_list",
                    CollectionDef::has_float_list,
                    CollectionDef::get_float_list,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CollectionDef_AnyList>(
                    "any_list",
                    CollectionDef::has_any_list,
                    CollectionDef::get_any_list,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CollectionDef>(
                    "CollectionDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CollectionDef {
    fn clear(&mut self) {
        self.clear_node_list();
        self.clear_bytes_list();
        self.clear_int64_list();
        self.clear_float_list();
        self.clear_any_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CollectionDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CollectionDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CollectionDef_NodeList {
    // message fields
    pub value: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CollectionDef_NodeList {}

impl CollectionDef_NodeList {
    pub fn new() -> CollectionDef_NodeList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CollectionDef_NodeList {
        static mut instance: ::protobuf::lazy::Lazy<CollectionDef_NodeList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CollectionDef_NodeList,
        };
        unsafe {
            instance.get(CollectionDef_NodeList::new)
        }
    }

    // repeated string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.value, ::protobuf::RepeatedField::new())
    }

    pub fn get_value(&self) -> &[::std::string::String] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for CollectionDef_NodeList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.value {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.value {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CollectionDef_NodeList {
    fn new() -> CollectionDef_NodeList {
        CollectionDef_NodeList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CollectionDef_NodeList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    CollectionDef_NodeList::get_value_for_reflect,
                    CollectionDef_NodeList::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CollectionDef_NodeList>(
                    "CollectionDef_NodeList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CollectionDef_NodeList {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CollectionDef_NodeList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CollectionDef_NodeList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CollectionDef_BytesList {
    // message fields
    pub value: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CollectionDef_BytesList {}

impl CollectionDef_BytesList {
    pub fn new() -> CollectionDef_BytesList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CollectionDef_BytesList {
        static mut instance: ::protobuf::lazy::Lazy<CollectionDef_BytesList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CollectionDef_BytesList,
        };
        unsafe {
            instance.get(CollectionDef_BytesList::new)
        }
    }

    // repeated bytes value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.value, ::protobuf::RepeatedField::new())
    }

    pub fn get_value(&self) -> &[::std::vec::Vec<u8>] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for CollectionDef_BytesList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.value {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.value {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CollectionDef_BytesList {
    fn new() -> CollectionDef_BytesList {
        CollectionDef_BytesList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CollectionDef_BytesList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    CollectionDef_BytesList::get_value_for_reflect,
                    CollectionDef_BytesList::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CollectionDef_BytesList>(
                    "CollectionDef_BytesList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CollectionDef_BytesList {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CollectionDef_BytesList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CollectionDef_BytesList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CollectionDef_Int64List {
    // message fields
    pub value: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CollectionDef_Int64List {}

impl CollectionDef_Int64List {
    pub fn new() -> CollectionDef_Int64List {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CollectionDef_Int64List {
        static mut instance: ::protobuf::lazy::Lazy<CollectionDef_Int64List> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CollectionDef_Int64List,
        };
        unsafe {
            instance.get(CollectionDef_Int64List::new)
        }
    }

    // repeated int64 value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<i64>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[i64] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.value
    }
}

impl ::protobuf::Message for CollectionDef_Int64List {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.value.is_empty() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.value))?;
            for v in &self.value {
                os.write_int64_no_tag(*v)?;
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CollectionDef_Int64List {
    fn new() -> CollectionDef_Int64List {
        CollectionDef_Int64List::new()
    }

    fn descriptor_static(_: ::std::option::Option<CollectionDef_Int64List>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "value",
                    CollectionDef_Int64List::get_value_for_reflect,
                    CollectionDef_Int64List::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CollectionDef_Int64List>(
                    "CollectionDef_Int64List",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CollectionDef_Int64List {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CollectionDef_Int64List {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CollectionDef_Int64List {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CollectionDef_FloatList {
    // message fields
    pub value: ::std::vec::Vec<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CollectionDef_FloatList {}

impl CollectionDef_FloatList {
    pub fn new() -> CollectionDef_FloatList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CollectionDef_FloatList {
        static mut instance: ::protobuf::lazy::Lazy<CollectionDef_FloatList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CollectionDef_FloatList,
        };
        unsafe {
            instance.get(CollectionDef_FloatList::new)
        }
    }

    // repeated float value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<f32>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[f32] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.value
    }
}

impl ::protobuf::Message for CollectionDef_FloatList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.value.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.value.len() as u32) + (self.value.len() * 4) as u32;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.value.is_empty() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.value.len() * 4) as u32)?;
            for v in &self.value {
                os.write_float_no_tag(*v)?;
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CollectionDef_FloatList {
    fn new() -> CollectionDef_FloatList {
        CollectionDef_FloatList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CollectionDef_FloatList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "value",
                    CollectionDef_FloatList::get_value_for_reflect,
                    CollectionDef_FloatList::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CollectionDef_FloatList>(
                    "CollectionDef_FloatList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CollectionDef_FloatList {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CollectionDef_FloatList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CollectionDef_FloatList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CollectionDef_AnyList {
    // message fields
    pub value: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CollectionDef_AnyList {}

impl CollectionDef_AnyList {
    pub fn new() -> CollectionDef_AnyList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CollectionDef_AnyList {
        static mut instance: ::protobuf::lazy::Lazy<CollectionDef_AnyList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CollectionDef_AnyList,
        };
        unsafe {
            instance.get(CollectionDef_AnyList::new)
        }
    }

    // repeated .google.protobuf.Any value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        ::std::mem::replace(&mut self.value, ::protobuf::RepeatedField::new())
    }

    pub fn get_value(&self) -> &[::protobuf::well_known_types::Any] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &mut self.value
    }
}

impl ::protobuf::Message for CollectionDef_AnyList {
    fn is_initialized(&self) -> bool {
        for v in &self.value {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.value {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.value {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CollectionDef_AnyList {
    fn new() -> CollectionDef_AnyList {
        CollectionDef_AnyList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CollectionDef_AnyList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                    "value",
                    CollectionDef_AnyList::get_value_for_reflect,
                    CollectionDef_AnyList::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CollectionDef_AnyList>(
                    "CollectionDef_AnyList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CollectionDef_AnyList {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CollectionDef_AnyList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CollectionDef_AnyList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TensorInfo {
    // message fields
    pub dtype: super::types::DataType,
    pub tensor_shape: ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto>,
    // message oneof groups
    encoding: ::std::option::Option<TensorInfo_oneof_encoding>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TensorInfo {}

#[derive(Clone,PartialEq)]
pub enum TensorInfo_oneof_encoding {
    name(::std::string::String),
    coo_sparse(TensorInfo_CooSparse),
}

impl TensorInfo {
    pub fn new() -> TensorInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TensorInfo {
        static mut instance: ::protobuf::lazy::Lazy<TensorInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TensorInfo,
        };
        unsafe {
            instance.get(TensorInfo::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.encoding = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        match self.encoding {
            ::std::option::Option::Some(TensorInfo_oneof_encoding::name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.encoding = ::std::option::Option::Some(TensorInfo_oneof_encoding::name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TensorInfo_oneof_encoding::name(_)) = self.encoding {
        } else {
            self.encoding = ::std::option::Option::Some(TensorInfo_oneof_encoding::name(::std::string::String::new()));
        }
        match self.encoding {
            ::std::option::Option::Some(TensorInfo_oneof_encoding::name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        if self.has_name() {
            match self.encoding.take() {
                ::std::option::Option::Some(TensorInfo_oneof_encoding::name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_name(&self) -> &str {
        match self.encoding {
            ::std::option::Option::Some(TensorInfo_oneof_encoding::name(ref v)) => v,
            _ => "",
        }
    }

    // .tensorflow.TensorInfo.CooSparse coo_sparse = 4;

    pub fn clear_coo_sparse(&mut self) {
        self.encoding = ::std::option::Option::None;
    }

    pub fn has_coo_sparse(&self) -> bool {
        match self.encoding {
            ::std::option::Option::Some(TensorInfo_oneof_encoding::coo_sparse(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_coo_sparse(&mut self, v: TensorInfo_CooSparse) {
        self.encoding = ::std::option::Option::Some(TensorInfo_oneof_encoding::coo_sparse(v))
    }

    // Mutable pointer to the field.
    pub fn mut_coo_sparse(&mut self) -> &mut TensorInfo_CooSparse {
        if let ::std::option::Option::Some(TensorInfo_oneof_encoding::coo_sparse(_)) = self.encoding {
        } else {
            self.encoding = ::std::option::Option::Some(TensorInfo_oneof_encoding::coo_sparse(TensorInfo_CooSparse::new()));
        }
        match self.encoding {
            ::std::option::Option::Some(TensorInfo_oneof_encoding::coo_sparse(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_coo_sparse(&mut self) -> TensorInfo_CooSparse {
        if self.has_coo_sparse() {
            match self.encoding.take() {
                ::std::option::Option::Some(TensorInfo_oneof_encoding::coo_sparse(v)) => v,
                _ => panic!(),
            }
        } else {
            TensorInfo_CooSparse::new()
        }
    }

    pub fn get_coo_sparse(&self) -> &TensorInfo_CooSparse {
        match self.encoding {
            ::std::option::Option::Some(TensorInfo_oneof_encoding::coo_sparse(ref v)) => v,
            _ => TensorInfo_CooSparse::default_instance(),
        }
    }

    // .tensorflow.DataType dtype = 2;

    pub fn clear_dtype(&mut self) {
        self.dtype = super::types::DataType::DT_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_dtype(&mut self, v: super::types::DataType) {
        self.dtype = v;
    }

    pub fn get_dtype(&self) -> super::types::DataType {
        self.dtype
    }

    fn get_dtype_for_reflect(&self) -> &super::types::DataType {
        &self.dtype
    }

    fn mut_dtype_for_reflect(&mut self) -> &mut super::types::DataType {
        &mut self.dtype
    }

    // .tensorflow.TensorShapeProto tensor_shape = 3;

    pub fn clear_tensor_shape(&mut self) {
        self.tensor_shape.clear();
    }

    pub fn has_tensor_shape(&self) -> bool {
        self.tensor_shape.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tensor_shape(&mut self, v: super::tensor_shape::TensorShapeProto) {
        self.tensor_shape = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor_shape(&mut self) -> &mut super::tensor_shape::TensorShapeProto {
        if self.tensor_shape.is_none() {
            self.tensor_shape.set_default();
        }
        self.tensor_shape.as_mut().unwrap()
    }

    // Take field
    pub fn take_tensor_shape(&mut self) -> super::tensor_shape::TensorShapeProto {
        self.tensor_shape.take().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::new())
    }

    pub fn get_tensor_shape(&self) -> &super::tensor_shape::TensorShapeProto {
        self.tensor_shape.as_ref().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::default_instance())
    }

    fn get_tensor_shape_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto> {
        &self.tensor_shape
    }

    fn mut_tensor_shape_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto> {
        &mut self.tensor_shape
    }
}

impl ::protobuf::Message for TensorInfo {
    fn is_initialized(&self) -> bool {
        if let Some(TensorInfo_oneof_encoding::coo_sparse(ref v)) = self.encoding {
            if !v.is_initialized() {
                return false;
            }
        }
        for v in &self.tensor_shape {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.encoding = ::std::option::Option::Some(TensorInfo_oneof_encoding::name(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.encoding = ::std::option::Option::Some(TensorInfo_oneof_encoding::coo_sparse(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.dtype = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tensor_shape)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(2, self.dtype);
        }
        if let Some(ref v) = self.tensor_shape.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.encoding {
            match v {
                &TensorInfo_oneof_encoding::name(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &TensorInfo_oneof_encoding::coo_sparse(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(2, self.dtype.value())?;
        }
        if let Some(ref v) = self.tensor_shape.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.encoding {
            match v {
                &TensorInfo_oneof_encoding::name(ref v) => {
                    os.write_string(1, v)?;
                },
                &TensorInfo_oneof_encoding::coo_sparse(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TensorInfo {
    fn new() -> TensorInfo {
        TensorInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<TensorInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "name",
                    TensorInfo::has_name,
                    TensorInfo::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TensorInfo_CooSparse>(
                    "coo_sparse",
                    TensorInfo::has_coo_sparse,
                    TensorInfo::get_coo_sparse,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    TensorInfo::get_dtype_for_reflect,
                    TensorInfo::mut_dtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_shape::TensorShapeProto>>(
                    "tensor_shape",
                    TensorInfo::get_tensor_shape_for_reflect,
                    TensorInfo::mut_tensor_shape_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TensorInfo>(
                    "TensorInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TensorInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_coo_sparse();
        self.clear_dtype();
        self.clear_tensor_shape();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TensorInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TensorInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TensorInfo_CooSparse {
    // message fields
    pub values_tensor_name: ::std::string::String,
    pub indices_tensor_name: ::std::string::String,
    pub dense_shape_tensor_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TensorInfo_CooSparse {}

impl TensorInfo_CooSparse {
    pub fn new() -> TensorInfo_CooSparse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TensorInfo_CooSparse {
        static mut instance: ::protobuf::lazy::Lazy<TensorInfo_CooSparse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TensorInfo_CooSparse,
        };
        unsafe {
            instance.get(TensorInfo_CooSparse::new)
        }
    }

    // string values_tensor_name = 1;

    pub fn clear_values_tensor_name(&mut self) {
        self.values_tensor_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_values_tensor_name(&mut self, v: ::std::string::String) {
        self.values_tensor_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_values_tensor_name(&mut self) -> &mut ::std::string::String {
        &mut self.values_tensor_name
    }

    // Take field
    pub fn take_values_tensor_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.values_tensor_name, ::std::string::String::new())
    }

    pub fn get_values_tensor_name(&self) -> &str {
        &self.values_tensor_name
    }

    fn get_values_tensor_name_for_reflect(&self) -> &::std::string::String {
        &self.values_tensor_name
    }

    fn mut_values_tensor_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.values_tensor_name
    }

    // string indices_tensor_name = 2;

    pub fn clear_indices_tensor_name(&mut self) {
        self.indices_tensor_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_indices_tensor_name(&mut self, v: ::std::string::String) {
        self.indices_tensor_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_indices_tensor_name(&mut self) -> &mut ::std::string::String {
        &mut self.indices_tensor_name
    }

    // Take field
    pub fn take_indices_tensor_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.indices_tensor_name, ::std::string::String::new())
    }

    pub fn get_indices_tensor_name(&self) -> &str {
        &self.indices_tensor_name
    }

    fn get_indices_tensor_name_for_reflect(&self) -> &::std::string::String {
        &self.indices_tensor_name
    }

    fn mut_indices_tensor_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.indices_tensor_name
    }

    // string dense_shape_tensor_name = 3;

    pub fn clear_dense_shape_tensor_name(&mut self) {
        self.dense_shape_tensor_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_dense_shape_tensor_name(&mut self, v: ::std::string::String) {
        self.dense_shape_tensor_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dense_shape_tensor_name(&mut self) -> &mut ::std::string::String {
        &mut self.dense_shape_tensor_name
    }

    // Take field
    pub fn take_dense_shape_tensor_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.dense_shape_tensor_name, ::std::string::String::new())
    }

    pub fn get_dense_shape_tensor_name(&self) -> &str {
        &self.dense_shape_tensor_name
    }

    fn get_dense_shape_tensor_name_for_reflect(&self) -> &::std::string::String {
        &self.dense_shape_tensor_name
    }

    fn mut_dense_shape_tensor_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.dense_shape_tensor_name
    }
}

impl ::protobuf::Message for TensorInfo_CooSparse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.values_tensor_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.indices_tensor_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.dense_shape_tensor_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.values_tensor_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.values_tensor_name);
        }
        if !self.indices_tensor_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.indices_tensor_name);
        }
        if !self.dense_shape_tensor_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.dense_shape_tensor_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.values_tensor_name.is_empty() {
            os.write_string(1, &self.values_tensor_name)?;
        }
        if !self.indices_tensor_name.is_empty() {
            os.write_string(2, &self.indices_tensor_name)?;
        }
        if !self.dense_shape_tensor_name.is_empty() {
            os.write_string(3, &self.dense_shape_tensor_name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TensorInfo_CooSparse {
    fn new() -> TensorInfo_CooSparse {
        TensorInfo_CooSparse::new()
    }

    fn descriptor_static(_: ::std::option::Option<TensorInfo_CooSparse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "values_tensor_name",
                    TensorInfo_CooSparse::get_values_tensor_name_for_reflect,
                    TensorInfo_CooSparse::mut_values_tensor_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "indices_tensor_name",
                    TensorInfo_CooSparse::get_indices_tensor_name_for_reflect,
                    TensorInfo_CooSparse::mut_indices_tensor_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "dense_shape_tensor_name",
                    TensorInfo_CooSparse::get_dense_shape_tensor_name_for_reflect,
                    TensorInfo_CooSparse::mut_dense_shape_tensor_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TensorInfo_CooSparse>(
                    "TensorInfo_CooSparse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TensorInfo_CooSparse {
    fn clear(&mut self) {
        self.clear_values_tensor_name();
        self.clear_indices_tensor_name();
        self.clear_dense_shape_tensor_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TensorInfo_CooSparse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TensorInfo_CooSparse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignatureDef {
    // message fields
    pub inputs: ::std::collections::HashMap<::std::string::String, TensorInfo>,
    pub outputs: ::std::collections::HashMap<::std::string::String, TensorInfo>,
    pub method_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignatureDef {}

impl SignatureDef {
    pub fn new() -> SignatureDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignatureDef {
        static mut instance: ::protobuf::lazy::Lazy<SignatureDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignatureDef,
        };
        unsafe {
            instance.get(SignatureDef::new)
        }
    }

    // repeated .tensorflow.SignatureDef.InputsEntry inputs = 1;

    pub fn clear_inputs(&mut self) {
        self.inputs.clear();
    }

    // Param is passed by value, moved
    pub fn set_inputs(&mut self, v: ::std::collections::HashMap<::std::string::String, TensorInfo>) {
        self.inputs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_inputs(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, TensorInfo> {
        &mut self.inputs
    }

    // Take field
    pub fn take_inputs(&mut self) -> ::std::collections::HashMap<::std::string::String, TensorInfo> {
        ::std::mem::replace(&mut self.inputs, ::std::collections::HashMap::new())
    }

    pub fn get_inputs(&self) -> &::std::collections::HashMap<::std::string::String, TensorInfo> {
        &self.inputs
    }

    fn get_inputs_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, TensorInfo> {
        &self.inputs
    }

    fn mut_inputs_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, TensorInfo> {
        &mut self.inputs
    }

    // repeated .tensorflow.SignatureDef.OutputsEntry outputs = 2;

    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
    }

    // Param is passed by value, moved
    pub fn set_outputs(&mut self, v: ::std::collections::HashMap<::std::string::String, TensorInfo>) {
        self.outputs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_outputs(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, TensorInfo> {
        &mut self.outputs
    }

    // Take field
    pub fn take_outputs(&mut self) -> ::std::collections::HashMap<::std::string::String, TensorInfo> {
        ::std::mem::replace(&mut self.outputs, ::std::collections::HashMap::new())
    }

    pub fn get_outputs(&self) -> &::std::collections::HashMap<::std::string::String, TensorInfo> {
        &self.outputs
    }

    fn get_outputs_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, TensorInfo> {
        &self.outputs
    }

    fn mut_outputs_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, TensorInfo> {
        &mut self.outputs
    }

    // string method_name = 3;

    pub fn clear_method_name(&mut self) {
        self.method_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_method_name(&mut self, v: ::std::string::String) {
        self.method_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_method_name(&mut self) -> &mut ::std::string::String {
        &mut self.method_name
    }

    // Take field
    pub fn take_method_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.method_name, ::std::string::String::new())
    }

    pub fn get_method_name(&self) -> &str {
        &self.method_name
    }

    fn get_method_name_for_reflect(&self) -> &::std::string::String {
        &self.method_name
    }

    fn mut_method_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.method_name
    }
}

impl ::protobuf::Message for SignatureDef {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TensorInfo>>(wire_type, is, &mut self.inputs)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TensorInfo>>(wire_type, is, &mut self.outputs)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.method_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TensorInfo>>(1, &self.inputs);
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TensorInfo>>(2, &self.outputs);
        if !self.method_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.method_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TensorInfo>>(1, &self.inputs, os)?;
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TensorInfo>>(2, &self.outputs, os)?;
        if !self.method_name.is_empty() {
            os.write_string(3, &self.method_name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SignatureDef {
    fn new() -> SignatureDef {
        SignatureDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignatureDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TensorInfo>>(
                    "inputs",
                    SignatureDef::get_inputs_for_reflect,
                    SignatureDef::mut_inputs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TensorInfo>>(
                    "outputs",
                    SignatureDef::get_outputs_for_reflect,
                    SignatureDef::mut_outputs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "method_name",
                    SignatureDef::get_method_name_for_reflect,
                    SignatureDef::mut_method_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignatureDef>(
                    "SignatureDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignatureDef {
    fn clear(&mut self) {
        self.clear_inputs();
        self.clear_outputs();
        self.clear_method_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignatureDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignatureDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AssetFileDef {
    // message fields
    pub tensor_info: ::protobuf::SingularPtrField<TensorInfo>,
    pub filename: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AssetFileDef {}

impl AssetFileDef {
    pub fn new() -> AssetFileDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AssetFileDef {
        static mut instance: ::protobuf::lazy::Lazy<AssetFileDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AssetFileDef,
        };
        unsafe {
            instance.get(AssetFileDef::new)
        }
    }

    // .tensorflow.TensorInfo tensor_info = 1;

    pub fn clear_tensor_info(&mut self) {
        self.tensor_info.clear();
    }

    pub fn has_tensor_info(&self) -> bool {
        self.tensor_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tensor_info(&mut self, v: TensorInfo) {
        self.tensor_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor_info(&mut self) -> &mut TensorInfo {
        if self.tensor_info.is_none() {
            self.tensor_info.set_default();
        }
        self.tensor_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_tensor_info(&mut self) -> TensorInfo {
        self.tensor_info.take().unwrap_or_else(|| TensorInfo::new())
    }

    pub fn get_tensor_info(&self) -> &TensorInfo {
        self.tensor_info.as_ref().unwrap_or_else(|| TensorInfo::default_instance())
    }

    fn get_tensor_info_for_reflect(&self) -> &::protobuf::SingularPtrField<TensorInfo> {
        &self.tensor_info
    }

    fn mut_tensor_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TensorInfo> {
        &mut self.tensor_info
    }

    // string filename = 2;

    pub fn clear_filename(&mut self) {
        self.filename.clear();
    }

    // Param is passed by value, moved
    pub fn set_filename(&mut self, v: ::std::string::String) {
        self.filename = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filename(&mut self) -> &mut ::std::string::String {
        &mut self.filename
    }

    // Take field
    pub fn take_filename(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.filename, ::std::string::String::new())
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }

    fn get_filename_for_reflect(&self) -> &::std::string::String {
        &self.filename
    }

    fn mut_filename_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.filename
    }
}

impl ::protobuf::Message for AssetFileDef {
    fn is_initialized(&self) -> bool {
        for v in &self.tensor_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tensor_info)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.filename)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.tensor_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.filename.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.filename);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.tensor_info.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.filename.is_empty() {
            os.write_string(2, &self.filename)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AssetFileDef {
    fn new() -> AssetFileDef {
        AssetFileDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<AssetFileDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TensorInfo>>(
                    "tensor_info",
                    AssetFileDef::get_tensor_info_for_reflect,
                    AssetFileDef::mut_tensor_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename",
                    AssetFileDef::get_filename_for_reflect,
                    AssetFileDef::mut_filename_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AssetFileDef>(
                    "AssetFileDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AssetFileDef {
    fn clear(&mut self) {
        self.clear_tensor_info();
        self.clear_filename();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AssetFileDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AssetFileDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n4tensorflow/tensorflow/core/protobuf/meta_graph.proto\x12\ntensorflow\
    \x1a\x19google/protobuf/any.proto\x1a%tensorflow/core/framework/graph.pr\
    oto\x1a&tensorflow/core/framework/op_def.proto\x1a,tensorflow/core/frame\
    work/tensor_shape.proto\x1a%tensorflow/core/framework/types.proto\x1a$te\
    nsorflow/core/protobuf/saver.proto\"\x81\x07\n\x0cMetaGraphDef\x12H\n\rm\
    eta_info_def\x18\x01\x20\x01(\x0b2$.tensorflow.MetaGraphDef.MetaInfoDefR\
    \x0bmetaInfoDef\x121\n\tgraph_def\x18\x02\x20\x01(\x0b2\x14.tensorflow.G\
    raphDefR\x08graphDef\x121\n\tsaver_def\x18\x03\x20\x01(\x0b2\x14.tensorf\
    low.SaverDefR\x08saverDef\x12R\n\x0ecollection_def\x18\x04\x20\x03(\x0b2\
    +.tensorflow.MetaGraphDef.CollectionDefEntryR\rcollectionDef\x12O\n\rsig\
    nature_def\x18\x05\x20\x03(\x0b2*.tensorflow.MetaGraphDef.SignatureDefEn\
    tryR\x0csignatureDef\x12>\n\x0easset_file_def\x18\x06\x20\x03(\x0b2\x18.\
    tensorflow.AssetFileDefR\x0cassetFileDef\x1a\xa3\x02\n\x0bMetaInfoDef\
    \x12,\n\x12meta_graph_version\x18\x01\x20\x01(\tR\x10metaGraphVersion\
    \x12<\n\x10stripped_op_list\x18\x02\x20\x01(\x0b2\x12.tensorflow.OpListR\
    \x0estrippedOpList\x12/\n\x08any_info\x18\x03\x20\x01(\x0b2\x14.google.p\
    rotobuf.AnyR\x07anyInfo\x12\x12\n\x04tags\x18\x04\x20\x03(\tR\x04tags\
    \x12-\n\x12tensorflow_version\x18\x05\x20\x01(\tR\x11tensorflowVersion\
    \x124\n\x16tensorflow_git_version\x18\x06\x20\x01(\tR\x14tensorflowGitVe\
    rsion\x1a[\n\x12CollectionDefEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\
    \x03key\x12/\n\x05value\x18\x02\x20\x01(\x0b2\x19.tensorflow.CollectionD\
    efR\x05value:\x028\x01\x1aY\n\x11SignatureDefEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\tR\x03key\x12.\n\x05value\x18\x02\x20\x01(\x0b2\x18.tensor\
    flow.SignatureDefR\x05value:\x028\x01\"\xb6\x04\n\rCollectionDef\x12A\n\
    \tnode_list\x18\x01\x20\x01(\x0b2\".tensorflow.CollectionDef.NodeListH\0\
    R\x08nodeList\x12D\n\nbytes_list\x18\x02\x20\x01(\x0b2#.tensorflow.Colle\
    ctionDef.BytesListH\0R\tbytesList\x12D\n\nint64_list\x18\x03\x20\x01(\
    \x0b2#.tensorflow.CollectionDef.Int64ListH\0R\tint64List\x12D\n\nfloat_l\
    ist\x18\x04\x20\x01(\x0b2#.tensorflow.CollectionDef.FloatListH\0R\tfloat\
    List\x12>\n\x08any_list\x18\x05\x20\x01(\x0b2!.tensorflow.CollectionDef.\
    AnyListH\0R\x07anyList\x1a\x20\n\x08NodeList\x12\x14\n\x05value\x18\x01\
    \x20\x03(\tR\x05value\x1a!\n\tBytesList\x12\x14\n\x05value\x18\x01\x20\
    \x03(\x0cR\x05value\x1a%\n\tInt64List\x12\x18\n\x05value\x18\x01\x20\x03\
    (\x03R\x05valueB\x02\x10\x01\x1a%\n\tFloatList\x12\x18\n\x05value\x18\
    \x01\x20\x03(\x02R\x05valueB\x02\x10\x01\x1a5\n\x07AnyList\x12*\n\x05val\
    ue\x18\x01\x20\x03(\x0b2\x14.google.protobuf.AnyR\x05valueB\x06\n\x04kin\
    d\"\x81\x03\n\nTensorInfo\x12\x14\n\x04name\x18\x01\x20\x01(\tH\0R\x04na\
    me\x12A\n\ncoo_sparse\x18\x04\x20\x01(\x0b2\x20.tensorflow.TensorInfo.Co\
    oSparseH\0R\tcooSparse\x12*\n\x05dtype\x18\x02\x20\x01(\x0e2\x14.tensorf\
    low.DataTypeR\x05dtype\x12?\n\x0ctensor_shape\x18\x03\x20\x01(\x0b2\x1c.\
    tensorflow.TensorShapeProtoR\x0btensorShape\x1a\xa0\x01\n\tCooSparse\x12\
    ,\n\x12values_tensor_name\x18\x01\x20\x01(\tR\x10valuesTensorName\x12.\n\
    \x13indices_tensor_name\x18\x02\x20\x01(\tR\x11indicesTensorName\x125\n\
    \x17dense_shape_tensor_name\x18\x03\x20\x01(\tR\x14denseShapeTensorNameB\
    \n\n\x08encoding\"\xd5\x02\n\x0cSignatureDef\x12<\n\x06inputs\x18\x01\
    \x20\x03(\x0b2$.tensorflow.SignatureDef.InputsEntryR\x06inputs\x12?\n\
    \x07outputs\x18\x02\x20\x03(\x0b2%.tensorflow.SignatureDef.OutputsEntryR\
    \x07outputs\x12\x1f\n\x0bmethod_name\x18\x03\x20\x01(\tR\nmethodName\x1a\
    Q\n\x0bInputsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12,\n\
    \x05value\x18\x02\x20\x01(\x0b2\x16.tensorflow.TensorInfoR\x05value:\x02\
    8\x01\x1aR\n\x0cOutputsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\
    \x12,\n\x05value\x18\x02\x20\x01(\x0b2\x16.tensorflow.TensorInfoR\x05val\
    ue:\x028\x01\"c\n\x0cAssetFileDef\x127\n\x0btensor_info\x18\x01\x20\x01(\
    \x0b2\x16.tensorflow.TensorInfoR\ntensorInfo\x12\x1a\n\x08filename\x18\
    \x02\x20\x01(\tR\x08filenameB0\n\x18org.tensorflow.frameworkB\x0fMetaGra\
    phProtosP\x01\xf8\x01\x01J\xb3Z\n\x07\x12\x05\0\0\xbd\x02\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\
    \x08\x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\
    \n\x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\
    \0\x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\
    \x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\
    \x08\x12\x03\x04\00\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\00\n\x0c\n\
    \x05\x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\
    \x02\0\x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\
    \x04\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e/\n\x08\n\
    \x01\x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\
    \x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\
    \x02\x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\
    \x03\x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\
    \n\x01\x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\
    \x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\
    \x03\x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\
    \x03\x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\
    \x02\x03\0\x12\x03\x08\x07\"\n\t\n\x02\x03\x01\x12\x03\n\x07.\n\t\n\x02\
    \x03\x02\x12\x03\x0b\x07/\n\t\n\x02\x03\x03\x12\x03\x0c\x075\n\t\n\x02\
    \x03\x04\x12\x03\r\x07.\n\t\n\x02\x03\x05\x12\x03\x0e\x07-\n\x8a\x04\n\
    \x02\x04\0\x12\x04\x1e\0R\x01\x1a\xfd\x03\x20NOTE:\x20This\x20protocol\
    \x20buffer\x20is\x20evolving,\x20and\x20will\x20go\x20through\x20revisio\
    ns\x20in\x20the\n\x20coming\x20months.\n\n\x20Protocol\x20buffer\x20cont\
    aining\x20the\x20following\x20which\x20are\x20necessary\x20to\x20restart\
    \n\x20training,\x20run\x20inference.\x20It\x20can\x20be\x20used\x20to\
    \x20serialize/de-serialize\x20memory\n\x20objects\x20necessary\x20for\
    \x20running\x20computation\x20in\x20a\x20graph\x20when\x20crossing\x20th\
    e\n\x20process\x20boundary.\x20It\x20can\x20be\x20used\x20for\x20long\
    \x20term\x20storage\x20of\x20graphs,\n\x20cross-language\x20execution\
    \x20of\x20graphs,\x20etc.\n\x20\x20\x20MetaInfoDef\n\x20\x20\x20GraphDef\
    \n\x20\x20\x20SaverDef\n\x20\x20\x20CollectionDef\n\x20\x20\x20TensorInf\
    o\n\x20\x20\x20SignatureDef\n\n\n\n\x03\x04\0\x01\x12\x03\x1e\x08\x14\n\
    \xa5\x01\n\x04\x04\0\x03\0\x12\x04!\x02?\x03\x1a\x96\x01\x20Meta\x20info\
    rmation\x20regarding\x20the\x20graph\x20to\x20be\x20exported.\x20\x20To\
    \x20be\x20used\x20by\x20users\n\x20of\x20this\x20protocol\x20buffer\x20t\
    o\x20encode\x20information\x20regarding\x20their\x20meta\x20graph.\n\n\
    \x0c\n\x05\x04\0\x03\0\x01\x12\x03!\n\x15\n\x86\x01\n\x06\x04\0\x03\0\
    \x02\0\x12\x03$\x04\"\x1aw\x20User\x20specified\x20Version\x20string.\
    \x20Can\x20be\x20the\x20name\x20of\x20the\x20model\x20and\x20revision,\n\
    \x20steps\x20this\x20model\x20has\x20been\x20trained\x20to,\x20etc.\n\n\
    \x0f\n\x07\x04\0\x03\0\x02\0\x04\x12\x04$\x04!\x17\n\x0e\n\x07\x04\0\x03\
    \0\x02\0\x05\x12\x03$\x04\n\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\x03$\
    \x0b\x1d\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03$\x20!\n\x8b\x01\n\x06\
    \x04\0\x03\0\x02\x01\x12\x03(\x04\x20\x1a|\x20A\x20copy\x20of\x20the\x20\
    OpDefs\x20used\x20by\x20the\x20producer\x20of\x20this\x20graph_def.\n\
    \x20Descriptions\x20and\x20Ops\x20not\x20used\x20in\x20graph_def\x20are\
    \x20stripped\x20out.\n\n\x0f\n\x07\x04\0\x03\0\x02\x01\x04\x12\x04(\x04$\
    \"\n\x0e\n\x07\x04\0\x03\0\x02\x01\x06\x12\x03(\x04\n\n\x0e\n\x07\x04\0\
    \x03\0\x02\x01\x01\x12\x03(\x0b\x1b\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\
    \x12\x03(\x1e\x1f\nw\n\x06\x04\0\x03\0\x02\x02\x12\x03,\x04%\x1ah\x20A\
    \x20serialized\x20protobuf.\x20Can\x20be\x20the\x20time\x20this\x20meta\
    \x20graph\x20is\x20created,\x20or\n\x20modified,\x20or\x20name\x20of\x20\
    the\x20model.\n\n\x0f\n\x07\x04\0\x03\0\x02\x02\x04\x12\x04,\x04(\x20\n\
    \x0e\n\x07\x04\0\x03\0\x02\x02\x06\x12\x03,\x04\x17\n\x0e\n\x07\x04\0\
    \x03\0\x02\x02\x01\x12\x03,\x18\x20\n\x0e\n\x07\x04\0\x03\0\x02\x02\x03\
    \x12\x03,#$\n\xba\x02\n\x06\x04\0\x03\0\x02\x03\x12\x034\x04\x1d\x1a\xaa\
    \x02\x20User\x20supplied\x20tag(s)\x20on\x20the\x20meta_graph\x20and\x20\
    included\x20graph_def.\n\n\x20MetaGraphDefs\x20should\x20be\x20tagged\
    \x20with\x20their\x20capabilities\x20or\x20use-cases.\n\x20Examples:\x20\
    \"train\",\x20\"serve\",\x20\"gpu\",\x20\"tpu\",\x20etc.\n\x20These\x20t\
    ags\x20enable\x20loaders\x20to\x20access\x20the\x20MetaGraph(s)\x20appro\
    priate\x20for\x20a\n\x20specific\x20use-case\x20or\x20runtime\x20environ\
    ment.\n\n\x0e\n\x07\x04\0\x03\0\x02\x03\x04\x12\x034\x04\x0c\n\x0e\n\x07\
    \x04\0\x03\0\x02\x03\x05\x12\x034\r\x13\n\x0e\n\x07\x04\0\x03\0\x02\x03\
    \x01\x12\x034\x14\x18\n\x0e\n\x07\x04\0\x03\0\x02\x03\x03\x12\x034\x1b\
    \x1c\n\xb3\x01\n\x06\x04\0\x03\0\x02\x04\x12\x039\x04\"\x1a\xa3\x01\x20T\
    he\x20__version__\x20string\x20of\x20the\x20tensorflow\x20build\x20used\
    \x20to\x20write\x20this\x20graph.\n\x20This\x20will\x20be\x20populated\
    \x20by\x20the\x20framework,\x20which\x20will\x20overwrite\x20any\x20user\
    \n\x20supplied\x20value.\n\n\x0f\n\x07\x04\0\x03\0\x02\x04\x04\x12\x049\
    \x044\x1d\n\x0e\n\x07\x04\0\x03\0\x02\x04\x05\x12\x039\x04\n\n\x0e\n\x07\
    \x04\0\x03\0\x02\x04\x01\x12\x039\x0b\x1d\n\x0e\n\x07\x04\0\x03\0\x02\
    \x04\x03\x12\x039\x20!\n\xb7\x01\n\x06\x04\0\x03\0\x02\x05\x12\x03>\x04&\
    \x1a\xa7\x01\x20The\x20__git_version__\x20string\x20of\x20the\x20tensorf\
    low\x20build\x20used\x20to\x20write\x20this\n\x20graph.\x20This\x20will\
    \x20be\x20populated\x20by\x20the\x20framework,\x20which\x20will\x20overw\
    rite\x20any\n\x20user\x20supplied\x20value.\n\n\x0f\n\x07\x04\0\x03\0\
    \x02\x05\x04\x12\x04>\x049\"\n\x0e\n\x07\x04\0\x03\0\x02\x05\x05\x12\x03\
    >\x04\n\n\x0e\n\x07\x04\0\x03\0\x02\x05\x01\x12\x03>\x0b!\n\x0e\n\x07\
    \x04\0\x03\0\x02\x05\x03\x12\x03>$%\n\x0b\n\x04\x04\0\x02\0\x12\x03@\x02\
    \x20\n\r\n\x05\x04\0\x02\0\x04\x12\x04@\x02?\x03\n\x0c\n\x05\x04\0\x02\0\
    \x06\x12\x03@\x02\r\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03@\x0e\x1b\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03@\x1e\x1f\n\x18\n\x04\x04\0\x02\x01\x12\x03C\
    \x02\x19\x1a\x0b\x20GraphDef.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04C\x02\
    @\x20\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03C\x02\n\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03C\x0b\x14\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03C\x17\x18\
    \n\x18\n\x04\x04\0\x02\x02\x12\x03F\x02\x19\x1a\x0b\x20SaverDef.\n\n\r\n\
    \x05\x04\0\x02\x02\x04\x12\x04F\x02C\x19\n\x0c\n\x05\x04\0\x02\x02\x06\
    \x12\x03F\x02\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03F\x0b\x14\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03F\x17\x18\no\n\x04\x04\0\x02\x03\x12\x03J\
    \x020\x1ab\x20collection_def:\x20Map\x20from\x20collection\x20name\x20to\
    \x20collections.\n\x20See\x20CollectionDef\x20section\x20for\x20details.\
    \n\n\r\n\x05\x04\0\x02\x03\x04\x12\x04J\x02F\x19\n\x0c\n\x05\x04\0\x02\
    \x03\x06\x12\x03J\x02\x1c\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03J\x1d+\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03J./\nc\n\x04\x04\0\x02\x04\x12\x03N\
    \x02.\x1aV\x20signature_def:\x20Map\x20from\x20user\x20supplied\x20key\
    \x20for\x20a\x20signature\x20to\x20a\x20single\n\x20SignatureDef.\n\n\r\
    \n\x05\x04\0\x02\x04\x04\x12\x04N\x02J0\n\x0c\n\x05\x04\0\x02\x04\x06\
    \x12\x03N\x02\x1b\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03N\x1c)\n\x0c\n\
    \x05\x04\0\x02\x04\x03\x12\x03N,-\n@\n\x04\x04\0\x02\x05\x12\x03Q\x02+\
    \x1a3\x20Asset\x20file\x20def\x20to\x20be\x20used\x20with\x20the\x20defi\
    ned\x20graph.\n\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x03Q\x02\n\n\x0c\n\x05\
    \x04\0\x02\x05\x06\x12\x03Q\x0b\x17\n\x0c\n\x05\x04\0\x02\x05\x01\x12\
    \x03Q\x18&\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03Q)*\n\xdf\x0f\n\x02\x04\
    \x01\x12\x06\x92\x01\0\xc8\x01\x01\x1a\xd0\x0f\x20CollectionDef\x20shoul\
    d\x20cover\x20most\x20collections.\n\x20To\x20add\x20a\x20user-defined\
    \x20collection,\x20do\x20one\x20of\x20the\x20following:\n\x201.\x20For\
    \x20simple\x20data\x20types,\x20such\x20as\x20string,\x20int,\x20float:\
    \n\x20\x20\x20\x20\x20\x20tf.add_to_collection(\"your_collection_name\",\
    \x20your_simple_value)\n\x20\x20\x20\x20strings\x20will\x20be\x20stored\
    \x20as\x20bytes_list.\n\n\x202.\x20For\x20Protobuf\x20types,\x20there\
    \x20are\x20three\x20ways\x20to\x20add\x20them:\n\x20\x20\x20\x201)\x20tf\
    .add_to_collection(\"your_collection_name\",\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20your_proto.SerializeToString())\n\n\x20\x20\x20\x20\x20\x20\
    \x20collection_def\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20key:\x20\"u\
    ser_defined_bytes_collection\"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20valu\
    e\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20bytes_list\x20{\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20\"queue_na\
    me:\x20\\\"test_queue\\\"\\n\"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\
    \x20}\n\n\x20\x20or\n\n\x20\x20\x20\x202)\x20tf.add_to_collection(\"your\
    _collection_name\",\x20str(your_proto))\n\n\x20\x20\x20\x20\x20\x20\x20c\
    ollection_def\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20key:\x20\"user_d\
    efined_string_collection\"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20value\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20bytes_list\x20{\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20\"\\n\\ntest_queue\
    \"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20}\n\n\x20\x20or\n\n\x20\
    \x20\x20\x203)\x20any_buf\x20=\x20any_pb2.Any()\n\x20\x20\x20\x20\x20\
    \x20\x20tf.add_to_collection(\"your_collection_name\",\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20any_buf.Pack(your_proto))\n\n\x20\x20\x20\x20\x20\
    \x20\x20collection_def\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20key:\
    \x20\"user_defined_any_collection\"\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20value\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20any_list\x20\
    {\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20value\x20{\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20type_url:\x20\"t\
    ype.googleapis.com/tensorflow.QueueRunnerDef\"\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20\"\\n\\ntest_queue\"\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20}\n\x20\
    \x20\x20\x20\x20\x20\x20}\n\n\x203.\x20For\x20Python\x20objects,\x20impl\
    ement\x20to_proto()\x20and\x20from_proto(),\x20and\x20register\n\x20\x20\
    \x20\x20them\x20in\x20the\x20following\x20manner:\n\x20\x20\x20\x20ops.r\
    egister_proto_function(\"your_collection_name\",\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20proto_type,\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20to_proto=YourPythonObject.to_proto,\
    \n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20from_proto=Y\
    ourPythonObject.from_proto)\n\x20\x20\x20\x20These\x20functions\x20will\
    \x20be\x20invoked\x20to\x20serialize\x20and\x20de-serialize\x20the\n\x20\
    \x20\x20\x20collection.\x20For\x20example,\n\x20\x20\x20\x20ops.register\
    _proto_function(ops.GraphKeys.GLOBAL_VARIABLES,\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20proto_type=variable_pb2.VariableDef,\
    \n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20to_proto=Var\
    iable.to_proto,\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20from_proto=Variable.from_proto)\n\n\x0b\n\x03\x04\x01\x01\x12\x04\
    \x92\x01\x08\x15\n\x98\x02\n\x04\x04\x01\x03\0\x12\x06\x9d\x01\x02\x9f\
    \x01\x03\x1a\x87\x02\x20NodeList\x20is\x20used\x20for\x20collecting\x20n\
    odes\x20in\x20graph.\x20For\x20example\n\x20collection_def\x20{\n\x20\
    \x20\x20key:\x20\"summaries\"\n\x20\x20\x20value\x20{\n\x20\x20\x20\x20\
    \x20node_list\x20{\n\x20\x20\x20\x20\x20\x20\x20value:\x20\"input_produc\
    er/ScalarSummary:0\"\n\x20\x20\x20\x20\x20\x20\x20value:\x20\"shuffle_ba\
    tch/ScalarSummary:0\"\n\x20\x20\x20\x20\x20\x20\x20value:\x20\"ImageSumm\
    ary:0\"\n\x20\x20\x20\x20\x20}\n\x20\x20\x20}\n\n\r\n\x05\x04\x01\x03\0\
    \x01\x12\x04\x9d\x01\n\x12\n\x0e\n\x06\x04\x01\x03\0\x02\0\x12\x04\x9e\
    \x01\x04\x1e\n\x0f\n\x07\x04\x01\x03\0\x02\0\x04\x12\x04\x9e\x01\x04\x0c\
    \n\x0f\n\x07\x04\x01\x03\0\x02\0\x05\x12\x04\x9e\x01\r\x13\n\x0f\n\x07\
    \x04\x01\x03\0\x02\0\x01\x12\x04\x9e\x01\x14\x19\n\x0f\n\x07\x04\x01\x03\
    \0\x02\0\x03\x12\x04\x9e\x01\x1c\x1d\n\x96\x03\n\x04\x04\x01\x03\x01\x12\
    \x06\xae\x01\x02\xb0\x01\x03\x1a\x85\x03\x20BytesList\x20is\x20used\x20f\
    or\x20collecting\x20strings\x20and\x20serialized\x20protobufs.\x20For\n\
    \x20example:\n\x20collection_def\x20{\n\x20\x20\x20key:\x20\"trainable_v\
    ariables\"\n\x20\x20\x20value\x20{\n\x20\x20\x20\x20\x20bytes_list\x20{\
    \n\x20\x20\x20\x20\x20\x20\x20value:\x20\"\\n\\017conv1/weights:0\\022\\\
    024conv1/weights/Assign\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\\032\\024conv1/weights/read:0\"\n\x20\x20\x20\x20\x20\x20\
    \x20value:\x20\"\\n\\016conv1/biases:0\\022\\023conv1/biases/Assign\\032\
    \n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\\023conv1/bia\
    ses/read:0\"\n\x20\x20\x20\x20\x20}\n\x20\x20\x20}\n\x20}\n\n\r\n\x05\
    \x04\x01\x03\x01\x01\x12\x04\xae\x01\n\x13\n\x0e\n\x06\x04\x01\x03\x01\
    \x02\0\x12\x04\xaf\x01\x04\x1d\n\x0f\n\x07\x04\x01\x03\x01\x02\0\x04\x12\
    \x04\xaf\x01\x04\x0c\n\x0f\n\x07\x04\x01\x03\x01\x02\0\x05\x12\x04\xaf\
    \x01\r\x12\n\x0f\n\x07\x04\x01\x03\x01\x02\0\x01\x12\x04\xaf\x01\x13\x18\
    \n\x0f\n\x07\x04\x01\x03\x01\x02\0\x03\x12\x04\xaf\x01\x1b\x1c\nN\n\x04\
    \x04\x01\x03\x02\x12\x06\xb3\x01\x02\xb5\x01\x03\x1a>\x20Int64List\x20is\
    \x20used\x20for\x20collecting\x20int,\x20int64\x20and\x20long\x20values.\
    \n\n\r\n\x05\x04\x01\x03\x02\x01\x12\x04\xb3\x01\n\x13\n\x0e\n\x06\x04\
    \x01\x03\x02\x02\0\x12\x04\xb4\x01\x04-\n\x0f\n\x07\x04\x01\x03\x02\x02\
    \0\x04\x12\x04\xb4\x01\x04\x0c\n\x0f\n\x07\x04\x01\x03\x02\x02\0\x05\x12\
    \x04\xb4\x01\r\x12\n\x0f\n\x07\x04\x01\x03\x02\x02\0\x01\x12\x04\xb4\x01\
    \x13\x18\n\x0f\n\x07\x04\x01\x03\x02\x02\0\x03\x12\x04\xb4\x01\x1b\x1c\n\
    \x0f\n\x07\x04\x01\x03\x02\x02\0\x08\x12\x04\xb4\x01\x1d,\n\x12\n\n\x04\
    \x01\x03\x02\x02\0\x08\xe7\x07\0\x12\x04\xb4\x01\x1e+\n\x13\n\x0b\x04\
    \x01\x03\x02\x02\0\x08\xe7\x07\0\x02\x12\x04\xb4\x01\x1e$\n\x14\n\x0c\
    \x04\x01\x03\x02\x02\0\x08\xe7\x07\0\x02\0\x12\x04\xb4\x01\x1e$\n\x15\n\
    \r\x04\x01\x03\x02\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x04\xb4\x01\x1e$\n\
    \x13\n\x0b\x04\x01\x03\x02\x02\0\x08\xe7\x07\0\x03\x12\x04\xb4\x01'+\n@\
    \n\x04\x04\x01\x03\x03\x12\x06\xb8\x01\x02\xba\x01\x03\x1a0\x20FloatList\
    \x20is\x20used\x20for\x20collecting\x20float\x20values.\n\n\r\n\x05\x04\
    \x01\x03\x03\x01\x12\x04\xb8\x01\n\x13\n\x0e\n\x06\x04\x01\x03\x03\x02\0\
    \x12\x04\xb9\x01\x04-\n\x0f\n\x07\x04\x01\x03\x03\x02\0\x04\x12\x04\xb9\
    \x01\x04\x0c\n\x0f\n\x07\x04\x01\x03\x03\x02\0\x05\x12\x04\xb9\x01\r\x12\
    \n\x0f\n\x07\x04\x01\x03\x03\x02\0\x01\x12\x04\xb9\x01\x13\x18\n\x0f\n\
    \x07\x04\x01\x03\x03\x02\0\x03\x12\x04\xb9\x01\x1b\x1c\n\x0f\n\x07\x04\
    \x01\x03\x03\x02\0\x08\x12\x04\xb9\x01\x1d,\n\x12\n\n\x04\x01\x03\x03\
    \x02\0\x08\xe7\x07\0\x12\x04\xb9\x01\x1e+\n\x13\n\x0b\x04\x01\x03\x03\
    \x02\0\x08\xe7\x07\0\x02\x12\x04\xb9\x01\x1e$\n\x14\n\x0c\x04\x01\x03\
    \x03\x02\0\x08\xe7\x07\0\x02\0\x12\x04\xb9\x01\x1e$\n\x15\n\r\x04\x01\
    \x03\x03\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x04\xb9\x01\x1e$\n\x13\n\x0b\
    \x04\x01\x03\x03\x02\0\x08\xe7\x07\0\x03\x12\x04\xb9\x01'+\n<\n\x04\x04\
    \x01\x03\x04\x12\x06\xbd\x01\x02\xbf\x01\x03\x1a,\x20AnyList\x20is\x20us\
    ed\x20for\x20collecting\x20Any\x20protos.\n\n\r\n\x05\x04\x01\x03\x04\
    \x01\x12\x04\xbd\x01\n\x11\n\x0e\n\x06\x04\x01\x03\x04\x02\0\x12\x04\xbe\
    \x01\x04+\n\x0f\n\x07\x04\x01\x03\x04\x02\0\x04\x12\x04\xbe\x01\x04\x0c\
    \n\x0f\n\x07\x04\x01\x03\x04\x02\0\x06\x12\x04\xbe\x01\r\x20\n\x0f\n\x07\
    \x04\x01\x03\x04\x02\0\x01\x12\x04\xbe\x01!&\n\x0f\n\x07\x04\x01\x03\x04\
    \x02\0\x03\x12\x04\xbe\x01)*\n\x0e\n\x04\x04\x01\x08\0\x12\x06\xc1\x01\
    \x02\xc7\x01\x03\n\r\n\x05\x04\x01\x08\0\x01\x12\x04\xc1\x01\x08\x0c\n\
    \x0c\n\x04\x04\x01\x02\0\x12\x04\xc2\x01\x04\x1b\n\r\n\x05\x04\x01\x02\0\
    \x06\x12\x04\xc2\x01\x04\x0c\n\r\n\x05\x04\x01\x02\0\x01\x12\x04\xc2\x01\
    \r\x16\n\r\n\x05\x04\x01\x02\0\x03\x12\x04\xc2\x01\x19\x1a\n\x0c\n\x04\
    \x04\x01\x02\x01\x12\x04\xc3\x01\x04\x1d\n\r\n\x05\x04\x01\x02\x01\x06\
    \x12\x04\xc3\x01\x04\r\n\r\n\x05\x04\x01\x02\x01\x01\x12\x04\xc3\x01\x0e\
    \x18\n\r\n\x05\x04\x01\x02\x01\x03\x12\x04\xc3\x01\x1b\x1c\n\x0c\n\x04\
    \x04\x01\x02\x02\x12\x04\xc4\x01\x04\x1d\n\r\n\x05\x04\x01\x02\x02\x06\
    \x12\x04\xc4\x01\x04\r\n\r\n\x05\x04\x01\x02\x02\x01\x12\x04\xc4\x01\x0e\
    \x18\n\r\n\x05\x04\x01\x02\x02\x03\x12\x04\xc4\x01\x1b\x1c\n\x0c\n\x04\
    \x04\x01\x02\x03\x12\x04\xc5\x01\x04\x1d\n\r\n\x05\x04\x01\x02\x03\x06\
    \x12\x04\xc5\x01\x04\r\n\r\n\x05\x04\x01\x02\x03\x01\x12\x04\xc5\x01\x0e\
    \x18\n\r\n\x05\x04\x01\x02\x03\x03\x12\x04\xc5\x01\x1b\x1c\n\x0c\n\x04\
    \x04\x01\x02\x04\x12\x04\xc6\x01\x04\x19\n\r\n\x05\x04\x01\x02\x04\x06\
    \x12\x04\xc6\x01\x04\x0b\n\r\n\x05\x04\x01\x02\x04\x01\x12\x04\xc6\x01\
    \x0c\x14\n\r\n\x05\x04\x01\x02\x04\x03\x12\x04\xc6\x01\x17\x18\nN\n\x02\
    \x04\x02\x12\x06\xcb\x01\0\xe9\x01\x01\x1a@\x20Information\x20about\x20a\
    \x20Tensor\x20necessary\x20for\x20feeding\x20or\x20retrieval.\n\n\x0b\n\
    \x03\x04\x02\x01\x12\x04\xcb\x01\x08\x12\nf\n\x04\x04\x02\x03\0\x12\x06\
    \xce\x01\x02\xd9\x01\x03\x1aV\x20For\x20sparse\x20tensors,\x20The\x20COO\
    \x20encoding\x20stores\x20a\x20triple\x20of\x20values,\x20indices,\n\x20\
    and\x20shape.\n\n\r\n\x05\x04\x02\x03\0\x01\x12\x04\xce\x01\n\x13\n\x9a\
    \x01\n\x06\x04\x02\x03\0\x02\0\x12\x04\xd1\x01\x04\"\x1a\x89\x01\x20The\
    \x20shape\x20of\x20the\x20values\x20Tensor\x20is\x20[?].\x20\x20Its\x20d\
    type\x20must\x20be\x20the\x20dtype\x20of\n\x20the\x20SparseTensor\x20as\
    \x20a\x20whole,\x20given\x20in\x20the\x20enclosing\x20TensorInfo.\n\n\
    \x11\n\x07\x04\x02\x03\0\x02\0\x04\x12\x06\xd1\x01\x04\xce\x01\x15\n\x0f\
    \n\x07\x04\x02\x03\0\x02\0\x05\x12\x04\xd1\x01\x04\n\n\x0f\n\x07\x04\x02\
    \x03\0\x02\0\x01\x12\x04\xd1\x01\x0b\x1d\n\x0f\n\x07\x04\x02\x03\0\x02\0\
    \x03\x12\x04\xd1\x01\x20!\nL\n\x06\x04\x02\x03\0\x02\x01\x12\x04\xd4\x01\
    \x04#\x1a<\x20The\x20indices\x20Tensor\x20must\x20have\x20dtype\x20int64\
    \x20and\x20shape\x20[?,\x20?].\n\n\x11\n\x07\x04\x02\x03\0\x02\x01\x04\
    \x12\x06\xd4\x01\x04\xd1\x01\"\n\x0f\n\x07\x04\x02\x03\0\x02\x01\x05\x12\
    \x04\xd4\x01\x04\n\n\x0f\n\x07\x04\x02\x03\0\x02\x01\x01\x12\x04\xd4\x01\
    \x0b\x1e\n\x0f\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x04\xd4\x01!\"\n\xa1\
    \x01\n\x06\x04\x02\x03\0\x02\x02\x12\x04\xd8\x01\x04'\x1a\x90\x01\x20The\
    \x20dynamic\x20logical\x20shape\x20represented\x20by\x20the\x20SparseTen\
    sor\x20is\x20recorded\x20in\n\x20the\x20Tensor\x20referenced\x20here.\
    \x20\x20It\x20must\x20have\x20dtype\x20int64\x20and\x20shape\x20[?].\n\n\
    \x11\n\x07\x04\x02\x03\0\x02\x02\x04\x12\x06\xd8\x01\x04\xd4\x01#\n\x0f\
    \n\x07\x04\x02\x03\0\x02\x02\x05\x12\x04\xd8\x01\x04\n\n\x0f\n\x07\x04\
    \x02\x03\0\x02\x02\x01\x12\x04\xd8\x01\x0b\"\n\x0f\n\x07\x04\x02\x03\0\
    \x02\x02\x03\x12\x04\xd8\x01%&\n\x0e\n\x04\x04\x02\x08\0\x12\x06\xdb\x01\
    \x02\xe3\x01\x03\n\r\n\x05\x04\x02\x08\0\x01\x12\x04\xdb\x01\x08\x10\nI\
    \n\x04\x04\x02\x02\0\x12\x04\xdd\x01\x04\x14\x1a;\x20For\x20dense\x20`Te\
    nsor`s,\x20the\x20name\x20of\x20the\x20tensor\x20in\x20the\x20graph.\n\n\
    \r\n\x05\x04\x02\x02\0\x05\x12\x04\xdd\x01\x04\n\n\r\n\x05\x04\x02\x02\0\
    \x01\x12\x04\xdd\x01\x0b\x0f\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\xdd\x01\
    \x12\x13\n\xee\x01\n\x04\x04\x02\x02\x01\x12\x04\xe2\x01\x04\x1d\x1a\xdf\
    \x01\x20There\x20are\x20many\x20possible\x20encodings\x20of\x20sparse\
    \x20matrices\n\x20(https://en.wikipedia.org/wiki/Sparse_matrix).\x20\x20\
    Currently,\x20TensorFlow\n\x20uses\x20only\x20the\x20COO\x20encoding.\
    \x20\x20This\x20is\x20supported\x20and\x20documented\x20in\x20the\n\x20S\
    parseTensor\x20Python\x20class.\n\n\r\n\x05\x04\x02\x02\x01\x06\x12\x04\
    \xe2\x01\x04\r\n\r\n\x05\x04\x02\x02\x01\x01\x12\x04\xe2\x01\x0e\x18\n\r\
    \n\x05\x04\x02\x02\x01\x03\x12\x04\xe2\x01\x1b\x1c\n\x0c\n\x04\x04\x02\
    \x02\x02\x12\x04\xe4\x01\x02\x15\n\x0f\n\x05\x04\x02\x02\x02\x04\x12\x06\
    \xe4\x01\x02\xe3\x01\x03\n\r\n\x05\x04\x02\x02\x02\x06\x12\x04\xe4\x01\
    \x02\n\n\r\n\x05\x04\x02\x02\x02\x01\x12\x04\xe4\x01\x0b\x10\n\r\n\x05\
    \x04\x02\x02\x02\x03\x12\x04\xe4\x01\x13\x14\n\xdf\x01\n\x04\x04\x02\x02\
    \x03\x12\x04\xe8\x01\x02$\x1a\xd0\x01\x20The\x20static\x20shape\x20shoul\
    d\x20be\x20recorded\x20here,\x20to\x20the\x20extent\x20that\x20it\x20can\
    \n\x20be\x20known\x20in\x20advance.\x20\x20In\x20the\x20case\x20of\x20a\
    \x20SparseTensor,\x20this\x20field\x20describes\n\x20the\x20logical\x20s\
    hape\x20of\x20the\x20represented\x20tensor\x20(aka\x20dense_shape).\n\n\
    \x0f\n\x05\x04\x02\x02\x03\x04\x12\x06\xe8\x01\x02\xe4\x01\x15\n\r\n\x05\
    \x04\x02\x02\x03\x06\x12\x04\xe8\x01\x02\x12\n\r\n\x05\x04\x02\x02\x03\
    \x01\x12\x04\xe8\x01\x13\x1f\n\r\n\x05\x04\x02\x02\x03\x03\x12\x04\xe8\
    \x01\"#\n\xbe\n\n\x02\x04\x03\x12\x06\xa4\x02\0\xb2\x02\x01\x1a\xaf\n\
    \x20SignatureDef\x20defines\x20the\x20signature\x20of\x20a\x20computatio\
    n\x20supported\x20by\x20a\x20TensorFlow\n\x20graph.\n\n\x20For\x20exampl\
    e,\x20a\x20model\x20with\x20two\x20loss\x20computations,\x20sharing\x20a\
    \x20single\x20input,\n\x20might\x20have\x20the\x20following\x20signature\
    _def\x20map.\n\n\x20Note\x20that\x20across\x20the\x20two\x20SignatureDef\
    s\x20\"loss_A\"\x20and\x20\"loss_B\",\x20the\x20input\x20key,\n\x20outpu\
    t\x20key,\x20and\x20method_name\x20are\x20identical,\x20and\x20will\x20b\
    e\x20used\x20by\x20system(s)\x20that\n\x20implement\x20or\x20rely\x20upo\
    n\x20this\x20particular\x20loss\x20method.\x20The\x20output\x20tensor\
    \x20names\n\x20differ,\x20demonstrating\x20how\x20different\x20outputs\
    \x20can\x20exist\x20for\x20the\x20same\x20method.\n\n\x20signature_def\
    \x20{\n\x20\x20\x20key:\x20\"loss_A\"\n\x20\x20\x20value\x20{\n\x20\x20\
    \x20\x20\x20inputs\x20{\n\x20\x20\x20\x20\x20\x20\x20key:\x20\"input\"\n\
    \x20\x20\x20\x20\x20\x20\x20value\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20name:\x20\"input:0\"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20dtype:\x20\
    DT_STRING\n\x20\x20\x20\x20\x20\x20\x20\x20\x20tensor_shape:\x20...\n\
    \x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\
    \x20outputs\x20{\n\x20\x20\x20\x20\x20\x20\x20key:\x20\"loss_output\"\n\
    \x20\x20\x20\x20\x20\x20\x20value\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20name:\x20\"loss_output_A:0\"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20dt\
    ype:\x20DT_FLOAT\n\x20\x20\x20\x20\x20\x20\x20\x20\x20tensor_shape:\x20.\
    ..\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\x20\x20\x20}\
    \n\x20\x20\x20...\n\x20\x20\x20method_name:\x20\"some/package/compute_lo\
    ss\"\n\x20}\n\x20signature_def\x20{\n\x20\x20\x20key:\x20\"loss_B\"\n\
    \x20\x20\x20value\x20{\n\x20\x20\x20\x20\x20inputs\x20{\n\x20\x20\x20\
    \x20\x20\x20\x20key:\x20\"input\"\n\x20\x20\x20\x20\x20\x20\x20value\x20\
    {\n\x20\x20\x20\x20\x20\x20\x20\x20\x20name:\x20\"input:0\"\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20dtype:\x20DT_STRING\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20tensor_shape:\x20...\n\x20\x20\x20\x20\x20\x20\x20}\n\
    \x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20outputs\x20{\n\x20\x20\x20\
    \x20\x20\x20\x20key:\x20\"loss_output\"\n\x20\x20\x20\x20\x20\x20\x20val\
    ue\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20name:\x20\"loss_output_B:0\
    \"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20dtype:\x20DT_FLOAT\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20tensor_shape:\x20...\n\x20\x20\x20\x20\x20\x20\
    \x20}\n\x20\x20\x20\x20\x20}\n\x20\x20\x20}\n\x20\x20\x20...\n\x20\x20\
    \x20method_name:\x20\"some/package/compute_loss\"\n\x20}\n\n\x0b\n\x03\
    \x04\x03\x01\x12\x04\xa4\x02\x08\x14\n'\n\x04\x04\x03\x02\0\x12\x04\xa6\
    \x02\x02%\x1a\x19\x20Named\x20input\x20parameters.\n\n\x0f\n\x05\x04\x03\
    \x02\0\x04\x12\x06\xa6\x02\x02\xa4\x02\x16\n\r\n\x05\x04\x03\x02\0\x06\
    \x12\x04\xa6\x02\x02\x19\n\r\n\x05\x04\x03\x02\0\x01\x12\x04\xa6\x02\x1a\
    \x20\n\r\n\x05\x04\x03\x02\0\x03\x12\x04\xa6\x02#$\n(\n\x04\x04\x03\x02\
    \x01\x12\x04\xa8\x02\x02&\x1a\x1a\x20Named\x20output\x20parameters.\n\n\
    \x0f\n\x05\x04\x03\x02\x01\x04\x12\x06\xa8\x02\x02\xa6\x02%\n\r\n\x05\
    \x04\x03\x02\x01\x06\x12\x04\xa8\x02\x02\x19\n\r\n\x05\x04\x03\x02\x01\
    \x01\x12\x04\xa8\x02\x1a!\n\r\n\x05\x04\x03\x02\x01\x03\x12\x04\xa8\x02$\
    %\n\x91\x04\n\x04\x04\x03\x02\x02\x12\x04\xb1\x02\x02\x19\x1a\x82\x04\
    \x20Extensible\x20method_name\x20information\x20enabling\x20third-party\
    \x20users\x20to\x20mark\x20a\n\x20SignatureDef\x20as\x20supporting\x20a\
    \x20particular\x20method.\x20This\x20enables\x20producers\x20and\n\x20co\
    nsumers\x20of\x20SignatureDefs,\x20e.g.\x20a\x20model\x20definition\x20l\
    ibrary\x20and\x20a\x20serving\n\x20library\x20to\x20have\x20a\x20clear\
    \x20hand-off\x20regarding\x20the\x20semantics\x20of\x20a\x20computation.\
    \n\n\x20Note\x20that\x20multiple\x20SignatureDefs\x20in\x20a\x20single\
    \x20MetaGraphDef\x20may\x20have\x20the\x20same\n\x20method_name.\x20This\
    \x20is\x20commonly\x20used\x20to\x20support\x20multi-headed\x20computati\
    on,\n\x20where\x20a\x20single\x20graph\x20computation\x20may\x20return\
    \x20multiple\x20results.\n\n\x0f\n\x05\x04\x03\x02\x02\x04\x12\x06\xb1\
    \x02\x02\xa8\x02&\n\r\n\x05\x04\x03\x02\x02\x05\x12\x04\xb1\x02\x02\x08\
    \n\r\n\x05\x04\x03\x02\x02\x01\x12\x04\xb1\x02\t\x14\n\r\n\x05\x04\x03\
    \x02\x02\x03\x12\x04\xb1\x02\x17\x18\nb\n\x02\x04\x04\x12\x06\xb6\x02\0\
    \xbd\x02\x01\x1aT\x20An\x20asset\x20file\x20def\x20for\x20a\x20single\
    \x20file\x20or\x20a\x20set\x20of\x20sharded\x20files\x20with\x20the\x20s\
    ame\n\x20name.\n\n\x0b\n\x03\x04\x04\x01\x12\x04\xb6\x02\x08\x14\n9\n\
    \x04\x04\x04\x02\0\x12\x04\xb8\x02\x02\x1d\x1a+\x20The\x20tensor\x20to\
    \x20bind\x20the\x20asset\x20filename\x20to.\n\n\x0f\n\x05\x04\x04\x02\0\
    \x04\x12\x06\xb8\x02\x02\xb6\x02\x16\n\r\n\x05\x04\x04\x02\0\x06\x12\x04\
    \xb8\x02\x02\x0c\n\r\n\x05\x04\x04\x02\0\x01\x12\x04\xb8\x02\r\x18\n\r\n\
    \x05\x04\x04\x02\0\x03\x12\x04\xb8\x02\x1b\x1c\n\xbd\x01\n\x04\x04\x04\
    \x02\x01\x12\x04\xbc\x02\x02\x16\x1a\xae\x01\x20The\x20filename\x20withi\
    n\x20an\x20assets\x20directory.\x20Note:\x20does\x20not\x20include\x20th\
    e\x20path\n\x20prefix,\x20i.e.\x20directories.\x20For\x20an\x20asset\x20\
    at\x20/tmp/path/vocab.txt,\x20the\x20filename\n\x20would\x20be\x20\"voca\
    b.txt\".\n\n\x0f\n\x05\x04\x04\x02\x01\x04\x12\x06\xbc\x02\x02\xb8\x02\
    \x1d\n\r\n\x05\x04\x04\x02\x01\x05\x12\x04\xbc\x02\x02\x08\n\r\n\x05\x04\
    \x04\x02\x01\x01\x12\x04\xbc\x02\t\x11\n\r\n\x05\x04\x04\x02\x01\x03\x12\
    \x04\xbc\x02\x14\x15b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
