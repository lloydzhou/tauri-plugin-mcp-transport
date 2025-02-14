// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::collections::HashMap;
use std::process::{Command, Stdio, Child};
use std::io::{Write, BufRead, BufReader};
use std::sync::Mutex;
use lazy_static::lazy_static;
use tauri::Emitter;

lazy_static! {
    static ref MCP_PROCESSES: Mutex<HashMap<String, Child>> = Mutex::new(HashMap::new());
}

#[derive(Clone, serde::Serialize, Debug)]
pub struct ChunkPayload {
    name: String,
    chunk: String,
}

#[tauri::command]
pub async fn mcpstart(window: tauri::Window, name: String, command: String, args: String) {
    // if name == "mpm" 尝试找一下是否有 mpm 这个命令，在什么位置？
    // 启动子进程
    let mut child = match Command::new(command)
        .arg(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
            Ok(child) => child,
            Err(e) => {
                println!("Failed to start process: {}", e);
                return;
            }
        };

    // 获取子进程的stdout
    let stdout = child.stdout.take().expect("failed to get stdout");

    // 插入子进程到全局变量
    MCP_PROCESSES.lock().unwrap().insert(name.clone(), child);

    // 异步读取子进程的stdout
    let reader = BufReader::new(stdout);
    let n = name.clone();
    tauri::async_runtime::spawn(async move {
        for line in reader.lines() {
            let text = line.expect("failed to read line");
            println!("process output: {}", text);
            if let Err(e) = window.emit("mcpchannel", ChunkPayload {
                name: n.clone(),
                chunk: text,
            }) {
                println!("Failed to emit chunk payload: {:?}", e);
            }
        }
    });
}

#[tauri::command]
pub fn mcpmessage(name: String, message: String) {
    let mut processes = MCP_PROCESSES.lock().unwrap();
    if let Some(child) = processes.get_mut(&name) {
        if let Some(stdin) = child.stdin.as_mut() {
            writeln!(stdin, "{}", message).expect("failed to write to stdin");
        } else {
            println!("Failed to get stdin for process: {}", name);
        }
    } else {
        println!("No process found with name: {}", name);
    }
}

#[tauri::command]
pub fn mcpstop(name: String) {
    let mut processes = MCP_PROCESSES.lock().unwrap();
    if let Some(mut child) = processes.remove(&name) {
        child.kill().expect("failed to kill process");
        println!("Process {} killed", name);
    } else {
        println!("No process found with name: {}", name);
    }
}