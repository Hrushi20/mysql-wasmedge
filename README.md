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


# Run the below commands to enable fast authentication
```
docker ps 
```

```
docker exec -it {container_id} /bin/bash
```

```
mysql -u user -p  // Then enter the password after this line
```

```
exit // exit for sql and docker container
```

Run the wasmfile again-

```
 wasmedge target/wasm32-wasi/debug/test-sql.wasm    // Executes the wasm file to connect with sql database 
```

### Behaviour- 
We can now connect to the mysql database successfully

### Note- 
However, if we change the username to root, we can't use the above process and hence fails to connect to the database.
 


