# Rust WebAPI CRUD Example (Axum, PostgreSQL, JWT auth)

### Installation
1. Clone repo.
2. Copy .env.example to .env and set ur values.
   - windows
      ```sh
      copy .env.example .env
      ```
   - unix
      ```sh
      cp .env.example .env
      ```
3. Build and install dependecies:
   ```sh
   cargo build
   ```
4. Run:
   ```sh
   cargo run
   ```
5. Swagger route is /swagger-ui

### Functionalities
- registration/authorization
- todo CRUD

### Technologies/liblaries
- rust axum,
- postgres
- sqlx
- utoipa swagger
- validator
