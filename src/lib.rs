use std::collections::HashMap;
use zed_extension_api::{
    self as zed, Extension, LanguageServerId, Worktree,
    set_language_server_installation_status, LanguageServerInstallationStatus,
    settings::LspSettings,
    npm_install_package, npm_package_installed_version, npm_package_latest_version,
    node_binary_path,
};

const LSP_PACKAGE_NAME: &str = "vscode-solidity-server";

struct Solidity {
    cached_binary_path: Option<String>,
}

impl Solidity {
    fn ensure_language_server_installed(
        &self,
        language_server_id: &LanguageServerId,
    ) -> zed::Result<()> {
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        // Check current installed version
        let installed_version = npm_package_installed_version(LSP_PACKAGE_NAME)?;

        // Check latest available version
        let latest_version = npm_package_latest_version(LSP_PACKAGE_NAME)?;

        match installed_version {
            Some(current_version) => {
                if current_version != latest_version {
                    // Update to latest version
                    set_language_server_installation_status(
                        language_server_id,
                        &LanguageServerInstallationStatus::Downloading,
                    );
                    npm_install_package(LSP_PACKAGE_NAME, &latest_version)?;
                }
            }
            None => {
                // Install package if not present
                set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::Downloading,
                );
                npm_install_package(LSP_PACKAGE_NAME, &latest_version)?;
            }
        }

        Ok(())
    }

    fn get_lsp_binary_path(&mut self) -> zed::Result<String> {
        // Use cached path if exists
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        // Get node path
        let node_path = node_binary_path()?.strip_suffix("/node")
            .unwrap_or(&node_binary_path()?)
            .to_string();
        // The binary might be in bin/vscode-solidity-server instead of bin/server
        let binary_path = format!("{}/{}", node_path, LSP_PACKAGE_NAME);
        // .nvm/versions/node/v20.5.1/bin/node/lib/node_modules/vscode-solidity-server/bin/vscode-solidity-server
        // .nvm/versions/node/v20.5.1/bin/vscode-solidity-server

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl Extension for Solidity {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        // Ensure LSP is installed
        self.ensure_language_server_installed(language_server_id)?;

        Ok(zed::Command {
            command: self.get_lsp_binary_path()?,
            args: vec!["--stdio".to_string()],
            // env: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(Solidity);
