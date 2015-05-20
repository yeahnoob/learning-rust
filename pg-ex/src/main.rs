extern crate postgres;

use postgres::{Connection, SslMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    let conn = Connection::connect("postgres://bloguser:xxxxxx@localhost/blogdb", &SslMode::None)
            .unwrap();
/*
    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
*/
    let me = Person {
        id: 1,
        name: "Tom Hacks".to_string(),
        data: None
    };
    conn.execute("INSERT INTO person (id, name, data) VALUES ($1, $2, $3)",
                 &[&me.id, &me.name, &me.data]).unwrap();

    let stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
    for row in stmt.query(&[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        println!("Found person {}", person.name);
    }
}
