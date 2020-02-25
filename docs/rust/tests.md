# Tests

The Rust community thinks about tests in terms of two main categories: unit tests and integration tests. 

## Unit Testing

**Unit tests** are small and more focused, testing one module in isolation at a time, and can test private interfaces.

## Integration Testing

**Integration tests** are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

Their purpose is to test whether many parts of your library work together correctly

They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API.

Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well. 

As you add more integration tests, you might want to make more than one file in the tests directory to help organize them; for example, you can group the test functions by the functionality they’re testing. As mentioned earlier, each file in the tests directory is compiled as its own separate crate.

Files in the `tests` directory don’t share the same behavior as files in `src` do. Please see

## Testing Style

### Unit Tests

All tests:
- Are contained within `/src` directory
- Must be inside the same file it is testing
- In each file to contain the test functions and to annotate the module with `#[cfg(test)]`.
  - This annotation stops the tests from being built and ran
  - `cfg` stands for configuration and tells Rust that the following item should only be included given a certain configuration option. Only runs when explicity `cargo test` is used
- Rust’s privacy rules do allow you to test private functions - i.e. `fn` instead of `pub fn`
  - Instead private functions will need to be brung into scope, in the same file

### Integration Tests
All tests:
- Are contained within `/test` directory - at the same level of `src`
- Don't require the need of `#[cfg(test)]`, as the following annotation `#[test]` does fine

Cargo knows:
- How to look for the integration files in this directory

## Submodules in integration tests
`[Need to learn more of the how modules work, before one can tests sub modules individually]`

`[To be continued]`
