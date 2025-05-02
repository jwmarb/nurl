## Rust Video Presentation Script

---

### **Part 1: Introduction, Team, and Why Rust**

**Speaker 1 (0:00–6:40)**

Hello everyone, and welcome to our presentation on our Rust-based URL shortener project, nurl. I'm [Name], and joining me are [Name] and [Name]. Each of us will be presenting for about five minutes, covering different aspects of our project and the Rust language.

#### **Why We Chose Rust and This Project**

We chose Rust for this project because it's well known for its safety, speed, and modern approach to systems programming. Rust's memory safety guarantees, lack of a garbage collector, and excellent concurrency support make it ideal for backend services where reliability and performance are critical[19][21]. Our application-a URL shortener with authentication-benefits from these features, especially since it handles user data and needs to be robust against bugs and security vulnerabilities.

#### **Brief Overview of the Application**

Our application allows users to register, log in, and create or delete shortened URLs. Each user has their own set of URLs, and all operations are protected by JWT-based authentication. The backend is built with the Actix-web framework and uses PostgreSQL for data persistence.

#### **Project File Structure**

Here's a quick look at our codebase structure:

- `main.rs`: Application entry point and server setup
- `constants.rs`: Configuration constants
- `middleware.rs`: Middleware for JWT authentication
- `service.rs`: Business logic (URL creation, deletion, etc.)
- `structs.rs`: Data models (User, URL, API responses)
- `utils.rs`: Utility functions (database initialization, environment checks)
- `routes/`: API endpoints (auth, register, shorten)

#### **Why Rust Over Other Languages**

Compared to other languages we've used (Python, Java, HTML/CSS, Prolog, Standard ML, Chapel, C, MIPS assembly):

- **Rust vs. Python**: Rust is compiled and much faster, with strict compile-time checks for memory safety. Python is easier to write but slower and less safe for concurrent, memory-sensitive tasks[13][18][20][22].
- **Rust vs. Java**: Rust offers better performance and memory safety by eliminating garbage collection. Java is easier for rapid development but can suffer from garbage collection pauses and less predictable performance[14].
- **Rust vs. C**: Rust provides similar low-level control but with safer abstractions, preventing common bugs like buffer overflows and dangling pointers[21].
- **Rust vs. HTML/CSS**: HTML/CSS is for web content structure and presentation, not backend logic or memory management[15].

---

### **Part 2: Rust Language Tutorial and Paradigms**

**Speaker 2 (6:40–13:20)**

#### **Brief Tutorial: Rust Basics Used in Our Project**

- **Variables and Types**: Rust uses `let` to declare variables, which are immutable by default. Use `mut` for mutability. Types are inferred or can be specified explicitly[11].
- **Functions**: Declared with `fn`. Example:  
  ```rust
  fn main() {
      println!("Hello, World!");
  }
  ```
- **Structs and Traits**: Used for modeling data and behavior. For example, our `User` and `ShortenedUrl` structs in `structs.rs` model users and URLs, respectively[9].
- **Error Handling**: Rust uses the `Result` and `Option` types for error handling, ensuring errors are handled explicitly.
- **Async/Await**: Our endpoints are asynchronous for scalability, using `async fn` and `.await` with Actix-web.

#### **Overview of Rust: History and Usage**

- **History**: Rust was started by Graydon Hoare in 2006 as a side project at Mozilla, aiming to create a safer alternative to C/C++ for systems programming. Mozilla officially sponsored it in 2009, and version 1.0 was released in 2015[11][19].
- **Usage**: Rust is used for performance-critical backend systems, operating systems, embedded devices, and increasingly in web backend services[17][19].
- **Community**: Rust has a growing, vibrant community with strong documentation and support[19].

#### **Paradigms and Features**

- **Multi-Paradigm**: Rust supports both object-oriented and functional programming paradigms, allowing flexibility in code organization and composition[12].
- **Ownership and Borrowing**: Rust’s unique ownership system enforces memory safety at compile time, preventing data races and many classes of bugs without a garbage collector[21].
- **Type System**: Rust is statically typed, with powerful generics and type inference, making code both safe and expressive.

#### **Comparison with Other Languages (Terminology)**

| Feature           | Rust                  | Python             | Java               | C                   |
|-------------------|----------------------|--------------------|--------------------|---------------------|
| Memory Management | Ownership/Borrowing  | Garbage Collector  | Garbage Collector  | Manual              |
| Concurrency       | Fearless, safe       | GIL limits         | Threads, less safe | Manual, error-prone |
| Typing            | Static, inferred     | Dynamic            | Static             | Static              |
| Safety            | Compile-time checks  | Runtime errors     | Runtime errors     | Unsafe by default   |
| Performance       | Near C/C++           | Slower             | Good, but overhead | Fast                |

---

### **Part 3: Program Walkthrough, Features, and Demo**

**Speaker 3 (13:20–20:00)**

#### **Program Overview and Code Understanding**

- **Startup**: The app initializes environment variables and database tables on startup (`main.rs`, `utils.rs`)[6][10].
- **Registration/Login**: Users register with a username and password, which is hashed using bcrypt. Login verifies credentials and issues a JWT token (`register.rs`, `auth.rs`)[1][3].
- **JWT Middleware**: Each request to protected endpoints is checked for a valid JWT, extracting the username and ensuring only authenticated users can access or modify their own URLs (`middleware.rs`)[7].
- **Shorten/Delete URL**: Authenticated users can create short URLs, optionally specifying a custom path and expiration. The service ensures uniqueness and handles both creation and updating (`shorten.rs`, `service.rs`)[4][8].
- **Data Models**: The `User` and `ShortenedUrl` structs define our database schema and API responses (`structs.rs`)[9].

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

Rust enabled us to build a robust, safe, and high-performance backend for our URL shortener. Its unique memory model, strong typing, and concurrency support made it a great fit for this project. Thank you for watching-we’re happy to answer any questions!

---

**[End of Script]**

*Each speaker should cover their section in about 6 minutes and 40 seconds to ensure equal participation and stay within the 20-minute limit.*