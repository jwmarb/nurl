# **Part 1: Introduction, Team, and Why Rust**

## <u>**Speaker 1</u>**

Hello everyone, and welcome to our presentation on our Rust-based URL shortener project, nurl. I'm [Name], and joining me are [Name] and [Name]. Each of us will be presenting for about five minutes, covering different aspects of our project and the Rust language.

#### **Why We Chose Rust and This Project**
So, why did we choose rust for our project?:

Firstly, we chose rust since all of us were interested in writing something in rust for the first time, and wanted to see how it was. However, more feature-wise, we chose Rust for because it's well known for its safety, speed, and modern approach to systems programming. Rust's memory safety guarantees, lack of a garbage collector, and excellent concurrency support make it ideal for backend services where reliability and performance are critical. Our application, a URL shortener with authentication, benefits from these features, especially since it handles user data and needs to be robust against bugs and security vulnerabilities (if it was to be deployed in a production environment and scaled).

#### **Brief Overview of the Application**
As a super-brief overview of the app, we made a URL shortener that allows users to resgister to make an account,
log into the account they made, and then view a dashboard where they can create and manage shortened URLs. Each user has their own set of URLs, and all operations are protected by JWT-based authentication for secturity. The backend is built with the Actix-web framework and uses PostgreSQL for storing the data and ensuring it persists, like you would in a prod environment app.

#### **Rust VS Other Languages**
So, we have used a good amount of other languages throughout our times as CS majors. For the sake of conciseness, lets limit the comparisons to the ones we have used in classes we have taken. Namely, Python, Java, C, Javascript, Prolog, Standard ML, Chapel, MIPS assembly.

---

### Rust vs. Python
- **Type System:** Rust is statically typed and checks types at compile time; Python is dynamically typed and checks types at runtime.
- **Memory Management:** Rust uses ownership and borrowing; Python relies on automatic garbage collection.
- **Concurrency:** Rust supports safe, native concurrency and prevents data races at compile time; Python is limited by the Global Interpreter Lock (GIL) and uses multiprocessing for parallelism.
- **Performance:** Rust is compiled and generally much faster; Python is interpreted and slower, but quick for prototyping.
- **<u>OPTIONAL FOR EXTRA YAP TIME:</u> Syntax:** Rust enforces strict syntax and explicit mutability; Python is flexible, concise, and beginner-friendly.
- **Ecosystem:** Rust’s ecosystem is growing, especially for systems and async programming; Python’s ecosystem is vast and mature, especially for scripting, data science, AI, and web dev.

---

### Rust vs. Java
- **Type System:** Both Rust and Java are statically typed, but Rust uses type inference more extensively.
- **Memory Management:** Rust manages memory via ownership and borrowing; Java uses automatic garbage collection (JVM).
- **Concurrency:** Rust ensures thread safety and prevents data races at compile time; Java uses threads and synchronized blocks, but concurrency issues are checked at runtime.
- **Performance:** Rust compiles to machine code for predictable, high performance; Java compiles to bytecode and uses a JIT compiler, which can introduce overhead and unpredictable pauses.
- **Paradigms:** Rust supports multiple paradigms (functional, procedural, OOP); Java is primarily object-oriented with some functional features.
- **<u>OPTIONAL FOR EXTRA YAP TIME:</u> Syntax:** Rust’s syntax is strict and modern, requiring explicit mutability and lifetimes; Java is verbose and class-based.
- **Ecosystem:** Rust’s ecosystem is growing, especially for systems and backend development; Java’s ecosystem is mature and extensive, especially for enterprise and Android.

---

### Rust vs. C
- **Type System:** Both are statically typed and compiled, but Rust’s type system is safer and supports type inference.
- **Memory Management:** Rust manages memory safely using ownership and borrowing, preventing common bugs like buffer overflows and dangling pointers; C requires manual memory management, which is error-prone.
- **Concurrency:** Rust provides safe concurrency and prevents race conditions at compile time; C’s concurrency is manual and can easily introduce bugs.
- **<u>OPTIONAL FOR EXTRA YAP TIME:</u> Syntax:** Rust offers modern syntax and strictness, requiring explicit mutability; C’s syntax is minimal and permissive, which can lead to subtle errors.
- **Tooling:** Rust’s tooling (Cargo, crates.io) is modern and user-friendly; C’s tooling is minimal and more fragmented.
- **Ecosystem:** Rust’s ecosystem is growing, especially for modern systems programming; C’s ecosystem is mature and widely used for legacy systems.

---

### Rust vs. JavaScript
- **Type System:** Rust is statically typed and checks types at compile time; JavaScript is dynamically typed and checks types at runtime.
- **Memory Management:** Rust manages memory with ownership and borrowing, with no garbage collector; JavaScript uses automatic garbage collection.
- **Concurrency:** Rust supports safe, native concurrency and multithreading; JavaScript is single-threaded (event loop) and uses async callbacks/promises.
- **Performance:** Rust is compiled and offers near C/C++ performance; JavaScript is interpreted/JIT-compiled and generally slower, but optimized for web.
- **<u>OPTIONAL FOR EXTRA YAP TIME:</u> Syntax:** Rust’s syntax is strict and explicit; JavaScript is flexible and permissive.
- **Paradigms:** Rust supports multiple paradigms (functional, procedural, OOP); JavaScript is multi-paradigm (functional, OOP, event-driven).
- **Use Cases:** Rust is used for systems, backend, CLI, and embedded programming; JavaScript is dominant for web frontend, backend (Node.js), and scripting.
- **Ecosystem:** Rust’s ecosystem is growing, especially for backend and systems; JavaScript’s ecosystem (npm) is massive and mature for web development.

---

### Rust vs. Prolog

- **Type System:** Rust is statically and strongly typed with compile-time checks; Prolog is dynamically typed and untyped at the variable level, relying on unification and pattern matching.
- **Paradigm:** Rust is multi-paradigm (imperative, functional, some OOP); Prolog is a declarative, logic programming language focused on expressing relations and rules.
- **Memory Management:** Rust uses ownership and borrowing for memory safety without a garbage collector; Prolog abstracts memory management, typically using garbage collection.
- **Concurrency:** Rust has built-in, thread-safe concurrency; Prolog doesn't have it as a core feature
- **<u>OPTIONAL FOR EXTRA YAP TIME:</u> Syntax:** Rust uses curly-brace, C-like syntax; Prolog uses facts, rules, and queries in a syntax based on logic and relations.
- **Performance:** Rust is compiled to machine code and highly performant; Prolog is typically interpreted or compiled to bytecode for a virtual machine, and is slower for most procedural tasks.
- **Use Cases:** Rust is used for systems, backend, and performance-critical applications; Prolog excels in AI, symbolic reasoning, theorem proving, and language parsing.

---

### Rust vs. Standard ML

- **Type System:** Both are statically and strongly typed, but Rust emphasizes ownership and lifetimes, while Standard ML uses a Hindley-Milner type system with type inference.
- **Paradigm:** Rust is multi-paradigm (imperative, functional, OOP); Standard ML is primarily functional, with strong support for algebraic data types and pattern matching.
- **Memory Management:** Rust enforces memory safety through ownership and borrowing without garbage collection; Standard ML uses automatic garbage collection.
- **Concurrency:** Rust has built-in concurrency and prevents data races at compile time; Standard ML does not have built-in concurrency primitives.
- **<u>OPTIONAL FOR EXTRA YAP TIME:</u> Syntax:** Rust uses curly-brace, C-like syntax; Standard ML uses a functional, expression-based syntax.
- **Performance:** Rust compiles to efficient machine code using LLVM; Standard ML is compiled, but typically not as optimized for systems-level performance.
- **Use Cases:** Rust is used for systems, backend, and embedded programming; Standard ML is used in academia, language research, and for teaching functional programming concepts.

---

### Rust vs. Chapel

- **Type System:** Both are statically typed; Rust’s type system emphasizes safety and lifetimes, while Chapel’s is designed for parallelism and ease of use.
- **Paradigm:** Rust is multi-paradigm (imperative, functional, OOP); Chapel is designed for parallel programming, especially for high-performance computing (HPC).
- **Memory Management:** Rust uses ownership and borrowing for memory safety without garbage collection; Chapel uses garbage collection and allows more implicit memory management.
- **Concurrency/Parallelism:** Rust provides safe concurrency with threads and async; Chapel is built around parallelism and distributed computing, using Partitioned Global Address Space (PGAS).
- **<u>OPTIONAL FOR EXTRA YAP TIME:</u> Syntax:** Rust uses curly-brace, C-like syntax; Chapel has a more Python-like, readable syntax with explicit parallel constructs.
- **Performance:** Rust is compiled for high performance and control; Chapel is also compiled and optimized for parallel and distributed workloads.
- **Use Cases:** Rust is used for systems, backend, and embedded development; Chapel is used for scientific computing, HPC, and data-parallel tasks.
- **Ecosystem:** Rust’s ecosystem is broad and modern; Chapel’s ecosystem is specialized for HPC and parallel computing.

---

### Rust vs. Assembly

- **Type System:** Rust is statically and strongly typed; MIPS assembly has no type system, everything is just bits and registers.
- **Paradigm:** Rust is multi-paradigm (imperative, functional, OOP); assembly is purely imperative and low-level.
- **Memory Management:** Rust uses ownership and borrowing for memory safety; assembly requires manual memory management, with direct control over memory and registers.
- **Concurrency:** Rust has built-in, safe concurrency; assembly has no built-in concurrency support-parallelism must be managed at the hardware or OS level.
- **<u>OPTIONAL FOR EXTRA YAP TIME:</u> Syntax:** Rust uses high-level, curly-brace syntax; assembly uses low-level mnemonic instructions and direct addressing.
- **Performance:** Rust is compiled to efficient machine code, near the performance of C; assembly is as close to the hardware as possible, offering maximum control and performance (at the expense of programmer's sanity).
- **Use Cases:** Rust is used for systems, backend, and embedded programming; assembly is used for embedded systems, OS kernels.

---

# **Part 2: Rust Language Tutorial and Paradigms**

## **<u>Speaker 2</u>**

So, now that we have talked about some basic language comparisons and our rust usage, lets talk a little bit about the history + use cases for rust, along with its features.

#### **Overview of Rust: History and Usage**

- **Origins and Motivation:**  
  Rust was created by Graydon Hoare in 2006 while he was at Mozilla, inspired by the need for a safer alternative to C and C++ for systems programming. Hoare’s motivation stemmed from issues with memory bugs (like a broken elevator due to software errors), which are common in languages that require manual memory management.

- **Mozilla Sponsorship and Open Source:**  
  Mozilla began sponsoring Rust in 2009, aiming to use it for browser engines. In 2010, Rust was released as open source, allowing the community to contribute. The first stable release, Rust 1.0, came out in 2015.

- **Evolution and Influences:**  
  Rust’s design drew on decades of programming language research, blending together ideas from CLU, Erlang, Mesa, and others. Overall language was built to focus on safety and simplicity. Its unique ownership model replaced the need for garbage collection, a big feature that sets Rust apart from both C++ and memory-managed languages.

- **Adoption and Impact:**  
  Since 1.0, Rust has been used in high-profile projects, including Mozilla’s Servo browser engine and parts of Firefox. Companies like Microsoft have adopted Rust for rewriting core libraries to improve security and reliability.

- **Current Use Cases:**  
  Rust is now widely used for:
  - Systems-level programming (operating systems, browser engines)
  - Performance-critical backend services (like the web API in our project)
  - WebAssembly applications
  - Security-sensitive software

---

#### **Paradigms and Features**

- **Multi-Paradigm Flexibility:**  
  Rust is designed as a multi-paradigm language, blending object-oriented programming (OOP) and functional programming (FP) concepts. This allows developers to use structured and encapsulated designs when needed, or to leverage functional techniques like higher-order functions, pattern matching, and immutability.

- **Ownership and Borrowing (Memory Safety):**  
   Every value in Rust has a single owner, and the compiler enforces strict rules about how data is borrowed and mutated. The "borrow checker" is the tool that enforces these rules, ensuring memory and thread safety.

- **Zero-Cost Abstractions:**  
  Rust’s abstractions (like iterators, traits, and generics) are designed to compile down to highly efficient machine code with no runtime overhead. This allows devs to write expressive and high-level code without sacrificing performance.

- **Type System and Generics:**  
  Rust is statically and strongly typed, with a powerful type inference engine. Generics and traits enable code reuse and abstraction while maintaining type safety.

- **Concurrency and Parallelism:**  
  Rust’s type system enforces safe concurrency. Data races are prevented at compile time, making it possible to write multithreaded code that is both fast and reliable.

- **Security and Reliability:**  
  Rust is "secure by design," reducing vulnerabilities like buffer overflows and injection attacks. Its strict compile-time checks and lack of undefined behavior make it a strong choice for critical/sensitive applications.

---

Now, let's discuss the rust features that WE used in our project that would be important to understand for the next section when we showcase and demo the code:

#### **Core Rust Features in Web Development**
- **Async/Await Pattern**:  
  ```rust
  #[post("/api/auth")]
  async fn login(form: web::Json, db: web::Data) -> impl Responder {
      // Async database query
      let user = sqlx::query_as::("SELECT * FROM users...")
          .fetch_one(db.get_ref())
          .await;
  }
  ```
  Rust's async/await (via Tokio runtime in Actix-web) enables non-blocking I/O for high concurrency. The `.await` keyword suspends execution without blocking threads.

- **Type Safety with SQLx**:
  ```rust
  let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users...")
      .bind(&form.username)
      .fetch_one(pool)
      .await?;
  ```
  SQLx performs compile-time SQL query validation and type checking against your database schema.

- **Error Propagation**:
  ```rust
  let hashed = hash(&form.password, DEFAULT_COST)
      .map_err(|_| APIResponse::error_message("Hashing failed"))?;
  ```
  The `?` operator propagates errors while `map_err` converts error types for consistent API responses.

#### **Key Libraries Explained**
1. **Actix-web Framework**:
   - Actor-based web framework with middleware support
   - Routes are declared with procedural macros:
     ```rust
     #[post("/api/shorten")]
     async fn shorten_url(/* ... */) -> impl Responder { /* ... */ }
     ```
   - Uses extractors for request data:
     ```rust
     web::Json // Auto-deserializes JSON
     web::Data    // Database connection pool
     ```

2. **SQLx + PostgreSQL**:
   - Connection pooling for efficient DB access:
     ```rust
     PgPoolOptions::new()
         .max_connections(5)
         .connect(*POSTGRESQL_URL)
     ```
   - Compile-time checked queries:
     ```rust
     sqlx::query_as!(User, "SELECT id, username...")
     ```

3. **Bcrypt Password Hashing**:
   ```rust
   let hashed = hash(password, DEFAULT_COST)?; // Registration
   verify(password, &stored_hash)?;            // Login
   ```
   Uses adaptive hashing with salt to resist brute-force attacks.

4. **JWT Authentication**:
   - Token generation:
     ```rust
     jsonwebtoken::encode(&Header, &Claims { ... }, &EncodingKey)
     ```
   - Middleware verification:
     ```rust
     decode::(token, &DecodingKey, &Validation)
     ```

#### **Database Schema Management**
- **Schema Initialization**:
  ```rust
  CREATE TABLE users (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      username TEXT NOT NULL UNIQUE,
      password TEXT NOT NULL
  );
  ```
  Uses PostgreSQL UUIDs and constraints for data integrity.

- **Connection Pooling**:
  ```rust
  app.app_data(pool.clone()); // Shared across threads
  ```
  Managed via `PgPoolOptions` for optimal performance.

#### **Advanced Patterns**
- **Middleware Architecture**:
  ```rust
  impl Transform for ExtractUsernameJWT {
      fn new_transform(&self, service: S) -> Self::Future {
          // JWT processing logic
      }
  }
  ```
  Creates reusable authentication middleware.

- **Type-Driven API Responses**:
  ```rust
  #[derive(Serialize)]
  pub struct APIResponse {
      pub error: Option,
      pub data: Option
  }
  ```
  Ensures consistent error handling across endpoints.

- **Ownership in Database Operations**:
  ```rust
  pub async fn create_or_update_url(
      user: &User,  // Borrowed reference
      pool: &PgPool // Shared reference
  ) -> Result {
      // Ownership managed through async/.await
  }
  ```
  Rust's ownership system prevents data races in concurrent DB access.

---

<!-- 
- **Variables and Types**: Rust uses `let` to declare variables, which are immutable by default. Use `mut` for mutability. Types are inferred or can be specified explicitly.
- **Functions**: Declared with `fn`. Example:  
  ```rust
  fn main() {
      println!("Hello, World!");
  }
  ```
- **Structs and Traits**: Used for modeling data and behavior. For example, our `User` and `ShortenedUrl` structs in `structs.rs` model users and URLs, respectively.
- **Error Handling**: Rust uses the `Result` and `Option` types for error handling, ensuring errors are handled explicitly.
- **Async/Await**: Our endpoints are asynchronous for scalability, using `async fn` and `.await` with Actix-web. -->

---

# **Part 3: Program Walkthrough, Features, and Demo**

## **<u>Joseph:</u>**

#### **Project File Structure**
Here's a quick look at our codebase structure:

- `main.rs`: Application entry point and server setup
- `constants.rs`: Configuration constants
- `middleware.rs`: Middleware for JWT authentication
- `service.rs`: Business logic (URL creation, deletion, etc.)
- `structs.rs`: Data models (User, URL, API responses)
- `utils.rs`: Utility functions (database initialization, environment checks)
- `routes/`: API endpoints (auth, register, shorten)

#### **Program Overview and Code Understanding**

- **Startup**: The app initializes environment variables and database tables on startup (`main.rs`, `utils.rs`).
- **Registration/Login**: Users register with a username and password, which is hashed using bcrypt. Login verifies credentials and issues a JWT token (`register.rs`, `auth.rs`).
- **JWT Middleware**: Each request to protected endpoints is checked for a valid JWT, extracting the username and ensuring only authenticated users can access or modify their own URLs (`middleware.rs`).
- **Shorten/Delete URL**: Authenticated users can create short URLs, optionally specifying a custom path and expiration. The service ensures uniqueness and handles both creation and updating (`shorten.rs`, `service.rs`).
- **Data Models**: The `User` and `ShortenedUrl` structs define our database schema and API responses (`structs.rs`).

#### **Interesting Rust Features in Our Program**

- **Actix-Web Framework**: Provides a fast, asynchronous web server with middleware support.
- **Ownership and Lifetimes**: Ensures safe sharing of database connections and user data across threads.
- **Type Safety**: All API responses and data models are strongly typed, reducing runtime errors.
- **Error Handling**: Uses Rust’s `Result` type to handle errors gracefully at every step.

#### **Demo: Evidence It Works**

- **Register a User**: Show a registration request and response.
- **Login**: Demonstrate login and JWT token retrieval.
- **Shorten a URL**: Use the token to create a shortened URL.
- **Delete a URL**: Show deletion of a URL and error handling if unauthorized.

**(Screen share or pre-recorded demo showing these API calls and responses, highlighting relevant code sections as each feature is demonstrated.)**

#### **Conclusion**

Rust enabled us to build a robust, safe, and high-performance backend for our URL shortener. Its unique memory model, strong typing, and concurrency support made it a great fit for this project. Thanks for watching!