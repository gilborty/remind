# remind
Program to append timestamped diary messages to a file.

I get overwhelmed by other note taking apps, so this attempts to solve that. It simply takes text, timestamps it, and appends it to a file using markdown.

If it is a new day, it will add a header with the new day so there is some sense of organization.

Sample log file:
```
 # 2024_07_29
   * [2024-07-29_09:49:23][Start of the log]
   * [2024-07-29_09:49:59][added option to have backups to the thing I was working on]
   * [2024-07-29_09:50:57][lunch time]
 # 2024-07-30
   * [2024-07-30_10:05:03][in weekly sync]
```

# bash/zsh

```
remind() {
  local log_string="$*"
  <path to exe>/remind "<path to log>/log.md" "$log_string"
}
```
