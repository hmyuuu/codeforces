use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Default)]
struct Config {
    handle: Option<String>,
    cookies: Option<String>,
}

#[derive(Parser)]
#[command(name = "cf")]
#[command(about = "Codeforces solution template generator")]
#[command(
    long_about = "Generate solution files from templates for competitive programming.\n\n\
    Files are organized into:\n  \
    - solutions/{A,B,C,...}-set/ for Codeforces problems (e.g., 1900A)\n  \
    - solutions/Others/ for non-CF problems"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new solution file from template
    Gen {
        /// Problem name (e.g., 1900A -> A-set/, leetcode -> Others/)
        name: String,
        /// Language: py, cpp, hs (default: py)
        #[arg(short, long, default_value = "py")]
        lang: String,
        /// Use single test case template (no t loop)
        #[arg(short, long)]
        single: bool,
        /// Use fast I/O template (for large inputs)
        #[arg(short, long)]
        fast: bool,
    },
    /// List available templates and solutions
    List,
    /// Create sample I/O files for a problem
    Eg {
        /// Problem name (e.g., 1900A)
        name: String,
        /// Number of sample sets to create (default: 1)
        #[arg(default_value = "1")]
        count: usize,
    },
    /// Test solution against sample I/O
    Test {
        /// Problem name (e.g., 1900A)
        name: String,
        /// Specific test number (omit to run all)
        num: Option<usize>,
        /// Language: py, cpp, hs (default: py)
        #[arg(short, long, default_value = "py")]
        lang: String,
    },
    /// Login to Codeforces
    Login,
    /// Pull your submissions from Codeforces
    Pull {
        /// Problem name (e.g., 1900A) or contest ID (e.g., 1900)
        name: Option<String>,
        /// Only accepted submissions
        #[arg(short, long)]
        ac: bool,
    },
    /// Submit solution to Codeforces
    Submit {
        /// Problem name (e.g., 1900A)
        name: String,
    },
    /// Watch submission status
    Watch {
        /// Submission ID (optional, uses latest if not provided)
        #[arg(short, long)]
        id: Option<String>,
    },
}

fn get_template_dir() -> std::path::PathBuf {
    let from_exe = std::env::current_exe().ok().and_then(|mut path| {
        for _ in 0..3 {
            path = path.parent()?.to_path_buf();
        }
        Some(path.join("templates"))
    });

    if let Some(dir) = from_exe {
        if dir.is_dir() {
            return dir;
        }
    }

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let candidates = [cwd.join("cf").join("templates"), cwd.join("templates")];
    candidates
        .into_iter()
        .find(|p| p.is_dir())
        .unwrap_or_else(|| PathBuf::from("templates"))
}

fn get_config_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| Path::new(".").to_path_buf())
        .join("cf")
        .join("config.json")
}

fn load_config() -> Config {
    let path = get_config_path();
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        Config::default()
    }
}

fn save_config(config: &Config) {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&path, serde_json::to_string_pretty(config).unwrap());
}

fn parse_problem_name(name: &str) -> Option<(String, String)> {
    // Parse "1900A" -> ("1900", "A")
    let chars: Vec<char> = name.chars().collect();
    if chars.len() < 2 {
        return None;
    }

    // Find where digits end and letter begins
    let mut split_idx = chars.len();
    for (i, c) in chars.iter().enumerate().rev() {
        if c.is_ascii_digit() {
            split_idx = i + 1;
            break;
        }
    }

    if split_idx == 0 || split_idx == chars.len() {
        return None;
    }

    let contest_id: String = chars[..split_idx].iter().collect();
    let problem_letter: String = chars[split_idx..].iter().collect();

    Some((contest_id, problem_letter.to_uppercase()))
}

fn is_cf_problem(name: &str) -> bool {
    // CF problems are like "1900A", "123B" - digits followed by a letter
    let chars: Vec<char> = name.chars().collect();
    if chars.len() < 2 {
        return false;
    }
    let last = chars[chars.len() - 1];
    let has_digits = chars[..chars.len() - 1].iter().any(|c| c.is_ascii_digit());
    last.is_ascii_alphabetic() && has_digits
}

fn extract_problem_letter(name: &str) -> Option<char> {
    if is_cf_problem(name) {
        name.chars().last()
    } else {
        None
    }
}

fn generate(name: &str, lang: &str, single: bool, fast: bool) {
    let ext = match lang {
        "py" | "python" => "py",
        "cpp" | "c++" => "cpp",
        "hs" | "haskell" => "hs",
        _ => {
            eprintln!("Unknown language: {}. Use: py, cpp, hs", lang);
            return;
        }
    };

    // Select template variant (only for Python)
    let template_file: String = if ext == "py" {
        if fast {
            "template_fast.py".to_string()
        } else if single {
            "template_single.py".to_string()
        } else {
            "template.py".to_string()
        }
    } else {
        format!("template.{}", ext)
    };

    let template_path = get_template_dir().join(&template_file);

    // Extract problem letter (e.g., "1900A" -> "A") or use "Others"
    let dir_name = match extract_problem_letter(name) {
        Some(c) => format!("{}-set", c.to_ascii_uppercase()),
        None => "Others".to_string(),
    };

    let dir_path = Path::new("solutions").join(&dir_name);
    fs::create_dir_all(&dir_path).expect("Failed to create directory");

    let output_path = dir_path.join(format!("{}.{}", name, ext));

    if output_path.exists() {
        eprintln!("File {} already exists!", output_path.display());
        return;
    }

    match fs::read_to_string(&template_path) {
        Ok(content) => {
            fs::write(&output_path, content).expect("Failed to write file");
            println!("Created: {}", output_path.display());
        }
        Err(_) => eprintln!("Template not found: {:?}", template_path),
    }
}

fn count_solutions() -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    let solutions_dir = Path::new("solutions");

    if !solutions_dir.exists() {
        return counts;
    }

    if let Ok(entries) = fs::read_dir(solutions_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let dir_name = entry.file_name().to_string_lossy().to_string();
                let file_count = fs::read_dir(entry.path())
                    .map(|e| {
                        e.filter(|f| f.as_ref().map(|f| f.path().is_file()).unwrap_or(false))
                            .count()
                    })
                    .unwrap_or(0);
                if file_count > 0 {
                    counts.insert(dir_name, file_count);
                }
            }
        }
    }
    counts
}

fn list_templates() {
    println!("Templates:");
    println!("  py, python  - Python");
    println!("  cpp, c++    - C++");
    println!("  hs, haskell - Haskell");

    let counts = count_solutions();
    if !counts.is_empty() {
        println!("\nSolutions:");
        let total: usize = counts.values().sum();
        for (dir, count) in &counts {
            println!("  {}: {}", dir, count);
        }
        println!("  --------");
        println!("  Total: {}", total);
    }
}

fn get_samples_dir(name: &str) -> std::path::PathBuf {
    Path::new("samples").join(name)
}

fn create_samples(name: &str, count: usize) {
    let (contest_id, problem_letter) = match parse_problem_name(name) {
        Some(p) => p,
        None => {
            eprintln!("Invalid problem name: {}. Use format like 1900A", name);
            return;
        }
    };

    let samples_dir = get_samples_dir(name);
    fs::create_dir_all(&samples_dir).expect("Failed to create directory");

    let url = format!(
        "https://codeforces.com/contest/{}/problem/{}",
        contest_id, problem_letter
    );

    // Create empty sample files for manual input
    for i in 1..=count {
        let in_path = samples_dir.join(format!("in{}.txt", i));
        let ans_path = samples_dir.join(format!("ans{}.txt", i));

        if !in_path.exists() {
            fs::write(&in_path, "").expect("Failed to create input file");
        }
        if !ans_path.exists() {
            fs::write(&ans_path, "").expect("Failed to create answer file");
        }
    }

    println!("Created: {}", samples_dir.display());
    for i in 1..=count {
        println!("  in{}.txt, ans{}.txt", i, i);
    }
    println!();
    println!("Opening: {}", url);

    // Open browser for copy-paste
    #[cfg(target_os = "macos")]
    let _ = Command::new("open").arg(&url).spawn();
    #[cfg(target_os = "linux")]
    let _ = Command::new("xdg-open").arg(&url).spawn();
    #[cfg(target_os = "windows")]
    let _ = Command::new("cmd").args(["/C", "start", &url]).spawn();
}

fn find_solution_file(name: &str, lang: &str) -> Option<std::path::PathBuf> {
    let ext = match lang {
        "py" | "python" => "py",
        "cpp" | "c++" => "cpp",
        "hs" | "haskell" => "hs",
        _ => "py", // default to py
    };

    let letter = extract_problem_letter(name)?.to_ascii_uppercase();
    let dir = Path::new("solutions").join(format!("{}-set", letter));

    let path = dir.join(format!("{}.{}", name, ext));
    if path.exists() {
        return Some(path);
    }

    // Fallback: try other extensions
    for fallback_ext in &["py", "cpp", "hs"] {
        if *fallback_ext == ext {
            continue;
        }
        let path = dir.join(format!("{}.{}", name, fallback_ext));
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn test_solution(name: &str, num: Option<usize>, lang: &str) {
    let solution = match find_solution_file(name, lang) {
        Some(p) => p,
        None => {
            eprintln!("No solution file found for {}", name);
            return;
        }
    };

    let samples_dir = get_samples_dir(name);
    if !samples_dir.exists() {
        println!("No samples found. Run: cf eg {}", name);
        return;
    }

    let ext = solution.extension().and_then(|s| s.to_str()).unwrap_or("");
    println!("Testing: {}", solution.display());

    let cpp_exe = if ext == "cpp" {
        match compile_cpp(&solution) {
            Ok(exe) => Some(exe),
            Err(err) => {
                eprintln!("Compilation failed:\n{}", err.trim_end());
                return;
            }
        }
    } else {
        None
    };

    let mut passed = 0;
    let mut failed = 0;

    // If specific test number, only run that one
    let test_nums: Vec<usize> = if let Some(n) = num {
        vec![n]
    } else {
        // Find all test files
        let mut nums = vec![];
        let mut i = 1;
        while samples_dir.join(format!("in{}.txt", i)).exists() {
            nums.push(i);
            i += 1;
        }
        nums
    };

    if test_nums.is_empty() {
        eprintln!("No test cases found. Run: cf eg {}", name);
        return;
    }

    for test_num in &test_nums {
        let in_path = samples_dir.join(format!("in{}.txt", test_num));
        let ans_path = samples_dir.join(format!("ans{}.txt", test_num));

        if !in_path.exists() {
            eprintln!("Test {}: in{}.txt not found", test_num, test_num);
            continue;
        }
        if !ans_path.exists() {
            eprintln!("Test {}: ans{}.txt not found", test_num, test_num);
            continue;
        }

        let expected = fs::read_to_string(&ans_path)
            .unwrap_or_default()
            .trim()
            .to_string();

        let output = run_solution(&solution, ext, &in_path, cpp_exe.as_deref());

        let actual = output.trim().to_string();
        if actual == expected {
            println!("Test {}: PASSED", test_num);
            passed += 1;
        } else {
            println!("Test {}: FAILED", test_num);
            println!("  Expected: {}", expected.replace('\n', "\\n"));
            println!("  Got:      {}", actual.replace('\n', "\\n"));
            failed += 1;
        }
    }

    // Clean up compiled executable
    if let Some(exe) = cpp_exe {
        let _ = fs::remove_file(&exe);
    }

    println!("\nResults: {} passed, {} failed", passed, failed);
}

fn compile_cpp(source: &Path) -> Result<PathBuf, String> {
    let exe = if std::env::consts::EXE_EXTENSION.is_empty() {
        source.with_extension("")
    } else {
        source.with_extension(std::env::consts::EXE_EXTENSION)
    };

    let output = Command::new("g++")
        .args(["-std=gnu++23", "-O2", "-pipe", "-o"])
        .arg(&exe)
        .arg(source)
        .output()
        .map_err(|e| format!("Failed to run g++: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(if stderr.trim().is_empty() {
            "g++ failed (no stderr captured)".to_string()
        } else {
            stderr
        });
    }

    Ok(exe)
}

fn run_solution(path: &Path, ext: &str, input_path: &Path, cpp_exe: Option<&Path>) -> String {
    let input = fs::read_to_string(input_path).unwrap_or_default();

    let output = match ext {
        "py" => Command::new("python3")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(stdin) = child.stdin.as_mut() {
                    let _ = stdin.write_all(input.as_bytes());
                }
                child.wait_with_output()
            }),
        "cpp" => {
            let exe = match cpp_exe {
                Some(p) => p,
                None => return "Missing compiled executable".to_string(),
            };
            Command::new(exe)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .and_then(|mut child| {
                    use std::io::Write;
                    if let Some(stdin) = child.stdin.as_mut() {
                        let _ = stdin.write_all(input.as_bytes());
                    }
                    child.wait_with_output()
                })
        }
        "hs" => Command::new("runhaskell")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(stdin) = child.stdin.as_mut() {
                    let _ = stdin.write_all(input.as_bytes());
                }
                child.wait_with_output()
            }),
        _ => return "Unknown language".to_string(),
    };

    match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(e) => format!("Error: {}", e),
    }
}

fn login() {
    print!("CF Handle: ");
    io::stdout().flush().unwrap();
    let mut handle = String::new();
    io::stdin().read_line(&mut handle).unwrap();
    let handle = handle.trim().to_string();

    if handle.is_empty() {
        eprintln!("Handle cannot be empty");
        return;
    }

    println!("Verifying handle {}...", handle);

    // Verify handle exists via CF API
    let url = format!("https://codeforces.com/api/user.info?handles={}", handle);
    let client = reqwest::blocking::Client::new();

    match client.get(&url).send() {
        Ok(resp) => {
            let text = resp.text().unwrap_or_default();
            if text.contains("\"status\":\"OK\"") {
                println!("Handle verified! Saved.");
                let mut config = load_config();
                config.handle = Some(handle);
                save_config(&config);
            } else {
                eprintln!("Handle not found on Codeforces");
            }
        }
        Err(e) => eprintln!("Failed to verify: {}", e),
    }
}

fn pull(name: Option<String>, ac_only: bool) {
    let config = load_config();
    let handle = match config.handle {
        Some(h) => h,
        None => {
            eprintln!("Not logged in. Run: cf login");
            return;
        }
    };

    println!("Fetching submissions for {}...", handle);

    let url = format!(
        "https://codeforces.com/api/user.status?handle={}&from=1&count=100",
        handle
    );
    let client = reqwest::blocking::Client::new();

    let resp = match client.get(&url).send() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to fetch: {}", e);
            return;
        }
    };

    let text = resp.text().unwrap_or_default();
    let json: serde_json::Value = match serde_json::from_str(&text) {
        Ok(j) => j,
        Err(_) => {
            eprintln!("Failed to parse response");
            return;
        }
    };

    if json["status"] != "OK" {
        eprintln!("API error: {}", json["comment"]);
        return;
    }

    let submissions = match json["result"].as_array() {
        Some(s) => s,
        None => {
            eprintln!("No submissions found");
            return;
        }
    };

    let mut shown = 0;

    for sub in submissions {
        let verdict = sub["verdict"].as_str().unwrap_or("");
        if ac_only && verdict != "OK" {
            continue;
        }

        let contest_id = sub["contestId"].as_u64().unwrap_or(0);
        let problem_idx = sub["problem"]["index"].as_str().unwrap_or("");
        let problem_name = format!("{}{}", contest_id, problem_idx);

        // Filter by name if provided
        if let Some(ref filter) = name {
            let matches = problem_name
                .to_lowercase()
                .starts_with(&filter.to_lowercase())
                || filter
                    .parse::<u64>()
                    .map(|id| contest_id == id)
                    .unwrap_or(false);
            if !matches {
                continue;
            }
        }

        let lang = sub["programmingLanguage"].as_str().unwrap_or("");
        let status = if verdict == "OK" { "AC" } else { verdict };

        println!("{:<10} | {:<20} | {}", problem_name, lang, status);

        shown += 1;
        if shown >= 20 {
            println!("... (showing first 20)");
            break;
        }
    }

    if shown == 0 {
        println!("No matching submissions found");
    }
}

fn submit(name: &str) {
    let solution = match find_solution_file(name, "py") {
        Some(p) => p,
        None => {
            eprintln!("No solution file found for {}", name);
            return;
        }
    };

    let (contest_id, problem_letter) = match parse_problem_name(name) {
        Some(p) => p,
        None => {
            eprintln!("Invalid problem name: {}", name);
            return;
        }
    };

    let ext = solution
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("py");
    let url = format!(
        "https://codeforces.com/contest/{}/submit/{}",
        contest_id, problem_letter
    );

    println!("Opening: {}", url);
    println!("File: {}", solution.display());
    println!("Language: {}", ext);

    // Open browser
    #[cfg(target_os = "macos")]
    let _ = Command::new("open").arg(&url).spawn();
    #[cfg(target_os = "linux")]
    let _ = Command::new("xdg-open").arg(&url).spawn();
    #[cfg(target_os = "windows")]
    let _ = Command::new("cmd").args(["/C", "start", &url]).spawn();
}

fn watch(id: Option<String>) {
    let config = load_config();
    let handle = match config.handle {
        Some(h) => h,
        None => {
            eprintln!("Not logged in. Run: cf login");
            return;
        }
    };

    println!("Fetching latest submissions for {}...\n", handle);

    let url = format!(
        "https://codeforces.com/api/user.status?handle={}&from=1&count=10",
        handle
    );
    let client = reqwest::blocking::Client::new();

    let resp = match client.get(&url).send() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to fetch: {}", e);
            return;
        }
    };

    let text = resp.text().unwrap_or_default();
    let json: serde_json::Value = match serde_json::from_str(&text) {
        Ok(j) => j,
        Err(_) => {
            eprintln!("Failed to parse response");
            return;
        }
    };

    if json["status"] != "OK" {
        eprintln!("API error");
        return;
    }

    let submissions = match json["result"].as_array() {
        Some(s) => s,
        None => return,
    };

    // If specific ID provided, filter to that
    for sub in submissions {
        let sub_id = sub["id"].as_u64().unwrap_or(0);
        if let Some(ref filter_id) = id {
            if sub_id.to_string() != *filter_id {
                continue;
            }
        }

        let contest_id = sub["contestId"].as_u64().unwrap_or(0);
        let problem_idx = sub["problem"]["index"].as_str().unwrap_or("");
        let verdict = sub["verdict"].as_str().unwrap_or("TESTING");
        let time_ms = sub["timeConsumedMillis"].as_u64().unwrap_or(0);
        let memory_kb = sub["memoryConsumedBytes"].as_u64().unwrap_or(0) / 1024;

        let status = match verdict {
            "OK" => "✓ Accepted".to_string(),
            "TESTING" => "⏳ Testing...".to_string(),
            "WRONG_ANSWER" => format!("✗ Wrong Answer"),
            "TIME_LIMIT_EXCEEDED" => "✗ TLE".to_string(),
            "MEMORY_LIMIT_EXCEEDED" => "✗ MLE".to_string(),
            "COMPILATION_ERROR" => "✗ CE".to_string(),
            _ => verdict.to_string(),
        };

        println!(
            "{}{} | {} | {}ms | {}KB",
            contest_id, problem_idx, status, time_ms, memory_kb
        );
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen {
            name,
            lang,
            single,
            fast,
        } => generate(&name, &lang, single, fast),
        Commands::List => list_templates(),
        Commands::Eg { name, count } => create_samples(&name, count),
        Commands::Test { name, num, lang } => test_solution(&name, num, &lang),
        Commands::Login => login(),
        Commands::Pull { name, ac } => pull(name, ac),
        Commands::Submit { name } => submit(&name),
        Commands::Watch { id } => watch(id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cf_problem() {
        assert!(is_cf_problem("1900A"));
        assert!(is_cf_problem("123B"));
        assert!(is_cf_problem("1A"));
        assert!(!is_cf_problem("2000F1")); // ends with digit, not letter
    }

    #[test]
    fn test_is_cf_problem_invalid() {
        assert!(!is_cf_problem("abc"));
        assert!(!is_cf_problem("A"));
        assert!(!is_cf_problem("leetcode"));
        assert!(!is_cf_problem(""));
    }

    #[test]
    fn test_extract_problem_letter() {
        assert_eq!(extract_problem_letter("1900A"), Some('A'));
        assert_eq!(extract_problem_letter("123B"), Some('B'));
        assert_eq!(extract_problem_letter("1a"), Some('a'));
    }

    #[test]
    fn test_extract_problem_letter_others() {
        assert_eq!(extract_problem_letter("abc"), None);
        assert_eq!(extract_problem_letter("leetcode"), None);
        assert_eq!(extract_problem_letter(""), None);
    }
}
