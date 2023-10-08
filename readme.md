# HTMLPreprocessor

HTMLPreprocessor is a simple HTML preprocessor library written in Rust. It's designed to work seamlessly with Tauri and enables you to create a basic templating system, making it easier to manage and reuse HTML components across your project.

## Features

- Block Replacement: The library identifies HTML files containing `{% block header %}{% endblock header %}` and `{% block content %}{% endblock content %}` tags. It then substitutes these blocks with the corresponding content from your templates.

- File Copying: All files from the source directory, including non-HTML files, are copied over to the output directory.

- Template System: Allows you to create a base template and inject content from other HTML files, enhancing reusability and organization in your project.

- Integration with Tauri: HTMLPreprocessor is designed to work great with Tauri, a toolkit for building smaller, faster, and more secure desktop applications with a web frontend.

## Usage

Add HTMLPreprocessor as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
html_preprocessor = "0.1.0"
```

To use HTMLPreprocessor, you need to add a `build.rs` in your project root and use the HTMLPreprocessor in it. A simplified example would be:

```rust
fn main() {
    html_preprocessor::process_html_files("src_dir", "src_dir/template_dir" , "out_dir");
}
```

This will process all HTML files in the source directory, replace the blocks with the corresponding content from your templates, and output the results to the output directory. It integrates well with Tauri and other web front-end frameworks to help you manage your project more efficiently.
