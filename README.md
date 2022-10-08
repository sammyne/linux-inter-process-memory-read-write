# linux-inter-process-memory-read-write

## Environment
- Any Linux with Kernel >= 5.4

## Quickstart

### 1. Build
```bash
cargo build
```

### 2. Run Alice
```bash
target/debug/alice
```

Sample output goes as

```bash
this is Alice
Now execute
  sudo target/debug/bob 2968345 0x564b258e2010 28
Press any key
```

### 3. Run Bob

> The command to run is copied from sample output of Alice.

```bash
target/debug/bob 2968345 0x564b258e2010 28
```

Sample output goes as

```bash
opening /proc/2968345/mem, address is 564b258e2010
string at 0x564b258e2010 in process 2968345 is:
This is some text from Alice
```

### 4. Back to Alice
Standard output should produce a new line as
```bash
foo has been changed to 'This is some text from Bob:)'
```

Which means memory of alice has been overriden by Bob.

## Reference
- [Linux: read and write another process's memory](https://renenyffenegger.ch/notes/Linux/memory/read-write-another-processes-memory)
