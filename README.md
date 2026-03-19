# unidb-schema

## A note on the database schema

For simplicity, threads in a text channel with threads enabled (e.g. `#introductions`) and forum posts in a forum (e.g. `#server-projects`) are both handled as-, and called the same: "threads"

## About `Array<Nullable<Text>>` and `Vec<Option<String>>`

This is serialized like this because postgres cannot guarantee that any element in the array is not NULL or None in rust terms. I have written a helper `TextVec`, that guarantees that all values in it are some. Please use this newtype in the rust data model structs when contributing to the schema. It can be coverted to a `Vec<String>` by doing `.deref()` or `*my_text_vector` or `my_string_vector: Vec<String> = my_text_vector.into()`.

## Using the schema and data models in Rust

To add the schema and rust models as a dependency in a rust project:

```toml
unidb = { git = "https://github.com/Vimothy-s-Vestibule/unidb-schema" }
```

Other useful things to work with the db:

```toml

diesel = { version = "2.3.6", features = ["postgres", "chrono"] }
diesel-async = { version = "0.7.4", features = ["postgres", "deadpool"] }

# If you want to insert vectors use this to serialize them like:
# pgvector::Vector::from(Vec::<f32>::new())
pgvector = { version = "0.4.1", features = ["diesel"] }
# For reading env vars from .env file
dotenvy = "0.15.7"


```

Using it with tokio, diesel, diesel-async:

```rust
use unidb::diesel_schema::vestibule_users;
use unidb::models::{DiscordMessage, VestibuleUserRecord};

// ...
let database_url = env::var("DATABASE_URL").map_err(|e| AppError::AppError(Box::new(e)))?;



let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
let pool = Pool::builder(config).build().unwrap();

 let mut conn = pool.get().await.unwrap();

let all_users: Vec<(VestibuleUserRecord, DiscordMessage)> = vestibule_users::table
            .inner_join(unidb::diesel_schema::messages::table)
            .select((
                VestibuleUserRecord::as_select(),
                DiscordMessage::as_select(),
            ))
            .load(&mut conn)
            .await
            .unwrap_or_else(|e| {
                eprintln!("Failed to fetch initial users: {}", e);
                vec![]
            });

// ...
```
