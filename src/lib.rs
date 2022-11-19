//! bitvortex
//!
//! A library for processing and analysing data concurrently, asynchronously and in parallel
use futures::Stream;
use std::rc::Rc;

/// Source
/// A trait for a struct that can provide an output data stream
pub trait Source<T> {
    fn stream(&self) -> Box<dyn Stream<Item = T>>;
}

/// Pipe
/// Trait for the main processing elements acting on data streams
pub trait Pipe<InT, OutT>: Source<OutT> {
    /// connect a source to the pipe (returns an error if unsuccessful)
    fn pipe(&mut self, input: Rc<dyn Source<InT>>) -> Result<(), &'static str>;
    /// sever input connection to the pipe
    fn unpipe(&mut self);
    /// return a reference to the input source
    fn get_input(&self) -> Option<Rc<dyn Source<InT>>>;
}

/// data_bucket
/// Sub module holding the definitions of the data model for the library
pub mod data_bucket;
