# Codeforces CLI tool

# Run cf command without installing
cf *args:
    cargo run --manifest-path cf/Cargo.toml --quiet -- {{args}}

# Build release binary
build:
    cargo build --manifest-path cf/Cargo.toml --release

# Create a new solution
new name lang="py" *args:
    just cf new {{name}} -l {{lang}} {{args}}

# Create sample I/O files (opens browser)
eg name count="1":
    just cf eg {{name}} {{count}}

# List templates and solutions
list:
    just cf list

# Test solution against samples
test name lang="py" *args:
    just cf test {{name}} -l {{lang}} {{args}}

# Login to Codeforces
login:
    just cf login

# Watch your submissions
watch *args:
    just cf watch {{args}}

# Submit solution (opens browser)
submit name:
    just cf submit {{name}}
