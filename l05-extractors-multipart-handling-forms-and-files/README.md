# Lesson 05: Extractors: Multipart, Handling Forms and Files

1. Understanding Extractors in Actix Web:
    - Actix Web provides extractors like `Path`, `Json`, and `AppData` to automatically convert raw payloads into Rust types. 
    - Custom extractors can be created for unique client-side parameters, especially useful for handling multipart forms with files and other complex data.
2. Implementing the `FromRequest` Trait:
    - Custom extractors are implemented by defining the `FromRequest` trait. 
    - This involves handling the payload asynchronously, parsing it, and converting it into the desired Rust types.
3. Handling Multipart Form Data:
    - Multipart form data handling involves defining boundaries and parsing each part asynchronously. 
    - Each field in the multipart data must be processed, converting it from its raw form (bytes) to the appropriate Rust type (e.g., strings, vectors of bytes for files).

### Code reference is in the final project of this course =>

- [model.rs](https://github.com/SarathLUN/forked-build-twitter-api-clone-actix/blob/main/section_3/twitter-api/src/routes/profiles/model.rs)
- [profile_route_test.rs](https://github.com/SarathLUN/forked-build-twitter-api-clone-actix/blob/main/section_3/twitter-api/tests/routes/profiles/profile_route_test.rs)
- [actix_fixture.rs](https://github.com/SarathLUN/forked-build-twitter-api-clone-actix/blob/main/section_3/twitter-api/src/common_tests/actix_fixture.rs)