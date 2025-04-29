fn main() {
    tauri_build::build();
    tauri_specta::build::export()
        .commands_glob("src/commands.rs")
        .typescript_path("js/bindings.gen.ts")
        .run();
}
