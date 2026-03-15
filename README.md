To add the schema and rust models as a dependency in a rust project:
```toml
syl-scr-common = { git = "https://github.com/Vimothy-s-Vestibule/unidb-schema" }
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
use syl_scr_common::diesel_schema::vestibule_users;
use syl_scr_common::models::{DiscordMessage, RecordStatus, VestibuleUserRecord};

// ...
let database_url = env::var("DATABASE_URL").map_err(|e| AppError::AppError(Box::new(e)))?;



let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
let pool = Pool::builder(config).build().unwrap();

 let mut conn = pool.get().await.unwrap();

let all_users: Vec<(VestibuleUserRecord, DiscordMessage)> = vestibule_users::table
            .inner_join(syl_scr_common::diesel_schema::messages::table)
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

## About `Array<Nullable<Text>>` and `Vec<Option<String>>`
This is serialized like this because postgres cannot guarantee that any element in the array is not NULL or None in rust terms. I'll get right on writing a helper to convert this into a Vec<String> because we're enforcing that all Strings are Some in a Vec in the app logic and the rust compiler is doing that too.
