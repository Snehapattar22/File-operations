# File operations

## Introduction
### This Rust-based project implements file system utility functions covering path handling, directory traversal, file metadata, permissions, and efficient file I/O operations. The implementation is modular, uses only Rust’s standard library, and is designed to demonstrate system-level programming concepts and clean code organization.

## Implemented (311–330)
Path Utilities
311: Get Relative Path
312: Get Absolute Path
313: Get Current Working Directory
314: Change Working Directory

File & Directory Operations
315: Find Files (non-recursive)
316: Find Files Recursively
317: Get File Last Modified Time
318: Set File Last Modified Time (platform-safe stub)

Permissions & Ownership
319: Check File Permissions
320: Change File Permissions (stubbed safely)
321: Change File Owner (OS-specific stub)

Symbolic Links
322: Create Symbolic Link
323: Read Symbolic Link

File Locking
324: Lock File (conceptual stub)
325: Unlock File (conceptual stub)

File Read / Write
326: Append to File
327: Read File Lines
328: Write File Lines
329: Read File Chunk
330: Write File Chunk

## How to Run
### Build 
- cargo build
### Run
- cargo run

## How to Call Individual Functions
### To test or use any function, call it from main.rs
