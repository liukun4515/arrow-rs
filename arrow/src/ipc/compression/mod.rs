// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#[cfg(feature = "ipc_compression")]
mod ipc_compression;
#[cfg(feature = "ipc_compression")]
pub(crate) use ipc_compression::CompressionCodecType;

#[cfg(not(feature = "ipc_compression"))]
mod stubs;
#[cfg(not(feature = "ipc_compression"))]
pub(crate) use stubs::CompressionCodecType;

pub(crate) const LENGTH_EMPTY_COMPRESSED_DATA: i64 = 0;
pub(crate) const LENGTH_NO_COMPRESSED_DATA: i64 = -1;
pub(crate) const LENGTH_OF_PREFIX_DATA: i64 = 8;
