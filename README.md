# Libraries
## csv
It provides the interface to parse the body of POST
## serde & serde_json
Serialize & Deserialize the define structure
## tokio
Provides the asynchronous runtime. It allows the server to handle multiple clients concurrently.
## warp
a composable web server framework. It is built upon hyper. 
## mobc & mobc-postgres & tokio-postgres
**mobc**: probide interface to create a pool of connections to a database. Avoid opening a new database connection every time one is needed.  
**tokio-postgres**: an asynchronous, pipelined, PostgreSQL client  
**mob-postgres**: provide interface to use mobc with tokio-postgres  

# Server
The server expose a POST API on port 3030 under path /upload and accepts the CSV file as the body. 
## API(POST header)
Here is acceptable POST  
```
POST /upload HTTP/1.1  
Host: localhost:3030
Content-Type: application/x-www-form-urlencoded
Cache-Control: no-cache
Postman-Token: 88d8f9db-9583-7846-0349-f197752c24a8
<body>
```
## Processing
The server will extract the body of POST as csv file and parse out each row of it. The parsed data will be saved to a postgres database and serialized into a list of json objects. The response to the request will be a list of json objects parsed from the body of request.  
**Error Handling**  
1. If the data parsed from the body exists in the database(the primary key exists), a error message will send to user to indicate that something. The message contains the reason of failure.
2. If the data can't be parsed by the parser, we will notify the user with error

# Test Environment
## REST API Testing Tool
[Postman(Chrome Extension)](https://chrome.google.com/webstore/detail/postman/fhbjgbiflinjbdggehcddcbncdddomop?hl=en)
It is used to build POST request from csv file.
## Database
running the PostgresSQL with docker. It pulls the docker image [postgres](https://hub.docker.com/_/postgres?tab=tags) and expost the database on port 5050
```bash
docker run --name postgresql -e POSTGRES_USER=siliang -e POSTGRES_PASSWORD=000000 -p 5050:5432 -d postgres:latest
```
## run the server
```
cargo run --bin csv_parse_server
```
## Test API with Postman
There are some csv files under test_csv directory. Use Postman to create POST request with those csv to test the server.

## Test with client_for_test(WIP)
There is a http client program that could be used to test the server. Right now, it could send the POST request to server and parse out the response and print it to std output. So we need to manual check the response. In future, we could add assert on the response to check its correctness. Then the tests will be automatic.

# TODO
1. improve the log of server with log crate and env_logger crate
2. support partial success. We could continue parse the csv if a error happens. It requires more understanding of the client.(They may not want that)
3. improve error handling with clear message and better code
4. finish client_for_test. Make the test more automatic
