use std::fs;
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct ArduinoExtension {
    cached_lsp_path: Option<String>,
    cached_cli_path: Option<String>,
}

impl ArduinoExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Ok(lsp_settings) = LspSettings::for_worktree("arduino-language-server", worktree) {
            if let Some(binary) = lsp_settings.binary.as_ref() {
                if let Some(path) = binary.path.as_ref() {
                    return Ok(path.clone());
                }
            }
        }

        if let Some(path) = worktree.which("arduino-language-server") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_lsp_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "arduino/arduino-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let (os_str, arch_str) = platform_strings(platform, arch);

        let ext = match platform {
            zed::Os::Windows => "zip",
            _ => "tar.gz",
        };

        let asset_name = format!(
            "arduino-language-server_{}_{os_str}_{arch_str}.{ext}",
            release.version,
        );

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no release asset found matching {asset_name}"))?;

        let version_dir = format!("arduino-language-server-{}", release.version);
        let binary_name = match platform {
            zed::Os::Windows => "arduino-language-server.exe",
            _ => "arduino-language-server",
        };
        let binary_path = format!("{version_dir}/{binary_name}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let file_type = match platform {
                zed::Os::Windows => zed::DownloadedFileType::Zip,
                _ => zed::DownloadedFileType::GzipTar,
            };

            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|e| format!("failed to download arduino-language-server: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            clean_up_old_versions("arduino-language-server-", &version_dir);
        }

        self.cached_lsp_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn arduino_cli_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        if let Some(path) = worktree.which("arduino-cli") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_cli_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        let release = zed::latest_github_release(
            "arduino/arduino-cli",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let (os_str, arch_str) = platform_strings(platform, arch);

        let ext = match platform {
            zed::Os::Windows => "zip",
            _ => "tar.gz",
        };

        let version = release.version.strip_prefix('v').unwrap_or(&release.version);

        let asset_name = format!(
            "arduino-cli_{version}_{os_str}_{arch_str}.{ext}",
        );

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no release asset found matching {asset_name}"))?;

        let version_dir = format!("arduino-cli-{version}");
        let binary_name = match platform {
            zed::Os::Windows => "arduino-cli.exe",
            _ => "arduino-cli",
        };
        let binary_path = format!("{version_dir}/{binary_name}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            let file_type = match platform {
                zed::Os::Windows => zed::DownloadedFileType::Zip,
                _ => zed::DownloadedFileType::GzipTar,
            };

            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|e| format!("failed to download arduino-cli: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            clean_up_old_versions("arduino-cli-", &version_dir);
        }

        let work_dir = std::env::current_dir()
            .map_err(|e| format!("failed to get work directory: {e}"))?;
        let absolute_path = work_dir.join(&binary_path).to_string_lossy().to_string();

        self.cached_cli_path = Some(absolute_path.clone());
        Ok(absolute_path)
    }
}

fn platform_strings(platform: zed::Os, arch: zed::Architecture) -> (&'static str, &'static str) {
    let os_str = match platform {
        zed::Os::Mac => "macOS",
        zed::Os::Linux => "Linux",
        zed::Os::Windows => "Windows",
    };
    let arch_str = match arch {
        zed::Architecture::Aarch64 => "ARM64",
        zed::Architecture::X86 => "32bit",
        zed::Architecture::X8664 => "64bit",
    };
    (os_str, arch_str)
}

fn clean_up_old_versions(prefix: &str, current_dir: &str) {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with(prefix) && name.as_ref() != current_dir {
                fs::remove_dir_all(entry.path()).ok();
            }
        }
    }
}

impl zed::Extension for ArduinoExtension {
    fn new() -> Self {
        Self {
            cached_lsp_path: None,
            cached_cli_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.language_server_binary_path(language_server_id, worktree)?;

        let mut args: Vec<String> = Vec::new();
        let mut env: Vec<(String, String)> = worktree.shell_env();

        if let Ok(lsp_settings) = LspSettings::for_worktree("arduino-language-server", worktree) {
            if let Some(binary) = lsp_settings.binary.as_ref() {
                if let Some(binary_args) = &binary.arguments {
                    args.extend(binary_args.iter().cloned());
                }
                if let Some(binary_env) = &binary.env {
                    for (key, value) in binary_env {
                        env.push((key.clone(), value.clone()));
                    }
                }
            }
        }

        // Auto-detect or download arduino-cli
        if !args.iter().any(|a| a == "-cli") {
            let cli_path = self.arduino_cli_path(worktree)?;
            args.push("-cli".into());
            args.push(cli_path);
        }

        // Auto-set cli-config if not specified by user
        if !args.iter().any(|a| a == "-cli-config") {
            let config_path = "arduino-cli.yaml";
            if !fs::metadata(config_path).map_or(false, |stat| stat.is_file()) {
                fs::write(config_path, "board_manager:\n    additional_urls: []\n")
                    .map_err(|e| format!("failed to create arduino-cli config: {e}"))?;
            }
            let work_dir = std::env::current_dir()
                .map_err(|e| format!("failed to get work directory: {e}"))?;
            let absolute_config = work_dir.join(config_path).to_string_lossy().to_string();
            args.push("-cli-config".into());
            args.push(absolute_config);
        }

        // Auto-detect clangd from PATH
        if !args.iter().any(|a| a == "-clangd") {
            if let Some(clangd_path) = worktree.which("clangd") {
                args.push("-clangd".into());
                args.push(clangd_path);
            } else {
                return Err(
                    "clangd not found in PATH. Install it via your package manager:\n\
                     - macOS: brew install llvm\n\
                     - Ubuntu/Debian: sudo apt install clangd\n\
                     - Arch: sudo pacman -S clang\n\
                     - Fedora: sudo dnf install clang-tools-extra"
                        .into(),
                );
            }
        }

        Ok(zed::Command {
            command: binary_path,
            args,
            env,
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree("arduino-language-server", worktree)
            .ok()
            .and_then(|s| s.settings)
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(ArduinoExtension);
