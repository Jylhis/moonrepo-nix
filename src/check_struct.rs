use moon_pdk_api::*;

pub fn check(input: ExtendTaskCommandInput) {
    let _ = input.context.workspace_root;
    // let _ = input.project.source; // Uncomment this to check if it compiles
}
