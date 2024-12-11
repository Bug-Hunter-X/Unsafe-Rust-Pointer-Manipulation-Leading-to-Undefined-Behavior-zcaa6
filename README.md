# Unsafe Rust Pointer Bug

This repository demonstrates a common error when working with unsafe code and mutable pointers in Rust.  Modifying data through a raw pointer after the vector's ownership or scope changes can result in unpredictable behavior, memory corruption, and crashes.

The `bug.rs` file showcases the faulty code, while `bugSolution.rs` provides a corrected version using safe Rust practices.

This example highlights the importance of careful consideration when dealing with unsafe operations in Rust.  It's crucial to understand the implications of memory management and ensure data integrity to prevent unexpected errors.