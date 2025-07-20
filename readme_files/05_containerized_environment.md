# Chapter 5: Containerized Environment

In [Chapter 4: Application Configuration](04_application_configuration.md), we learned how our application smartly reads its settings from the environment. This allows us to run the same compiled code in different setups. We briefly saw how this works in a "containerized" world using a `docker-compose.yml` file.

Now, it's time to zoom out and understand what that containerized world actually is. This is the final piece of the puzzle that makes our project consistent, portable, and easy for anyone to run.

### The Problem: "It Works on My Machine!"

Have you ever tried to run a friend's project on your computer, only to spend hours installing the right version of a programming language, a specific database, and a dozen other tools? It's a common and frustrating experience. Every developer's machine is slightly different, leading to the classic excuse: "Well, it works on my machine!"

This is where a **Containerized Environment** comes to the rescue.

Think of it like buying **flat-pack furniture**. Instead of a carpenter showing up and building a custom table in your living room (which might not fit anywhere else), the furniture company sends you a box. This box contains:
1.  All the precisely cut parts (our application, the database).
2.  A clear, step-by-step instruction manual.

With this kit, you can build the *exact same* piece of furniture in your living room, your friend's garage, or a warehouse on the other side of the world. The result is always identical.

Our project uses **Docker** to create this "flat-pack kit". It has two instruction sheets:
-   `Dockerfile`: The manual for building one piece of furniture (our Rust app).
-   `docker-compose.yml`: The master plan for assembling all the pieces together (our app *and* the database) and making them work as a complete system.

### The `Dockerfile`: Instructions for Building Our App

The `Dockerfile` is a text file that contains a list of commands to build a **Docker image**. An image is like a lightweight, read-only template for our application—a perfectly packaged box containing our compiled Rust program and everything it needs to run.

Our `Dockerfile` uses a clever technique called a **multi-stage build**. It's like having a messy workshop and a clean showroom.

#### Stage 1: The Workshop (`builder`)

First, we use a large "workshop" environment that has all the tools we need to build our app.

```dockerfile
# File: Dockerfile

# Start with a big box that has all the Rust tools
FROM rust:1.88.0 as builder

# Set up a working area inside the box
WORKDIR /app

# Copy over the project's dependency list
COPY Cargo.toml Cargo.lock ./

# Create a dummy file to build and cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Now copy the real source code
RUN rm -rf src
COPY src ./src
COPY .sqlx ./.sqlx

# Build the final application using the cached dependencies
RUN cargo build --release
```
-   `FROM rust:1.88.0 as builder`: We start with an official Docker image that already has Rust version 1.88.0 installed. We name this stage `builder`.
-   `WORKDIR /app`: We create a folder inside the image to do our work.
-   `COPY` and `RUN cargo build`: We first build the dependencies and then the full application. This is a common caching trick to make future builds faster.
-   At the end of this stage, we have our compiled Rust program located at `/app/target/release/axum_postgres_rust`. However, this `builder` image is very large because it contains the entire Rust compiler and all the intermediate files.

#### Stage 2: The Showroom (Final Image)

We don't want to ship our big, messy workshop. We only want the finished product. So, we start over with a fresh, tiny box and copy *only* the compiled program into it.

```dockerfile
# File: Dockerfile

# Start with a new, tiny, empty Debian box
FROM debian:bullseye-slim

WORKDIR /app

# Copy the compiled binary from the "builder" workshop stage
COPY --from=builder /app/target/release/axum_postgres_rust .

# Tell Docker that the container will listen on this port
EXPOSE 7878

# The command to run when the container starts
CMD ["./axum_postgres_rust"]
```
-   `FROM debian:bullseye-slim`: We start with a very small, minimal base image.
-   `COPY --from=builder ...`: This is the magic! We reach into our `builder` stage and pull out just one file: the compiled application. We leave all the heavy tools and clutter behind.
-   `EXPOSE 7878`: This is like putting a sign on the box that says, "This box has a window on port 7878 for communication."
-   `CMD [...]`: This sets the default command to run when a container is started from this image. In our case, it simply runs our program.

The result is a tiny, efficient Docker image containing only what's necessary to run our application.

### `docker-compose.yml`: The Master Assembly Plan

Now we have a box for our app, but our system needs more than that. It also needs a database! The `docker-compose.yml` file is the master plan that defines all the services we need and orchestrates them.

Let's look at the two services defined in our file: `postgres` and `app`.

#### The `postgres` Service

This service sets up our PostgreSQL database.

```yaml
# File: docker-compose.yml

services:
  postgres:
    image: postgres:15
    environment:
      - POSTGRES_USER=root
      - POSTGRES_PASSWORD=1234
      - POSTGRES_DB=axum_postgres
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U root"]
      # ... other settings ...
    networks:
      - app-network
```
-   `image: postgres:15`: We don't build this one ourselves. We just tell Docker to download the official, pre-made PostgreSQL version 15 image.
-   `environment`: We configure the database by setting the username, password, and default database name.
-   `healthcheck`: This is a crucial instruction. It tells Docker Compose, "Don't consider this service ready until the `pg_isready` command inside it succeeds."

#### The `app` Service

This service runs our Rust application.

```yaml
# File: docker-compose.yml

services:
  app:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "7878:7878"
    depends_on:
      postgres:
        condition: service_healthy
    environment:
      - DATABASE_URL=postgres://root:1234@postgres:5432/axum_postgres
    networks:
      - app-network
```
-   `build: ...`: Instead of using a pre-made image, this tells Docker Compose: "Build the image for this service using the `Dockerfile` in the current directory (`.`)."
-   `ports: - "7878:7878"`: This connects our computer's port 7878 to the container's internal port 7878 (which we opened with `EXPOSE`). This is how we can access our API from our web browser.
-   `depends_on: ...`: A critical rule! "Do not start the `app` service until the `postgres` service's healthcheck is passing." This prevents our app from starting and immediately crashing because the database isn't ready.
-   `environment`: As we saw in [Chapter 4: Application Configuration](04_application_configuration.md), this injects the `DATABASE_URL` into our app. Notice the hostname is `postgres`. Docker Compose creates a private virtual network (`app-network`) where services can find each other by their service name.

### Running the Whole System

With these two files, `Dockerfile` and `docker-compose.yml`, running our entire project becomes incredibly simple. All a developer needs to do is open their terminal in the project root and run one command:

`docker-compose up`

This single command kicks off a fully automated process:

```mermaid
sequenceDiagram
    participant User
    participant DC as Docker Compose
    participant DB as Postgres Container
    participant App as App Container

    User->>+DC: Runs `docker-compose up`
    DC->>DC: Reads `docker-compose.yml`
    DC->>+DB: Starts `postgres` container from `postgres:15` image
    DC->>DB: Periodically runs healthcheck
    DB-->>DC: Healthcheck finally passes (database is ready!)
    DC->>DC: Builds `app` image using `Dockerfile`
    DC->>+App: Starts `app` container from the new image
    Note over App: Injects `DATABASE_URL` pointing to `postgres`
    App->>DB: Connects to database over private network
    DC-->>-User: System is running and ready!
```

In a matter of moments, you have a complete, isolated environment where your Rust application and PostgreSQL database are running and configured to work together perfectly.

### Conclusion

Congratulations on completing the entire `axum_postgres_rust` tutorial!

In this final chapter, you've learned about the power of a **Containerized Environment**. You now understand:
-   How a `Dockerfile` acts as a recipe to build a clean, minimal image of our Rust application.
-   How a `docker-compose.yml` file acts as a master plan to define, configure, and link multiple services like our app and a database.
-   How this approach solves the "it works on my machine" problem, providing a consistent and reproducible environment for anyone, anywhere.

By combining the powerful routing of [Axum Web Router](01_axum_web_router.md), the efficient [Handler Functions & Shared State](02_handler_functions___shared_state.md), the safety of [Compile-Time Verified SQL](03_compile_time_verified_sql.md), the flexibility of [Application Configuration](04_application_configuration.md), and the consistency of a Containerized Environment, you have seen how all the pieces of a modern, professional web service fit together.

You now have the knowledge to explore this project, make changes to the Rust code, and simply run `docker-compose up --build` to see your changes live. Happy coding

---
