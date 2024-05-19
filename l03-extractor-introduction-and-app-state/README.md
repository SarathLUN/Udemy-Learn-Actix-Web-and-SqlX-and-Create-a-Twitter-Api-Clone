# Lesson 03: Extractors: Introduction and App State

1. Extractors in Actix Web:
   - Extractors allow route handlers to access various types of data such as query strings, JSON bodies, path parameters, and global app state.
   - Extracted data comes in as statically typed Rust types.
2. App Data Extractor:
   - Used to manage globally shared state across different handler functions.
   - App data can be mutable or immutable depending on the application's needs.
3. Setting Up Mutable App Data:
   - Define a struct to hold the shared state.
   - Use `actix_web::web::Data` to wrap the shared state, which makes it thread-safe by using `Arc` (Atomic Reference Counting) and `Mutex` (Mutual Exclusion).
   - Clone the `Data` instance to allow each worker thread to have its reference to the shared state.
4. Mutable State Handling:
   - To modify the state, acquire a lock using `Mutex`, perform the mutation, and then unlock.
   - Demonstrated through a route that appends "world" to an existing message.
5. Immutable App Data:
   - If the state doesn't need to be mutable, avoid using `Mutex` to reduce thread contention.
   - Each worker thread can have its instance of immutable state data.
6. Practical Examples:
   - Examples include routes for updating the state and retrieving the state to verify changes.
   - start the server:

      ```shell
      cargo run
      ```

   - test update AppData:
      ```shell
      curl -X POST --location "http://localhost:8080"
      ```

   - test get AppData:
      ```shell
      curl -X GET --location "http://localhost:8080"
      ```
     
