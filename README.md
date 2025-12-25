# Codeforces Solutions

A repository storing my solutions to competitive programming problems on Codeforces.

## Usage

```bash
# Using just (task runner)
just new 1900A           # Python (default)
just new 1900A cpp       # C++

just eg 1900A            # Create sample I/O files
just test 1900A          # Test solution (Python default)
just test 1900A cpp      # Test C++ solution

just login               # Save your handle
just watch               # View your recent submissions
just watch -a            # View only AC submissions
just submit 1900A        # Open submit page in browser

just list                # List templates and solution counts
```

## Workflow

```bash
just new 1900A           # Create solution file
just eg 1900A            # Create sample files (opens browser)
# Copy samples into samples/1900A/in1.txt and ans1.txt
just test 1900A          # Run tests
just submit 1900A        # Open browser to submit
just watch               # Check status
```

## Commands

| Command | Description |
|---------|-------------|
| `new` | Create solution from template |
| `eg` | Create sample I/O files |
| `test` | Run solution against samples |
| `login` | Save handle for API |
| `watch` | View your submissions |
| `submit` | Open submit page in browser |
| `list` | List templates and solutions |


## File Organization

```
solutions/
  A-set/1900A.py
  B-set/2000B.cpp
  Others/leetcode.py
samples/
  1900A/in1.txt, ans1.txt
```

## Build

```bash
just build    # Build release binary
```

## Requirements

- `just` (task runner)
- Rust toolchain (`cargo`) for the CLI
- `python3` for Python solutions
- `g++` for C++ solutions
- `runhaskell` for Haskell solutions
