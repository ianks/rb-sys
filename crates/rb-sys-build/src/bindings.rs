use crate::utils::is_msvc;
use crate::RbConfig;
use linkify::{self, LinkFinder};
use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::path::PathBuf;

/// Generate bindings for the Ruby using bindgen.
pub fn generate(rbconfig: &RbConfig, static_ruby: bool) {
    let clang_args = vec![
        format!("-I{}", rbconfig.get("rubyhdrdir")),
        format!("-I{}", rbconfig.get("rubyarchhdrdir")),
        "-fms-extensions".to_string(),
    ];

    eprintln!("Using bindgen with clang args: {:?}", clang_args);

    let mut src_wrapper_h = File::open("wrapper.h").unwrap();
    let mut wrapper_h =
        File::create(PathBuf::from(env::var("OUT_DIR").unwrap()).join("wrapper.h")).unwrap();

    std::io::copy(&mut src_wrapper_h, &mut wrapper_h).expect("to copy wrapper.h");

    if !is_msvc() {
        writeln!(wrapper_h, "#ifdef HAVE_RUBY_ATOMIC_H").unwrap();
        writeln!(wrapper_h, "#include \"ruby/atomic.h\"").unwrap();
        writeln!(wrapper_h, "#endif").unwrap();
    }

    let bindings = default_bindgen(clang_args)
        .header("wrapper.h")
        .allowlist_file(".*ruby.*")
        .blocklist_item("ruby_abi_version")
        .blocklist_function("^__.*")
        .blocklist_item("RData")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let bindings = if cfg!(feature = "bindgen-rbimpls") {
        bindings
    } else {
        bindings
            .blocklist_item("^rbimpl_.*")
            .blocklist_item("^RBIMPL_.*")
    };

    let bindings = if cfg!(feature = "bindgen-deprecated-types") {
        bindings
    } else {
        bindings
            .blocklist_item("^ruby_fl_type.*")
            .blocklist_item("^_bindgen_ty_9.*")
    };

    write_bindings(bindings, "bindings-raw.rs", static_ruby, rbconfig);
    clean_docs();
    let _ = push_cargo_cfg_from_bindings();
}

fn clean_docs() {
    let path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings-raw.rs");
    let mut outfile =
        File::create(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs")).unwrap();
    let lines = read_lines(&path).unwrap();

    for line in lines {
        let line = line.unwrap();

        if line.contains("@deprecated") {
            outfile.write_all(b"#[deprecated]\n").unwrap();
        }

        if !line.contains("#[doc") {
            outfile.write_all(line.as_bytes()).unwrap();
        } else {
            let finder = LinkFinder::new();
            let mut outline = line.to_owned();
            let links: Vec<_> = finder.links(&line).collect();

            for link in links {
                outline.replace_range(
                    link.start()..link.end(),
                    format!("<{}>", link.as_str()).as_str(),
                );
            }

            // Remove anything cargo thinks is executable
            outline = outline.replace('`', "");

            outfile.write_all(outline.as_bytes()).unwrap();
        }

        outfile.write_all("\n".as_bytes()).unwrap();
    }
}

fn default_bindgen(clang_args: Vec<String>) -> bindgen::Builder {
    bindgen::Builder::default()
        .use_core()
        .rustified_enum("*")
        .no_copy("rb_data_type_struct")
        .derive_eq(true)
        .derive_debug(true)
        .clang_args(clang_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
}

fn write_bindings(builder: bindgen::Builder, path: &str, static_ruby: bool, rbconfig: &RbConfig) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut code = builder
        .generate()
        .unwrap_or_else(|_| panic!("Unable to generate bindings for {}", path))
        .to_string();

    if is_msvc() {
        qualify_symbols_for_msvc(&mut code, static_ruby, rbconfig);
    }

    let mut outfile = File::create(out_path.join(path)).expect("Couldn't create bindings file");
    write!(outfile, "{}", code).unwrap_or_else(|_| panic!("Couldn't write bindings for {}", path))
}

// This is needed because bindgen doesn't support the `__declspec(dllimport)` on
// global variables. Without it, symbols are not found.
// See https://stackoverflow.com/a/66182704/2057700
fn qualify_symbols_for_msvc(code: &mut str, is_static: bool, rbconfig: &RbConfig) {
    let kind = if is_static { "static" } else { "dylib" };

    let name = if is_static {
        rbconfig.libruby_static_name()
    } else {
        rbconfig.libruby_so_name()
    };

    let _ = code.replace(
        "extern \"C\" {",
        &format!(
            "#[link(name = \"{}\", kind = \"{}\")]\nextern \"C\" {{",
            name, kind
        ),
    );
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Add things like `#[cfg(ruby_use_transient_heap = "true")]` to the bindings config
fn push_cargo_cfg_from_bindings() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings-raw.rs");
    let lines = read_lines(&path)?;

    fn is_have_cfg(line: &str) -> bool {
        line.starts_with("pub const HAVE_RUBY")
            || line.starts_with("pub const HAVE_RB")
            || line.starts_with("pub const USE")
    }

    for line in lines {
        let line = line?;

        if is_have_cfg(&line) {
            if let Some(val) = ConfValue::new(&line) {
                let name = val.name().to_lowercase();
                let val = val.as_bool();
                println!("cargo:rustc-cfg=ruby_{}=\"{}\"", name, val);
                println!("cargo:defines_{}=\"{}\"", name, val);
            }
        }

        if line.starts_with("pub const RUBY_ABI_VERSION") {
            if let Some(val) = ConfValue::new(&line) {
                println!("cargo:ruby_abi_version=\"{}\"", val.value());
            }
        }
    }

    Ok(())
}

/// An autoconf constant in the bindings
struct ConfValue<'a> {
    raw: Cow<'a, str>,
}

impl<'a> ConfValue<'a> {
    pub fn new(raw: &'a str) -> Option<Self> {
        let prefix = "pub const ";

        if raw.starts_with(prefix) {
            let raw = raw.trim_start_matches(prefix).trim_end_matches(';').into();
            Some(Self { raw })
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        self.raw_parts().0.split(':').next().unwrap()
    }

    pub fn value(&self) -> &str {
        self.raw_parts().1
    }

    pub fn as_bool(&self) -> bool {
        self.value() == "1"
    }

    fn raw_parts(&self) -> (&str, &str) {
        let mut parts = self.raw.split(" = ");
        let name = parts.next().unwrap();
        let value = parts.next().unwrap();
        (name, value)
    }
}
