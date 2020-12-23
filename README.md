# Mini Grep

## A reduced version of the traditional [Grep](https://www.gnu.org/software/grep/) program

# Usage

## To Run with cargo: ```cargo run `pattern` `filename` [(--case-insensitive) or (--case-sensitive)]```

## The default setting is to be case sensitive. Setting the ```CASE_INSENSITIVE``` environment variable to any value has the same function as passing the ```--case-insensitive``` flag. However, the flag takes precedence over the environment variable.