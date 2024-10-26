# RUST - Hello World Project

This project consists of a Rust HTTP service and a React frontend. The frontend fetches a message from the Rust service, and both are containerized with Docker. This README includes instructions for setting up the project and running it locally, including running the Rust service using Cargo.

![image](https://github.com/user-attachments/assets/900a75b1-fe2f-460a-a8a2-4a04a377c105)

Client-Server interaction where the frontend relies on the backend to provide dynamic content

![image](https://github.com/user-attachments/assets/9d28f272-7be8-4c70-83b6-867afc6c46c6)

## Prerequisites

### For Rust Service

- Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
- Install Cargo, the Rust package manager (comes with the Rust installation).

### For React Frontend

- Ensure you have [Node.js](https://nodejs.org/) installed on your machine.
- Install React and React DOM, along with their type definitions for TypeScript:
  ```bash
  npm install react react-dom
  npm install --save-dev @types/react @types/react-dom

  
## Running the Rust Service

To run the Rust service:

1. Navigate to the Rust service directory (where `Cargo.toml` is located).
2. Build the Rust project:
   ```bash
   cargo build
3. Run the Rust project:
   ```bash
   cargo run

Make sure the Rust service is running on http://localhost:3030/hello/world


