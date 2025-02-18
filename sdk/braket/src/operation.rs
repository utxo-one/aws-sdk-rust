// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CancelJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`cancel_job`](crate::client::Client::cancel_job).
///
/// See [`crate::client::fluent_builders::CancelJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelJob {
    _private: (),
}
impl CancelJob {
    /// Creates a new builder-style object to manufacture [`CancelJobInput`](crate::input::CancelJobInput).
    pub fn builder() -> crate::input::cancel_job_input::Builder {
        crate::input::cancel_job_input::Builder::default()
    }
    /// Creates a new `CancelJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelJob {
    type Output = std::result::Result<crate::output::CancelJobOutput, crate::error::CancelJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_cancel_job_error(response)
        } else {
            crate::operation_deser::parse_cancel_job_response(response)
        }
    }
}

/// Operation shape for `CancelQuantumTask`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`cancel_quantum_task`](crate::client::Client::cancel_quantum_task).
///
/// See [`crate::client::fluent_builders::CancelQuantumTask`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelQuantumTask {
    _private: (),
}
impl CancelQuantumTask {
    /// Creates a new builder-style object to manufacture [`CancelQuantumTaskInput`](crate::input::CancelQuantumTaskInput).
    pub fn builder() -> crate::input::cancel_quantum_task_input::Builder {
        crate::input::cancel_quantum_task_input::Builder::default()
    }
    /// Creates a new `CancelQuantumTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelQuantumTask {
    type Output = std::result::Result<
        crate::output::CancelQuantumTaskOutput,
        crate::error::CancelQuantumTaskError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_cancel_quantum_task_error(response)
        } else {
            crate::operation_deser::parse_cancel_quantum_task_response(response)
        }
    }
}

/// Operation shape for `CreateJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_job`](crate::client::Client::create_job).
///
/// See [`crate::client::fluent_builders::CreateJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateJob {
    _private: (),
}
impl CreateJob {
    /// Creates a new builder-style object to manufacture [`CreateJobInput`](crate::input::CreateJobInput).
    pub fn builder() -> crate::input::create_job_input::Builder {
        crate::input::create_job_input::Builder::default()
    }
    /// Creates a new `CreateJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateJob {
    type Output = std::result::Result<crate::output::CreateJobOutput, crate::error::CreateJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_job_error(response)
        } else {
            crate::operation_deser::parse_create_job_response(response)
        }
    }
}

/// Operation shape for `CreateQuantumTask`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_quantum_task`](crate::client::Client::create_quantum_task).
///
/// See [`crate::client::fluent_builders::CreateQuantumTask`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateQuantumTask {
    _private: (),
}
impl CreateQuantumTask {
    /// Creates a new builder-style object to manufacture [`CreateQuantumTaskInput`](crate::input::CreateQuantumTaskInput).
    pub fn builder() -> crate::input::create_quantum_task_input::Builder {
        crate::input::create_quantum_task_input::Builder::default()
    }
    /// Creates a new `CreateQuantumTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateQuantumTask {
    type Output = std::result::Result<
        crate::output::CreateQuantumTaskOutput,
        crate::error::CreateQuantumTaskError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_quantum_task_error(response)
        } else {
            crate::operation_deser::parse_create_quantum_task_response(response)
        }
    }
}

/// Operation shape for `GetDevice`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_device`](crate::client::Client::get_device).
///
/// See [`crate::client::fluent_builders::GetDevice`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDevice {
    _private: (),
}
impl GetDevice {
    /// Creates a new builder-style object to manufacture [`GetDeviceInput`](crate::input::GetDeviceInput).
    pub fn builder() -> crate::input::get_device_input::Builder {
        crate::input::get_device_input::Builder::default()
    }
    /// Creates a new `GetDevice` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDevice {
    type Output = std::result::Result<crate::output::GetDeviceOutput, crate::error::GetDeviceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_device_error(response)
        } else {
            crate::operation_deser::parse_get_device_response(response)
        }
    }
}

/// Operation shape for `GetJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_job`](crate::client::Client::get_job).
///
/// See [`crate::client::fluent_builders::GetJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetJob {
    _private: (),
}
impl GetJob {
    /// Creates a new builder-style object to manufacture [`GetJobInput`](crate::input::GetJobInput).
    pub fn builder() -> crate::input::get_job_input::Builder {
        crate::input::get_job_input::Builder::default()
    }
    /// Creates a new `GetJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetJob {
    type Output = std::result::Result<crate::output::GetJobOutput, crate::error::GetJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_job_error(response)
        } else {
            crate::operation_deser::parse_get_job_response(response)
        }
    }
}

/// Operation shape for `GetQuantumTask`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_quantum_task`](crate::client::Client::get_quantum_task).
///
/// See [`crate::client::fluent_builders::GetQuantumTask`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetQuantumTask {
    _private: (),
}
impl GetQuantumTask {
    /// Creates a new builder-style object to manufacture [`GetQuantumTaskInput`](crate::input::GetQuantumTaskInput).
    pub fn builder() -> crate::input::get_quantum_task_input::Builder {
        crate::input::get_quantum_task_input::Builder::default()
    }
    /// Creates a new `GetQuantumTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetQuantumTask {
    type Output =
        std::result::Result<crate::output::GetQuantumTaskOutput, crate::error::GetQuantumTaskError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_quantum_task_error(response)
        } else {
            crate::operation_deser::parse_get_quantum_task_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `SearchDevices`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`search_devices`](crate::client::Client::search_devices).
///
/// See [`crate::client::fluent_builders::SearchDevices`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SearchDevices {
    _private: (),
}
impl SearchDevices {
    /// Creates a new builder-style object to manufacture [`SearchDevicesInput`](crate::input::SearchDevicesInput).
    pub fn builder() -> crate::input::search_devices_input::Builder {
        crate::input::search_devices_input::Builder::default()
    }
    /// Creates a new `SearchDevices` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SearchDevices {
    type Output =
        std::result::Result<crate::output::SearchDevicesOutput, crate::error::SearchDevicesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_search_devices_error(response)
        } else {
            crate::operation_deser::parse_search_devices_response(response)
        }
    }
}

/// Operation shape for `SearchJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`search_jobs`](crate::client::Client::search_jobs).
///
/// See [`crate::client::fluent_builders::SearchJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SearchJobs {
    _private: (),
}
impl SearchJobs {
    /// Creates a new builder-style object to manufacture [`SearchJobsInput`](crate::input::SearchJobsInput).
    pub fn builder() -> crate::input::search_jobs_input::Builder {
        crate::input::search_jobs_input::Builder::default()
    }
    /// Creates a new `SearchJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SearchJobs {
    type Output =
        std::result::Result<crate::output::SearchJobsOutput, crate::error::SearchJobsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_search_jobs_error(response)
        } else {
            crate::operation_deser::parse_search_jobs_response(response)
        }
    }
}

/// Operation shape for `SearchQuantumTasks`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`search_quantum_tasks`](crate::client::Client::search_quantum_tasks).
///
/// See [`crate::client::fluent_builders::SearchQuantumTasks`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SearchQuantumTasks {
    _private: (),
}
impl SearchQuantumTasks {
    /// Creates a new builder-style object to manufacture [`SearchQuantumTasksInput`](crate::input::SearchQuantumTasksInput).
    pub fn builder() -> crate::input::search_quantum_tasks_input::Builder {
        crate::input::search_quantum_tasks_input::Builder::default()
    }
    /// Creates a new `SearchQuantumTasks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SearchQuantumTasks {
    type Output = std::result::Result<
        crate::output::SearchQuantumTasksOutput,
        crate::error::SearchQuantumTasksError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_search_quantum_tasks_error(response)
        } else {
            crate::operation_deser::parse_search_quantum_tasks_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
