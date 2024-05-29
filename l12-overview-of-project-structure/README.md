# Lesson 12: Overview of Project Structure

1. Project structure:
    - the `src` folder contains both `main.rs` and `lib.rs`.
    - `lib.rs` is used to allow integration tests to access the core project code, which wouldn't possible with just a binary project.
    - the `Cargo.toml` file is configured to define the library and the main binary.
2. Integration Test:
    - integration tests are placed in a separate folder and are considered their own package, necessitating the `lib.rs` structure to enable proper access to the core project.
3. Execution flow:
    - the main functionality is contained within the lib file, while `main.rs` serves as an initializer to kick off the application only.
4. Practical Lab Exercise:
   - run the server:
   ```shell
   cargo run --bin server
   ```
   - output:
   ```shell
   Compiling l12-overview-of-project-structure v0.1.0 (~/Public/udemy-Learn-Actix-Web-and-SqlX-and-Create-a-Twitter-Api-Clone/l12-overview-of-project-structure)
   Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.09s
   Running `target/debug/server`
   Twitter API is running!
   ```
   - run the test:
   ```shell
   cargo test
   ```
   - output:
   ```shell
   Compiling l12-overview-of-project-structure v0.1.0 (~/Public/udemy-Learn-Actix-Web-and-SqlX-and-Create-a-Twitter-Api-Clone/l12-overview-of-project-structure)
   Finished `test` profile [unoptimized + debuginfo] target(s) in 0.17s
   Running unittests src/lib.rs (target/debug/deps/twitter_api-235c5fb349dd8187)

   running 0 tests

   test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        Running unittests src/main.rs (target/debug/deps/server-3a1aef7078d7e580)

   running 0 tests

   test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        Running tests/integration_test.rs (target/debug/deps/integration_test-13d5bb142182b7ce)

   running 1 test
   test test_run ... ok

   test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

      Doc-tests twitter_api

   running 0 tests

   test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   ```
   - with this example, we can see that both `main.rs` and `integration_test.rs` can access to the core functionalities that expose from `lib.rs` without and barer.