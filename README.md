export postgresql://username:password@host:port/dbname
diesel setup
ROCKET_PORT=$PORT ROCKET_DATABASES={myDb={url="postgres://administrator:pass@localhost/todo"}} cargo run
ROCKET_PROFILE=release cargo run
ROCKET_PROFILE=release ROCKET_DATABASES={myDb={url="postgres://administrator:pass@localhost/todo"}} cargo run
