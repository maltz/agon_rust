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
pub struct Class {
    // message fields
    pub label: ::std::string::String,
    pub score: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Class {}

impl Class {
    pub fn new() -> Class {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Class {
        static mut instance: ::protobuf::lazy::Lazy<Class> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Class,
        };
        unsafe {
            instance.get(Class::new)
        }
    }

    // string label = 1;

    pub fn clear_label(&mut self) {
        self.label.clear();
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: ::std::string::String) {
        self.label = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_label(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }

    // Take field
    pub fn take_label(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.label, ::std::string::String::new())
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    fn get_label_for_reflect(&self) -> &::std::string::String {
        &self.label
    }

    fn mut_label_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }

    // float score = 2;

    pub fn clear_score(&mut self) {
        self.score = 0.;
    }

    // Param is passed by value, moved
    pub fn set_score(&mut self, v: f32) {
        self.score = v;
    }

    pub fn get_score(&self) -> f32 {
        self.score
    }

    fn get_score_for_reflect(&self) -> &f32 {
        &self.score
    }

    fn mut_score_for_reflect(&mut self) -> &mut f32 {
        &mut self.score
    }
}

impl ::protobuf::Message for Class {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.label)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.score = tmp;
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
        if !self.label.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.label);
        }
        if self.score != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.label.is_empty() {
            os.write_string(1, &self.label)?;
        }
        if self.score != 0. {
            os.write_float(2, self.score)?;
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

impl ::protobuf::MessageStatic for Class {
    fn new() -> Class {
        Class::new()
    }

    fn descriptor_static(_: ::std::option::Option<Class>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "label",
                    Class::get_label_for_reflect,
                    Class::mut_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "score",
                    Class::get_score_for_reflect,
                    Class::mut_score_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Class>(
                    "Class",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Class {
    fn clear(&mut self) {
        self.clear_label();
        self.clear_score();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Class {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Class {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Classifications {
    // message fields
    pub classes: ::protobuf::RepeatedField<Class>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Classifications {}

impl Classifications {
    pub fn new() -> Classifications {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Classifications {
        static mut instance: ::protobuf::lazy::Lazy<Classifications> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Classifications,
        };
        unsafe {
            instance.get(Classifications::new)
        }
    }

    // repeated .tensorflow.serving.Class classes = 1;

    pub fn clear_classes(&mut self) {
        self.classes.clear();
    }

    // Param is passed by value, moved
    pub fn set_classes(&mut self, v: ::protobuf::RepeatedField<Class>) {
        self.classes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_classes(&mut self) -> &mut ::protobuf::RepeatedField<Class> {
        &mut self.classes
    }

    // Take field
    pub fn take_classes(&mut self) -> ::protobuf::RepeatedField<Class> {
        ::std::mem::replace(&mut self.classes, ::protobuf::RepeatedField::new())
    }

    pub fn get_classes(&self) -> &[Class] {
        &self.classes
    }

    fn get_classes_for_reflect(&self) -> &::protobuf::RepeatedField<Class> {
        &self.classes
    }

    fn mut_classes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Class> {
        &mut self.classes
    }
}

impl ::protobuf::Message for Classifications {
    fn is_initialized(&self) -> bool {
        for v in &self.classes {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.classes)?;
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
        for value in &self.classes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.classes {
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

impl ::protobuf::MessageStatic for Classifications {
    fn new() -> Classifications {
        Classifications::new()
    }

    fn descriptor_static(_: ::std::option::Option<Classifications>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Class>>(
                    "classes",
                    Classifications::get_classes_for_reflect,
                    Classifications::mut_classes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Classifications>(
                    "Classifications",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Classifications {
    fn clear(&mut self) {
        self.clear_classes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Classifications {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Classifications {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClassificationResult {
    // message fields
    pub classifications: ::protobuf::RepeatedField<Classifications>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClassificationResult {}

impl ClassificationResult {
    pub fn new() -> ClassificationResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClassificationResult {
        static mut instance: ::protobuf::lazy::Lazy<ClassificationResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClassificationResult,
        };
        unsafe {
            instance.get(ClassificationResult::new)
        }
    }

    // repeated .tensorflow.serving.Classifications classifications = 1;

    pub fn clear_classifications(&mut self) {
        self.classifications.clear();
    }

    // Param is passed by value, moved
    pub fn set_classifications(&mut self, v: ::protobuf::RepeatedField<Classifications>) {
        self.classifications = v;
    }

    // Mutable pointer to the field.
    pub fn mut_classifications(&mut self) -> &mut ::protobuf::RepeatedField<Classifications> {
        &mut self.classifications
    }

    // Take field
    pub fn take_classifications(&mut self) -> ::protobuf::RepeatedField<Classifications> {
        ::std::mem::replace(&mut self.classifications, ::protobuf::RepeatedField::new())
    }

    pub fn get_classifications(&self) -> &[Classifications] {
        &self.classifications
    }

    fn get_classifications_for_reflect(&self) -> &::protobuf::RepeatedField<Classifications> {
        &self.classifications
    }

    fn mut_classifications_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Classifications> {
        &mut self.classifications
    }
}

impl ::protobuf::Message for ClassificationResult {
    fn is_initialized(&self) -> bool {
        for v in &self.classifications {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.classifications)?;
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
        for value in &self.classifications {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.classifications {
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

impl ::protobuf::MessageStatic for ClassificationResult {
    fn new() -> ClassificationResult {
        ClassificationResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClassificationResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Classifications>>(
                    "classifications",
                    ClassificationResult::get_classifications_for_reflect,
                    ClassificationResult::mut_classifications_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClassificationResult>(
                    "ClassificationResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClassificationResult {
    fn clear(&mut self) {
        self.clear_classifications();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClassificationResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClassificationResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClassificationRequest {
    // message fields
    pub model_spec: ::protobuf::SingularPtrField<super::model::ModelSpec>,
    pub input: ::protobuf::SingularPtrField<super::input::Input>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClassificationRequest {}

impl ClassificationRequest {
    pub fn new() -> ClassificationRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClassificationRequest {
        static mut instance: ::protobuf::lazy::Lazy<ClassificationRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClassificationRequest,
        };
        unsafe {
            instance.get(ClassificationRequest::new)
        }
    }

    // .tensorflow.serving.ModelSpec model_spec = 1;

    pub fn clear_model_spec(&mut self) {
        self.model_spec.clear();
    }

    pub fn has_model_spec(&self) -> bool {
        self.model_spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model_spec(&mut self, v: super::model::ModelSpec) {
        self.model_spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_model_spec(&mut self) -> &mut super::model::ModelSpec {
        if self.model_spec.is_none() {
            self.model_spec.set_default();
        }
        self.model_spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_model_spec(&mut self) -> super::model::ModelSpec {
        self.model_spec.take().unwrap_or_else(|| super::model::ModelSpec::new())
    }

    pub fn get_model_spec(&self) -> &super::model::ModelSpec {
        self.model_spec.as_ref().unwrap_or_else(|| super::model::ModelSpec::default_instance())
    }

    fn get_model_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<super::model::ModelSpec> {
        &self.model_spec
    }

    fn mut_model_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::model::ModelSpec> {
        &mut self.model_spec
    }

    // .tensorflow.serving.Input input = 2;

    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    pub fn has_input(&self) -> bool {
        self.input.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input(&mut self, v: super::input::Input) {
        self.input = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_input(&mut self) -> &mut super::input::Input {
        if self.input.is_none() {
            self.input.set_default();
        }
        self.input.as_mut().unwrap()
    }

    // Take field
    pub fn take_input(&mut self) -> super::input::Input {
        self.input.take().unwrap_or_else(|| super::input::Input::new())
    }

    pub fn get_input(&self) -> &super::input::Input {
        self.input.as_ref().unwrap_or_else(|| super::input::Input::default_instance())
    }

    fn get_input_for_reflect(&self) -> &::protobuf::SingularPtrField<super::input::Input> {
        &self.input
    }

    fn mut_input_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::input::Input> {
        &mut self.input
    }
}

impl ::protobuf::Message for ClassificationRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.model_spec {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.input {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.model_spec)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.input)?;
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
        if let Some(ref v) = self.model_spec.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.input.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.model_spec.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.input.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ClassificationRequest {
    fn new() -> ClassificationRequest {
        ClassificationRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClassificationRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::model::ModelSpec>>(
                    "model_spec",
                    ClassificationRequest::get_model_spec_for_reflect,
                    ClassificationRequest::mut_model_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::input::Input>>(
                    "input",
                    ClassificationRequest::get_input_for_reflect,
                    ClassificationRequest::mut_input_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClassificationRequest>(
                    "ClassificationRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClassificationRequest {
    fn clear(&mut self) {
        self.clear_model_spec();
        self.clear_input();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClassificationRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClassificationRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClassificationResponse {
    // message fields
    pub result: ::protobuf::SingularPtrField<ClassificationResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClassificationResponse {}

impl ClassificationResponse {
    pub fn new() -> ClassificationResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClassificationResponse {
        static mut instance: ::protobuf::lazy::Lazy<ClassificationResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClassificationResponse,
        };
        unsafe {
            instance.get(ClassificationResponse::new)
        }
    }

    // .tensorflow.serving.ClassificationResult result = 1;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ClassificationResult) {
        self.result = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result(&mut self) -> &mut ClassificationResult {
        if self.result.is_none() {
            self.result.set_default();
        }
        self.result.as_mut().unwrap()
    }

    // Take field
    pub fn take_result(&mut self) -> ClassificationResult {
        self.result.take().unwrap_or_else(|| ClassificationResult::new())
    }

    pub fn get_result(&self) -> &ClassificationResult {
        self.result.as_ref().unwrap_or_else(|| ClassificationResult::default_instance())
    }

    fn get_result_for_reflect(&self) -> &::protobuf::SingularPtrField<ClassificationResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClassificationResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for ClassificationResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.result {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.result)?;
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
        if let Some(ref v) = self.result.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.result.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ClassificationResponse {
    fn new() -> ClassificationResponse {
        ClassificationResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClassificationResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClassificationResult>>(
                    "result",
                    ClassificationResponse::get_result_for_reflect,
                    ClassificationResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClassificationResponse>(
                    "ClassificationResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClassificationResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClassificationResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClassificationResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,tensorflow_serving/apis/classification.proto\x12\x12tensorflow.servin\
    g\x1a#tensorflow_serving/apis/input.proto\x1a#tensorflow_serving/apis/mo\
    del.proto\"3\n\x05Class\x12\x14\n\x05label\x18\x01\x20\x01(\tR\x05label\
    \x12\x14\n\x05score\x18\x02\x20\x01(\x02R\x05score\"F\n\x0fClassificatio\
    ns\x123\n\x07classes\x18\x01\x20\x03(\x0b2\x19.tensorflow.serving.ClassR\
    \x07classes\"e\n\x14ClassificationResult\x12M\n\x0fclassifications\x18\
    \x01\x20\x03(\x0b2#.tensorflow.serving.ClassificationsR\x0fclassificatio\
    ns\"\x86\x01\n\x15ClassificationRequest\x12<\n\nmodel_spec\x18\x01\x20\
    \x01(\x0b2\x1d.tensorflow.serving.ModelSpecR\tmodelSpec\x12/\n\x05input\
    \x18\x02\x20\x01(\x0b2\x19.tensorflow.serving.InputR\x05input\"Z\n\x16Cl\
    assificationResponse\x12@\n\x06result\x18\x01\x20\x01(\x0b2(.tensorflow.\
    serving.ClassificationResultR\x06resultB\x03\xf8\x01\x01J\xe0\x08\n\x06\
    \x12\x04\0\0*\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x08\x12\
    \x03\x02\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x02\0\x1f\n\x0c\n\x05\
    \x08\xe7\x07\0\x02\x12\x03\x02\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\
    \x03\x02\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x02\x07\x17\
    \n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x02\x1a\x1e\n\t\n\x02\x03\0\x12\
    \x03\x04\x07,\n\t\n\x02\x03\x01\x12\x03\x05\x07,\n\x08\n\x01\x02\x12\x03\
    \x07\x08\x1a\n\x1d\n\x02\x04\0\x12\x04\n\0\x10\x01\x1a\x11\x20A\x20singl\
    e\x20class.\n\n\n\n\x03\x04\0\x01\x12\x03\n\x08\r\n*\n\x04\x04\0\x02\0\
    \x12\x03\x0c\x02\x13\x1a\x1d\x20Label\x20or\x20name\x20of\x20the\x20clas\
    s.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x0c\x02\n\x0f\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\x0c\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0c\t\
    \x0e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c\x11\x12\n\\\n\x04\x04\0\x02\
    \x01\x12\x03\x0f\x02\x12\x1aO\x20Score\x20for\x20this\x20class\x20(e.g.,\
    \x20the\x20probability\x20the\x20item\x20belongs\x20to\x20this\n\x20clas\
    s).\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0f\x02\x0c\x13\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03\x0f\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x0f\x08\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0f\x10\x11\nE\n\
    \x02\x04\x01\x12\x04\x13\0\x15\x01\x1a9\x20List\x20of\x20classes\x20for\
    \x20a\x20single\x20item\x20(tensorflow.Example).\n\n\n\n\x03\x04\x01\x01\
    \x12\x03\x13\x08\x17\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x14\x02\x1d\n\x0c\
    \n\x05\x04\x01\x02\0\x04\x12\x03\x14\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\
    \x12\x03\x14\x0b\x10\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x14\x11\x18\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x14\x1b\x1c\nn\n\x02\x04\x02\x12\
    \x04\x19\0\x1b\x01\x1ab\x20Contains\x20one\x20result\x20per\x20input\x20\
    example,\x20in\x20the\x20same\x20order\x20as\x20the\x20input\x20in\n\x20\
    ClassificationRequest.\n\n\n\n\x03\x04\x02\x01\x12\x03\x19\x08\x1c\n\x0b\
    \n\x04\x04\x02\x02\0\x12\x03\x1a\x02/\n\x0c\n\x05\x04\x02\x02\0\x04\x12\
    \x03\x1a\x02\n\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x1a\x0b\x1a\n\x0c\n\
    \x05\x04\x02\x02\0\x01\x12\x03\x1a\x1b*\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03\x1a-.\n\x1c\n\x02\x04\x03\x12\x04\x1f\0%\x012\x10\x20RPC\x20Int\
    erfaces\n\n\n\n\x03\x04\x03\x01\x12\x03\x1f\x08\x1d\n#\n\x04\x04\x03\x02\
    \0\x12\x03!\x02\x1b\x1a\x16\x20Model\x20Specification.\n\n\r\n\x05\x04\
    \x03\x02\0\x04\x12\x04!\x02\x1f\x1f\n\x0c\n\x05\x04\x03\x02\0\x06\x12\
    \x03!\x02\x0b\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03!\x0c\x16\n\x0c\n\x05\
    \x04\x03\x02\0\x03\x12\x03!\x19\x1a\n\x1a\n\x04\x04\x03\x02\x01\x12\x03$\
    \x02%\x1a\r\x20Input\x20data.\n\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04$\
    \x02!\x1b\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x03$\x02\x1a\n\x0c\n\x05\
    \x04\x03\x02\x01\x01\x12\x03$\x1b\x20\n\x0c\n\x05\x04\x03\x02\x01\x03\
    \x12\x03$#$\n\n\n\x02\x04\x04\x12\x04'\0*\x01\n\n\n\x03\x04\x04\x01\x12\
    \x03'\x08\x1e\n,\n\x04\x04\x04\x02\0\x12\x03)\x02\"\x1a\x1f\x20Result\
    \x20of\x20the\x20classification.\n\n\r\n\x05\x04\x04\x02\0\x04\x12\x04)\
    \x02'\x20\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03)\x02\x16\n\x0c\n\x05\x04\
    \x04\x02\0\x01\x12\x03)\x17\x1d\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03)\
    \x20!b\x06proto3\
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
