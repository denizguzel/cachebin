use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::models::{Environment, RiskLevel, WslDistro};
use crate::wsl;

pub struct CacheLocation {
    pub id: &'static str,
    pub category: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub risk: RiskLevel,
    pub rebuildable: bool,
}

pub struct Candidate {
    pub location: &'static CacheLocation,
    pub path: PathBuf,
    pub name: Option<String>,
    pub environment: Environment,
}

const NPM_CACHE: CacheLocation = CacheLocation {
    id: "npm-cache",
    category: "Node.js",
    name: "npm cache",
    description: "Downloaded npm package tarballs",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const PNPM_STORE: CacheLocation = CacheLocation {
    id: "pnpm-store",
    category: "Node.js",
    name: "pnpm store",
    description: "Content-addressable pnpm package store",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const YARN_CACHE: CacheLocation = CacheLocation {
    id: "yarn-cache",
    category: "Node.js",
    name: "Yarn cache",
    description: "Downloaded Yarn package cache",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const BUN_CACHE: CacheLocation = CacheLocation {
    id: "bun-cache",
    category: "Node.js",
    name: "Bun install cache",
    description: "Bun package installation cache",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const CARGO_REGISTRY: CacheLocation = CacheLocation {
    id: "cargo-registry",
    category: "Rust",
    name: "Cargo registry",
    description: "Cargo registry index and downloaded crates",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const GO_MOD: CacheLocation = CacheLocation {
    id: "go-modules",
    category: "Go",
    name: "Go module cache",
    description: "Downloaded Go module sources",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const GO_BUILD: CacheLocation = CacheLocation {
    id: "go-build",
    category: "Go",
    name: "Go build cache",
    description: "Compiled Go build artifacts",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const GRADLE_CACHES: CacheLocation = CacheLocation {
    id: "gradle-caches",
    category: "Java",
    name: "Gradle caches",
    description: "Gradle dependency and build caches",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const MAVEN_REPO: CacheLocation = CacheLocation {
    id: "maven-repository",
    category: "Java",
    name: "Maven repository",
    description: "Downloaded Maven dependencies",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const PIP_CACHE: CacheLocation = CacheLocation {
    id: "pip-cache",
    category: "Python",
    name: "pip cache",
    description: "pip wheel and source download cache",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const UV_CACHE: CacheLocation = CacheLocation {
    id: "uv-cache",
    category: "Python",
    name: "uv cache",
    description: "uv package manager cache",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const NUGET_PACKAGES: CacheLocation = CacheLocation {
    id: "nuget-packages",
    category: ".NET",
    name: "NuGet packages",
    description: "Restored NuGet package cache",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const VSCODE_CACHE: CacheLocation = CacheLocation {
    id: "vscode-cache",
    category: "IDE",
    name: "VS Code renderer cache",
    description: "VS Code resource and blob caches",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const VSCODE_CACHED_DATA: CacheLocation = CacheLocation {
    id: "vscode-cachedata",
    category: "IDE",
    name: "VS Code CachedData",
    description: "Cached VS Code extension and workbench data",
    risk: RiskLevel::Safe,
    rebuildable: true,
};

const JETBRAINS_CACHE: CacheLocation = CacheLocation {
    id: "jetbrains-caches",
    category: "IDE",
    name: "JetBrains caches",
    description: "JetBrains IDE caches and search indexes",
    risk: RiskLevel::Caution,
    rebuildable: true,
};

const PLAYWRIGHT: CacheLocation = CacheLocation {
    id: "playwright-browsers",
    category: "Browsers",
    name: "Playwright browsers",
    description: "Downloaded Playwright browser binaries",
    risk: RiskLevel::Caution,
    rebuildable: true,
};

const PUPPETEER: CacheLocation = CacheLocation {
    id: "puppeteer-browsers",
    category: "Browsers",
    name: "Puppeteer cache",
    description: "Downloaded Puppeteer Chrome binaries",
    risk: RiskLevel::Caution,
    rebuildable: true,
};

pub fn windows_candidates(env: &HashMap<String, String>) -> Vec<Candidate> {
    let local = env.get("LOCALAPPDATA").map(PathBuf::from);
    let roaming = env.get("APPDATA").map(PathBuf::from);
    let home = env.get("USERPROFILE").map(PathBuf::from);
    let mut out = Vec::new();

    push(&mut out, &NPM_CACHE, local.as_ref().map(|p| p.join("npm-cache")), None);
    push(&mut out, &PNPM_STORE, local.as_ref().map(|p| p.join("pnpm").join("store")), None);
    push(&mut out, &YARN_CACHE, local.as_ref().map(|p| p.join("Yarn").join("Cache")), None);
    push(&mut out, &BUN_CACHE, local.as_ref().map(|p| p.join("bun").join("install").join("cache")), None);
    push(&mut out, &CARGO_REGISTRY, home.as_ref().map(|p| p.join(".cargo").join("registry")), None);
    push(&mut out, &GO_MOD, home.as_ref().map(|p| p.join("go").join("pkg").join("mod")), None);
    push(&mut out, &GO_BUILD, local.as_ref().map(|p| p.join("go-build")), None);
    push(&mut out, &GRADLE_CACHES, home.as_ref().map(|p| p.join(".gradle").join("caches")), None);
    push(&mut out, &MAVEN_REPO, home.as_ref().map(|p| p.join(".m2").join("repository")), None);
    push(&mut out, &PIP_CACHE, local.as_ref().map(|p| p.join("pip").join("Cache")), None);
    push(&mut out, &UV_CACHE, local.as_ref().map(|p| p.join("uv").join("cache")), None);
    push(&mut out, &NUGET_PACKAGES, home.as_ref().map(|p| p.join(".nuget").join("packages")), None);
    push(&mut out, &VSCODE_CACHE, roaming.as_ref().map(|p| p.join("Code").join("Cache")), None);
    push(&mut out, &VSCODE_CACHED_DATA, roaming.as_ref().map(|p| p.join("Code").join("CachedData")), None);
    push(&mut out, &PLAYWRIGHT, local.as_ref().map(|p| p.join("ms-playwright")), None);
    push(&mut out, &PUPPETEER, local.as_ref().map(|p| p.join("puppeteer")), None);

    if let Some(jetbrains_root) = local.as_ref().map(|p| p.join("JetBrains")) {
        push_jetbrains(&mut out, &jetbrains_root);
    }

    out
}

pub fn wsl_candidates(distro: &WslDistro) -> Vec<Candidate> {
    let homes = wsl::home_dirs(distro);
    wsl_candidates_for_homes(distro, &homes)
}

pub fn wsl_candidates_for_homes(distro: &WslDistro, homes: &[String]) -> Vec<Candidate> {
    let root = PathBuf::from(&distro.root);
    let environment = Environment::wsl(&distro.name);
    let mut out = Vec::new();

    for home in homes {
        let normalized = home.trim_start_matches(['/', '\\']).replace('/', "\\");
        let base = root.join(normalized);

        push(&mut out, &NPM_CACHE, Some(base.join(".npm")), None);
        push(&mut out, &PNPM_STORE, Some(base.join(".cache").join("pnpm").join("store")), None);
        push(&mut out, &YARN_CACHE, Some(base.join(".cache").join("yarn")), None);
        push(&mut out, &BUN_CACHE, Some(base.join(".bun").join("install").join("cache")), None);
        push(&mut out, &CARGO_REGISTRY, Some(base.join(".cargo").join("registry")), None);
        push(&mut out, &GO_MOD, Some(base.join("go").join("pkg").join("mod")), None);
        push(&mut out, &GO_BUILD, Some(base.join(".cache").join("go-build")), None);
        push(&mut out, &GRADLE_CACHES, Some(base.join(".gradle").join("caches")), None);
        push(&mut out, &MAVEN_REPO, Some(base.join(".m2").join("repository")), None);
        push(&mut out, &PIP_CACHE, Some(base.join(".cache").join("pip")), None);
        push(&mut out, &UV_CACHE, Some(base.join(".cache").join("uv")), None);
        push(&mut out, &PLAYWRIGHT, Some(base.join(".cache").join("ms-playwright")), None);
    }

    for candidate in &mut out {
        candidate.environment = environment.clone();
    }

    out
}

fn push(out: &mut Vec<Candidate>, location: &'static CacheLocation, path: Option<PathBuf>, name: Option<String>) {
    if let Some(path) = path {
        out.push(Candidate {
            location,
            path,
            name,
            environment: Environment::windows(),
        });
    }
}

fn push_jetbrains(out: &mut Vec<Candidate>, root: &PathBuf) {
    let Ok(products) = fs::read_dir(root) else {
        return;
    };

    for product in products.filter_map(Result::ok) {
        let base = product.path();
        if !base.is_dir() {
            continue;
        }

        for sub in ["caches", "index"] {
            let path = base.join(sub);
            if path.is_dir() {
                let product_name = product.file_name().to_string_lossy().to_string();
                out.push(Candidate {
                    location: &JETBRAINS_CACHE,
                    path,
                    name: Some(format!("{product_name} {sub}")),
                    environment: Environment::windows(),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_with(base: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("LOCALAPPDATA".into(), format!(r"{base}\Local"));
        map.insert("APPDATA".into(), format!(r"{base}\Roaming"));
        map.insert("USERPROFILE".into(), format!(r"{base}\User"));
        map
    }

    #[test]
    fn windows_candidates_resolve_known_paths() {
        let env = env_with(r"C:\fake");
        let candidates = windows_candidates(&env);

        let paths = candidates
            .iter()
            .map(|c| c.path.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        assert!(paths.contains(&r"C:\fake\Local\npm-cache".to_string()));
        assert!(paths.contains(&r"C:\fake\User\.cargo\registry".to_string()));
        assert!(paths.contains(&r"C:\fake\User\go\pkg\mod".to_string()));
        assert!(paths.contains(&r"C:\fake\Local\ms-playwright".to_string()));
        assert!(candidates.iter().all(|c| c.environment.kind == "windows"));
        assert!(candidates.iter().all(|c| c.environment.distro.is_none()));
    }

    #[test]
    fn windows_candidates_empty_without_env_vars() {
        let candidates = windows_candidates(&HashMap::new());
        assert!(candidates.is_empty());
    }

    #[test]
    fn jetbrains_candidates_include_caches_and_index() {
        let root = std::env::temp_dir().join(format!("cachebin-jb-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("RustRover").join("caches")).unwrap();
        std::fs::create_dir_all(root.join("RustRover").join("index")).unwrap();

        let mut out = Vec::new();
        push_jetbrains(&mut out, &root);

        let names = out
            .iter()
            .map(|c| c.name.clone().unwrap_or_default())
            .collect::<Vec<_>>();

        assert!(names.contains(&"RustRover caches".to_string()));
        assert!(names.contains(&"RustRover index".to_string()));
        assert!(out.iter().all(|c| c.location.id == JETBRAINS_CACHE.id));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn wsl_candidates_build_unc_paths_for_homes() {
        let distro = WslDistro {
            name: "Ubuntu".into(),
            state: "Running".into(),
            version: 2,
            root: r"\\wsl.localhost\Ubuntu".into(),
        };
        let homes = vec!["/home/dev".to_string(), "/root".to_string()];

        let candidates = wsl_candidates_for_homes(&distro, &homes);

        assert!(!candidates.is_empty());
        assert!(candidates.iter().all(|c| c.environment.kind == "wsl"));
        assert!(candidates.iter().all(|c| c.environment.distro.as_deref() == Some("Ubuntu")));

        let paths = candidates
            .iter()
            .map(|c| c.path.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        assert!(paths.iter().any(|p| p.contains(r"\\wsl.localhost\Ubuntu\home\dev\.npm")));
        assert!(paths.iter().any(|p| p.contains(r"\\wsl.localhost\Ubuntu\root\.cargo\registry")));
    }
}
