# ROR-KvDB
A simple Key–Value Database.   

## Tutorial Blog (Chinese Only) :    
<a href="https://github.com/MAKIROR/Makiror_Articles/blob/main/articles_zh_cn/Rust/Make_a_simple_KV_database_with_Rust.md" target="_blank">Part 1: Basic Storage Functions</a></br>
Part 2: User System and Network Services (writing)

## Local mode
Enter REPL(Read-Eval-Print Loop) mode and perform database operations locally.

start command:
```
rdb local -p [optional:data file path]
```
### Commands
Start database and you can use these commands:

```
open [data file path]
add [key] [value] [optional: type of data]
delete [key]
get [key]
compact
user create [username] [password] [level]
quit
```
<br>


## Server mode
Start a remote kv database that can accept client connections.(No REPL)

### Configuration file
The path to the configuration file is: ./config/server.toml
```
# server name
name = "Default server"

# ip (Both ipv4 and ipv6 are allowed)
ip = "127.0.0.1"

# port
port = "11451"

# Allow customization, the data type is i64, it will affect the result of "generating user uid according to configuration"
worker_id = 0

# Allow customization, the data type is i64, it will affect the result of "generating user uid according to configuration"
data_center_id = 0

# The directory for storing data files, all data files accessed by clients must be in this
data_path = "./data/"

# If the client is inactive for a certain period of time, it will automatically disconnect (Sec)
timeout = 300
```
!: worker_id and data_center_id in local mode, it will not be affected by the configuration file, it will be 0.

<br>

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

### Commands 
The user's level determines which commands can be used.
```
get [key] (all)
add [key] [value] [optional: type of data] (level 2-4)
delete [key] (level 3-4)
compact (level 2-4)
user create [username] [password] [level] (level 4)
quit (all)
```

  But in this mode, only the result of the operation will be displayed after the operation, and there will be no detailed output like the local mode.

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
### Open
```
open [data file path]
```
Switch to another database, if the path is valid but the file does not exist, it will be create automatically.

### Add
```
add [key] [value] [optional: type of data]
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
| Char | Vec\<char> | char |

If you don't specify a type, whatever it is will be treated as a String type.

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

This form just a concept, the actual Uncompacted is not expressed in this form. When a data file, Uncompacted data size exceeds 1KB, the database will automatically perform compact.    
After the above table is compacted, it will become like this:
| DataFile | Uncompacted  |
| :----: | :----: |
| add("name","Aaron") | false |

Look! Invalid data is cleaned up!

### Quit
```
quit
```