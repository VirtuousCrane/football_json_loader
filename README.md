# football_json_loader
The football_json_loader cmd tool helps you load a the [football.json](https://github.com/openfootball/football.json) dataset into an sqlite database.

## What this program does
- Clones the football.json repository into /tmp
- Read through all the files in the football.json repository, parses all the json files, and save them into an SQLite dataset

## Table of Contents
- [To Install](#to-install)
- [To Use](#to-use)

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

# TODO: More Documentation
