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
pub struct InferenceTask {
    // message fields
    pub model_spec: ::protobuf::SingularPtrField<super::model::ModelSpec>,
    pub method_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InferenceTask {}

impl InferenceTask {
    pub fn new() -> InferenceTask {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InferenceTask {
        static mut instance: ::protobuf::lazy::Lazy<InferenceTask> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InferenceTask,
        };
        unsafe {
            instance.get(InferenceTask::new)
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

    // string method_name = 2;

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

impl ::protobuf::Message for InferenceTask {
    fn is_initialized(&self) -> bool {
        for v in &self.model_spec {
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
        if let Some(ref v) = self.model_spec.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.method_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.method_name);
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
        if !self.method_name.is_empty() {
            os.write_string(2, &self.method_name)?;
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

impl ::protobuf::MessageStatic for InferenceTask {
    fn new() -> InferenceTask {
        InferenceTask::new()
    }

    fn descriptor_static(_: ::std::option::Option<InferenceTask>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::model::ModelSpec>>(
                    "model_spec",
                    InferenceTask::get_model_spec_for_reflect,
                    InferenceTask::mut_model_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "method_name",
                    InferenceTask::get_method_name_for_reflect,
                    InferenceTask::mut_method_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InferenceTask>(
                    "InferenceTask",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InferenceTask {
    fn clear(&mut self) {
        self.clear_model_spec();
        self.clear_method_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InferenceTask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InferenceTask {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InferenceResult {
    // message fields
    pub model_spec: ::protobuf::SingularPtrField<super::model::ModelSpec>,
    // message oneof groups
    result: ::std::option::Option<InferenceResult_oneof_result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InferenceResult {}

#[derive(Clone,PartialEq)]
pub enum InferenceResult_oneof_result {
    classification_result(super::classification::ClassificationResult),
    regression_result(super::regression::RegressionResult),
}

impl InferenceResult {
    pub fn new() -> InferenceResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InferenceResult {
        static mut instance: ::protobuf::lazy::Lazy<InferenceResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InferenceResult,
        };
        unsafe {
            instance.get(InferenceResult::new)
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

    // .tensorflow.serving.ClassificationResult classification_result = 2;

    pub fn clear_classification_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_classification_result(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(InferenceResult_oneof_result::classification_result(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_classification_result(&mut self, v: super::classification::ClassificationResult) {
        self.result = ::std::option::Option::Some(InferenceResult_oneof_result::classification_result(v))
    }

    // Mutable pointer to the field.
    pub fn mut_classification_result(&mut self) -> &mut super::classification::ClassificationResult {
        if let ::std::option::Option::Some(InferenceResult_oneof_result::classification_result(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(InferenceResult_oneof_result::classification_result(super::classification::ClassificationResult::new()));
        }
        match self.result {
            ::std::option::Option::Some(InferenceResult_oneof_result::classification_result(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_classification_result(&mut self) -> super::classification::ClassificationResult {
        if self.has_classification_result() {
            match self.result.take() {
                ::std::option::Option::Some(InferenceResult_oneof_result::classification_result(v)) => v,
                _ => panic!(),
            }
        } else {
            super::classification::ClassificationResult::new()
        }
    }

    pub fn get_classification_result(&self) -> &super::classification::ClassificationResult {
        match self.result {
            ::std::option::Option::Some(InferenceResult_oneof_result::classification_result(ref v)) => v,
            _ => super::classification::ClassificationResult::default_instance(),
        }
    }

    // .tensorflow.serving.RegressionResult regression_result = 3;

    pub fn clear_regression_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_regression_result(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(InferenceResult_oneof_result::regression_result(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_regression_result(&mut self, v: super::regression::RegressionResult) {
        self.result = ::std::option::Option::Some(InferenceResult_oneof_result::regression_result(v))
    }

    // Mutable pointer to the field.
    pub fn mut_regression_result(&mut self) -> &mut super::regression::RegressionResult {
        if let ::std::option::Option::Some(InferenceResult_oneof_result::regression_result(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(InferenceResult_oneof_result::regression_result(super::regression::RegressionResult::new()));
        }
        match self.result {
            ::std::option::Option::Some(InferenceResult_oneof_result::regression_result(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_regression_result(&mut self) -> super::regression::RegressionResult {
        if self.has_regression_result() {
            match self.result.take() {
                ::std::option::Option::Some(InferenceResult_oneof_result::regression_result(v)) => v,
                _ => panic!(),
            }
        } else {
            super::regression::RegressionResult::new()
        }
    }

    pub fn get_regression_result(&self) -> &super::regression::RegressionResult {
        match self.result {
            ::std::option::Option::Some(InferenceResult_oneof_result::regression_result(ref v)) => v,
            _ => super::regression::RegressionResult::default_instance(),
        }
    }
}

impl ::protobuf::Message for InferenceResult {
    fn is_initialized(&self) -> bool {
        for v in &self.model_spec {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(InferenceResult_oneof_result::classification_result(ref v)) = self.result {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InferenceResult_oneof_result::regression_result(ref v)) = self.result {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.model_spec)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(InferenceResult_oneof_result::classification_result(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(InferenceResult_oneof_result::regression_result(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &InferenceResult_oneof_result::classification_result(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InferenceResult_oneof_result::regression_result(ref v) => {
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
        if let Some(ref v) = self.model_spec.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &InferenceResult_oneof_result::classification_result(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InferenceResult_oneof_result::regression_result(ref v) => {
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

impl ::protobuf::MessageStatic for InferenceResult {
    fn new() -> InferenceResult {
        InferenceResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<InferenceResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::model::ModelSpec>>(
                    "model_spec",
                    InferenceResult::get_model_spec_for_reflect,
                    InferenceResult::mut_model_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::classification::ClassificationResult>(
                    "classification_result",
                    InferenceResult::has_classification_result,
                    InferenceResult::get_classification_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::regression::RegressionResult>(
                    "regression_result",
                    InferenceResult::has_regression_result,
                    InferenceResult::get_regression_result,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InferenceResult>(
                    "InferenceResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InferenceResult {
    fn clear(&mut self) {
        self.clear_model_spec();
        self.clear_classification_result();
        self.clear_regression_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InferenceResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InferenceResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MultiInferenceRequest {
    // message fields
    pub tasks: ::protobuf::RepeatedField<InferenceTask>,
    pub input: ::protobuf::SingularPtrField<super::input::Input>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MultiInferenceRequest {}

impl MultiInferenceRequest {
    pub fn new() -> MultiInferenceRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MultiInferenceRequest {
        static mut instance: ::protobuf::lazy::Lazy<MultiInferenceRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MultiInferenceRequest,
        };
        unsafe {
            instance.get(MultiInferenceRequest::new)
        }
    }

    // repeated .tensorflow.serving.InferenceTask tasks = 1;

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    // Param is passed by value, moved
    pub fn set_tasks(&mut self, v: ::protobuf::RepeatedField<InferenceTask>) {
        self.tasks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tasks(&mut self) -> &mut ::protobuf::RepeatedField<InferenceTask> {
        &mut self.tasks
    }

    // Take field
    pub fn take_tasks(&mut self) -> ::protobuf::RepeatedField<InferenceTask> {
        ::std::mem::replace(&mut self.tasks, ::protobuf::RepeatedField::new())
    }

    pub fn get_tasks(&self) -> &[InferenceTask] {
        &self.tasks
    }

    fn get_tasks_for_reflect(&self) -> &::protobuf::RepeatedField<InferenceTask> {
        &self.tasks
    }

    fn mut_tasks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<InferenceTask> {
        &mut self.tasks
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

impl ::protobuf::Message for MultiInferenceRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.tasks {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tasks)?;
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
        for value in &self.tasks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.input.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tasks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for MultiInferenceRequest {
    fn new() -> MultiInferenceRequest {
        MultiInferenceRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MultiInferenceRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<InferenceTask>>(
                    "tasks",
                    MultiInferenceRequest::get_tasks_for_reflect,
                    MultiInferenceRequest::mut_tasks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::input::Input>>(
                    "input",
                    MultiInferenceRequest::get_input_for_reflect,
                    MultiInferenceRequest::mut_input_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MultiInferenceRequest>(
                    "MultiInferenceRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MultiInferenceRequest {
    fn clear(&mut self) {
        self.clear_tasks();
        self.clear_input();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MultiInferenceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MultiInferenceRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MultiInferenceResponse {
    // message fields
    pub results: ::protobuf::RepeatedField<InferenceResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MultiInferenceResponse {}

impl MultiInferenceResponse {
    pub fn new() -> MultiInferenceResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MultiInferenceResponse {
        static mut instance: ::protobuf::lazy::Lazy<MultiInferenceResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MultiInferenceResponse,
        };
        unsafe {
            instance.get(MultiInferenceResponse::new)
        }
    }

    // repeated .tensorflow.serving.InferenceResult results = 1;

    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    // Param is passed by value, moved
    pub fn set_results(&mut self, v: ::protobuf::RepeatedField<InferenceResult>) {
        self.results = v;
    }

    // Mutable pointer to the field.
    pub fn mut_results(&mut self) -> &mut ::protobuf::RepeatedField<InferenceResult> {
        &mut self.results
    }

    // Take field
    pub fn take_results(&mut self) -> ::protobuf::RepeatedField<InferenceResult> {
        ::std::mem::replace(&mut self.results, ::protobuf::RepeatedField::new())
    }

    pub fn get_results(&self) -> &[InferenceResult] {
        &self.results
    }

    fn get_results_for_reflect(&self) -> &::protobuf::RepeatedField<InferenceResult> {
        &self.results
    }

    fn mut_results_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<InferenceResult> {
        &mut self.results
    }
}

impl ::protobuf::Message for MultiInferenceResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.results {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.results)?;
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
        for value in &self.results {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.results {
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

impl ::protobuf::MessageStatic for MultiInferenceResponse {
    fn new() -> MultiInferenceResponse {
        MultiInferenceResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<MultiInferenceResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<InferenceResult>>(
                    "results",
                    MultiInferenceResponse::get_results_for_reflect,
                    MultiInferenceResponse::mut_results_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MultiInferenceResponse>(
                    "MultiInferenceResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MultiInferenceResponse {
    fn clear(&mut self) {
        self.clear_results();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MultiInferenceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MultiInferenceResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'tensorflow_serving/apis/inference.proto\x12\x12tensorflow.serving\x1a\
    ,tensorflow_serving/apis/classification.proto\x1a#tensorflow_serving/api\
    s/input.proto\x1a#tensorflow_serving/apis/model.proto\x1a(tensorflow_ser\
    ving/apis/regression.proto\"n\n\rInferenceTask\x12<\n\nmodel_spec\x18\
    \x01\x20\x01(\x0b2\x1d.tensorflow.serving.ModelSpecR\tmodelSpec\x12\x1f\
    \n\x0bmethod_name\x18\x02\x20\x01(\tR\nmethodName\"\x8f\x02\n\x0fInferen\
    ceResult\x12<\n\nmodel_spec\x18\x01\x20\x01(\x0b2\x1d.tensorflow.serving\
    .ModelSpecR\tmodelSpec\x12_\n\x15classification_result\x18\x02\x20\x01(\
    \x0b2(.tensorflow.serving.ClassificationResultH\0R\x14classificationResu\
    lt\x12S\n\x11regression_result\x18\x03\x20\x01(\x0b2$.tensorflow.serving\
    .RegressionResultH\0R\x10regressionResultB\x08\n\x06result\"\x81\x01\n\
    \x15MultiInferenceRequest\x127\n\x05tasks\x18\x01\x20\x03(\x0b2!.tensorf\
    low.serving.InferenceTaskR\x05tasks\x12/\n\x05input\x18\x02\x20\x01(\x0b\
    2\x19.tensorflow.serving.InputR\x05input\"W\n\x16MultiInferenceResponse\
    \x12=\n\x07results\x18\x01\x20\x03(\x0b2#.tensorflow.serving.InferenceRe\
    sultR\x07resultsB\x03\xf8\x01\x01J\xee\x0e\n\x06\x12\x04\n\07\x01\n\xa0\
    \x04\n\x01\x0c\x12\x03\n\0\x122\x95\x04\x20This\x20file\x20contains\x20m\
    essages\x20for\x20various\x20machine\x20learning\x20inferences\n\x20such\
    \x20as\x20regression\x20and\x20classification.\n\n\x20In\x20many\x20appl\
    ications\x20more\x20than\x20one\x20type\x20of\x20inference\x20is\x20desi\
    red\x20for\x20a\x20single\n\x20input.\x20\x20For\x20example,\x20given\
    \x20meteorologic\x20data\x20an\x20application\x20may\x20want\x20to\n\x20\
    perform\x20a\x20classification\x20to\x20determine\x20if\x20we\x20should\
    \x20expect\x20rain,\x20snow\x20or\x20sun\n\x20and\x20also\x20perform\x20\
    a\x20regression\x20to\x20predict\x20the\x20temperature.\n\x20Sharing\x20\
    the\x20single\x20input\x20data\x20between\x20two\x20inference\x20tasks\
    \x20can\x20be\x20accomplished\n\x20using\x20MultiInferenceRequest\x20and\
    \x20MultiInferenceResponse.\n\n\x08\n\x01\x08\x12\x03\x0c\0\x1f\n\x0b\n\
    \x04\x08\xe7\x07\0\x12\x03\x0c\0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\
    \x03\x0c\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x0c\x07\x17\n\x0e\
    \n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x0c\x07\x17\n\x0c\n\x05\x08\xe7\
    \x07\0\x03\x12\x03\x0c\x1a\x1e\n\t\n\x02\x03\0\x12\x03\x0e\x075\n\t\n\
    \x02\x03\x01\x12\x03\x0f\x07,\n\t\n\x02\x03\x02\x12\x03\x10\x07,\n\t\n\
    \x02\x03\x03\x12\x03\x11\x071\n\x08\n\x01\x02\x12\x03\x13\x08\x1a\nJ\n\
    \x02\x04\0\x12\x04\x16\0\x1d\x01\x1a>\x20Inference\x20request\x20such\
    \x20as\x20classification,\x20regression,\x20etc...\n\n\n\n\x03\x04\0\x01\
    \x12\x03\x16\x08\x15\n\x0b\n\x04\x04\0\x02\0\x12\x03\x17\x02\x1b\n\r\n\
    \x05\x04\0\x02\0\x04\x12\x04\x17\x02\x16\x17\n\x0c\n\x05\x04\0\x02\0\x06\
    \x12\x03\x17\x02\x0b\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x17\x0c\x16\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x17\x19\x1a\n\xbd\x01\n\x04\x04\0\x02\
    \x01\x12\x03\x1c\x02\x19\x1a\xaf\x01\x20Signature's\x20method_name.\x20S\
    hould\x20be\x20one\x20of\x20the\x20method\x20names\x20defined\x20in\n\
    \x20third_party/tensorflow/python/saved_model/signature_constants.py.\n\
    \x20e.g.\x20\"tensorflow/serving/classify\".\n\n\r\n\x05\x04\0\x02\x01\
    \x04\x12\x04\x1c\x02\x17\x1b\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x1c\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x1c\t\x14\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03\x1c\x17\x18\nK\n\x02\x04\x01\x12\x04\x20\0'\x01\
    \x1a?\x20Inference\x20result,\x20matches\x20the\x20type\x20of\x20request\
    \x20or\x20is\x20an\x20error.\n\n\n\n\x03\x04\x01\x01\x12\x03\x20\x08\x17\
    \n\x0b\n\x04\x04\x01\x02\0\x12\x03!\x02\x1b\n\r\n\x05\x04\x01\x02\0\x04\
    \x12\x04!\x02\x20\x19\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03!\x02\x0b\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03!\x0c\x16\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03!\x19\x1a\n\x0c\n\x04\x04\x01\x08\0\x12\x04#\x02&\x03\n\x0c\
    \n\x05\x04\x01\x08\0\x01\x12\x03#\x08\x0e\n\x0b\n\x04\x04\x01\x02\x01\
    \x12\x03$\x043\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03$\x04\x18\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03$\x19.\n\x0c\n\x05\x04\x01\x02\x01\x03\
    \x12\x03$12\n\x0b\n\x04\x04\x01\x02\x02\x12\x03%\x04+\n\x0c\n\x05\x04\
    \x01\x02\x02\x06\x12\x03%\x04\x14\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\
    \x03%\x15&\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03%)*\n@\n\x02\x04\x02\
    \x12\x04*\00\x01\x1a4\x20Inference\x20request\x20containing\x20one\x20or\
    \x20more\x20requests.\n\n\n\n\x03\x04\x02\x01\x12\x03*\x08\x1d\n\x1f\n\
    \x04\x04\x02\x02\0\x12\x03,\x02#\x1a\x12\x20Inference\x20tasks.\n\n\x0c\
    \n\x05\x04\x02\x02\0\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\x02\x02\0\x06\
    \x12\x03,\x0b\x18\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03,\x19\x1e\n\x0c\n\
    \x05\x04\x02\x02\0\x03\x12\x03,!\"\n\x1a\n\x04\x04\x02\x02\x01\x12\x03/\
    \x02\x12\x1a\r\x20Input\x20data.\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\
    /\x02,#\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03/\x02\x07\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03/\x08\r\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03/\
    \x10\x11\nA\n\x02\x04\x03\x12\x043\07\x01\x1a5\x20Inference\x20request\
    \x20containing\x20one\x20or\x20more\x20responses.\n\n\n\n\x03\x04\x03\
    \x01\x12\x033\x08\x1e\nv\n\x04\x04\x03\x02\0\x12\x036\x02'\x1ai\x20List\
    \x20of\x20results;\x20one\x20for\x20each\x20InferenceTask\x20in\x20the\
    \x20request,\x20returned\x20in\x20the\n\x20same\x20order\x20as\x20the\
    \x20request.\n\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x036\x02\n\n\x0c\n\x05\
    \x04\x03\x02\0\x06\x12\x036\x0b\x1a\n\x0c\n\x05\x04\x03\x02\0\x01\x12\
    \x036\x1b\"\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x036%&b\x06proto3\
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
