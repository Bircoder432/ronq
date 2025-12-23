**ronq**

ronq is a CLI utility for working with RON (Rusty Object Notation) files, similar to jq for JSON.

**Features:**

* Extract values using keys and indices, e.g., `users[0].id`
* Supports reading from stdin or from a file
* Colorful and readable output
* Navigate nested RON structures easily

**Installation:**

```
cargo install ronq
```

**Usage:**

Reading a RON file:

```
rq -f example.ron
```

Accessing a key:

```
rq users -f example.ron
```

Accessing nested keys and list elements:

```
rq users[0].id -f example.ron
```

Reading from stdin:

```
cat example.ron | rq users[0].username
```

**Arguments:**

* `-f, --file <FILE>` — path to the RON file (defaults to stdin)
* `<key>` — optional path to a key or element. Use `[index]` for arrays and `.` for nested fields

**Example RON file:**

```
(
    users: [
        (
            id: 1,
            username: "alice",
            score: 123.45
        ),
        (
            id: 2,
            username: "bob",
            score: 67.89
        )
    ]
)
```

**Example commands:**

```
rq users -f example.ron
rq users[0].username -f example.ron
rq users[1].score -f example.ron
```
