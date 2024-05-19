# Lesson 02: Architecture of Actix Web

1. Actix Web Overview:
   - Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust.
   - It handles HTTP protocol functionalities, including GET, POST, PUT, DELETE requests, status codes, and HTTP headers.

2. Setting Up an Actix Web Server:
   - The HTTP server handles protocol requirements and is initialized with minimal code.
   - Business logic, route handling, and custom middleware reside within an `App` instance.

3. Asynchronous Main Function:
    - Convert the main function to asynchronous using a macro attribute for async runtime, typically provided by `tokio`.

4. Initializing and Running the Server:
    - Create a service instance and bind it to an IP address and port.
    - Use `await` to run the server asynchronously.

5. Application Instance:
    - Acts as a logical container for middleware, shared state, and route handlers.
    - Route handlers are associated with HTTP verbs and defined functions.

6. Grouping Mechanisms:
    - **Service**: Logical container to group related routes.
    - **Scope**: Label or sub-URL for grouping routes, useful for versioning APIs.

7. Example Code:
    - Simple setup for a “Hello World” endpoint using Actix Web.
    - Use tools like Postman to test HTTP requests beyond what a browser can do.

### Test Code

- start server:

```shell
cargo run
```

- test get v1 profile:

```shell
curl -X GET --location "http://localhost:8080/v1/profile"
```

- test post v1 profile:

```shell
curl -X POST --location "http://localhost:8080/v1/profile"
```

- test get v2 profile:

```shell
curl -X GET --location "http://localhost:8080/v2/profile"
```

- test post v2 profile:

```shell
curl -X POST --location "http://localhost:8080/v2/profile"
```
