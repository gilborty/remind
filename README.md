# remind
Program to append timestamped diary messages to a file

# bash/zsh

```
remind() {
  local log_string="$*"
  <path to exe>/remind "<path to log>/log.md" "$log_string"
}
```
