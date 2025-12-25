# Codeforces CLI tool

# Run cf command without installing
cf *args:
    cargo run --manifest-path cf/Cargo.toml --quiet -- {{args}}

# Build release binary
build:
    cargo build --manifest-path cf/Cargo.toml --release

# Generate a new solution
gen name lang="py":
    just cf gen {{name}} -l {{lang}}

# Create sample I/O files (opens browser)
eg name count="1":
    just cf eg {{name}} {{count}}

# List templates and solutions
list:
    just cf list

# Test solution against samples
test name *args:
    just cf test {{name}} {{args}}

# Login to Codeforces
login:
    just cf login

# Pull submissions from CF
pull *args:
    just cf pull {{args}}

# Submit solution (opens browser)
submit name:
    just cf submit {{name}}

# Watch submission status
watch:
    just cf watch
