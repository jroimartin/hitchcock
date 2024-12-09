//! Builds Dear ImGui.

fn main() {
    const FILES: [&str; 8] = [
        "third_party/cimgui/cimgui.cpp",
        "third_party/cimgui/imgui/imgui.cpp",
        "third_party/cimgui/imgui/imgui_demo.cpp",
        "third_party/cimgui/imgui/imgui_draw.cpp",
        "third_party/cimgui/imgui/imgui_tables.cpp",
        "third_party/cimgui/imgui/imgui_widgets.cpp",
        "third_party/cimgui/imgui/backends/imgui_impl_glfw.cpp",
        "third_party/cimgui/imgui/backends/imgui_impl_opengl3.cpp",
    ];
    let mut builder = cc::Build::new();
    let mut b = builder
        .cpp(true)
        .define("IMGUI_IMPL_API", "extern \"C\" ")
        .include("third_party/cimgui/imgui");
    for f in FILES {
        println!("cargo::rerun-if-changed={f}");
        b = b.file(f);
    }
    b.compile("imgui")
}
