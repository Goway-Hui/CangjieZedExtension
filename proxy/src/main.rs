mod http;
mod log;
mod lsp;
mod platform;

use http::handle_http;
use lsp::{parse_lsp_content, raw_has_id, write_raw, write_to_stdout, LspReader};
use platform::spawn_parent_monitor;
use serde_json::Value;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufReader, Write},
    net::TcpListener,


    process::{self, Command, Stdio},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
};

fn patch_code_lens_body(body_json: &str) -> Option<String> {
    let mut json: serde_json::Value = serde_json::from_str(body_json).ok()?;
    let capabilities = json.get_mut("result")?.get_mut("capabilities")?;
    let code_lens = capabilities.get("codeLensProvider")?;
    if code_lens != &serde_json::Value::Bool(true) {
        return None;
    }
    capabilities["codeLensProvider"] = serde_json::json!({"resolveProvider": true});
    serde_json::to_string(&json).ok()
}

fn hex_encode(s: &str) -> String {
    s.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}

/// Write a diagnostic line to the proxy log file (always works, unlike LSP window/logMessage).
fn diag_log(log_path: &str, msg: &str) {
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = writeln!(f, "{msg}");
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let bin = args.next().unwrap_or_else(|| {
        eprintln!("Usage: cangjie-lsp-proxy <LSPServer path> [args...]");
        lsp_error!("Usage: cangjie-lsp-proxy <LSPServer path> [args...]");
        process::exit(1);
    });
    let child_args: Vec<String> = args.collect();

    lsp_info!("cangjie-lsp-proxy starting: bin={bin}");

    let proxy_id = hex_encode(
        env::current_dir()
            .unwrap()
            .to_string_lossy()
            .trim_end_matches('/'),
    );
    let workdir = env::temp_dir().join("cangjie-lsp-proxy");
    fs::create_dir_all(&workdir).unwrap();

    let log_path = "D:/cangjie-proxy-debug.log".to_string();
    diag_log(&log_path, &format!("START proxy={} bin={}", std::process::id(), bin));

    // Spawn LSPServer
    let mut cmd = Command::new(&bin);
    cmd.args(&child_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    #[cfg(windows)]
    if bin.ends_with(".bat") || bin.ends_with(".cmd") {
        cmd = Command::new("cmd");
        cmd.arg("/C")
            .arg(&bin)
            .args(&child_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
    }

    let mut child = cmd.spawn().unwrap_or_else(|e| {
        eprintln!("Failed to spawn {bin}: {e}");
        lsp_error!("Failed to spawn {bin}: {e}");
        process::exit(1);
    });

    lsp_info!("LSPServer process spawned (pid={})", child.id());
    diag_log(&log_path, &format!("LSPServer spawned pid={}", child.id()));

    let child_stdin = Arc::new(Mutex::new(child.stdin.take().unwrap()));
    let child_stdout = child.stdout.take().unwrap();
    let alive = Arc::new(AtomicBool::new(true));

    let pending: Arc<Mutex<HashMap<Value, mpsc::Sender<Value>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let port_file = workdir.join("proxy").join(&proxy_id);
    fs::create_dir_all(port_file.parent().unwrap()).unwrap();
    fs::write(&port_file, port.to_string()).unwrap();

    lsp_info!("HTTP server listening on 127.0.0.1:{port}");

    let id_counter = Arc::new(AtomicU64::new(1));

    // --- Thread 1: Zed stdin -> LSPServer stdin ---
    let stdin_writer = Arc::clone(&child_stdin);
    let alive_stdin = Arc::clone(&alive);
    let log_path_t1 = log_path.clone();
    thread::spawn(move || {
        let stdin = io::stdin().lock();
        let mut reader = LspReader::new(BufReader::new(stdin));
        let mut seq = 0u64;
        while alive_stdin.load(Ordering::Relaxed) {
            match reader.read_message() {
                Ok(Some(raw)) => {
                    seq += 1;
                    if let Some(msg) = parse_lsp_content(&raw) {
                        if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
                            if let Some(id) = msg.get("id") {
                                diag_log(&log_path_t1, &format!("ZED->LS #{seq} REQ id={} method={}", id, method));
                            } else {
                                diag_log(&log_path_t1, &format!("ZED->LS #{seq} NOTIFY method={}", method));
                            }
                        }
                        // Log didOpen params to see the exact URI
                        if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
                            if method == "textDocument/didOpen" {
                                if let Some(uri) = msg.pointer("/params/textDocument/uri").and_then(|v| v.as_str()) {
                                    diag_log(&log_path_t1, &format!("  didOpen uri={}", uri));
                                }
                            }
                        }
                        // Log full initialize request to see initializationOptions
                        if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
                            if method == "initialize" {
                                if let Some(params) = msg.get("params") {
                                    if let Some(rootUri) = params.get("rootUri").and_then(|v| v.as_str()) {
                                        diag_log(&log_path_t1, &format!("  INIT rootUri={}", rootUri));
                                    }
                                    if let Some(opts) = params.get("initializationOptions") {
                                        diag_log(&log_path_t1, &format!("  INIT initOpts={}", opts));
                                    }
                                }
                            }
                        }
                    }
                    let mut w = stdin_writer.lock().unwrap();
                    if w.write_all(&raw).is_err() || w.flush().is_err() {
                        diag_log(&log_path_t1, "ZED->LS stdin write ERROR");
                        break;
                    }
                }
                Ok(None) => { diag_log(&log_path_t1, "ZED->LS EOF"); break; }
                Err(e) => { diag_log(&log_path_t1, &format!("ZED->LS read ERROR: {e}")); break; }
            }
        }
        alive_stdin.store(false, Ordering::Relaxed);
    });

    // --- Thread 2: LSPServer stdout -> Zed stdout (patch codeLens, route HTTP responses) ---
    let pending_out = Arc::clone(&pending);
    let alive_out = Arc::clone(&alive);
    let log_path_t2 = log_path.clone();
    thread::spawn(move || {
        let mut reader = LspReader::new(BufReader::new(child_stdout));
        let mut patched_init = false;
        while alive_out.load(Ordering::Relaxed) {
            match reader.read_message() {
                Ok(Some(raw)) => {
                    // Fast path: notifications can't be responses we intercept
                    if !raw_has_id(&raw) {
                        write_raw(&mut io::stdout().lock(), &raw);
                        continue;
                    }

                    let Some(msg) = parse_lsp_content(&raw) else {
                        write_raw(&mut io::stdout().lock(), &raw);
                        continue;
                    };

                    // Log all responses from LSPServer
                    if let Some(id) = msg.get("id") {
                        let has_result = msg.get("result").is_some();
                        let has_error = msg.get("error").is_some();
                        if has_error {
                            diag_log(&log_path_t2, &format!("LS->ZED RSP id={} ERROR={}", id, msg.get("error").unwrap()));
                        } else if has_result {
                            let r = msg.get("result").unwrap();
                            if r.is_null() {
                                diag_log(&log_path_t2, &format!("LS->ZED RSP id={} result=null full={}", id, msg));
                            } else if r.is_array() {
                                diag_log(&log_path_t2, &format!("LS->ZED RSP id={} result=array[{}]", id, r.as_array().map_or(0, |a| a.len())));
                            } else if r.is_object() {
                                let uri = r.get("uri").and_then(|u| u.as_str()).unwrap_or("?");
                                diag_log(&log_path_t2, &format!("LS->ZED RSP id={} result=object uri={}", id, uri));
                            } else {
                                diag_log(&log_path_t2, &format!("LS->ZED RSP id={} result=other", id));
                            }
                        }
                    }

                    // Route responses to pending HTTP requests
                    if let Some(id) = msg.get("id") {
                        if let Some(tx) = pending_out.lock().unwrap().remove(id) {
                            let _ = tx.send(msg);
                            continue;
                        }
                    }

                    // Log initialize response capabilities
                    if !patched_init && msg.get("result").is_some() {
                        if let Some(caps) = msg.pointer("/result/capabilities") {
                            let def = caps.get("definitionProvider").map_or("MISSING", |v| if v.as_bool() == Some(true) { "true" } else { "false/other" });
                            let hover = caps.get("hoverProvider").map_or("MISSING", |v| if v.as_bool() == Some(true) { "true" } else { "false/other" });
                            let sema = caps.get("semanticTokensProvider").map_or("MISSING", |_| "present");
                            let comp = caps.get("completionProvider").map_or("MISSING", |_| "present");
                            let refs = caps.get("referencesProvider").map_or("MISSING", |v| if v.as_bool() == Some(true) { "true" } else { "false/other" });
                            diag_log(&log_path_t2, &format!("LS->ZED INIT def={def} hover={hover} refs={refs} sema={sema} comp={comp}"));
                        }
                    }

                    // Patch codeLensProvider in initialize response
                    if !patched_init && msg.get("result").is_some() {
                        if let Some(body_json) = std::str::from_utf8(
                            &raw[raw
                                .windows(4)
                                .position(|w| w == lsp::HEADER_SEP)
                                .unwrap()
                                + 4..],
                        )
                        .ok()
                        {
                            if let Some(patched) = patch_code_lens_body(body_json) {
                                patched_init = true;
                                write_to_stdout(&serde_json::from_str::<Value>(&patched).unwrap());
                                diag_log(&log_path_t2, "LS->ZED INIT patched codeLensProvider");
                                continue;
                            }
                        }
                        patched_init = true;
                    }

                    write_raw(&mut io::stdout().lock(), &raw);
                }
                Ok(None) => { diag_log(&log_path_t2, "LS->ZED EOF"); break; }
                Err(e) => { diag_log(&log_path_t2, &format!("LS->ZED read ERROR: {e}")); break; }
            }
        }
        alive_out.store(false, Ordering::Relaxed);
    });

    // --- Thread 3: HTTP server for extension requests ---
    let http_writer = Arc::clone(&child_stdin);
    let http_pending = Arc::clone(&pending);
    let http_alive = Arc::clone(&alive);
    let http_id_counter = Arc::clone(&id_counter);
    let http_proxy_id = proxy_id.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {
            if !http_alive.load(Ordering::Relaxed) {
                break;
            }
            let Ok(stream) = stream else { continue };
            let writer = Arc::clone(&http_writer);
            let pend = Arc::clone(&http_pending);
            let counter = Arc::clone(&http_id_counter);
            let pid = http_proxy_id.clone();

            thread::spawn(move || {
                handle_http(stream, writer, pend, counter, &pid);
            });
        }
    });

    // --- Thread 4: Parent process monitor ---
    spawn_parent_monitor(Arc::clone(&alive), child.id());

    // Wait for child to exit
    let status = child.wait();
    lsp_info!("LSPServer process exited: {status:?}");
    diag_log(&log_path, &format!("LSPServer exited: {status:?}"));
    alive.store(false, Ordering::Relaxed);
    let _ = fs::remove_file(&port_file);
}
