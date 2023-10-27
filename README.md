# football_json_loader
The football_json_loader cmd tool helps you load a the [football.json](https://github.com/openfootball/football.json) dataset into an sqlite database.

## What this program does
- Clones the football.json repository into /tmp
- Read through all the files in the football.json repository, parses all the json files, and save them into an SQLite dataset

## Table of Contents
- [To Install](#to-install)
- [To Use](#to-use)
- [Data Processing](#data-processing)
- [Database Schema](#database-schema)

# To Install
1) Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2) Install this program
```
cargo install --git https://github.com/VirtuousCrane/football_json_loader.git
```

# To Use
```
Usage:
  football_json_loader [OPTIONS]


Optional arguments:
  -h,--help             Show this help message and exit
  -v,--verbose          Whether or not to show all logs
  -w,--warnings         Whether or not to show logs (Warnings only)
  -f,--file_loc FILE_LOC
                        Where to save the SQLite Database
```

## Example
### Generate a football_json.sqlite file
```
football_json_loader
```

### Generate a football_json.sqlite file + show warnings
```
football_json_loader --warnings
```

### Generate a football_json.sqlite file + show all logs
```
football_json_loader --verbose
```

### Generate a file at a specific location
```
football_json_loader --file_loc ~/football_json.sqlite
```

# Nerdy Stuff
## Data Processing
Despite what was shown on football.json project's homepage, the data format is very inconsistent. Therefore, I have defined several structs and enums to parse the JSON properly.

Currently, the program still can't process some rows of data because the club names weren't present in the corresponding .club files.

## Database Schema
All SQL code used to create the databases can be viewed in db.rs

# TODO: More Documentation
