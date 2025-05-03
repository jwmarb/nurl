# nurl - URL Shortener

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**nurl** is a URL shortener service built with Rust (Backend) and React (Frontend). It was developed as a final project for a Computer Science class at the University of Arizona in under 24 hours. It provides a simple way to shorten URLs and track their usage.

> **Note:** nurl is **not production-ready**. Authentication needs further development, as currently, anyone can register and create unlimited shortened URLs. This project is intended for educational purposes only.

## Live Demo

You can view the live demo at [here](https://nurl.jwmarb.com/)

## Project Structure

```
.
├── .dockerignore
├── Dockerfile
├── README.md
├── docker-compose.dev.yaml
├── docker-compose.prod.yaml
├── docker-compose.testing.yaml
├── frontend/             # React Frontend
│   ├── ... (Vite, React, TypeScript files)
│   └── src/
│       └── ...
├── backend/              # Rust Backend (Actix Web)
│   ├── Cargo.toml
│   └── src/
│       └── ...
└── script.md
```

## Getting Started

Follow these instructions to get a local copy up and running for development and testing.

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later recommended) with [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [Docker](https://docs.docker.com/get-docker/) & [Docker Compose](https://docs.docker.com/compose/install/) (for containerized deployment)

### Development

**1. Frontend:**

Navigate to the frontend directory:

```sh
cd frontend
```

Install dependencies:

```sh
pnpm install
```

Run the development server:

```sh
pnpm dev
```

The frontend will typically be available at `http://localhost:5173`.

**2. Backend:**

Navigate to the backend directory:

```sh
cd backend
```

Run the development server:

```sh
cargo run
```

The backend API will typically be available at `http://localhost:8080`. Rust's package manager, Cargo, will automatically download and compile necessary dependencies.

**3. Database:**

Ensure your PostgreSQL database is running. You can use the existing `docker-compose.dev.yaml` file to set up a local database:

```sh
docker compose -f docker-compose.dev.yaml up
```

### Running the Application

Ensure that all three services (frontend, backend, database) are up. To access the application, simply go to `http://localhost:5173` in your web browser. Do not go to `http://localhost:8080` directly, as the frontend is responsible for routing and serving the application.

## Deployment

You can deploy `nurl` using Docker.

### Build the Docker images:

Use the production compose file to build the images for both frontend and backend.

```sh
docker compose -f docker-compose.prod.yaml build
```

### Run the Docker containers:

Start the services defined in the production compose file.

```sh
docker compose -f docker-compose.prod.yaml up
```

This command will start both the frontend and backend services in detached mode (add `-d` for background execution). The application should be accessible via the port configured in your `docker-compose.prod.yaml` (often port 80 or another specified port for the web server/proxy).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details (Optional: Create a LICENSE.md file).
