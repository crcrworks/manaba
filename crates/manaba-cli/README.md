## Installation

```
cargo install manaba-cli
```

## Usage

```sh
 manaba check        # List assignment include reports and exams
 manaba report       # List reports
 manaba exam         # List exams
 manaba timetable    # Show timetable
 manaba browse       # Open manaba page in browser
 manaba config-path  # show manaba-cli config path
```

### Options

`--all` flag shows all contents.

```
manaba report --all
```

`--warn` flag shows assignments with approaching deadlines.

```
manaba report --warn
```

## Configuration

Run this command to show config file path:

```
manaba config-path
```

default value:

```toml
base_url = "https://ct.ritsumei.ac.jp/ct/"
cookie_domain = "ct.ritsumei.ac.jp"

[timetable]
# "1限" = "9:00 ~ 10:35"
# "2限" = "10:45 ~ 12:20"
# "3限" = "13:10 ~ 14:45"
# "4限" = "14:55 ~ 16:30"
# "5限" = "16:40 ~ 18:15"
# "6限" = "18:25 ~ 20:00"
# "7限" = "20:10 ~ 21:45"

[color]
white = "D3C6AA",
black = "272E33",
red = "E67E80",
blue = "7FBBB3",
aqua = "83C092",
yellow = "DBBC7F",
green = "A7C080",
gray = "4F5B58",
```
