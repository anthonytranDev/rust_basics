# Commands

## Cargo

### cargo check 
This command quickly checks your code to make sure it compiles but doesn’t produce an executable

### cargo build
Do this once a project has been git cloned

### cargo build --release
When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug.

One profile for development i.e. build and one for stage/production

Command above is used to benchmark code

### cargo test <unit-fn-name>
Tests a specific glob of that function name

### cargo test --test <specific_integration_test_filename>
Tests a specific glob of that integration test file name

### cargo test -- --test-thread=1
Stops tests from running in parallel, useful when testing for states that do not depend on each other

### cargo test -- --nocapture
Disables the output capture behavior

### cargo test -- --ignored
To only run the ignored tests

### cargo test -- --help
Displays the options you can use after the separator --.

### cargo doc --open
Enables to install the docs from the all the dependencies in your project

## rustup

### rustup docs --book
Opens the beginers guide up
