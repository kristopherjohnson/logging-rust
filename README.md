This is a simple example/playground for demonstrating how to implement a custom
logger in Rust.

For more information, see <https://docs.rs/log/0.4.11/log/#implementing-a-logger>

Example output:

```
2020-12-12 17:36:06.216 ERROR [test1][logging:src/lib.rs:62] This is an error message
2020-12-12 17:36:06.223  WARN [test2][logging:src/lib.rs:65] This is a warning message
2020-12-12 17:36:06.229  INFO [test3][logging:src/lib.rs:68] This is an informational message
2020-12-12 17:36:06.236 DEBUG [test4][logging:src/lib.rs:71] This is a debug message
2020-12-12 17:36:06.242 TRACE [test5][logging:src/lib.rs:74] This is a trace message
2020-12-12 17:36:06.247  INFO [logging][logging:src/lib.rs:77] This informational message has no target specified
2020-12-12 17:36:06.253 ERROR [test1][logging:src/lib.rs:62] This is an error message
2020-12-12 17:36:06.259  WARN [test2][logging:src/lib.rs:65] This is a warning message
2020-12-12 17:36:06.265  INFO [test3][logging:src/lib.rs:68] This is an informational message
2020-12-12 17:36:06.271 DEBUG [test4][logging:src/lib.rs:71] This is a debug message
2020-12-12 17:36:06.278 TRACE [test5][logging:src/lib.rs:74] This is a trace message
2020-12-12 17:36:06.283  INFO [logging][logging:src/lib.rs:77] This informational message has no target specified
2020-12-12 17:36:06.289 ERROR [test1][logging:src/lib.rs:62] This is an error message
2020-12-12 17:36:06.296  WARN [test2][logging:src/lib.rs:65] This is a warning message
2020-12-12 17:36:06.302  INFO [test3][logging:src/lib.rs:68] This is an informational message
2020-12-12 17:36:06.308 DEBUG [test4][logging:src/lib.rs:71] This is a debug message
2020-12-12 17:36:06.313 TRACE [test5][logging:src/lib.rs:74] This is a trace message
2020-12-12 17:36:06.319  INFO [logging][logging:src/lib.rs:77] This informational message has no target specified
```
