# Authenticated Todo API in Rust and Axum

This project demonstrate the use of Axum written in rust to develop a rest application for a todo application with email and password authentication.

### Routes

| Routes             | Methods |                 Response |
|--------------------|:-------:|-------------------------:|
| /api/status        |   GET   |                  message |
| /api/auth/login    |  POST   |               jwt-tokens |
| /api/auth/register |  POST   |                     user |
| /api/user          |   GET   |                    users |
| /api/user/:id      |   GET   |                     user |
| /api/user/:id      |  PATCH  |                     user |
| /api/user/:id      | DELETE  |             deleted-user |
| /api/todo          |   GET   |            todos-by-user |
| /api/todo/:todo_id |   GET   |             todo-by-user |
| /api/todo          |  POST   |                     todo |
| /api/todo/:todo_id |  PATCH  |             todo-by-user |
| /api/todo/:todo_id |   PUT   | toggled-todo (completed) |
| /api/todo/:todo_id | DELETE  |             deleted-todo |


## Run
The following are steps to run the code base
- Step 1: Install seo-orm-cli
	```bash
    cargo install sea-orm-cli
    ```
- Step 2: Run the migration using sea-orm
	```bash
    sea-orm-cli migrate up
    ```
- Step 3: Run the rust code
    ```bash
    cargo run
    ```
    or run with watch
	- Install cargo-watch
	    ```bash
	    cargo install cargo-watch
	    ```
	- Now run with watch flag
		```bash
		cargo watch -x run --delay 2 -C src
	    ```

## Config
The following has to be add to env or be added by using a .env file at the root of the program
```env
PORT=3000# Port API should listen on
RUST_LOG=authenticated_todo# flag to display logs
DATABASE_URL=postgres://user:password@localhost:5432/authenticated_todo# Postgres database url
JWT_SECRET=secret# jwt secret
```


# Thanks