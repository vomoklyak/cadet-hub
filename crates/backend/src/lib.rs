use crate::error::CadetHubBeError;

pub type CadetHubBeResult<T> = Result<T, CadetHubBeError>;

pub mod context;
pub mod error;
mod facade;
mod mapper;
mod repository;
mod service;
