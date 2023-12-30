# count-hours
CLI tool to help manage hours worked on a project.

## Usage
Modes:
- log
  -> start/end + file
- report
  -> total + file + hourly wage
  -> start_hours + file      (print in XX:XX format, one time per line, print blank or 00:00 for days not work so that it's easy to copy/paste into an excel document)
  -> end_hours + file

(For start and end hours, handle multiple entries in a day by "merging" them)

### Report mode
```console
cargo run report total temp.csv -t 1 -w 1
```


## CSV format
KISS

```code
<start time 1>,<end time 1>
<start time 2>,<end time 2>
```

For example
```code
2023-12-29 20:30:00,2023-12-29 23:30:00
2023-12-30 19:00:00,2023-12-30 22:00:00
```


## TODO:
- Use [time](https://docs.rs/time/latest/time/struct.Instant.html#method.now) instead of chrono ?
