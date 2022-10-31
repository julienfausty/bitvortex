# bitvortex
A data processing library focussed on asynchronous concurrency

## Statement of intent

This library is meant to be an assembly of useful algorithms for data analytics natively written in rust. It makes heavy use of the `filter/pipeline` paradigm with a focus on concurrency and asynchronous execution as well as data parallelism.

## Building

Being a rust project, bitvortex uses the cargo framework for building and testing.

## Contributing
### Development Workflow

For now, please perform changes in a separate branch (hopefully well named to reflect the changes it provides) and propose a merge request for review before merging into master.

### Formating

Let's try and follow the rust standard naming conventions.

Be sure to run rustfmt on your code before proposing it up for review and adding it into the repository (right now all the formating is just the default rust formating).

### Testing

Every new development should have associated unit testing. Developments that do not have unit tests will most likely not pass review.

Integration testing should be implemented when deemed fit.
