//! Form detection — identify files containing web forms for audit.

use std::fs;
use std::path::Path;

/// File extensions that may contain web forms.
const FORM_EXTENSIONS: &[&str] = &[
    "html", "htm", "jsx", "tsx", "vue", "svelte", "astro", "php", "erb", "hbs",
];

/// Patterns indicating a form is present.
const FORM_PATTERNS: &[&str] = &[
    "<form",
    "<Form",
    "<input",
    "<Input",
    "<select",
    "<Select",
    "<textarea",
    "<Textarea",
    "useForm",
    "handleSubmit",
    "onSubmit",
    "FormData",
];

/// Check if a single file contains web form elements.
pub fn file_has_form(path: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if !FORM_EXTENSIONS.contains(&ext) {
        return false;
    }

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    FORM_PATTERNS.iter().any(|pat| content.contains(pat))
}

/// Scan a directory for files containing forms.
pub fn find_form_files(project_dir: &str) -> Vec<String> {
    let mut form_files = Vec::new();

    for entry in walkdir::WalkDir::new(project_dir)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !name.starts_with('.')
                && name != "node_modules"
                && name != "target"
                && name != "__pycache__"
                && name != "dist"
                && name != "build"
        })
        .filter_map(|e| e.ok())
    {
        if !entry.path().is_file() {
            continue;
        }

        let path_str = entry.path().to_string_lossy().to_string();
        if file_has_form(&path_str) {
            let rel = entry
                .path()
                .strip_prefix(project_dir)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();
            form_files.push(rel);
        }
    }

    form_files
}

/// Check if a project has any forms (quick check for convergence engine).
pub fn project_has_forms(project_dir: &str) -> bool {
    !find_form_files(project_dir).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_has_form_html() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("page.html");
        fs::write(&path, "<html><body><form action='/submit'><input type='text'/></form></body></html>").unwrap();
        assert!(file_has_form(path.to_str().unwrap()));
    }

    #[test]
    fn test_file_has_form_tsx() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Form.tsx");
        fs::write(&path, "export default function MyForm() { return <form onSubmit={handleSubmit}><Input /></form> }").unwrap();
        assert!(file_has_form(path.to_str().unwrap()));
    }

    #[test]
    fn test_file_no_form() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("page.html");
        fs::write(&path, "<html><body><h1>Hello</h1></body></html>").unwrap();
        assert!(!file_has_form(path.to_str().unwrap()));
    }

    #[test]
    fn test_non_web_file_ignored() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("main.rs");
        fs::write(&path, "fn main() { let form = true; }").unwrap();
        assert!(!file_has_form(path.to_str().unwrap()));
    }

    #[test]
    fn test_find_form_files() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("project");
        let src = project.join("src");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("form.html"), "<form><input/></form>").unwrap();
        fs::write(src.join("about.html"), "<h1>About</h1>").unwrap();
        fs::write(src.join("app.tsx"), "const x = <form onSubmit={fn}/>").unwrap();

        let files = find_form_files(project.to_str().unwrap());
        assert_eq!(files.len(), 2, "found: {:?}", files);
    }

    #[test]
    fn test_react_hook_form_detected() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Login.tsx");
        fs::write(&path, "import { useForm } from 'react-hook-form';\nconst { handleSubmit } = useForm();").unwrap();
        assert!(file_has_form(path.to_str().unwrap()));
    }

    #[test]
    fn test_project_has_forms_false() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("readme.md"), "# Hello").unwrap();
        assert!(!project_has_forms(dir.path().to_str().unwrap()));
    }
}
