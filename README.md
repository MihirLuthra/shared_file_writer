# shared_file_writer

Spawns multiple threads and writes a message to a shared file.

# Usage

```text
shared_file_writer 0.1.0

USAGE:
    shared_file_writer [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file-path <file-path>          Shared file. Created if doesn't exist [default: /tmp/shm]
    -m, --message <message>              Message by each thread [default: Hello shared file.]
    -s, --seconds <seconds>              Delay in releasing lock. (in secs) [default: 0.5]
    -t, --thread-count <thread-count>    Number of threads [default: 2]

```

# Example

```text
shared_file_writer -f shm.txt
```

This will create a file in your current directory (overwrite if it already exists) with contents similar to this:

```text
$ cat shm.txt 
Hello shared file.[by thread-1]
Hello shared file.[by thread-2]
$
```

# Custom message and threads

```text
shared_file_writer -f shm.txt --message "I am shared memory writer. " --thread-count 10
```

This will produce file similar to this:

```text
$ cat shm.txt
I am shared memory writer. [by thread-1]
I am shared memory writer. [by thread-3]
I am shared memory writer. [by thread-4]
I am shared memory writer. [by thread-2]
I am shared memory writer. [by thread-6]
I am shared memory writer. [by thread-5]
I am shared memory writer. [by thread-7]
I am shared memory writer. [by thread-8]
I am shared memory writer. [by thread-9]
I am shared memory writer. [by thread-10]
$
```
