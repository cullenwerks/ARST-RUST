//! Tauri commands for checking (and installing) the runtime prerequisites the dedicated server
//! needs before it can start.

use crate::models::Prerequisite;
use crate::services::prereq_service;
use crate::services::wsl_service::ServerTarget;
use crate::state::AppState;

/// Resolves the directory the dedicated server is installed into, mirroring the derivation in
/// `ProcessService::run_steamcmd_then_server` so a prerequisite check always inspects the same
/// files a launch would actually use.
async fn server_working_dir(state: &tauri::State<'_, AppState>) -> Option<std::path::PathBuf> {
    let install_dir = state.file_io.lock().await.install_dir()?.to_path_buf();
    let use_experimental = state.config.lock().await.use_experimental_server;
    Some(crate::services::process_service::server_install_subdir(&install_dir, use_experimental))
}

/// Reports the runtime prerequisites for the given server target. Safe to call at any time —
/// checks are read-only probes and never modify the system.
#[tauri::command]
pub async fn check_prerequisites(
    state: tauri::State<'_, AppState>,
    server_target: ServerTarget,
) -> Result<Vec<Prerequisite>, String> {
    if !server_target.is_supported_on_this_host() {
        return Err(format!(
            "The {} server target isn't available on this build of Longbow.",
            server_target.display_name()
        ));
    }

    match &server_target {
        ServerTarget::Windows => Ok(prereq_service::check_windows_runtime()),
        ServerTarget::Wsl { distro } => {
            let Some(working_dir) = server_working_dir(&state).await else {
                return Ok(Vec::new());
            };
            // Inconclusive (server not installed yet, no `ldd`) reports nothing rather than a
            // false alarm — see `check_wsl_runtime`.
            match prereq_service::check_wsl_runtime(
                distro.as_deref(),
                &working_dir,
                crate::services::process_service::LINUX_SERVER_BINARY,
            )
            .await
            {
                Ok(Some(prereq)) => Ok(vec![prereq]),
                Ok(None) => Ok(Vec::new()),
                Err(e) => Err(e.to_string()),
            }
        }
        ServerTarget::Linux => {
            // Unlike the ldd-based check below, this doesn't need an install directory at all —
            // it's a pure host-machine check, so it's worth surfacing before the user has even
            // located/downloaded a server (and before they hit the fail-fast in
            // `ProcessService::run_steamcmd_then_server` by actually clicking Start).
            let mut results = vec![
                prereq_service::check_steamcmd_32bit_runtime(),
                prereq_service::check_linux_ca_bundle(),
            ];

            let Some(working_dir) = server_working_dir(&state).await else {
                return Ok(results);
            };
            match prereq_service::check_linux_runtime(
                &working_dir,
                crate::services::process_service::LINUX_SERVER_BINARY,
            )
            .await
            {
                Ok(Some(prereq)) => results.push(prereq),
                Ok(None) => {}
                Err(e) => return Err(e.to_string()),
            }
            Ok(results)
        }
    }
}

/// Installs a prerequisite Longbow is able to install itself. Only `"vcredist"` qualifies today;
/// everything else reports manual instructions via its `detail` text instead.
///
/// This downloads Microsoft's official redistributable and runs its installer, which prompts for
/// UAC consent — see `prereq_service::install_vc_redist`.
#[tauri::command]
pub async fn install_prerequisite(id: String) -> Result<(), String> {
    match id.as_str() {
        "vcredist" => prereq_service::install_vc_redist(
            env!("CARGO_PKG_VERSION"),
            &std::env::temp_dir().join("longbow-prereqs"),
        )
        .await
        .map_err(|e| e.to_string()),
        other => Err(format!("'{other}' can't be installed automatically.")),
    }
}
