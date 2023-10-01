# ROR-KvDB
A simple Key–Value Database.   
It is characterized by the realization of high-speed IO of a large amount of data based on the theory of Bitcask. \
Here is the develop branch.    
[release version branch](https://github.com/MAKIROR/ROR-KvDB/tree/release)

## Tutorial Blog (Chinese Only) :    
<a href="https://github.com/MAKIROR/Makiror_Articles/blob/main/articles_zh_cn/Rust/Make_a_simple_KV_database_with_Rust/Part1.md" target="_blank">Part 1: Basic Storage Functions</a></br>
<a href="https://github.com/MAKIROR/Makiror_Articles/blob/main/articles_zh_cn/Rust/Make_a_simple_KV_database_with_Rust/Part2-3.md" target="_blank">Paet 2-3: User System and Network Services</a></br>

## Library
Library for ROR Key-Value Database.(Rust)</br>
<a href="https://github.com/MAKIROR/Makiror_Articles/blob/main/articles_zh_cn/Rust/Make_a_simple_KV_database_with_Rust/Part2-3.md" target="_blank">https://github.com/MAKIROR/ROR-KvDB-Lib</a></br>

Library for developers, including data storage and remoting functions.</br>

## Local mode
Enter REPL(Read-Eval-Print Loop) mode and perform database operations locally.

start command:
```
rdb local -p [optional:data file path]
```
### Supported commands
Start database and you can use database with commands.

#### Commands for Database

```
open [data file path]
add [optional: type of data] [key] [value]
delete [key]
get [key]
list [values/entries]
typeof [key]
compact
quit
```

#### Commands for User

```
user create [username] [password] [level]
    | delete [username]
```

<br>

## Server mode
Start a remote kv database that can accept client connections.

### Configuration file
The path to the configuration file is: ./config/server.toml
```
# server name
name = "Default server"

# ip (Both ipv4 and ipv6 are allowed)
ip = "127.0.0.1"

# port
port = "11451"

# The directory for storing data files, all data files accessed by clients must be in this
data_path = "./data/"

# If the client is inactive for a certain period of time, it will automatically disconnect (Sec)
timeout = 300

# Enter a REPL-mode terminal at server startup
repl = true

# If repl is true, as the user and data file for the connection
local_user = "root@123456"
default_db = "default.data"

# Perform a refresh operation when receiving n client connections to release idle data files and invalid connections.This operation will not be performed when it is 0.
auto_fresh = 20
```

<br>

Initialize a server program through this command (generate configuration file):
```
rdb server init
```

## Client mode
Connect to a remote server and start the REPL.
### Connect
```
rdb connect -i [ip] -p [port] -u [user info] -f [data file]
```
#### example:
```
rdb connect -i 127.0.0.1 -p 11451 -u makiror@123456 -f test.data
```
When you start connect without parameters, it will ask you to enter these after the program starts.
<br>

### Supported commands
The user's level determines which commands can be used.


```
get [key] (all)
typeof [key] (all)
add [optional: type of data] [key] [value] (level 2-4)
delete [key] (level 3-4)
compact (level 2-4)
quit (all)
```

#### Commands for User

```
user create [username] [password] [level] (level 4)
    | delete [username] (level 4)
```

But in this mode, only the result of the operation will be displayed after the operation, and there will be no detailed output like the local mode.

> Since the data file corresponding to a client is allocated when the connection is established, it will be troublesome to redirect the data file to support the open command, so this version does not support command 'open'.

The path of the data file is [server preset path + parameter]. If the parameter has no folder but only the file name, the file will be create automatically.

#### example
The server will automatically create this file:
```
rdb connect -i 127.0.0.1 -p 11451 -u makiror@123456 -f test.data
```
but this will not:
```
rdb connect -i 127.0.0.1 -p 11451 -u makiror@123456 -f test/test.data
```

<br>

## Commands
Detailed Explanation of Database Commands.

### Disambiguation

If you need to insert content that duplicates type keywords like "int" or "string" in places such as keys/values, It is necessary to use quotes in the command for disambiguation. for example:

```
add int "user" 114514 //'user' is key
or
add string "string" hey
```

If the sentence contains spaces, it's also necessary to use quotes to represent it as a whole.

```
add string "string" "THIS IS A STRING"
```

In this program's syntax convention, parentheses represent a sub-expression, while quotes are used to denote the entirety of a literal when dealing with spaces. Therefore, using parentheses to represent a complete literal, like this, is not allowed:

```
add string "string" (THIS IS A STRING)
```

<br>

### Open
```
open [data file path]
```
Switch to another database, if the path is valid but the file does not exist, it will be create automatically.

### Add
```
add [optional: type of data] [key] [value]
```
These data types are currently supported, and you can express it with these names:
| Type | Data type in Rust | Express |
| :----: | :----: | :----: |
| Null | Null | null |
| Bool | bool | bool |
| Int32 | i32 | int / i32 |
| Int64 | i64 | long / i64 |
| Float32 | f32 | float / f32 |
| Float64 | f64 | double / f64 |
| String | String | string |
| Char | char | char |
| Array | Vec\<DataType> | char |

If you don't specify a type, whatever it is will be treated as a String type.

* In the current version, ROR Database already supports the recursive type Array, which can be expressed like this on the command line:

```
add array(string) group [makiror,aaron,adonis]
```

Use parentheses for included datatype, so you can also define recursive types like this:
```
add array(array(string)) group [[little cat, cute], [sweet potato, lovely], [brown dog, handsome]]
```
```
test > get group
Array([Array([String("cat"), String("cute")]), Array([String("potato"), String("lovely")]), Array([String("dog"), String("handsome")])])
```

#### Example:
```
test.data > add name makiror
Successfully added data of type String 'name' : 'makiror'
test.data > add age 14 int
Successfully added data of type Int32 'age' : '14'
```
If the same key is added successively, the former will be overwritten by the latter.

### Delete
```
delete [key]
```
Delete a data entry. If the key does not exist, it will return error "KeyNotFound".

### Get
```
get [key]
```
Query a value from the database. If the key does not exist, it will return error "KeyNotFound".     
It will be converted to string type when outputting, but in the corresponding function of kv.rs, it will return a corresponding type.

#### Example:
```
test.data > add name makiror
Successfully added data of type String 'name' : 'makiror'
test.data > get name
makiror
```

### typeof
```
typeof [key]
```
Print the type of Key-Value data.

| Express | Output |
| :----: | :----: |
| null | Null |
| bool | Bool |
| int / i32 | Int |
| long / i64 | Long |
| float / f32 | Float |
| double / i64 | Double |
| string | String |
| char | Char |
| array | Array |

### list
```
list [values/entries]
```
Print all values/entries in data file.    
This command is only allowed in local mode.    
It just gets all the data and returns it. This operation will consume more memory when the amount of data is large, so I don't recommend using this command.


### Compact
```
compact
```
When you use two 'add' successively to modify the data corresponding to the same key, the overwritten  'add' will not be deleted, but as 'uncompacted' data. The same goes for the delete command.

| DataFile | Uncompacted  |
| :----: | :----: |
| add("name","Makiror") | true |
| add("name","Aaron") | false |
| add("age", 17) | true |
| delete("age") | true |

This form just a concept, the actual Uncompacted is not expressed in this form. When a data file uncompacted data size exceeds 1KB, the database will automatically perform compact.    
After the above table is compacted, it will become like this:
| DataFile | Uncompacted  |
| :----: | :----: |
| add("name","Aaron") | false |

Invalid data is cleaned up.

### Quit
```
quit
```

###  User
Register or delete user
```
user create [username] [password] [level]
    | delete [username]
```

### Update plan
+ Basic syntax analysis support (done)
+ recursive datatype (done)
+ Basic expression support
+ Conditional expression query
