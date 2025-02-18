// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteObject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_object`](crate::client::Client::delete_object).
///
/// See [`crate::client::fluent_builders::DeleteObject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteObject {
    _private: (),
}
impl DeleteObject {
    /// Creates a new builder-style object to manufacture [`DeleteObjectInput`](crate::input::DeleteObjectInput).
    pub fn builder() -> crate::input::delete_object_input::Builder {
        crate::input::delete_object_input::Builder::default()
    }
    /// Creates a new `DeleteObject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteObject {
    type Output =
        std::result::Result<crate::output::DeleteObjectOutput, crate::error::DeleteObjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_object_error(response)
        } else {
            crate::operation_deser::parse_delete_object_response(response)
        }
    }
}

/// Operation shape for `DescribeObject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_object`](crate::client::Client::describe_object).
///
/// See [`crate::client::fluent_builders::DescribeObject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeObject {
    _private: (),
}
impl DescribeObject {
    /// Creates a new builder-style object to manufacture [`DescribeObjectInput`](crate::input::DescribeObjectInput).
    pub fn builder() -> crate::input::describe_object_input::Builder {
        crate::input::describe_object_input::Builder::default()
    }
    /// Creates a new `DescribeObject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeObject {
    type Output =
        std::result::Result<crate::output::DescribeObjectOutput, crate::error::DescribeObjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_object_error(response)
        } else {
            crate::operation_deser::parse_describe_object_response(response)
        }
    }
}

/// Operation shape for `GetObject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_object`](crate::client::Client::get_object).
///
/// See [`crate::client::fluent_builders::GetObject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetObject {
    _private: (),
}
impl GetObject {
    /// Creates a new builder-style object to manufacture [`GetObjectInput`](crate::input::GetObjectInput).
    pub fn builder() -> crate::input::get_object_input::Builder {
        crate::input::get_object_input::Builder::default()
    }
    /// Creates a new `GetObject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for GetObject {
    type Output = std::result::Result<crate::output::GetObjectOutput, crate::error::GetObjectError>;
    fn parse_unloaded(
        &self,
        response: &mut aws_smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_get_object(response))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_get_object_error(response)
    }
}

/// Operation shape for `ListItems`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_items`](crate::client::Client::list_items).
///
/// See [`crate::client::fluent_builders::ListItems`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListItems {
    _private: (),
}
impl ListItems {
    /// Creates a new builder-style object to manufacture [`ListItemsInput`](crate::input::ListItemsInput).
    pub fn builder() -> crate::input::list_items_input::Builder {
        crate::input::list_items_input::Builder::default()
    }
    /// Creates a new `ListItems` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListItems {
    type Output = std::result::Result<crate::output::ListItemsOutput, crate::error::ListItemsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_items_error(response)
        } else {
            crate::operation_deser::parse_list_items_response(response)
        }
    }
}

/// Operation shape for `PutObject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_object`](crate::client::Client::put_object).
///
/// See [`crate::client::fluent_builders::PutObject`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutObject {
    _private: (),
}
impl PutObject {
    /// Creates a new builder-style object to manufacture [`PutObjectInput`](crate::input::PutObjectInput).
    pub fn builder() -> crate::input::put_object_input::Builder {
        crate::input::put_object_input::Builder::default()
    }
    /// Creates a new `PutObject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutObject {
    type Output = std::result::Result<crate::output::PutObjectOutput, crate::error::PutObjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_object_error(response)
        } else {
            crate::operation_deser::parse_put_object_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
