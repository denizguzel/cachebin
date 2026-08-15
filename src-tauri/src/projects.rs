use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::models::{Environment, ProjectArtifact, RiskLevel};
use crate::size;

pub struct ProjectType {
    markers: &'static [&'static str],
    artifacts: &'static [&'static str],
    label: &'static str,
}

const PROJECT_TYPES: &[ProjectType] = &[
    ProjectType {
        markers: &["package.json", "pnpm-workspace.yaml", "bun.lockb", "yarn.lock", "deno.json"],
        artifacts: &[
            "node_modules", ".next", ".nuxt", ".svelte-kit", ".angular", ".turbo", ".vite",
            ".parcel-cache", ".yarn/cache", ".cache", ".expo", "dist", "build", "out",
            "coverage", "storybook-static",
        ],
        label: "Node.js / JS",
    },
    ProjectType {
        markers: &[
            "pyproject.toml",
            "requirements.txt",
            "setup.py",
            "setup.cfg",
            "Pipfile",
            "poetry.lock",
            "uv.lock",
        ],
        artifacts: &[
            ".venv", "venv", "env", "__pycache__", ".pytest_cache", ".mypy_cache",
            ".ruff_cache", ".tox", ".nox", "dist", "build", "*.egg-info", ".eggs",
            "htmlcov", "coverage",
        ],
        label: "Python",
    },
    ProjectType {
        markers: &["Cargo.toml"],
        artifacts: &["target"],
        label: "Rust",
    },
    ProjectType {
        markers: &["Package.swift"],
        artifacts: &[".build", ".swiftpm"],
        label: "Swift PM",
    },
    ProjectType {
        markers: &["Podfile"],
        artifacts: &["Pods"],
        label: "CocoaPods",
    },
    ProjectType {
        markers: &["Cartfile", "Cartfile.private"],
        artifacts: &["Carthage/Build", "Carthage/Checkouts"],
        label: "Carthage",
    },
    ProjectType {
        markers: &["*.xcodeproj", "*.xcworkspace"],
        artifacts: &["build", "DerivedData"],
        label: "Xcode",
    },
    ProjectType {
        markers: &["go.mod"],
        artifacts: &["vendor"],
        label: "Go",
    },
    ProjectType {
        markers: &["build.gradle", "build.gradle.kts", "settings.gradle", "settings.gradle.kts"],
        artifacts: &["build", ".gradle", "app/build", "out"],
        label: "Gradle",
    },
    ProjectType {
        markers: &["pom.xml"],
        artifacts: &["target"],
        label: "Maven",
    },
    ProjectType {
        markers: &["composer.json"],
        artifacts: &["vendor"],
        label: "PHP/Composer",
    },
    ProjectType {
        markers: &["Gemfile"],
        artifacts: &["vendor/bundle", ".bundle", "tmp/cache"],
        label: "Ruby",
    },
    ProjectType {
        markers: &["pubspec.yaml"],
        artifacts: &[".dart_tool", "build"],
        label: "Flutter/Dart",
    },
    ProjectType {
        markers: &["CMakeLists.txt"],
        artifacts: &["build", "cmake-build-debug", "cmake-build-release"],
        label: "CMake",
    },
    ProjectType {
        markers: &["main.tf"],
        artifacts: &[".terraform"],
        label: "Terraform",
    },
    ProjectType {
        markers: &["project.godot"],
        artifacts: &[".godot", ".import"],
        label: "Godot",
    },
    // Build/ and Builds/ are deliberately absent: they hold exported player builds, not caches.
    ProjectType {
        markers: &["ProjectSettings/ProjectVersion.txt", "Assembly-CSharp.csproj"],
        artifacts: &["Library", "Temp", "Logs", "obj"],
        label: "Unity",
    },
    // Saved/ and Build/ are deliberately absent: editor prefs/autosaves and packaged output.
    ProjectType {
        markers: &["*.uproject"],
        artifacts: &["Binaries", "Intermediate", "DerivedDataCache"],
        label: "Unreal",
    },
    // Must stay below the game engines: a Unity project ships an Assembly-CSharp.csproj too,
    // and the first type to claim a directory is the one that names it.
    ProjectType {
        markers: &["*.sln", "*.csproj", "*.fsproj", "*.vbproj"],
        artifacts: &["bin", "obj", "packages"],
        label: ".NET",
    },
    ProjectType {
        markers: &["stack.yaml", "*.cabal"],
        artifacts: &[".stack-work", "dist-newstyle", "dist"],
        label: "Haskell",
    },
    ProjectType {
        markers: &["mix.exs"],
        artifacts: &["_build", "deps", ".elixir_ls"],
        label: "Elixir",
    },
    ProjectType {
        markers: &["build.zig"],
        artifacts: &["zig-out", "zig-cache", ".zig-cache"],
        label: "Zig",
    },
    // deps live in lib/, source in src/. lib/ is only cleaned when shard.lock proves `shards install`.
    ProjectType {
        markers: &["shard.yml"],
        artifacts: &["lib", ".crystal"],
        label: "Crystal",
    },
];

/// Artifact names that collide with user/source folders; only surfaced with `has_cache_proof`.
const AMBIGUOUS_ARTIFACT_NAMES: &[&str] = &["env", "venv", ".venv", "lib"];

/// Directories never treated as projects or descended into while recursing.
const SKIP_DIRS: &[&str] = &[
    "node_modules", ".git", "target", ".build", "build", "vendor", ".dart_tool", "Pods",
    "__pycache__", ".venv", "venv", "env", "lib", ".terraform", ".next", ".nuxt",
    ".svelte-kit", ".angular", ".turbo", ".vite", ".parcel-cache", ".yarn", ".gradle",
    "Carthage", "DerivedData", "bin", "obj", ".godot", ".import", "Library", "Temp",
    "Logs", "Binaries", "Intermediate", "Saved", "DerivedDataCache", ".stack-work",
    "dist-newstyle", "_build", "deps", "zig-out", "zig-cache", ".zig-cache",
    ".pytest_cache", ".mypy_cache", ".ruff_cache", ".tox", ".nox", ".eggs",
];

/// Entries in the home directory that are never scanned as one-level projects, either because they
/// are standard Windows home folders or because they are already scanned in full as roots.
const HOME_SCAN_EXCLUSIONS: &[&str] = &[
    "AppData", "Documents", "Desktop", "Downloads", "Pictures", "Videos", "Music", "Public",
    "OneDrive", "Contacts", "Favorites", "Links", "Searches", "Saved Games", "Templates",
    "3D Objects", "Projects", "Code", "repos", "src", "source", "workspace", "Developer",
];

const MAX_SCAN_DEPTH: usize = 5;
const MIN_ARTIFACT_BYTES: u64 = 10 * 1024 * 1024;
const MAX_RESULTS: usize = 100;

pub fn project_scan_roots(env: &HashMap<String, String>, dirs: &[String]) -> Vec<PathBuf> {
    let Some(home) = env.get("USERPROFILE").or_else(|| env.get("HOME")) else {
        return Vec::new();
    };
    let home = PathBuf::from(home);

    dirs.iter()
        .map(|dir| home.join(dir))
        .filter(|path| path.is_dir())
        .collect()
}

/// Scans the project roots and the home directory's top-level folders, returning project build
/// artifacts (node_modules, target/, build/, .venv, ...) sorted by size, deduplicated and capped.
pub fn scan(
    env: &HashMap<String, String>,
    dirs: &[String],
    cancelled: &AtomicBool,
) -> Result<Vec<ProjectArtifact>, String> {
    let mut artifacts = Vec::new();
    let roots = project_scan_roots(env, dirs);

    for root in &roots {
        if cancelled.load(Ordering::Relaxed) {
            return Err("Scan cancelled".into());
        }
        find_project_artifacts(root, &mut artifacts, MAX_SCAN_DEPTH, 0, cancelled)?;
    }

    scan_home_subdirs(env, &mut artifacts, cancelled)?;

    let mut seen = HashSet::new();
    artifacts.retain(|artifact| seen.insert(artifact.path.clone()));
    artifacts.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
    artifacts.truncate(MAX_RESULTS);

    Ok(artifacts)
}

fn scan_home_subdirs(
    env: &HashMap<String, String>,
    artifacts: &mut Vec<ProjectArtifact>,
    cancelled: &AtomicBool,
) -> Result<(), String> {
    let Some(home) = env.get("USERPROFILE").or_else(|| env.get("HOME")) else {
        return Ok(());
    };
    let home = Path::new(home);

    let Ok(entries) = fs::read_dir(home) else {
        return Ok(());
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') || HOME_SCAN_EXCLUSIONS.contains(&name.as_str()) {
            continue;
        }
        if !entry.path().is_dir() {
            continue;
        }
        find_project_artifacts(&entry.path(), artifacts, 1, 0, cancelled)?;
    }

    Ok(())
}

fn find_project_artifacts(
    path: &Path,
    artifacts: &mut Vec<ProjectArtifact>,
    max_depth: usize,
    depth: usize,
    cancelled: &AtomicBool,
) -> Result<(), String> {
    if depth >= max_depth {
        return Ok(());
    }
    if cancelled.load(Ordering::Relaxed) {
        return Err("Scan cancelled".into());
    }

    // One directory read, reused for both the marker check and the recursion pass.
    let Ok(entries) = fs::read_dir(path) else {
        return Ok(());
    };
    let children = entries
        .flatten()
        .map(|entry| (entry.file_name().to_string_lossy().to_string(), entry.path()))
        .collect::<Vec<_>>();
    let names = children.iter().map(|(name, _)| name.as_str()).collect::<Vec<_>>();

    let mut seen = HashSet::new();
    let mut any_found = false;

    for project_type in PROJECT_TYPES {
        let has_marker = project_type
            .markers
            .iter()
            .any(|marker| entry_exists(path, &names, marker));
        if !has_marker {
            continue;
        }

        for artifact_name in project_type.artifacts {
            for artifact_path in resolve_entries(path, &names, artifact_name) {
                if !seen.insert(artifact_path.clone()) {
                    continue;
                }
                if !artifact_path.is_dir() {
                    continue;
                }

                let leaf = artifact_path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_default();
                if AMBIGUOUS_ARTIFACT_NAMES.contains(&leaf.as_str())
                    && !has_cache_proof(&artifact_path, path, &leaf)
                {
                    continue;
                }

                let (size_bytes, _) = size::dir_size(&artifact_path, cancelled)?;
                if size_bytes < MIN_ARTIFACT_BYTES {
                    continue;
                }

                let project_name = path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_default();

                artifacts.push(ProjectArtifact {
                    id: artifact_id(&artifact_path),
                    project_path: path.to_string_lossy().to_string(),
                    name: leaf.clone(),
                    path: artifact_path.to_string_lossy().to_string(),
                    environment: Environment::windows(),
                    size_bytes,
                    risk: artifact_risk(&leaf),
                    description: format!("{} build artifact in {}", project_type.label, project_name),
                });
                any_found = true;
            }
        }
    }

    // A directory that owns a project artifact is a project root: stop here. This keeps the scan
    // fast and avoids surfacing the same artifact again from a nested project.
    if any_found {
        return Ok(());
    }

    for (name, child) in &children {
        if name.starts_with('.') && name != ".build" {
            continue;
        }
        if SKIP_DIRS.contains(&name.as_str()) {
            continue;
        }
        if child.is_dir() {
            find_project_artifacts(child, artifacts, max_depth, depth + 1, cancelled)?;
        }
    }

    Ok(())
}

/// Checks whether a file/dir matching `pattern` (literal, glob, or `/`-relative) is inside `parent`.
fn entry_exists(parent: &Path, names: &[&str], pattern: &str) -> bool {
    if pattern.contains('/') {
        return parent.join(pattern).exists();
    }
    if pattern.contains('*') {
        return names.iter().any(|name| glob_match(name, pattern));
    }
    names.contains(&pattern)
}

/// Returns full paths of entries inside `parent` matching `pattern`.
fn resolve_entries(parent: &Path, names: &[&str], pattern: &str) -> Vec<PathBuf> {
    if pattern.contains('/') {
        let full = parent.join(pattern);
        return if full.exists() { vec![full] } else { Vec::new() };
    }
    if pattern.contains('*') {
        return names
            .iter()
            .filter(|name| glob_match(name, pattern))
            .map(|name| parent.join(name))
            .collect();
    }
    if names.contains(&pattern) {
        vec![parent.join(pattern)]
    } else {
        Vec::new()
    }
}

/// Minimal glob supporting a single `*` as a prefix or suffix.
fn glob_match(name: &str, pattern: &str) -> bool {
    if !pattern.contains('*') {
        return name == pattern;
    }
    if pattern.starts_with('*') {
        return name.ends_with(&pattern[1..]);
    }
    if pattern.ends_with('*') {
        return name.starts_with(&pattern[..pattern.len() - 1]);
    }
    name == pattern
}

/// Definitive proof that an ambiguously-named directory is a regenerable cache, not user data.
fn has_cache_proof(artifact_path: &Path, project_path: &Path, name: &str) -> bool {
    match name {
        "env" | "venv" | ".venv" => {
            artifact_path.join("pyvenv.cfg").exists() || artifact_path.join("bin").join("activate").exists()
        }
        "lib" => project_path.join("shard.lock").exists(),
        _ => true,
    }
}

fn artifact_risk(name: &str) -> RiskLevel {
    if matches!(name, ".terraform" | "Pods" | "vendor" | "vendor/bundle" | "Carthage/Build") {
        RiskLevel::Caution
    } else {
        RiskLevel::Safe
    }
}

fn artifact_id(path: &Path) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    path.hash(&mut hasher);
    format!("project-{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    fn env_with_home(home: &Path) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("USERPROFILE".into(), home.to_string_lossy().into_owned());
        map
    }

    fn default_dirs() -> Vec<String> {
        crate::settings::SCAN_DIR_OPTIONS.iter().map(|dir| dir.to_string()).collect()
    }

    fn write_big(path: &Path) {
        std::fs::create_dir_all(path.parent().expect("parent")).unwrap();
        std::fs::write(path, vec![0u8; 11 * 1024 * 1024]).unwrap();
    }

    #[test]
    fn project_scan_roots_only_include_existing_dirs() {
        let home = std::env::temp_dir().join(format!("cachebin-roots-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(home.join("Projects")).unwrap();
        std::fs::create_dir_all(home.join("repos")).unwrap();

        let roots = project_scan_roots(&env_with_home(&home), &default_dirs());
        let names = roots
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
            .collect::<Vec<_>>();

        assert!(names.contains(&"Projects".to_string()));
        assert!(names.contains(&"repos".to_string()));
        assert!(!names.contains(&"Code".to_string()));
        assert!(!roots.is_empty());

        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn finds_node_modules_in_project() {
        let root = std::env::temp_dir().join(format!("cachebin-proj-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let project = root.join("Projects").join("myapp");
        std::fs::create_dir_all(&project).unwrap();
        std::fs::write(project.join("package.json"), "{}").unwrap();
        write_big(&project.join("node_modules").join("pkg").join("index.js"));

        let mut env = env_with_home(&root);
        env.insert("HOME".into(), root.to_string_lossy().into_owned());
        let artifacts = scan(&env, &default_dirs(), &AtomicBool::new(false)).expect("scan");

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].name, "node_modules");
        assert_eq!(artifacts[0].risk, RiskLevel::Safe);
        assert!(artifacts[0].description.contains("Node.js / JS"));
        assert!(artifacts[0].project_path.ends_with("myapp"));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn venv_without_proof_is_not_surfaced() {
        let root = std::env::temp_dir().join(format!("cachebin-venv-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let project = root.join("Projects").join("python-app");
        std::fs::create_dir_all(project.join("venv")).unwrap();
        std::fs::write(project.join("requirements.txt"), "requests\n").unwrap();
        // Bare venv/ with no pyvenv.cfg or bin/activate is user data, not a cache.
        write_big(&project.join("venv").join("bin").join("python"));

        let mut env = env_with_home(&root);
        env.insert("HOME".into(), root.to_string_lossy().into_owned());
        let artifacts = scan(&env, &default_dirs(), &AtomicBool::new(false)).expect("scan");

        assert!(artifacts.is_empty());

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn venv_with_pyvenv_cfg_is_surfaced() {
        let root = std::env::temp_dir().join(format!("cachebin-venv2-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let project = root.join("Projects").join("python-app");
        std::fs::create_dir_all(project.join(".venv")).unwrap();
        std::fs::write(project.join("requirements.txt"), "requests\n").unwrap();
        std::fs::write(project.join(".venv").join("pyvenv.cfg"), "home = ...\n").unwrap();
        write_big(&project.join(".venv").join("bin").join("python"));

        let mut env = env_with_home(&root);
        env.insert("HOME".into(), root.to_string_lossy().into_owned());
        let artifacts = scan(&env, &default_dirs(), &AtomicBool::new(false)).expect("scan");

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].name, ".venv");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn small_artifacts_below_threshold_are_ignored() {
        let root = std::env::temp_dir().join(format!("cachebin-small-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let project = root.join("Projects").join("tiny");
        std::fs::create_dir_all(project.join("node_modules")).unwrap();
        std::fs::write(project.join("package.json"), "{}").unwrap();
        std::fs::write(project.join("node_modules").join("index.js"), "x").unwrap();

        let mut env = env_with_home(&root);
        env.insert("HOME".into(), root.to_string_lossy().into_owned());
        let artifacts = scan(&env, &default_dirs(), &AtomicBool::new(false)).expect("scan");

        assert!(artifacts.is_empty());

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn glob_match_handles_prefix_suffix_and_literal() {
        assert!(glob_match("MyApp.csproj", "*.csproj"));
        assert!(glob_match("core", "core*"));
        assert!(glob_match("exact", "exact"));
        assert!(!glob_match("app.exe", "*.csproj"));
        assert!(!glob_match("other", "exact"));
    }

    #[test]
    fn artifact_risk_classifies_caution_names() {
        assert_eq!(artifact_risk(".terraform"), RiskLevel::Caution);
        assert_eq!(artifact_risk("Pods"), RiskLevel::Caution);
        assert_eq!(artifact_risk("vendor"), RiskLevel::Caution);
        assert_eq!(artifact_risk("Carthage/Build"), RiskLevel::Caution);
        assert_eq!(artifact_risk("node_modules"), RiskLevel::Safe);
        assert_eq!(artifact_risk("target"), RiskLevel::Safe);
    }

    #[test]
    fn artifact_id_is_stable_per_path() {
        let a = std::path::Path::new(r"C:\proj\node_modules");
        let b = std::path::Path::new(r"C:\proj\target");
        assert_eq!(artifact_id(a), artifact_id(a));
        assert_ne!(artifact_id(a), artifact_id(b));
    }

    #[test]
    fn duplicate_artifact_across_project_types_is_deduped() {
        // A Unity project ships an Assembly-CSharp.csproj, so it also matches the .NET markers;
        // both types list "obj" and the artifact must be surfaced only once.
        let root = std::env::temp_dir().join(format!("cachebin-dedupe-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let project = root.join("Projects").join("game");
        std::fs::create_dir_all(&project).unwrap();
        std::fs::write(project.join("Assembly-CSharp.csproj"), "<Project />").unwrap();
        write_big(&project.join("obj").join("Debug").join("game.exe"));

        let mut env = env_with_home(&root);
        env.insert("HOME".into(), root.to_string_lossy().into_owned());
        let artifacts = scan(&env, &default_dirs(), &AtomicBool::new(false)).expect("scan");

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].name, "obj");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn stops_after_finding_artifact_in_nested_project() {
        // A project root that owns an artifact is not descended into, so a nested project inside
        // it is never scanned and the artifact is not surfaced twice.
        let root = std::env::temp_dir().join(format!("cachebin-nested-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let repo = root.join("Projects").join("repo");
        std::fs::create_dir_all(repo.join("sub")).unwrap();
        std::fs::write(repo.join("package.json"), "{}").unwrap();
        write_big(&repo.join("node_modules").join("pkg").join("index.js"));
        std::fs::write(repo.join("sub").join("package.json"), "{}").unwrap();
        write_big(&repo.join("sub").join("node_modules").join("pkg").join("index.js"));

        let mut env = env_with_home(&root);
        env.insert("HOME".into(), root.to_string_lossy().into_owned());
        let artifacts = scan(&env, &default_dirs(), &AtomicBool::new(false)).expect("scan");

        assert_eq!(artifacts.len(), 1);
        assert!(artifacts[0].project_path.ends_with("repo"));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn does_not_recurse_into_skipped_directories() {
        // node_modules is a known artifact dir, so it is never treated as a project root itself.
        let root = std::env::temp_dir().join(format!("cachebin-skip-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let bare = root.join("Projects").join("bare");
        std::fs::create_dir_all(bare.join("node_modules")).unwrap();
        std::fs::write(bare.join("node_modules").join("package.json"), "{}").unwrap();
        write_big(&bare.join("node_modules").join("pkg").join("index.js"));

        let mut env = env_with_home(&root);
        env.insert("HOME".into(), root.to_string_lossy().into_owned());
        let artifacts = scan(&env, &default_dirs(), &AtomicBool::new(false)).expect("scan");

        assert!(artifacts.is_empty());

        let _ = std::fs::remove_dir_all(&root);
    }
}
