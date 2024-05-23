# Lesson 04: Extractors: The Core Extractors Path and JSON

1. Dependencies:
    - ensure you include `serde` and `serde_json` for serialization and deserialization when dealing with JSON data in
      Rust.
2. Setting up Types:
    - define structs for the data types you'll handle, ensuring they are `Deserialize` if you plan to parse them from
      JSON.
3. Path Extractor:
    - use `Path` extractor from Actix to handle path parameters.
    - when defining routes with path parameters, wrap the parameter inside a type and ensure it's deserializable.
    - Example: if querying users by ID, create a struct like `struct UserID { id: u32 }` to capture the ID from the
      path.
4. App State:
    - use `app_data` to share state across different handlers.
    - Example of this lesson involves setting up a global, mutable list of users within an `RwLock` for concurrent
      access.
5. Path parameter handling:
    - extract parameters directly from the URL and convert them to Rust types using the `Path` extractor.
    - Example: `async fn get_user(data: web::Data<AppState>, path: web::Path<UserID>) -> impl Responder`.
6. JSON extractor:
    - use the `Json` extractor to handle JSON bodies in POST requests.
    - define a type to represent the incoming JSON structure and ensure it is `Deserialize`.
    - Example: create a new user from a JSON payload by defining a struct `NewUser` and using `web::Json<NewUser>` in
      the handler.
7. In-Memory data store:
    - for demonstration, maintain an in-memory data store using a list (vector) of users, protected by an `RwLock`.
    - insert and query data from this list, showcasing the use of both path and JSON extractors.
8. Concurrency Handling:
    - use `RwLock` for read-write access to the in-memory data store, allowing concurrent reads and exclusive writes.
9. Route Handlers:
    - demonstrate handling different routes, one for querying users by ID and another for adding new users via JSON POST
      requests.
    - Example: defining routes within a scope and resource, such
      as `scope("/api").service(web::resource("/user/{id}").route(web::get().to(get_user)))`.
10. Basic Error Handling:
    - handle possible errors in extracting and processing data, such as using `unwrap` cautiously.

### Test the Lab:

- start server:
```shell
cargo run
```

- there will be a list of users as below:
```json
{ "id": 1, "user_name": "tony", "full_name": "Tony Stark" }
{ "id": 2, "user_name": "stave", "full_name": "Steven Roger" }
{ "id": 3, "user_name": "natasha", "full_name": "Natasha" }
```

- we can start to query user_name by ID:
```shell
curl -X GET --location "http://localhost:8001/v1/user/1"
```

- we can start to insert user and get back the user_id of created user:
```shell
curl -X POST --location "http://localhost:8001/v1/user" \
    -H "Content-Type: application/json" \
    -d '{
          "user_name": "tim",
          "full_name": "Tim Tom"
        }'
```

- we can query new user by ID we just get:
```shell
curl -X GET --location "http://localhost:8001/v1/user/4"
```
