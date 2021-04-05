# microservice-integration-test (WIP)
The idea behind this repository is simply to make a few tests with microservices that use Restful communication between them, solve a few docker, docker-compose and kubernetes challenges and also explore some Go and Rust libraries such as Actix, Warp, Tokio and Serde-json.

There is no unit or integration testing, there is no security implemented, even passwords are plain text and just copy pasted in a few places, code is not well formatted or documented... these things were not the focus of this test.

In many places there are more optimal ways to do things, but again, this was oriented towards learning and having fun.

## Microservices
### rust-web-app
Microservice implemented in Rust that makes use of Actix as a http server to listen to requests and redirects them towards go-web-app or towards rust-sql-db
### rust-sql-db
Microservice implemented in Rust that makes use of Warp as a http server to listen to requests. These requests are related to the Postgresql database that is running inside this microservice which allows it to get and post new notes. The database is initialized through Diesel.
### go-web-app
Microservice implemente in Golang that currently uses the standard tcplistener library to listen to requests that are handled internally or redirected towards rust-web-app. In the future this microservice should also connect with a mongoDB microservice.
### go-nosql-db
Not implemented yet.

