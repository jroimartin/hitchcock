//! Builds third-party dependencies.

fn main() {
    build_imgui();
    build_stb_image();
}

fn build_imgui() {
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

fn build_stb_image() {
    println!("cargo::rerun-if-changed=third_party/stb_image/stb_image.c");
    cc::Build::new()
        .file("third_party/stb_image/stb_image.c")
        .compile("stb_image");
}
