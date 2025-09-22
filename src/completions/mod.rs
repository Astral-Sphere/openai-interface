//! Given a prompt, the model will return one or more predicted completions,
//! and can also return the probabilities of alternative tokens at each position.
//! Compared to the `chat` API, this one does not provide the ability to have
//! multiple rounds of conversation. This API is getting deprecated in favor of the
//! `chat` API.

pub mod request;
