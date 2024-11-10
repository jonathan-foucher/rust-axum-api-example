## Introduction
This project is an example of a Rust API using Axum, Diesel and a postgres database.

## Run the project
### Database
Install postgres locally or run it through docker with :
```
docker run -p 5432:5432 -e POSTGRES_DB=my_database -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres postgres
```

You will need to run the migration with diesel to create the movie table :
```
diesel migration run
```

### Application
Once the postgres database is started and the movie table created, you can build and start the Rust project with Cargo and try it out.

Start the application
```
cargo build && cargo run --config config.toml
```

Get all the movies
```
curl --location 'http://localhost:3000/api/movies'
```

Save a movie
```
curl --request POST \
  --url http://localhost:3000/api/movies \
  --header 'Content-Type: application/json' \
  --data '{
	"id": 26,
	"title": "Some movie title",
	"release_date": "2022-02-26"
}'
```

Delete a movie
```
curl --request DELETE \
  --url http://localhost:3000/api/movies/26
```
