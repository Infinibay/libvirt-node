use napi::bindgen_prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// High-level wrapper for QEMU Guest Agent commands.
/// Provides convenient methods for common guest operations.
#[napi]
pub struct GuestAgent {
    machine: crate::machine::Machine,
}

/// Result of executing a command in the guest.
#[napi]
pub struct ExecResult {
    /// Process ID of the executed command
    pub pid: i32,
    /// Exit code (if command has finished)
    pub exitcode: Option<i32>,
    /// Standard output
    pub stdout: Option<String>,
    /// Standard error
    pub stderr: Option<String>,
    /// Whether the command has exited
    pub exited: bool,
}

/// Status of a running command.
#[napi]
pub struct ExecStatus {
    /// Whether the command has exited
    pub exited: bool,
    /// Exit code (if exited)
    pub exitcode: Option<i32>,
    /// Signal that terminated the process (if any)
    pub signal: Option<i32>,
    /// Standard output data (base64 encoded)
    pub out_data: Option<String>,
    /// Standard error data (base64 encoded)
    pub err_data: Option<String>,
}

/// Guest file information.
#[napi]
pub struct GuestFileInfo {
    /// File handle
    pub handle: i32,
    /// Whether operation was successful
    pub success: bool,
    /// Error message if operation failed
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct QGACommand {
    execute: String,
    arguments: Option<Value>,
}

#[derive(Serialize, Deserialize)]
struct QGAResponse {
    #[serde(rename = "return")]
    return_value: Option<Value>,
    error: Option<Value>,
}

#[napi]
impl GuestAgent {
    /// Create a new GuestAgent wrapper for a machine.
    ///
    /// # Arguments
    ///
    /// * `machine` - The Machine instance to wrap.
    ///
    /// # Returns
    ///
    /// A new GuestAgent instance.
    #[napi(constructor)]
    pub fn new(machine: &crate::machine::Machine) -> Self {
        Self {
            machine: machine.clone(),
        }
    }

    /// Execute a command in the guest.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The command to execute.
    /// * `args` - Optional array of arguments.
    /// * `capture_output` - Whether to capture stdout/stderr.
    ///
    /// # Returns
    ///
    /// ExecResult with command execution details, or null on error.
    ///
    /// # Example (in JavaScript)
    ///
    /// ```javascript
    /// const { Machine, GuestAgent } = require('libvirt-node');
    ///
    /// async function executeCommand() {
    ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
    ///   const agent = new GuestAgent(machine);
    ///   
    ///   // Windows example
    ///   const result = await agent.exec('powershell.exe', ['-Command', 'Get-Service'], true);
    ///   if (result) {
    ///     console.log('Output:', result.stdout);
    ///   }
    ///   
    ///   // Linux example
    ///   const result = await agent.exec('/bin/systemctl', ['status', 'nginx'], true);
    ///   if (result) {
    ///     console.log('Output:', result.stdout);
    ///   }
    /// }
    ///
    /// executeCommand();
    /// ```
    #[napi]
    pub fn exec(&self, cmd: String, args: Option<Vec<String>>, capture_output: Option<bool>) -> Option<ExecResult> {
        let capture = capture_output.unwrap_or(true);
        
        // Build the guest-exec command
        let command = json!({
            "execute": "guest-exec",
            "arguments": {
                "path": cmd,
                "arg": args.unwrap_or_default(),
                "capture-output": capture
            }
        });

        // Execute via qemu_agent_command
        match self.machine.qemu_agent_command(command.to_string(), 30, 0) {
            Some(response_str) => {
                // Parse the response
                if let Ok(response) = serde_json::from_str::<Value>(&response_str) {
                    if let Some(ret) = response.get("return") {
                        let pid = ret.get("pid").and_then(|p| p.as_i64()).unwrap_or(-1) as i32;
                        
                        // Immediately check status to get output if available
                        if let Some(status) = self.exec_status(pid) {
                            return Some(ExecResult {
                                pid,
                                exitcode: status.exitcode,
                                stdout: status.out_data.and_then(|data| {
                                    base64::decode(&data).ok().and_then(|bytes| String::from_utf8(bytes).ok())
                                }),
                                stderr: status.err_data.and_then(|data| {
                                    base64::decode(&data).ok().and_then(|bytes| String::from_utf8(bytes).ok())
                                }),
                                exited: status.exited,
                            });
                        }
                        
                        return Some(ExecResult {
                            pid,
                            exitcode: None,
                            stdout: None,
                            stderr: None,
                            exited: false,
                        });
                    }
                }
                None
            },
            None => None
        }
    }

    /// Check the status of a running command.
    ///
    /// # Arguments
    ///
    /// * `pid` - The process ID returned by exec().
    ///
    /// # Returns
    ///
    /// ExecStatus with command status, or null on error.
    #[napi]
    pub fn exec_status(&self, pid: i32) -> Option<ExecStatus> {
        let command = json!({
            "execute": "guest-exec-status",
            "arguments": {
                "pid": pid
            }
        });

        match self.machine.qemu_agent_command(command.to_string(), 5, 0) {
            Some(response_str) => {
                if let Ok(response) = serde_json::from_str::<Value>(&response_str) {
                    if let Some(ret) = response.get("return") {
                        return Some(ExecStatus {
                            exited: ret.get("exited").and_then(|e| e.as_bool()).unwrap_or(false),
                            exitcode: ret.get("exitcode").and_then(|e| e.as_i64()).map(|e| e as i32),
                            signal: ret.get("signal").and_then(|s| s.as_i64()).map(|s| s as i32),
                            out_data: ret.get("out-data").and_then(|d| d.as_str()).map(String::from),
                            err_data: ret.get("err-data").and_then(|d| d.as_str()).map(String::from),
                        });
                    }
                }
                None
            },
            None => None
        }
    }

    /// Read a file from the guest.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file in the guest.
    ///
    /// # Returns
    ///
    /// The file contents as a string, or null on error.
    ///
    /// # Example (in JavaScript)
    ///
    /// ```javascript
    /// const { Machine, GuestAgent } = require('libvirt-node');
    ///
    /// async function readGuestFile() {
    ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
    ///   const agent = new GuestAgent(machine);
    ///   
    ///   // Read a configuration file
    ///   const content = await agent.fileRead('/etc/nginx/nginx.conf');
    ///   if (content) {
    ///     console.log('File content:', content);
    ///   }
    /// }
    ///
    /// readGuestFile();
    /// ```
    #[napi]
    pub fn file_read(&self, path: String) -> Option<String> {
        // Open the file
        let open_command = json!({
            "execute": "guest-file-open",
            "arguments": {
                "path": path,
                "mode": "r"
            }
        });

        let handle = match self.machine.qemu_agent_command(open_command.to_string(), 5, 0) {
            Some(response_str) => {
                if let Ok(response) = serde_json::from_str::<Value>(&response_str) {
                    response.get("return").and_then(|h| h.as_i64()).map(|h| h as i32)
                } else {
                    None
                }
            },
            None => None
        }?;

        // Read the file content
        let mut content = String::new();
        let mut count = 4096; // Read in chunks
        
        loop {
            let read_command = json!({
                "execute": "guest-file-read",
                "arguments": {
                    "handle": handle,
                    "count": count
                }
            });

            match self.machine.qemu_agent_command(read_command.to_string(), 5, 0) {
                Some(response_str) => {
                    if let Ok(response) = serde_json::from_str::<Value>(&response_str) {
                        if let Some(ret) = response.get("return") {
                            let eof = ret.get("eof").and_then(|e| e.as_bool()).unwrap_or(true);
                            
                            if let Some(buf_b64) = ret.get("buf-b64").and_then(|b| b.as_str()) {
                                if let Ok(decoded) = base64::decode(buf_b64) {
                                    if let Ok(text) = String::from_utf8(decoded) {
                                        content.push_str(&text);
                                    }
                                }
                            }
                            
                            if eof {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                },
                None => break
            }
        }

        // Close the file
        let close_command = json!({
            "execute": "guest-file-close",
            "arguments": {
                "handle": handle
            }
        });
        let _ = self.machine.qemu_agent_command(close_command.to_string(), 5, 0);

        if !content.is_empty() {
            Some(content)
        } else {
            None
        }
    }

    /// Write content to a file in the guest.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file in the guest.
    /// * `content` - The content to write.
    /// * `append` - Whether to append to the file (default: false, overwrites).
    ///
    /// # Returns
    ///
    /// true if successful, false otherwise.
    ///
    /// # Example (in JavaScript)
    ///
    /// ```javascript
    /// const { Machine, GuestAgent } = require('libvirt-node');
    ///
    /// async function writeGuestFile() {
    ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
    ///   const agent = new GuestAgent(machine);
    ///   
    ///   // Write a configuration file
    ///   const success = await agent.fileWrite(
    ///     '/tmp/config.json',
    ///     JSON.stringify({ key: 'value' }),
    ///     false
    ///   );
    ///   
    ///   if (success) {
    ///     console.log('File written successfully');
    ///   }
    /// }
    ///
    /// writeGuestFile();
    /// ```
    #[napi]
    pub fn file_write(&self, path: String, content: String, append: Option<bool>) -> bool {
        let mode = if append.unwrap_or(false) { "a" } else { "w" };
        
        // Open the file
        let open_command = json!({
            "execute": "guest-file-open",
            "arguments": {
                "path": path,
                "mode": mode
            }
        });

        let handle = match self.machine.qemu_agent_command(open_command.to_string(), 5, 0) {
            Some(response_str) => {
                if let Ok(response) = serde_json::from_str::<Value>(&response_str) {
                    response.get("return").and_then(|h| h.as_i64()).map(|h| h as i32)
                } else {
                    None
                }
            },
            None => None
        };

        if handle.is_none() {
            return false;
        }
        let handle = handle.unwrap();

        // Write the content
        let content_b64 = base64::encode(content.as_bytes());
        let write_command = json!({
            "execute": "guest-file-write",
            "arguments": {
                "handle": handle,
                "buf-b64": content_b64
            }
        });

        let write_success = match self.machine.qemu_agent_command(write_command.to_string(), 5, 0) {
            Some(response_str) => {
                if let Ok(response) = serde_json::from_str::<Value>(&response_str) {
                    response.get("return").is_some()
                } else {
                    false
                }
            },
            None => false
        };

        // Close the file
        let close_command = json!({
            "execute": "guest-file-close",
            "arguments": {
                "handle": handle
            }
        });
        let _ = self.machine.qemu_agent_command(close_command.to_string(), 5, 0);

        write_success
    }

    /// Get network interfaces information from the guest.
    ///
    /// # Returns
    ///
    /// JSON string with network interfaces information, or null on error.
    #[napi]
    pub fn get_network_interfaces(&self) -> Option<String> {
        let command = json!({
            "execute": "guest-network-get-interfaces"
        });

        self.machine.qemu_agent_command(command.to_string(), 5, 0)
    }

    /// Get the guest OS information.
    ///
    /// # Returns
    ///
    /// JSON string with OS information, or null on error.
    #[napi]
    pub fn get_os_info(&self) -> Option<String> {
        let command = json!({
            "execute": "guest-get-osinfo"
        });

        self.machine.qemu_agent_command(command.to_string(), 5, 0)
    }

    /// Shutdown the guest OS.
    ///
    /// # Arguments
    ///
    /// * `mode` - Optional shutdown mode: "powerdown", "reboot", "halt" (default: "powerdown").
    ///
    /// # Returns
    ///
    /// true if command was sent successfully, false otherwise.
    #[napi]
    pub fn shutdown(&self, mode: Option<String>) -> bool {
        let shutdown_mode = mode.unwrap_or_else(|| "powerdown".to_string());
        
        let command = json!({
            "execute": "guest-shutdown",
            "arguments": {
                "mode": shutdown_mode
            }
        });

        self.machine.qemu_agent_command(command.to_string(), 5, 0).is_some()
    }

    /// Sync/flush guest filesystems.
    ///
    /// # Returns
    ///
    /// true if successful, false otherwise.
    #[napi]
    pub fn sync(&self) -> bool {
        let command = json!({
            "execute": "guest-sync"
        });

        self.machine.qemu_agent_command(command.to_string(), 5, 0).is_some()
    }

    /// Set the guest time.
    ///
    /// # Arguments
    ///
    /// * `time` - Optional time in nanoseconds since epoch. If not provided, uses host time.
    ///
    /// # Returns
    ///
    /// true if successful, false otherwise.
    #[napi]
    pub fn set_time(&self, time: Option<i64>) -> bool {
        let command = if let Some(t) = time {
            json!({
                "execute": "guest-set-time",
                "arguments": {
                    "time": t
                }
            })
        } else {
            json!({
                "execute": "guest-set-time"
            })
        };

        self.machine.qemu_agent_command(command.to_string(), 5, 0).is_some()
    }

    /// Get list of users currently logged into the guest.
    ///
    /// # Returns
    ///
    /// JSON string with user list, or null on error.
    #[napi]
    pub fn get_users(&self) -> Option<String> {
        let command = json!({
            "execute": "guest-get-users"
        });

        self.machine.qemu_agent_command(command.to_string(), 5, 0)
    }

    /// Execute a raw QEMU Guest Agent command.
    ///
    /// # Arguments
    ///
    /// * `command` - The QGA command name.
    /// * `arguments` - Optional JSON string with command arguments.
    ///
    /// # Returns
    ///
    /// The raw JSON response string, or null on error.
    ///
    /// # Example (in JavaScript)
    ///
    /// ```javascript
    /// const { Machine, GuestAgent } = require('libvirt-node');
    ///
    /// async function rawCommand() {
    ///   const machine = await Machine.lookupByName(conn, 'your-domain-name');
    ///   const agent = new GuestAgent(machine);
    ///   
    ///   // Get memory blocks information
    ///   const result = await agent.rawCommand('guest-get-memory-blocks', null);
    ///   if (result) {
    ///     console.log('Memory blocks:', result);
    ///   }
    /// }
    ///
    /// rawCommand();
    /// ```
    #[napi]
    pub fn raw_command(&self, command: String, arguments: Option<String>) -> Option<String> {
        let cmd = if let Some(args_str) = arguments {
            if let Ok(args) = serde_json::from_str::<Value>(&args_str) {
                json!({
                    "execute": command,
                    "arguments": args
                })
            } else {
                json!({
                    "execute": command
                })
            }
        } else {
            json!({
                "execute": command
            })
        };

        self.machine.qemu_agent_command(cmd.to_string(), 30, 0)
    }
}

// Helper function for base64 encoding/decoding
mod base64 {
    pub fn encode(input: &[u8]) -> String {
        // Simple base64 encoding implementation
        const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        
        let mut i = 0;
        while i < input.len() {
            let b1 = input[i];
            let b2 = if i + 1 < input.len() { input[i + 1] } else { 0 };
            let b3 = if i + 2 < input.len() { input[i + 2] } else { 0 };
            
            result.push(TABLE[(b1 >> 2) as usize] as char);
            result.push(TABLE[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char);
            result.push(if i + 1 < input.len() {
                TABLE[(((b2 & 0x0f) << 2) | (b3 >> 6)) as usize] as char
            } else {
                '='
            });
            result.push(if i + 2 < input.len() {
                TABLE[(b3 & 0x3f) as usize] as char
            } else {
                '='
            });
            
            i += 3;
        }
        
        result
    }
    
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        let input = input.trim_end_matches('=');
        let mut result = Vec::new();
        
        let mut buffer = 0u32;
        let mut bits_collected = 0;
        
        for c in input.chars() {
            let value = match c {
                'A'..='Z' => c as u32 - 'A' as u32,
                'a'..='z' => c as u32 - 'a' as u32 + 26,
                '0'..='9' => c as u32 - '0' as u32 + 52,
                '+' => 62,
                '/' => 63,
                _ => return Err(format!("Invalid base64 character: {}", c)),
            };
            
            buffer = (buffer << 6) | value;
            bits_collected += 6;
            
            if bits_collected >= 8 {
                bits_collected -= 8;
                result.push((buffer >> bits_collected) as u8);
                buffer &= (1 << bits_collected) - 1;
            }
        }
        
        Ok(result)
    }
}