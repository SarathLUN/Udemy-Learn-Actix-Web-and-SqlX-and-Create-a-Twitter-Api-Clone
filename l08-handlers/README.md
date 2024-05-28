# Lesson 08: Handlers

1. Data model creation: create a data model that the handler will return, ensuring it derives the necessary traits like `Clone` and `Serialize`.
2. Global state management: use a global state to manage data temporarily, which is especially useful when a database is not yet integrated.
3. Path parameters: extract parameters from the path to filter and query data.
4. Custom Error types: implement custom error types to provide clear and specific error messages.
5. Response Handling: use the `Either` type to handle multiple possible return types, allowing for more sophisticated response handling, such as redirections.
6. Redirection: implement redirection logic within handlers to guide users to different endpoints base on certain conditions.
