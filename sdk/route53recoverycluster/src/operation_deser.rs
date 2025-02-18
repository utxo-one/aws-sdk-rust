// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_routing_control_state_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetRoutingControlStateOutput,
    crate::error::GetRoutingControlStateError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetRoutingControlStateError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::GetRoutingControlStateError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::GetRoutingControlStateError { meta: generic, kind: crate::error::GetRoutingControlStateErrorKind::AccessDeniedException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "EndpointTemporarilyUnavailableException" => crate::error::GetRoutingControlStateError { meta: generic, kind: crate::error::GetRoutingControlStateErrorKind::EndpointTemporarilyUnavailableException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::endpoint_temporarily_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_endpoint_temporarily_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::GetRoutingControlStateError { meta: generic, kind: crate::error::GetRoutingControlStateErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::GetRoutingControlStateError { meta: generic, kind: crate::error::GetRoutingControlStateErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::GetRoutingControlStateError { meta: generic, kind: crate::error::GetRoutingControlStateErrorKind::ThrottlingException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::GetRoutingControlStateError { meta: generic, kind: crate::error::GetRoutingControlStateErrorKind::ValidationException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::GetRoutingControlStateError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_routing_control_state_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetRoutingControlStateOutput,
    crate::error::GetRoutingControlStateError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_routing_control_state_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_routing_control_state(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetRoutingControlStateError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_routing_controls_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::ListRoutingControlsOutput,
    crate::error::ListRoutingControlsError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListRoutingControlsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListRoutingControlsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ListRoutingControlsError {
            meta: generic,
            kind: crate::error::ListRoutingControlsErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListRoutingControlsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "EndpointTemporarilyUnavailableException" => crate::error::ListRoutingControlsError {
            meta: generic,
            kind:
                crate::error::ListRoutingControlsErrorKind::EndpointTemporarilyUnavailableException(
                    {
                        #[allow(unused_mut)]
                        let mut tmp = {
                            #[allow(unused_mut)]let mut output = crate::error::endpoint_temporarily_unavailable_exception::Builder::default();
                            let _ = response;
                            output = crate::json_deser::deser_structure_crate_error_endpoint_temporarily_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListRoutingControlsError::unhandled)?;
                            output.build()
                        };
                        if tmp.message.is_none() {
                            tmp.message = _error_message;
                        }
                        tmp
                    },
                ),
        },
        "InternalServerException" => crate::error::ListRoutingControlsError {
            meta: generic,
            kind: crate::error::ListRoutingControlsErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListRoutingControlsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::ListRoutingControlsError {
            meta: generic,
            kind: crate::error::ListRoutingControlsErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListRoutingControlsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::ListRoutingControlsError {
            meta: generic,
            kind: crate::error::ListRoutingControlsErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListRoutingControlsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ListRoutingControlsError {
            meta: generic,
            kind: crate::error::ListRoutingControlsErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListRoutingControlsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListRoutingControlsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_routing_controls_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::ListRoutingControlsOutput,
    crate::error::ListRoutingControlsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_routing_controls_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_routing_controls(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::ListRoutingControlsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_update_routing_control_state_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::UpdateRoutingControlStateOutput,
    crate::error::UpdateRoutingControlStateError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::UpdateRoutingControlStateError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::UpdateRoutingControlStateError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::UpdateRoutingControlStateError { meta: generic, kind: crate::error::UpdateRoutingControlStateErrorKind::AccessDeniedException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ConflictException" => crate::error::UpdateRoutingControlStateError { meta: generic, kind: crate::error::UpdateRoutingControlStateErrorKind::ConflictException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "EndpointTemporarilyUnavailableException" => crate::error::UpdateRoutingControlStateError { meta: generic, kind: crate::error::UpdateRoutingControlStateErrorKind::EndpointTemporarilyUnavailableException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::endpoint_temporarily_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_endpoint_temporarily_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::UpdateRoutingControlStateError { meta: generic, kind: crate::error::UpdateRoutingControlStateErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::UpdateRoutingControlStateError { meta: generic, kind: crate::error::UpdateRoutingControlStateErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::UpdateRoutingControlStateError { meta: generic, kind: crate::error::UpdateRoutingControlStateErrorKind::ThrottlingException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::UpdateRoutingControlStateError { meta: generic, kind: crate::error::UpdateRoutingControlStateErrorKind::ValidationException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStateError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::UpdateRoutingControlStateError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_update_routing_control_state_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::UpdateRoutingControlStateOutput,
    crate::error::UpdateRoutingControlStateError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_routing_control_state_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_update_routing_control_states_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::UpdateRoutingControlStatesOutput,
    crate::error::UpdateRoutingControlStatesError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::UpdateRoutingControlStatesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::UpdateRoutingControlStatesError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::UpdateRoutingControlStatesError { meta: generic, kind: crate::error::UpdateRoutingControlStatesErrorKind::AccessDeniedException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStatesError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ConflictException" => crate::error::UpdateRoutingControlStatesError { meta: generic, kind: crate::error::UpdateRoutingControlStatesErrorKind::ConflictException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStatesError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "EndpointTemporarilyUnavailableException" => crate::error::UpdateRoutingControlStatesError { meta: generic, kind: crate::error::UpdateRoutingControlStatesErrorKind::EndpointTemporarilyUnavailableException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::endpoint_temporarily_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_endpoint_temporarily_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStatesError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::UpdateRoutingControlStatesError { meta: generic, kind: crate::error::UpdateRoutingControlStatesErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStatesError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::UpdateRoutingControlStatesError { meta: generic, kind: crate::error::UpdateRoutingControlStatesErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStatesError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ServiceLimitExceededException" => crate::error::UpdateRoutingControlStatesError { meta: generic, kind: crate::error::UpdateRoutingControlStatesErrorKind::ServiceLimitExceededException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::service_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStatesError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::UpdateRoutingControlStatesError { meta: generic, kind: crate::error::UpdateRoutingControlStatesErrorKind::ThrottlingException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStatesError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::UpdateRoutingControlStatesError { meta: generic, kind: crate::error::UpdateRoutingControlStatesErrorKind::ValidationException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoutingControlStatesError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::UpdateRoutingControlStatesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_update_routing_control_states_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::UpdateRoutingControlStatesOutput,
    crate::error::UpdateRoutingControlStatesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_routing_control_states_output::Builder::default();
        let _ = response;
        output.build()
    })
}
