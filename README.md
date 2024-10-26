# RUST - Hello World Project (RUST, React, Docker Compose)

This project consists of a Rust HTTP service and a React frontend. The frontend fetches a message from the Rust service, and both are containerized with Docker. This README includes instructions for setting up the project and running it locally, including running the Rust service using Cargo.

![image](https://github.com/user-attachments/assets/900a75b1-fe2f-460a-a8a2-4a04a377c105)

Client-Server interaction where the frontend relies on the backend to provide dynamic content

![image](https://github.com/user-attachments/assets/9d28f272-7be8-4c70-83b6-867afc6c46c6)

## Execution Of Rust HTTP service and a React frontend with Docker Compose

![](https://github.com/YashzAlphaGeek/Rust-React-The-Dockerized-Adventure/blob/master/docs/DockerCompose_Backend_Frontend.gif)

## Prerequisites

### For Rust Service Backend

- Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
- Install Cargo, the Rust package manager (comes with the Rust installation).

   Warp uses a filter-based system to handle requests, making it easy to compose handlers for different routes. It's ideal for building RESTful APIs and web applications due to its ergonomic syntax and powerful functionality.

   [Warp](https://docs.rs/warp/latest/warp/)

### For React Frontend

- Ensure you have [Node.js](https://nodejs.org/) installed on your machine.
- Install React and React DOM, along with their type definitions for React:
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

## Running the Docker Compose

- [Docker](https://www.docker.com/get-started) installed on your system.
- [Docker Compose](https://docs.docker.com/compose/install/) installed on your system.

   ```bash
   docker-compose up --build


