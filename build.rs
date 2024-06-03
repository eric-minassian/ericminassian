use std::process::Command;

fn main() {
    Command::new("npx")
        .args([
            "tailwindcss",
            "-c",
            "tailwind.config.js",
            "-i",
            "assets/styles/index.css",
            "-o",
            "build/index.css",
            "--minify",
        ])
        .status()
        .expect("failed to run tailwindcss");
}
