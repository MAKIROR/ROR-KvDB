# ROR-KvDB
A simple Key–Value Database.(v0.1.0)    
This version does not yet provide support for remote.

## Tutorial Blog (Chinese Only) :    
<a href="https://github.com/MAKIROR/Makiror_Articles/blob/main/articles_zh_cn/Rust/Make_a_simple_KV_database_with_Rust.md" target="_blank">Part 1: Basic Storage Functions</a></br>
Part 2: User System and Network Services (unfinished)

## Start
start command:
```
rdb [optional:data file path]
```
## Commands
Start database and you can use these commands:

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
