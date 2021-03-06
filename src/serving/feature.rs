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
pub struct BytesList {
    // message fields
    pub value: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BytesList {}

impl BytesList {
    pub fn new() -> BytesList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BytesList {
        static mut instance: ::protobuf::lazy::Lazy<BytesList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BytesList,
        };
        unsafe {
            instance.get(BytesList::new)
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

impl ::protobuf::Message for BytesList {
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

impl ::protobuf::MessageStatic for BytesList {
    fn new() -> BytesList {
        BytesList::new()
    }

    fn descriptor_static(_: ::std::option::Option<BytesList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    BytesList::get_value_for_reflect,
                    BytesList::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BytesList>(
                    "BytesList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BytesList {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BytesList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BytesList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FloatList {
    // message fields
    pub value: ::std::vec::Vec<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FloatList {}

impl FloatList {
    pub fn new() -> FloatList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FloatList {
        static mut instance: ::protobuf::lazy::Lazy<FloatList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FloatList,
        };
        unsafe {
            instance.get(FloatList::new)
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

impl ::protobuf::Message for FloatList {
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

impl ::protobuf::MessageStatic for FloatList {
    fn new() -> FloatList {
        FloatList::new()
    }

    fn descriptor_static(_: ::std::option::Option<FloatList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "value",
                    FloatList::get_value_for_reflect,
                    FloatList::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FloatList>(
                    "FloatList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FloatList {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FloatList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FloatList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Int64List {
    // message fields
    pub value: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Int64List {}

impl Int64List {
    pub fn new() -> Int64List {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Int64List {
        static mut instance: ::protobuf::lazy::Lazy<Int64List> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Int64List,
        };
        unsafe {
            instance.get(Int64List::new)
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

impl ::protobuf::Message for Int64List {
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

impl ::protobuf::MessageStatic for Int64List {
    fn new() -> Int64List {
        Int64List::new()
    }

    fn descriptor_static(_: ::std::option::Option<Int64List>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "value",
                    Int64List::get_value_for_reflect,
                    Int64List::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Int64List>(
                    "Int64List",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Int64List {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Int64List {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Int64List {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Feature {
    // message oneof groups
    kind: ::std::option::Option<Feature_oneof_kind>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Feature {}

#[derive(Clone,PartialEq)]
pub enum Feature_oneof_kind {
    bytes_list(BytesList),
    float_list(FloatList),
    int64_list(Int64List),
}

impl Feature {
    pub fn new() -> Feature {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Feature {
        static mut instance: ::protobuf::lazy::Lazy<Feature> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Feature,
        };
        unsafe {
            instance.get(Feature::new)
        }
    }

    // .tensorflow.BytesList bytes_list = 1;

    pub fn clear_bytes_list(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_bytes_list(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Feature_oneof_kind::bytes_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bytes_list(&mut self, v: BytesList) {
        self.kind = ::std::option::Option::Some(Feature_oneof_kind::bytes_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_bytes_list(&mut self) -> &mut BytesList {
        if let ::std::option::Option::Some(Feature_oneof_kind::bytes_list(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Feature_oneof_kind::bytes_list(BytesList::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Feature_oneof_kind::bytes_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bytes_list(&mut self) -> BytesList {
        if self.has_bytes_list() {
            match self.kind.take() {
                ::std::option::Option::Some(Feature_oneof_kind::bytes_list(v)) => v,
                _ => panic!(),
            }
        } else {
            BytesList::new()
        }
    }

    pub fn get_bytes_list(&self) -> &BytesList {
        match self.kind {
            ::std::option::Option::Some(Feature_oneof_kind::bytes_list(ref v)) => v,
            _ => BytesList::default_instance(),
        }
    }

    // .tensorflow.FloatList float_list = 2;

    pub fn clear_float_list(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_float_list(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Feature_oneof_kind::float_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float_list(&mut self, v: FloatList) {
        self.kind = ::std::option::Option::Some(Feature_oneof_kind::float_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_float_list(&mut self) -> &mut FloatList {
        if let ::std::option::Option::Some(Feature_oneof_kind::float_list(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Feature_oneof_kind::float_list(FloatList::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Feature_oneof_kind::float_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_float_list(&mut self) -> FloatList {
        if self.has_float_list() {
            match self.kind.take() {
                ::std::option::Option::Some(Feature_oneof_kind::float_list(v)) => v,
                _ => panic!(),
            }
        } else {
            FloatList::new()
        }
    }

    pub fn get_float_list(&self) -> &FloatList {
        match self.kind {
            ::std::option::Option::Some(Feature_oneof_kind::float_list(ref v)) => v,
            _ => FloatList::default_instance(),
        }
    }

    // .tensorflow.Int64List int64_list = 3;

    pub fn clear_int64_list(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_int64_list(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Feature_oneof_kind::int64_list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int64_list(&mut self, v: Int64List) {
        self.kind = ::std::option::Option::Some(Feature_oneof_kind::int64_list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_int64_list(&mut self) -> &mut Int64List {
        if let ::std::option::Option::Some(Feature_oneof_kind::int64_list(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Feature_oneof_kind::int64_list(Int64List::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Feature_oneof_kind::int64_list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_int64_list(&mut self) -> Int64List {
        if self.has_int64_list() {
            match self.kind.take() {
                ::std::option::Option::Some(Feature_oneof_kind::int64_list(v)) => v,
                _ => panic!(),
            }
        } else {
            Int64List::new()
        }
    }

    pub fn get_int64_list(&self) -> &Int64List {
        match self.kind {
            ::std::option::Option::Some(Feature_oneof_kind::int64_list(ref v)) => v,
            _ => Int64List::default_instance(),
        }
    }
}

impl ::protobuf::Message for Feature {
    fn is_initialized(&self) -> bool {
        if let Some(Feature_oneof_kind::bytes_list(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Feature_oneof_kind::float_list(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Feature_oneof_kind::int64_list(ref v)) = self.kind {
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
                    self.kind = ::std::option::Option::Some(Feature_oneof_kind::bytes_list(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Feature_oneof_kind::float_list(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Feature_oneof_kind::int64_list(is.read_message()?));
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
                &Feature_oneof_kind::bytes_list(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Feature_oneof_kind::float_list(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Feature_oneof_kind::int64_list(ref v) => {
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
                &Feature_oneof_kind::bytes_list(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Feature_oneof_kind::float_list(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Feature_oneof_kind::int64_list(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Feature {
    fn new() -> Feature {
        Feature::new()
    }

    fn descriptor_static(_: ::std::option::Option<Feature>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, BytesList>(
                    "bytes_list",
                    Feature::has_bytes_list,
                    Feature::get_bytes_list,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, FloatList>(
                    "float_list",
                    Feature::has_float_list,
                    Feature::get_float_list,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Int64List>(
                    "int64_list",
                    Feature::has_int64_list,
                    Feature::get_int64_list,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Feature>(
                    "Feature",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Feature {
    fn clear(&mut self) {
        self.clear_bytes_list();
        self.clear_float_list();
        self.clear_int64_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Feature {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Features {
    // message fields
    pub feature: ::std::collections::HashMap<::std::string::String, Feature>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Features {}

impl Features {
    pub fn new() -> Features {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Features {
        static mut instance: ::protobuf::lazy::Lazy<Features> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Features,
        };
        unsafe {
            instance.get(Features::new)
        }
    }

    // repeated .tensorflow.Features.FeatureEntry feature = 1;

    pub fn clear_feature(&mut self) {
        self.feature.clear();
    }

    // Param is passed by value, moved
    pub fn set_feature(&mut self, v: ::std::collections::HashMap<::std::string::String, Feature>) {
        self.feature = v;
    }

    // Mutable pointer to the field.
    pub fn mut_feature(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, Feature> {
        &mut self.feature
    }

    // Take field
    pub fn take_feature(&mut self) -> ::std::collections::HashMap<::std::string::String, Feature> {
        ::std::mem::replace(&mut self.feature, ::std::collections::HashMap::new())
    }

    pub fn get_feature(&self) -> &::std::collections::HashMap<::std::string::String, Feature> {
        &self.feature
    }

    fn get_feature_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, Feature> {
        &self.feature
    }

    fn mut_feature_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, Feature> {
        &mut self.feature
    }
}

impl ::protobuf::Message for Features {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Feature>>(wire_type, is, &mut self.feature)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Feature>>(1, &self.feature);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Feature>>(1, &self.feature, os)?;
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

impl ::protobuf::MessageStatic for Features {
    fn new() -> Features {
        Features::new()
    }

    fn descriptor_static(_: ::std::option::Option<Features>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Feature>>(
                    "feature",
                    Features::get_feature_for_reflect,
                    Features::mut_feature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Features>(
                    "Features",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Features {
    fn clear(&mut self) {
        self.clear_feature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Features {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Features {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FeatureList {
    // message fields
    pub feature: ::protobuf::RepeatedField<Feature>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FeatureList {}

impl FeatureList {
    pub fn new() -> FeatureList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FeatureList {
        static mut instance: ::protobuf::lazy::Lazy<FeatureList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FeatureList,
        };
        unsafe {
            instance.get(FeatureList::new)
        }
    }

    // repeated .tensorflow.Feature feature = 1;

    pub fn clear_feature(&mut self) {
        self.feature.clear();
    }

    // Param is passed by value, moved
    pub fn set_feature(&mut self, v: ::protobuf::RepeatedField<Feature>) {
        self.feature = v;
    }

    // Mutable pointer to the field.
    pub fn mut_feature(&mut self) -> &mut ::protobuf::RepeatedField<Feature> {
        &mut self.feature
    }

    // Take field
    pub fn take_feature(&mut self) -> ::protobuf::RepeatedField<Feature> {
        ::std::mem::replace(&mut self.feature, ::protobuf::RepeatedField::new())
    }

    pub fn get_feature(&self) -> &[Feature] {
        &self.feature
    }

    fn get_feature_for_reflect(&self) -> &::protobuf::RepeatedField<Feature> {
        &self.feature
    }

    fn mut_feature_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Feature> {
        &mut self.feature
    }
}

impl ::protobuf::Message for FeatureList {
    fn is_initialized(&self) -> bool {
        for v in &self.feature {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.feature)?;
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
        for value in &self.feature {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.feature {
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

impl ::protobuf::MessageStatic for FeatureList {
    fn new() -> FeatureList {
        FeatureList::new()
    }

    fn descriptor_static(_: ::std::option::Option<FeatureList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Feature>>(
                    "feature",
                    FeatureList::get_feature_for_reflect,
                    FeatureList::mut_feature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FeatureList>(
                    "FeatureList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FeatureList {
    fn clear(&mut self) {
        self.clear_feature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FeatureList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeatureList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FeatureLists {
    // message fields
    pub feature_list: ::std::collections::HashMap<::std::string::String, FeatureList>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FeatureLists {}

impl FeatureLists {
    pub fn new() -> FeatureLists {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FeatureLists {
        static mut instance: ::protobuf::lazy::Lazy<FeatureLists> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FeatureLists,
        };
        unsafe {
            instance.get(FeatureLists::new)
        }
    }

    // repeated .tensorflow.FeatureLists.FeatureListEntry feature_list = 1;

    pub fn clear_feature_list(&mut self) {
        self.feature_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_feature_list(&mut self, v: ::std::collections::HashMap<::std::string::String, FeatureList>) {
        self.feature_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_feature_list(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, FeatureList> {
        &mut self.feature_list
    }

    // Take field
    pub fn take_feature_list(&mut self) -> ::std::collections::HashMap<::std::string::String, FeatureList> {
        ::std::mem::replace(&mut self.feature_list, ::std::collections::HashMap::new())
    }

    pub fn get_feature_list(&self) -> &::std::collections::HashMap<::std::string::String, FeatureList> {
        &self.feature_list
    }

    fn get_feature_list_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, FeatureList> {
        &self.feature_list
    }

    fn mut_feature_list_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, FeatureList> {
        &mut self.feature_list
    }
}

impl ::protobuf::Message for FeatureLists {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FeatureList>>(wire_type, is, &mut self.feature_list)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FeatureList>>(1, &self.feature_list);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FeatureList>>(1, &self.feature_list, os)?;
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

impl ::protobuf::MessageStatic for FeatureLists {
    fn new() -> FeatureLists {
        FeatureLists::new()
    }

    fn descriptor_static(_: ::std::option::Option<FeatureLists>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FeatureList>>(
                    "feature_list",
                    FeatureLists::get_feature_list_for_reflect,
                    FeatureLists::mut_feature_list_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FeatureLists>(
                    "FeatureLists",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FeatureLists {
    fn clear(&mut self) {
        self.clear_feature_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FeatureLists {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeatureLists {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n0tensorflow/tensorflow/core/example/feature.proto\x12\ntensorflow\"!\n\
    \tBytesList\x12\x14\n\x05value\x18\x01\x20\x03(\x0cR\x05value\"%\n\tFloa\
    tList\x12\x18\n\x05value\x18\x01\x20\x03(\x02R\x05valueB\x02\x10\x01\"%\
    \n\tInt64List\x12\x18\n\x05value\x18\x01\x20\x03(\x03R\x05valueB\x02\x10\
    \x01\"\xb9\x01\n\x07Feature\x126\n\nbytes_list\x18\x01\x20\x01(\x0b2\x15\
    .tensorflow.BytesListH\0R\tbytesList\x126\n\nfloat_list\x18\x02\x20\x01(\
    \x0b2\x15.tensorflow.FloatListH\0R\tfloatList\x126\n\nint64_list\x18\x03\
    \x20\x01(\x0b2\x15.tensorflow.Int64ListH\0R\tint64ListB\x06\n\x04kind\"\
    \x98\x01\n\x08Features\x12;\n\x07feature\x18\x01\x20\x03(\x0b2!.tensorfl\
    ow.Features.FeatureEntryR\x07feature\x1aO\n\x0cFeatureEntry\x12\x10\n\
    \x03key\x18\x01\x20\x01(\tR\x03key\x12)\n\x05value\x18\x02\x20\x01(\x0b2\
    \x13.tensorflow.FeatureR\x05value:\x028\x01\"<\n\x0bFeatureList\x12-\n\
    \x07feature\x18\x01\x20\x03(\x0b2\x13.tensorflow.FeatureR\x07feature\"\
    \xb5\x01\n\x0cFeatureLists\x12L\n\x0cfeature_list\x18\x01\x20\x03(\x0b2)\
    .tensorflow.FeatureLists.FeatureListEntryR\x0bfeatureList\x1aW\n\x10Feat\
    ureListEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12-\n\x05value\
    \x18\x02\x20\x01(\x0b2\x17.tensorflow.FeatureListR\x05value:\x028\x01B,\
    \n\x16org.tensorflow.exampleB\rFeatureProtosP\x01\xf8\x01\x01J\xed\x16\n\
    \x06\x12\x047\0h\x02\n\xe1\x08\n\x01\x0c\x12\x037\0\x122\xd6\x08\x20Prot\
    ocol\x20messages\x20for\x20describing\x20features\x20for\x20machine\x20l\
    earning\x20model\n\x20training\x20or\x20inference.\n\n\x20There\x20are\
    \x20three\x20base\x20Feature\x20types:\n\x20\x20\x20-\x20bytes\n\x20\x20\
    \x20-\x20float\n\x20\x20\x20-\x20int64\n\n\x20A\x20Feature\x20contains\
    \x20Lists\x20which\x20may\x20hold\x20zero\x20or\x20more\x20values.\x20\
    \x20These\n\x20lists\x20are\x20the\x20base\x20values\x20BytesList,\x20Fl\
    oatList,\x20Int64List.\n\n\x20Features\x20are\x20organized\x20into\x20ca\
    tegories\x20by\x20name.\x20\x20The\x20Features\x20message\n\x20contains\
    \x20the\x20mapping\x20from\x20name\x20to\x20Feature.\n\n\x20Example\x20F\
    eatures\x20for\x20a\x20movie\x20recommendation\x20application:\n\x20\x20\
    \x20feature\x20{\n\x20\x20\x20\x20\x20key:\x20\"age\"\n\x20\x20\x20\x20\
    \x20value\x20{\x20float_list\x20{\n\x20\x20\x20\x20\x20\x20\x20value:\
    \x2029.0\n\x20\x20\x20\x20\x20}}\n\x20\x20\x20}\n\x20\x20\x20feature\x20\
    {\n\x20\x20\x20\x20\x20key:\x20\"movie\"\n\x20\x20\x20\x20\x20value\x20{\
    \x20bytes_list\x20{\n\x20\x20\x20\x20\x20\x20\x20value:\x20\"The\x20Shaw\
    shank\x20Redemption\"\n\x20\x20\x20\x20\x20\x20\x20value:\x20\"Fight\x20\
    Club\"\n\x20\x20\x20\x20\x20}}\n\x20\x20\x20}\n\x20\x20\x20feature\x20{\
    \n\x20\x20\x20\x20\x20key:\x20\"movie_ratings\"\n\x20\x20\x20\x20\x20val\
    ue\x20{\x20float_list\x20{\n\x20\x20\x20\x20\x20\x20\x20value:\x209.0\n\
    \x20\x20\x20\x20\x20\x20\x20value:\x209.7\n\x20\x20\x20\x20\x20}}\n\x20\
    \x20\x20}\n\x20\x20\x20feature\x20{\n\x20\x20\x20\x20\x20key:\x20\"sugge\
    stion\"\n\x20\x20\x20\x20\x20value\x20{\x20bytes_list\x20{\n\x20\x20\x20\
    \x20\x20\x20\x20value:\x20\"Inception\"\n\x20\x20\x20\x20\x20}}\n\x20\
    \x20\x20}\n\x20\x20\x20feature\x20{\n\x20\x20\x20\x20\x20key:\x20\"sugge\
    stion_purchased\"\n\x20\x20\x20\x20\x20value\x20{\x20int64_list\x20{\n\
    \x20\x20\x20\x20\x20\x20\x20value:\x201\n\x20\x20\x20\x20\x20}}\n\x20\
    \x20\x20}\n\x20\x20\x20feature\x20{\n\x20\x20\x20\x20\x20key:\x20\"purch\
    ase_price\"\n\x20\x20\x20\x20\x20value\x20{\x20float_list\x20{\n\x20\x20\
    \x20\x20\x20\x20\x20value:\x209.99\n\x20\x20\x20\x20\x20}}\n\x20\x20\x20\
    }\n\n\n\x08\n\x01\x08\x12\x038\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x038\
    \0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x038\x07\x17\n\r\n\x06\x08\xe7\
    \x07\0\x02\0\x12\x038\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\
    \x038\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x038\x1a\x1e\n\x08\n\x01\
    \x08\x12\x039\0.\n\x0b\n\x04\x08\xe7\x07\x01\x12\x039\0.\n\x0c\n\x05\x08\
    \xe7\x07\x01\x02\x12\x039\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x039\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x039\x07\x1b\n\
    \x0c\n\x05\x08\xe7\x07\x01\x07\x12\x039\x1e-\n\x08\n\x01\x08\x12\x03:\0\
    \"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03:\0\"\n\x0c\n\x05\x08\xe7\x07\x02\
    \x02\x12\x03:\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03:\x07\x1a\n\
    \x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03:\x07\x1a\n\x0c\n\x05\x08\
    \xe7\x07\x02\x03\x12\x03:\x1d!\n\x08\n\x01\x08\x12\x03;\0/\n\x0b\n\x04\
    \x08\xe7\x07\x03\x12\x03;\0/\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03;\
    \x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03;\x07\x13\n\x0e\n\x07\
    \x08\xe7\x07\x03\x02\0\x01\x12\x03;\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\
    \x07\x12\x03;\x16.\n\x08\n\x01\x02\x12\x03=\x08\x12\n=\n\x02\x04\0\x12\
    \x04@\0B\x01\x1a1\x20Containers\x20to\x20hold\x20repeated\x20fundamental\
    \x20values.\n\n\n\n\x03\x04\0\x01\x12\x03@\x08\x11\n\x0b\n\x04\x04\0\x02\
    \0\x12\x03A\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03A\x02\n\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03A\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03A\x11\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03A\x19\x1a\n\n\n\x02\x04\
    \x01\x12\x04C\0E\x01\n\n\n\x03\x04\x01\x01\x12\x03C\x08\x11\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03D\x02+\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03D\x02\
    \n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03D\x0b\x10\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03D\x11\x16\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03D\x19\
    \x1a\n\x0c\n\x05\x04\x01\x02\0\x08\x12\x03D\x1b*\n\x0f\n\x08\x04\x01\x02\
    \0\x08\xe7\x07\0\x12\x03D\x1c)\n\x10\n\t\x04\x01\x02\0\x08\xe7\x07\0\x02\
    \x12\x03D\x1c\"\n\x11\n\n\x04\x01\x02\0\x08\xe7\x07\0\x02\0\x12\x03D\x1c\
    \"\n\x12\n\x0b\x04\x01\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x03D\x1c\"\n\
    \x10\n\t\x04\x01\x02\0\x08\xe7\x07\0\x03\x12\x03D%)\n\n\n\x02\x04\x02\
    \x12\x04F\0H\x01\n\n\n\x03\x04\x02\x01\x12\x03F\x08\x11\n\x0b\n\x04\x04\
    \x02\x02\0\x12\x03G\x02+\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03G\x02\n\n\
    \x0c\n\x05\x04\x02\x02\0\x05\x12\x03G\x0b\x10\n\x0c\n\x05\x04\x02\x02\0\
    \x01\x12\x03G\x11\x16\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03G\x19\x1a\n\
    \x0c\n\x05\x04\x02\x02\0\x08\x12\x03G\x1b*\n\x0f\n\x08\x04\x02\x02\0\x08\
    \xe7\x07\0\x12\x03G\x1c)\n\x10\n\t\x04\x02\x02\0\x08\xe7\x07\0\x02\x12\
    \x03G\x1c\"\n\x11\n\n\x04\x02\x02\0\x08\xe7\x07\0\x02\0\x12\x03G\x1c\"\n\
    \x12\n\x0b\x04\x02\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x03G\x1c\"\n\x10\n\
    \t\x04\x02\x02\0\x08\xe7\x07\0\x03\x12\x03G%)\n1\n\x02\x04\x03\x12\x04K\
    \0R\x01\x1a%\x20Containers\x20for\x20non-sequential\x20data.\n\n\n\n\x03\
    \x04\x03\x01\x12\x03K\x08\x0f\n5\n\x04\x04\x03\x08\0\x12\x04M\x02Q\x03\
    \x1a'\x20Each\x20feature\x20can\x20be\x20exactly\x20one\x20kind.\n\n\x0c\
    \n\x05\x04\x03\x08\0\x01\x12\x03M\x08\x0c\n\x0b\n\x04\x04\x03\x02\0\x12\
    \x03N\x04\x1d\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03N\x04\r\n\x0c\n\x05\
    \x04\x03\x02\0\x01\x12\x03N\x0e\x18\n\x0c\n\x05\x04\x03\x02\0\x03\x12\
    \x03N\x1b\x1c\n\x0b\n\x04\x04\x03\x02\x01\x12\x03O\x04\x1d\n\x0c\n\x05\
    \x04\x03\x02\x01\x06\x12\x03O\x04\r\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\
    \x03O\x0e\x18\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03O\x1b\x1c\n\x0b\n\
    \x04\x04\x03\x02\x02\x12\x03P\x04\x1d\n\x0c\n\x05\x04\x03\x02\x02\x06\
    \x12\x03P\x04\r\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03P\x0e\x18\n\x0c\n\
    \x05\x04\x03\x02\x02\x03\x12\x03P\x1b\x1c\n\n\n\x02\x04\x04\x12\x04T\0W\
    \x01\n\n\n\x03\x04\x04\x01\x12\x03T\x08\x10\n0\n\x04\x04\x04\x02\0\x12\
    \x03V\x02#\x1a#\x20Map\x20from\x20feature\x20name\x20to\x20feature.\n\n\
    \r\n\x05\x04\x04\x02\0\x04\x12\x04V\x02T\x12\n\x0c\n\x05\x04\x04\x02\0\
    \x06\x12\x03V\x02\x16\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03V\x17\x1e\n\
    \x0c\n\x05\x04\x04\x02\0\x03\x12\x03V!\"\n\x89\x02\n\x02\x04\x05\x12\x04\
    a\0c\x01\x1a\xfc\x01\x20Containers\x20for\x20sequential\x20data.\n\n\x20\
    A\x20FeatureList\x20contains\x20lists\x20of\x20Features.\x20\x20These\
    \x20may\x20hold\x20zero\x20or\x20more\n\x20Feature\x20values.\n\n\x20Fea\
    tureLists\x20are\x20organized\x20into\x20categories\x20by\x20name.\x20\
    \x20The\x20FeatureLists\x20message\n\x20contains\x20the\x20mapping\x20fr\
    om\x20name\x20to\x20FeatureList.\n\n\n\n\n\x03\x04\x05\x01\x12\x03a\x08\
    \x13\n\x0b\n\x04\x04\x05\x02\0\x12\x03b\x02\x1f\n\x0c\n\x05\x04\x05\x02\
    \0\x04\x12\x03b\x02\n\n\x0c\n\x05\x04\x05\x02\0\x06\x12\x03b\x0b\x12\n\
    \x0c\n\x05\x04\x05\x02\0\x01\x12\x03b\x13\x1a\n\x0c\n\x05\x04\x05\x02\0\
    \x03\x12\x03b\x1d\x1e\n\n\n\x02\x04\x06\x12\x04e\0h\x01\n\n\n\x03\x04\
    \x06\x01\x12\x03e\x08\x14\n5\n\x04\x04\x06\x02\0\x12\x03g\x02,\x1a(\x20M\
    ap\x20from\x20feature\x20name\x20to\x20feature\x20list.\n\n\r\n\x05\x04\
    \x06\x02\0\x04\x12\x04g\x02e\x16\n\x0c\n\x05\x04\x06\x02\0\x06\x12\x03g\
    \x02\x1a\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03g\x1b'\n\x0c\n\x05\x04\x06\
    \x02\0\x03\x12\x03g*+b\x06proto3\
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
