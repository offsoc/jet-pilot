// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AboutMetadata, CustomMenuItem, Manager, Menu, Submenu, MenuEntry, MenuItem};

mod kubernetes;
mod shell;
mod logs;

#[derive(Clone, serde::Serialize)]
struct CheckForUpdatesPayload {}

fn main() { 
    let _ = fix_path_env::fix();
    
    let ctx = tauri::generate_context!();

    let mut builder = tauri::Builder::default();

    #[cfg(target_os = "macos")]
    {
        let metadata = AboutMetadata::new()
        .authors(vec!["@unxsist".to_string()])
        .website(String::from("https://www.jet-pilot.app"))
        .license(String::from("MIT"));

    let submenu = Submenu::new(
        "JET Pilot", 
        Menu::new()
            .add_native_item(
                tauri::MenuItem::About(
                    String::from("JET Pilot"), 
                    metadata                
                )
            )
            .add_item(CustomMenuItem::new("check_for_updates", "Check for Updates..."))
            .add_native_item(tauri::MenuItem::Separator)
            .add_native_item(tauri::MenuItem::Quit));

    let copyPasteMenu = Submenu::new(
        "Edit",
        Menu::with_items([
            MenuItem::Undo.into(),
            MenuItem::Redo.into(),
            MenuItem::Separator.into(),
            MenuItem::Cut.into(),
            MenuItem::Copy.into(),
            MenuItem::Paste.into(),
            MenuItem::Separator.into(),
            MenuItem::SelectAll.into(),
        ]),
    );
    let mut menu = Menu::new().add_submenu(submenu);
    menu = menu.add_submenu(copyPasteMenu);

        builder = builder.menu(menu).on_menu_event(|event| {
            match event.menu_item_id() {
                "check_for_updates" => {
                    event.window().emit("check_for_updates", CheckForUpdatesPayload {}).unwrap();
                }
                _ => {}
            }
        });
    }

    builder.invoke_handler(tauri::generate_handler![
            kubernetes::client::set_current_kubeconfig,
            kubernetes::client::list_contexts,
            kubernetes::client::get_context_auth_info,
            kubernetes::client::get_current_context,
            kubernetes::client::list_namespaces,
            kubernetes::client::get_core_api_versions,
            kubernetes::client::get_core_api_resources,
            kubernetes::client::get_api_groups,
            kubernetes::client::get_api_group_resources,
            kubernetes::client::list_pods,
            kubernetes::client::get_pod,
            kubernetes::client::delete_pod,
            kubernetes::client::list_deployments,
            kubernetes::client::restart_deployment,
            kubernetes::client::restart_statefulset,
            kubernetes::client::list_jobs,
            kubernetes::client::list_cronjobs,
            kubernetes::client::list_configmaps,
            kubernetes::client::list_secrets,
            kubernetes::client::list_services,
            kubernetes::client::list_ingresses,
            kubernetes::client::list_persistentvolumes,
            kubernetes::client::list_persistentvolumeclaims,
            kubernetes::client::replace_pod,
            kubernetes::client::replace_deployment,
            kubernetes::client::replace_job,
            kubernetes::client::replace_cronjob,
            kubernetes::client::replace_configmap,
            kubernetes::client::replace_secret,
            kubernetes::client::replace_service,
            kubernetes::client::replace_ingress,
            kubernetes::client::replace_persistentvolumeclaim,
            kubernetes::client::get_pod_metrics,
            shell::tty::create_tty_session,
            shell::tty::stop_tty_session,
            shell::tty::write_to_pty,
            logs::structured_logging::start_structured_logging_session,
            logs::structured_logging::repurpose_structured_logging_session,
            logs::structured_logging::end_structured_logging_session,
            logs::structured_logging::add_data_to_structured_logging_session,
            logs::structured_logging::add_facet_to_structured_logging_session,
            logs::structured_logging::set_facet_match_type_for_structured_logging_session,
            logs::structured_logging::remove_facet_from_structured_logging_session,
            logs::structured_logging::get_facets_for_structured_logging_session,
            logs::structured_logging::get_columns_for_structured_logging_session,
            logs::structured_logging::set_filtered_for_facet_value,
            logs::structured_logging::get_filtered_data_for_structured_logging_session,
        ])
        .setup(|_app| {
            let _window = _app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                use tauri_nspanel::cocoa;
                use tauri_nspanel::cocoa::appkit::NSWindow;
                use tauri_nspanel::cocoa::appkit::NSWindowTitleVisibility;
                use tauri_nspanel::cocoa::base::BOOL;

                unsafe {
                    let id = _window.ns_window().unwrap() as cocoa::base::id;
                    id.setHasShadow_(BOOL::from(false));
                    id.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleHidden);
                }
            }

            #[cfg(debug_assertions)]
            {
                _window.open_devtools();
            }

            Ok(())
        })
        .run(ctx)
        .expect("Error while starting JET Pilot");
}
