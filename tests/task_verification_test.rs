use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn get_task_command_by_tag(tag: &str) -> String {
    let tasks_json =
        fs::read_to_string("languages/cangjie/tasks.json").expect("Failed to read tasks.json");
    let tasks: Value = serde_json::from_str(&tasks_json).expect("Failed to parse tasks.json");
    let tasks_array = tasks.as_array().expect("tasks.json is not an array");

    for task in tasks_array {
        if let Some(tags) = task["tags"].as_array() {
            if tags.iter().any(|t| t.as_str() == Some(tag)) {
                let cmd = task["command"]
                    .as_str()
                    .expect("Command is not a string")
                    .to_string();
                // Convert PowerShell env var syntax to bash for test compatibility
                return cmd.replace("$env:", "$");
            }
        }
    }
    panic!("Task with tag '{}' not found", tag);
}

struct TestProject {
    temp_dir: PathBuf,
    zed_file: PathBuf,
}

impl TestProject {
    fn new(name: &str) -> Self {
        let temp_dir = std::env::temp_dir().join(format!("cangjie_test_{name}"));
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let zed_file = temp_dir.join("main.cj");
        fs::write(&zed_file, "main() { println(\"test\") }").unwrap();

        let bin_dir = temp_dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let cjpm_mock = bin_dir.join("cjpm");
        fs::write(&cjpm_mock, "#!/bin/sh\necho \"CJPM_CALLED: $@\"").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&cjpm_mock, fs::Permissions::from_mode(0o755)).unwrap();
        }

        Self {
            temp_dir,
            zed_file,
        }
    }

    fn task(&self, tag: &str) -> TaskRunner<'_> {
        let command = get_task_command_by_tag(tag);
        TaskRunner {
            project: self,
            command,
            zed_file: self.zed_file.clone(),
            extra_env: Vec::new(),
        }
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.temp_dir);
    }
}

struct TaskRunner<'a> {
    project: &'a TestProject,
    command: String,
    zed_file: PathBuf,
    extra_env: Vec<(&'static str, String)>,
}

impl<'a> TaskRunner<'a> {
    fn env(mut self, key: &'static str, value: String) -> Self {
        self.extra_env.push((key, value));
        self
    }

    fn run(self) -> String {
        let old_path = std::env::var("PATH").unwrap_or_default();
        let new_path = format!(
            "{}:{}",
            self.project.temp_dir.join("bin").to_string_lossy(),
            old_path
        );

        let mut cmd = std::process::Command::new("sh");
        cmd.arg("-c")
            .arg(&self.command)
            .env("ZED_FILE", self.zed_file.to_string_lossy().to_string())
            .env("ZED_DIRNAME", self.project.temp_dir.to_string_lossy().to_string())
            .env("PATH", &new_path)
            .current_dir(&self.project.temp_dir);

        for (k, v) in self.extra_env {
            cmd.env(k, v);
        }

        let output = cmd.output().expect("Failed to execute shell command");
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}

// ============================================================================
// Main Task Tests
// ============================================================================

#[test]
fn test_cjpm_run_task() {
    let project = TestProject::new("cjpm_run");
    let stdout = project.task("cangjie-main").run();

    assert!(
        stdout.contains("CJPM_CALLED: build"),
        "Should run cjpm build. Got: {}",
        stdout
    );
    assert!(
        stdout.contains("CJPM_CALLED: run"),
        "Should run cjpm run. Got: {}",
        stdout
    );
}

#[test]
fn test_cjpm_test_method_task() {
    let project = TestProject::new("cjpm_test_method");
    let stdout = project
        .task("cangjie-test-method")
        .env("ZED_CUSTOM_cangjie_test_name", "testAdd".to_string())
        .run();

    assert!(
        stdout.contains("CJPM_CALLED: test --filter testAdd"),
        "Should run cjpm test with filter. Got: {}",
        stdout
    );
}

#[test]
fn test_cjpm_test_class_task() {
    let project = TestProject::new("cjpm_test_class");
    let stdout = project
        .task("cangjie-test-class")
        .env("ZED_CUSTOM_cangjie_class_name", "CalculatorTest".to_string())
        .run();

    assert!(
        stdout.contains("CJPM_CALLED: test --filter CalculatorTest"),
        "Should run cjpm test with class filter. Got: {}",
        stdout
    );
}

#[test]
fn test_cjpm_test_all_task() {
    let project = TestProject::new("cjpm_test_all");
    let stdout = project.task("cangjie-test-all").run();

    assert!(
        stdout.contains("CJPM_CALLED: test"),
        "Should run all tests with cjpm test. Got: {}",
        stdout
    );
    assert!(
        !stdout.contains("--filter"),
        "Should not include filter for running all tests. Got: {}",
        stdout
    );
}

// ============================================================================
// Task structure validation
// ============================================================================

#[test]
fn test_task_label_interpolation() {
    let tasks_json =
        fs::read_to_string("languages/cangjie/tasks.json").expect("Failed to read tasks.json");
    let tasks: Value = serde_json::from_str(&tasks_json).expect("Failed to parse tasks.json");
    let tasks_array = tasks.as_array().expect("tasks.json is not an array");

    for task in tasks_array {
        assert!(
            task.get("label").and_then(|v| v.as_str()).is_some(),
            "Each task must have a label"
        );
        assert!(
            task.get("command").and_then(|v| v.as_str()).is_some(),
            "Each task must have a command"
        );
        assert!(
            task.get("tags").and_then(|v| v.as_array()).is_some(),
            "Each task must have tags"
        );
    }

    let all_tags: Vec<&str> = tasks_array
        .iter()
        .flat_map(|t| {
            t["tags"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|v| v.as_str())
        })
        .collect();

    assert!(all_tags.contains(&"cangjie-main"), "Missing cangjie-main tag");
    assert!(
        all_tags.contains(&"cangjie-test-method"),
        "Missing cangjie-test-method tag"
    );
    assert!(
        all_tags.contains(&"cangjie-test-class"),
        "Missing cangjie-test-class tag"
    );
    assert!(
        all_tags.contains(&"cangjie-test-all"),
        "Missing cangjie-test-all tag"
    );
}
