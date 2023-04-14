# Run the following commands-

```
docker-compose up -d     // Starts the mysql container
```

```
 cargo build --target=wasm32-wasi    // Compiles code to wasm
```

```
 wasmedge target/wasm32-wasi/debug/test-sql.wasm    // Executes the wasm file to connect with sql database 
```

### Behaviour-
An error should appear on your screen saying it can't connect to database. 


