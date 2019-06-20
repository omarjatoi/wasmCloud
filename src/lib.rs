//! A [WebAssembly Standard Capabilities Library](https://wascap.io) for Rust
//!
//! This library can be used for embedding, extracting, and validating capabilities claims
//! in WebAssembly modules. While there are some standard, well-known claims already defined,
//! you can add custom claims in your own namespaces if you like.
//!
//! The following example illustrates embedding a new set of claims
//! into a WebAssembly module, then extracting, validating, and examining those claims.
//! ```rust
//!use wascap::prelude::*;
//!
//!# fn read_unsigned_wasm() -> Vec<u8> {
//!#   include_bytes!("../examples/loop.wasm").to_vec()
//!# }
//!# fn main() -> Result<()> {
//! let unsigned = read_unsigned_wasm(); // Read a Wasm file into a byte vector
//! let issuer = KeyPair::new_account(); // Create an Ed25519 key pair to sign the module
//! let module = KeyPair::new_module(); // Create a key pair for the module itself
//!
//! // Grant the module some basic capabilities, with no date limits
//! let claims = ClaimsBuilder::new()
//!     .with_capability(caps::MESSAGING)
//!     .with_capability(caps::KEY_VALUE)
//!     .issuer(&issuer.public_key())
//!     .subject(&module.public_key())
//!     .build();
//!
//! // Sign the JWT and embed it into the WebAssembly module, returning the signed bytes
//! let embedded = wasm::embed_claims(&unsigned, &claims, &issuer)?;
//!
//! // Extract a signed JWT from a WebAssembly module's bytes (performs a check on
//! // the signed module hash)
//! let extracted = wasm::extract_claims(&embedded)?.unwrap();
//!
//! // Validate dates, signature, JWT structure, etc.
//! let v = validate_token(&extracted.jwt)?;
//!
//! assert_eq!(v.expired, false);
//! assert_eq!(v.cannot_use_yet, false);
//! assert_eq!(v.expires_human, "never");
//! assert_eq!(v.not_before_human, "immediately");
//! assert_eq!(extracted.claims.issuer, issuer.public_key());
//!
//!# Ok(())
//!# }
//! ```
//!
//! The `Ed25519` key functionality is provided by the [nkeys](https://docs.rs/nkeys) crate.

// Copyright 2015-2018 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

/// Wascap-specific result type
pub type Result<T> = std::result::Result<T, errors::Error>;
pub use errors::Error;

// https://tools.ietf.org/html/rfc7519#section-4.1

pub mod caps;
mod errors;
pub mod jwt;
pub mod wasm;

#[doc(hidden)]
#[cfg(feature = "cli")]
pub mod cli;

pub mod prelude {
    //! Public re-exports of the most commonly used wascap types
    pub use super::{Error, Result};
    pub use crate::caps;
    pub use crate::jwt::{validate_token, Claims, ClaimsBuilder};
    pub use crate::wasm;
    pub use nkeys::KeyPair;
}
