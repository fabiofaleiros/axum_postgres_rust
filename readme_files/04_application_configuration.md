# Chapter 4: Application Configuration

In [Chapter 3: Compile-Time Verified SQL](03_compile_time_verified_sql.md), we saw how the `sqlx` library connects to our database during compilation to check our queries. A critical question arises from this: how did `sqlx` know *which* database to connect to? The address, username, and password weren't written anywhere in our Rust code.

This chapter explains the answer. We will explore the professional practice of **Application Configuration**, a technique for keeping settings separate from your code.

### The Problem: Hard-Coded Settings are Brittle

Imagine you're building a mobile app that navigates you to a friend's house. You would never build your friend's exact address into the app's source code. If you did, you'd have to rewrite and reinstall the app every time you wanted to go somewhere new! Instead, the app lets you *input* the destination. The app's logic (how to read a map) is separate from its data (the specific address).

This is the same principle for our web server. We have settings like:
- The server's address (`127.0.0.1:7878`).
- The database connection URL (`postgres://user:pass@host:port/db`).

If we write these values directly into our `src/main.rs` file, we create a huge problem. What if we want to run our app on a different server that uses a different database? We'd have to change our code, recompile it, and redeploy it. This is slow, error-prone, and a bad practice.

### The Solution: Separating Config from Code

The solution is to make our application read its settings from the **environment** it's running in. This means our compiled code is a generic "engine" that can be configured on the fly, just like your navigation app.

We achieve this in two ways depending on the context:
1.  **For Local Development:** We use a simple text file named `.env`.
2.  **For Production/Containers:** We use the container system (`Docker`) to "inject" the settings.

The application code itself doesn't care where the settings come from; it just asks the operating system, "What is the database URL?" This makes our application incredibly portable and easy to manage.

### The `.env` File: Your Local Cheat Sheet

When you're running the application on your own computer for development, the easiest way to provide settings is with a `.env` file. This file lives in the root of our project and is intentionally kept simple.

```sh
# File: .env

DATABASE_URL="postgres://root:1234@localhost:5432/axum_postgres"
SERVER_ADDRESS="127.0.0.1:7878"
RUST_BACKTRACE=full
SQLX_OFFLINE=true
```
- **`DATABASE_URL`**: Tells the application (and `sqlx`) how to connect to the PostgreSQL database running on your local machine (`localhost`).
- **`SERVER_ADDRESS`**: Tells the Axum server which address and port to listen on for incoming requests.

This file is a set of key-value pairs. It's simple, easy to read, and—importantly—it's **not part of the compiled code**. You can change the port or database password here without ever touching the Rust source.

### Reading the Configuration in Rust

So, how does our application read the `.env` file? It happens right at the start of our `main` function in `src/main.rs`.

**Step 1: Load the `.env` file**

We use a handy little library called `dotenvy` to do the heavy lifting. This one line is all it takes:

```rust
// File: src/main.rs

#[tokio::main]
async fn main() {
    // expose the environment variables
    dotenvy::dotenv().expect("Failed to read .env file");

    // ... the rest of the main function
}
```
When `dotenvy::dotenv()` is called, it looks for a `.env` file in the project directory, reads all the key-value pairs, and loads them into the application's environment. It's like handing the cheat sheet to our program as it starts up.

**Step 2: Use the Variables**

Once the variables are loaded, we can read them using Rust's standard library.

```rust
// File: src/main.rs

// ... after dotenvy::dotenv()

// set variables from the environment variables
let server_address = std::env::var("SERVER_ADDRESS")
    .unwrap_or("127.0.0.1:3000".to_owned());
let database_url = std::env::var("DATABASE_URL")
    .expect("DATABASE_URL not found in the env file");
```
- `std::env::var("DATABASE_URL")`: This asks the environment, "Do you have a value for the key `DATABASE_URL`?"
- `.expect(...)`: If the `DATABASE_URL` is *not* found, the program will immediately stop with an error message. This is good! Our application is useless without a database, so we want to fail early if it's not configured correctly.
- `.unwrap_or(...)`: This is a softer alternative. It tries to get `SERVER_ADDRESS`, but if it's not found, it uses a default value (`"127.0.0.1:3000"`) instead of crashing.

These variables, `server_address` and `database_url`, are then used to start our server and connect to the database.

### Configuration in a Containerized World

This system becomes even more powerful when we run our application inside a container. In the next chapter, we will discuss containers in detail, but for now, let's look at how configuration works there.

Instead of a `.env` file, the `docker-compose.yml` file is responsible for injecting the environment variables.

```yaml
# File: docker-compose.yml

services:
  app:
    # ... build instructions ...
    ports:
      - "7878:7878"
    depends_on:
      # ...
    environment:
      - DATABASE_URL=postgres://root:1234@postgres:5432/axum_postgres
      - SERVER_ADDRESS=0.0.0.0:7878
      # ... other variables
```
- `environment:`: This section in the `docker-compose.yml` file defines all the environment variables that will be given to our application when it starts inside the container.
- Notice the `DATABASE_URL` is different! It points to `postgres` instead of `localhost`. This is because inside the container network, the database service is reachable by its service name, `postgres`.

Our Rust code doesn't change at all! The exact same `std::env::var("DATABASE_URL")` call works perfectly. It just receives a different value depending on how the application is launched.

### The Big Picture: Same Code, Different Environments

This diagram shows how our single, unchanged application can be configured for two different environments.

```mermaid
sequenceDiagram
    participant User
    participant App as Rust Application
    participant Local as Local Environment (.env)
    participant Docker as Docker Environment (docker-compose.yml)

    subgraph Scenario 1: Local Development
        User->>Local: Runs `cargo run`
        Local->>App: Starts the app
        App->>Local: `dotenvy` reads `.env` file
        App->>App: Gets `DATABASE_URL=...localhost...`
    end

    subgraph Scenario 2: Containerized
        User->>Docker: Runs `docker-compose up`
        Docker->>App: Starts the app inside a container
        Docker->>App: Injects environment variables
        App->>App: Gets `DATABASE_URL=...postgres...`
    end
```
The key takeaway is that the `Rust Application` box is identical in both scenarios. Its behavior is changed by its *environment*, not by its code. This is the essence of modern application configuration.

### Conclusion

You have now learned one of the most important principles of building robust and portable software: **separating configuration from code**. We saw how our `axum_postgres_rust` project avoids hard-coding settings and instead reads them from the environment.

You learned:
- Why hard-coding settings is a bad idea.
- How to use a `.env` file for easy local development.
- How to load these settings in Rust using the `dotenvy` crate and `std::env::var`.
- How the same application code can be configured differently for a containerized environment using `docker-compose.yml`.

This powerful pattern makes our application flexible, easy to manage, and ready for different deployment scenarios without a single code change.

Now that we understand how our application is configured inside a container, let's take a closer look at how that entire containerized world is set up.

Ready to dive into Docker? Let's proceed to the final chapter: [Chapter 5: Containerized Environment](05_containerized_environment.md).

---
