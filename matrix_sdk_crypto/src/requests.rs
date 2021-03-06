// Copyright 2020 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use matrix_sdk_common::{
    api::r0::{
        keys::{
            claim_keys::Response as KeysClaimResponse,
            get_keys::{IncomingRequest as KeysQueryRequest, Response as KeysQueryResponse},
            upload_keys::{Request as KeysUploadRequest, Response as KeysUploadResponse},
        },
        to_device::send_event_to_device::{
            IncomingRequest as ToDeviceRequest, Response as ToDeviceResponse,
        },
    },
    uuid::Uuid,
};

/// TODO
#[derive(Debug)]
pub enum OutgoingRequests {
    /// TODO
    KeysUpload(KeysUploadRequest),
    /// TODO
    KeysQuery(KeysQueryRequest),
    /// TODO
    ToDeviceRequest(ToDeviceRequest),
}

impl From<KeysQueryRequest> for OutgoingRequests {
    fn from(request: KeysQueryRequest) -> Self {
        OutgoingRequests::KeysQuery(request)
    }
}

impl From<KeysUploadRequest> for OutgoingRequests {
    fn from(request: KeysUploadRequest) -> Self {
        OutgoingRequests::KeysUpload(request)
    }
}

impl From<ToDeviceRequest> for OutgoingRequests {
    fn from(request: ToDeviceRequest) -> Self {
        OutgoingRequests::ToDeviceRequest(request)
    }
}

/// TODO
#[derive(Debug)]
pub enum IncomingResponse<'a> {
    /// TODO
    KeysUpload(&'a KeysUploadResponse),
    /// TODO
    KeysQuery(&'a KeysQueryResponse),
    /// TODO
    ToDevice(&'a ToDeviceResponse),
    ///
    KeysClaim(&'a KeysClaimResponse),
}

impl<'a> From<&'a KeysUploadResponse> for IncomingResponse<'a> {
    fn from(response: &'a KeysUploadResponse) -> Self {
        IncomingResponse::KeysUpload(response)
    }
}

impl<'a> From<&'a KeysQueryResponse> for IncomingResponse<'a> {
    fn from(response: &'a KeysQueryResponse) -> Self {
        IncomingResponse::KeysQuery(response)
    }
}

impl<'a> From<&'a ToDeviceResponse> for IncomingResponse<'a> {
    fn from(response: &'a ToDeviceResponse) -> Self {
        IncomingResponse::ToDevice(response)
    }
}

impl<'a> From<&'a KeysClaimResponse> for IncomingResponse<'a> {
    fn from(response: &'a KeysClaimResponse) -> Self {
        IncomingResponse::KeysClaim(response)
    }
}

/// TODO
#[derive(Debug, Clone)]
pub struct OutgoingRequest {
    /// The unique id of a request, needs to be passed when receiving a
    /// response.
    pub(crate) request_id: Uuid,
    /// TODO
    pub(crate) request: Arc<OutgoingRequests>,
}

impl OutgoingRequest {
    /// Get the unique id of this request.
    pub fn request_id(&self) -> &Uuid {
        &self.request_id
    }

    /// Get the underlying outgoing request.
    pub fn request(&self) -> &OutgoingRequests {
        &self.request
    }
}
