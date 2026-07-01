use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::thread;
use std::time::{Duration, SystemTime};

static ORIGINAL_TZ: OnceLock<String> = OnceLock::new();



// ============================================================
//  Localization Package (L10N) - No Emojis, Safe ASCII
// ============================================================
#[allow(dead_code)]
struct Translation {
    banner_sub: &'static str,
    tz_active: &'static str,
    scanning: &'static str,
    found_targets: &'static str,
    status_err: &'static str,
    status_active: &'static str,
    status_patched: &'static str,
    status_missing: &'static str,
    patching: &'static str,
    checking: &'static str,
    patched_ok: &'static str,
    no_feature: &'static str,
    patch_success: &'static str,
    summary: &'static str,
    restoring: &'static str,
    restored_ok: &'static str,
    no_backup: &'static str,
    restore_summary: &'static str,
    daemon_start: &'static str,
    single_check: &'static str,
    daemon_active: &'static str,
    daemon_desc: &'static str,
    stop_desc: &'static str,
    monitoring: &'static str,
    elapsed: &'static str,
    file_change: &'static str,
    recheck: &'static str,
    auto_repaired: &'static str,
    hotpatch_ok: &'static str,
    hotpatch_err: &'static str,
    daemon_exit: &'static str,
    install_alias: &'static str,
    alias_ok: &'static str,
    alias_tip: &'static str,
    alias_win_ok: &'static str,
    help_title: &'static str,
    help_usage: &'static str,
    help_cmds: &'static str,
    cmd_scan: &'static str,
    cmd_patch: &'static str,
    cmd_restore: &'static str,
    cmd_daemon: &'static str,
    cmd_start: &'static str,
    cmd_stop: &'static str,
    cmd_status: &'static str,
    cmd_logs: &'static str,
    cmd_alias: &'static str,
    help_adv: &'static str,
    adv1: &'static str,
    adv2: &'static str,
    adv3: &'static str,
    sys_os: &'static str,
    sys_node: &'static str,
    sys_tz: &'static str,
    sys_tz_cover: &'static str,
    path_label: &'static str,
    ver_label: &'static str,
    size_label: &'static str,
    panel_title: &'static str,
    panel_tz: &'static str,
    panel_original: &'static str,
    panel_tz_detect: &'static str,
    panel_domain: &'static str,
    panel_lab: &'static str,
    panel_steg: &'static str,
    panel_bottom: &'static str,
    panel_proxy: &'static str,
    panel_proxy_warn: &'static str,
}

const T_ZH: Translation = Translation {
    banner_sub: "  [SHIELD] CLAUDE SHIELD 自动安全防御套件  ",
    tz_active: "当前系统时区保护已启动: TZ=",
    scanning: "[SCAN] 正在扫描系统全局安装的 Claude Code...",
    found_targets: "共搜寻到 {n} 个安装路径:",
    status_err: "无法读取",
    status_active: "发现 {n} 处未屏蔽检测",
    status_patched: "已全面屏蔽安全防护",
    status_missing: "未在此版本检测到已知机制",
    patching: "[PATCH] 一键修补并开启全面保护机制...",
    checking: "正在检查: ",
    patched_ok: "已全面屏蔽保护，跳过。",
    no_feature: "未发现可修补的特征段。",
    patch_success: "成功修补并清除了 {n} 个活跃检测段！",
    summary: "修补总结: 成功处理 {success}/{total} 处路径。",
    restoring: "[RESTORE] 正在从备份恢复原始文件...",
    restored_ok: "已恢复: ",
    no_backup: "未发现备份文件: ",
    restore_summary: "恢复总结: 成功还原了 {n} 个文件备份。",
    daemon_start: "[DAEMON] 启动全局实时防护守护进程...",
    single_check: "正在对搜寻到的所有目标进行单次修补验证...",
    daemon_active: "实时防护守护已挂载! 已监控 {n} 个潜在目标文件:",
    daemon_desc: "后台监测采用原生操作系统事件驱动/轻量级文件状态变更监控。无更新时 CPU/IO 损耗为 0。",
    stop_desc: "按 Ctrl+C 或运行 'claude-shield stop' 可以退出防护模式。",
    monitoring: "实时监控中",
    elapsed: "已守护: ",
    file_change: "检测到文件变更: ",
    recheck: "重新进行安全审计...",
    auto_repaired: "重新发现活跃的检测逻辑，自动应用修补...",
    hotpatch_ok: "热修补应用成功 ({n} 处)。已恢复隐密安全配置。",
    hotpatch_err: "热修补自动处理失败: ",
    daemon_exit: "全局监控守护已安全退出。",
    install_alias: "[INSTALL] 正在为您挂载全局命令快捷别名 (CLAUDE SHIELD)...",
    alias_ok: "全局快捷别名挂载成功！",
    alias_tip: "重启终端或运行 'source {file}' 后，您可以在任何地方通过输入以下命令来启动保护:\n    - CLAUDE SHIELD\n    - claude shield\n    - claude-shield",
    alias_win_ok: "Windows 快捷脚本创建成功！您现在可以在任意 cmd/PowerShell 窗口中直接运行 'claude-shield' 或 'CLAUDE SHIELD'。",
    help_title: "使用方式:",
    help_usage: "  claude-shield <命令>",
    help_cmds: "命令列表:",
    cmd_scan: "搜寻并扫描系统中所有安装版本",
    cmd_patch: "一键修补/屏蔽全部安装版本的检测机制",
    cmd_restore: "从备份恢复全部修改过的二进制",
    cmd_daemon: "在前台启动守护监控，检测到更新自动修补",
    cmd_start: "在后台静默运行防护守护进程 (独立于当前终端)",
    cmd_stop: "停止在后台运行的防护守护进程",
    cmd_status: "检查后台防护守护进程的运行状态",
    cmd_logs: "查看后台守护程序的历史运行日志",
    cmd_alias: "在当前系统中为本工具挂载全局全局命令行快捷别名",
    help_adv: "优势:",
    adv1: "同时支持 cli.js 混淆脚本与 claude 原生可执行文件",
    adv2: "自动覆盖全局/局部环境变量（TZ=UTC）以进行二次防御",
    adv3: "7 处等长无损字节微调，不破坏软件签名或运行时执行校验",
    sys_os: "操作系统:   ",
    sys_node: "Node.js:    ",
    sys_tz: "原始时区:   ",
    sys_tz_cover: "时区已覆盖: TZ=UTCed",
    path_label: "路径: ",
    ver_label: "版本: ",
    size_label: "大小: ",
    panel_title: "+--- 保护状态 --------------------------------+",
    panel_tz: "  TZ 环境变量   ",
    panel_original: " (原始: ",
    panel_tz_detect: "  时区检测       ",
    panel_domain: "  域名黑名单     ",
    panel_lab: "  实验室关键词   ",
    panel_steg: "  隐写撇号       ",
    panel_bottom: "+----------------------------------------------+",
    panel_proxy: "  代理网关       ",
    panel_proxy_warn: "无 (存在真实IP直连泄漏风险!)",
};

const T_EN: Translation = Translation {
    banner_sub: "  [SHIELD] CLAUDE SHIELD - Auto Defense Suite  ",
    tz_active: "Current system timezone protection active: TZ=",
    scanning: "[SCAN] Scanning system-wide installations of Claude Code...",
    found_targets: "Found {n} installation path(s):",
    status_err: "Unable to read",
    status_active: "Found {n} active detection target(s)",
    status_patched: "Fully shielded & protected",
    status_missing: "No known mechanism detected in this version",
    patching: "[PATCH] Patching & enabling comprehensive protection...",
    checking: "Checking: ",
    patched_ok: "Already fully shielded, skipped.",
    no_feature: "No patchable signatures found.",
    patch_success: "Successfully patched & cleared {n} active detection segment(s)!",
    summary: "Patching Summary: Successfully processed {success}/{total} path(s).",
    restoring: "[RESTORE] Restoring original files from backups...",
    restored_ok: "Restored: ",
    no_backup: "No backup file found: ",
    restore_summary: "Restore Summary: Successfully restored {n} backup file(s).",
    daemon_start: "[DAEMON] Starting global real-time daemon sentinel...",
    single_check: "Performing validation & initial patch check on all targets...",
    daemon_active: "Daemon Sentinel Mounted! Monitoring {n} target file(s):",
    daemon_desc: "File watcher uses native OS event notifications or lightweight polling. Zero CPU/IO footprint when idle.",
    stop_desc: "Press Ctrl+C or run 'claude-shield stop' to stop protection.",
    monitoring: "Monitoring active",
    elapsed: "Elapsed: ",
    file_change: "File change detected: ",
    recheck: "Re-auditing binary...",
    auto_repaired: "Active detection logic reappeared, applying hot patch...",
    hotpatch_ok: "Hot patch successfully applied ({n} segment(s)). Safety restored.",
    hotpatch_err: "Auto hot-patching failed: ",
    daemon_exit: "Global Sentinel daemon stopped safely.",
    install_alias: "[INSTALL] Registering global command aliases (CLAUDE SHIELD)...",
    alias_ok: "Global aliases registered successfully!",
    alias_tip: "Restart your terminal or run 'source {file}'. You can now run protection from anywhere via:\n    - CLAUDE SHIELD\n    - claude shield\n    - claude-shield",
    alias_win_ok: "Windows shortcut scripts created! You can now run 'claude-shield' or 'CLAUDE SHIELD' directly in cmd/PowerShell.",
    help_title: "Usage:",
    help_usage: "  claude-shield <command>",
    help_cmds: "Commands:",
    cmd_scan: "Search and scan all installations in the system",
    cmd_patch: "One-click patch & shield detection mechanism on all versions",
    cmd_restore: "Restore all modified files from backups",
    cmd_daemon: "Run daemon sentinel in the foreground",
    cmd_start: "Start the sentinel daemon silently in the background",
    cmd_stop: "Stop the background sentinel daemon",
    cmd_status: "Check the status of the background sentinel daemon",
    cmd_logs: "View the runtime logs of the background daemon",
    cmd_alias: "Install global CLI alias/shortcuts for this tool",
    help_adv: "Key Benefits:",
    adv1: "Supports both cli.js script and native claude executables",
    adv2: "Overrides global/local TZ=UTC env for secondary stego defense",
    adv3: "7 surgical same-length byte patches keeping binary integrity",
    sys_os: "OS Platform: ",
    sys_node: "NodeJS:      ",
    sys_tz: "Original TZ: ",
    sys_tz_cover: "TZ Overridden: TZ=UTCed",
    path_label: "Path: ",
    ver_label: "Version: ",
    size_label: "Size: ",
    panel_title: "+--- Protection Status -----------------------+",
    panel_tz: "  TZ Env Var     ",
    panel_original: " (Orig: ",
    panel_tz_detect: "  TZ Detection   ",
    panel_domain: "  Domain Blacklist",
    panel_lab: "  AI Lab Keywords ",
    panel_steg: "  Stego Apostrophe",
    panel_bottom: "+----------------------------------------------+",
    panel_proxy: "  Proxy Gateway  ",
    panel_proxy_warn: "None (REAL IP LEAK RISK!)",
};

static mut LANG_ZH: bool = false;

fn get_translation() -> &'static Translation {
    unsafe {
        if LANG_ZH {
            &T_ZH
        } else {
            &T_EN
        }
    }
}

fn get_msg(key: &str) -> String {
    let t = get_translation();
    match key {
        "panelProxy" => t.panel_proxy.to_string(),
        "panelProxyWarn" => t.panel_proxy_warn.to_string(),
        "bannerSub" => t.banner_sub.to_string(),
        "tzActive" => t.tz_active.to_string(),
        "scanning" => t.scanning.to_string(),
        "statusErr" => t.status_err.to_string(),
        "statusPatched" => t.status_patched.to_string(),
        "statusMissing" => t.status_missing.to_string(),
        "patching" => t.patching.to_string(),
        "checking" => t.checking.to_string(),
        "patchedOk" => t.patched_ok.to_string(),
        "noFeature" => t.no_feature.to_string(),
        "restoring" => t.restoring.to_string(),
        "restoredOk" => t.restored_ok.to_string(),
        "noBackup" => t.no_backup.to_string(),
        "daemonStart" => t.daemon_start.to_string(),
        "singleCheck" => t.single_check.to_string(),
        "daemonDesc" => t.daemon_desc.to_string(),
        "stopDesc" => t.stop_desc.to_string(),
        "monitoring" => t.monitoring.to_string(),
        "elapsed" => t.elapsed.to_string(),
        "fileChange" => t.file_change.to_string(),
        "recheck" => t.recheck.to_string(),
        "autoRepaired" => t.auto_repaired.to_string(),
        "daemonExit" => t.daemon_exit.to_string(),
        "installAlias" => t.install_alias.to_string(),
        "aliasOk" => t.alias_ok.to_string(),
        "aliasWinOk" => t.alias_win_ok.to_string(),
        "helpTitle" => t.help_title.to_string(),
        "helpUsage" => t.help_usage.to_string(),
        "helpCmds" => t.help_cmds.to_string(),
        "cmdScan" => t.cmd_scan.to_string(),
        "cmdPatch" => t.cmd_patch.to_string(),
        "cmdRestore" => t.cmd_restore.to_string(),
        "cmdDaemon" => t.cmd_daemon.to_string(),
        "cmdStart" => t.cmd_start.to_string(),
        "cmdStop" => t.cmd_stop.to_string(),
        "cmdStatus" => t.cmd_status.to_string(),
        "cmdLogs" => t.cmd_logs.to_string(),
        "cmdAlias" => t.cmd_alias.to_string(),
        "helpAdv" => t.help_adv.to_string(),
        "adv1" => t.adv1.to_string(),
        "adv2" => t.adv2.to_string(),
        "adv3" => t.adv3.to_string(),
        "sysOS" => t.sys_os.to_string(),
        "sysNode" => t.sys_node.to_string(),
        "sysTz" => t.sys_tz.to_string(),
        "sysTzCover" => t.sys_tz_cover.to_string(),
        "path" => t.path_label.to_string(),
        "ver" => t.ver_label.to_string(),
        "size" => t.size_label.to_string(),
        "panelTitle" => t.panel_title.to_string(),
        "panelTz" => t.panel_tz.to_string(),
        "panelOriginal" => t.panel_original.to_string(),
        "panelTzDetect" => t.panel_tz_detect.to_string(),
        "panelDomain" => t.panel_domain.to_string(),
        "panelLab" => t.panel_lab.to_string(),
        "panelSteg" => t.panel_steg.to_string(),
        "panelBottom" => t.panel_bottom.to_string(),
        _ => key.to_string(),
    }
}

// ============================================================
//  Terminal Styling Configuration
// ============================================================
static mut NO_COLOR: bool = false;

fn c(code: &str) -> String {
    unsafe {
        if NO_COLOR {
            "".to_string()
        } else {
            format!("\x1b[{}m", code)
        }
    }
}

fn s_r() -> String { c("0") }
fn s_bold() -> String { c("1") }
fn s_dim() -> String { c("2") }
fn s_red() -> String { c("31") }
fn s_green() -> String { c("32") }
fn s_yellow() -> String { c("33") }
#[allow(dead_code)]
fn s_blue() -> String { c("34") }
fn s_cyan() -> String { c("36") }
fn s_bg_cyan() -> String { c("46") }
fn s_white() -> String { c("37") }

const ICON_OK: &str = "[v]";
const ICON_FAIL: &str = "[x]";
const ICON_WARN: &str = "[!]";
const ICON_INFO: &str = "[i]";
#[allow(dead_code)]
const ICON_DOT: &str = "*";

// ============================================================
//  Logger and Directory Helpers
// ============================================================
fn get_shield_dir() -> PathBuf {
    let mut home = env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            env::var("USERPROFILE")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
        });
    home.push(".claude-shield");
    home
}

fn get_log_file() -> PathBuf {
    let mut d = get_shield_dir();
    d.push("daemon.log");
    d
}

fn get_pid_file() -> PathBuf {
    let mut d = get_shield_dir();
    d.push("daemon.pid");
    d
}

fn write_log(message: &str) {
    let dir = get_shield_dir();
    let _ = fs::create_dir_all(&dir);
    let log_path = get_log_file();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    // Fallback naive date formatter
    let timestamp = format!("UNIX_{}", now);
    if let Ok(mut file) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
    {
        let _ = writeln!(file, "[{}] {}", timestamp, message);
    }
}

// ============================================================
//  Patch Rule Definitions
// ============================================================
#[allow(dead_code)]
struct PatchRule {
    id: &'static str,
    group: &'static str,
    desc: &'static str,
    find: Vec<u8>,
    from: Vec<u8>,
    to: Vec<u8>,
}

fn dec(enc: &[u8]) -> Vec<u8> {
    enc.iter().map(|&b| b ^ 0x5A).collect()
}

// Byte arrays corresponding to the 7 Same-Length signature modifications
fn get_patches() -> Vec<PatchRule> {
    vec![
        PatchRule {
            id: "TZ_SHANGHAI",
            group: "timezone",
            desc: "Asia/Shanghai",
            find: dec(&[103, 103, 103, 120, 27, 41, 51, 59, 117, 9, 50, 59, 52, 61, 50, 59, 51, 120, 38, 38]),
            from: dec(&[27, 41, 51, 59, 117, 9, 50, 59, 52, 61, 50, 59, 51]),
            to: dec(&[31, 47, 40, 53, 42, 63, 117, 22, 53, 52, 62, 53, 52]),
        },
        PatchRule {
            id: "TZ_URUMQI",
            group: "timezone",
            desc: "Asia/Urumqi",
            find: dec(&[103, 103, 103, 120, 27, 41, 51, 59, 117, 15, 40, 47, 55, 43, 51, 120]),
            from: dec(&[27, 41, 51, 59, 117, 15, 40, 47, 55, 43, 51]),
            to: dec(&[31, 47, 40, 53, 42, 63, 117, 21, 41, 54, 53]),
        },
        PatchRule {
            id: "DOMAIN_LIST",
            group: "domain",
            desc: "Domain List",
            find: dec(&[21, 30, 12, 105, 17, 30, 53, 107, 23, 25, 110, 108, 23, 52, 15, 110, 20, 30, 0, 105]),
            from: dec(&[21, 30, 12, 105, 17, 30, 53, 107, 23, 25, 110, 108, 23, 52, 15, 110, 20, 30, 0, 105]),
            to: dec(&[2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]),
        },
        PatchRule {
            id: "LAB_KEYWORDS",
            group: "lab",
            desc: "Lab Keywords",
            find: dec(&[10, 32, 110, 113, 17, 35, 61, 113, 10, 48, 24, 105, 20, 48, 11, 106, 20, 9, 61, 32]),
            from: dec(&[10, 32, 110, 113, 17, 35, 61, 113, 10, 48, 24, 105, 20, 48, 11, 106, 20, 9, 61, 32]),
            to: dec(&[2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]),
        },
        PatchRule {
            id: "STEG_2019",
            group: "steg",
            desc: "Steg U+2019",
            find: dec(&[40, 63, 46, 47, 40, 52, 120, 6, 47, 104, 106, 107, 99, 120]),
            from: dec(&[6, 47, 104, 106, 107, 99]),
            to: dec(&[6, 47, 106, 106, 104, 109]),
        },
        PatchRule {
            id: "STEG_02BC",
            group: "steg",
            desc: "Steg U+02BC",
            find: dec(&[40, 63, 46, 47, 40, 52, 120, 6, 47, 106, 104, 24, 25, 120]),
            from: dec(&[6, 47, 106, 104, 24, 25]),
            to: dec(&[6, 47, 106, 106, 104, 109]),
        },
        PatchRule {
            id: "STEG_02B9",
            group: "steg",
            desc: "Steg U+02B9",
            find: dec(&[40, 63, 46, 47, 40, 52, 120, 6, 47, 106, 104, 24, 99, 120]),
            from: dec(&[6, 47, 106, 104, 24, 99]),
            to: dec(&[6, 47, 106, 106, 104, 109]),
        },
    ]
}

// ============================================================
//  UI Output Helpers
// ============================================================
fn ok(msg: &str) { println!("    {}  {}", s_green() + ICON_OK + &s_r(), msg); }
fn fail(msg: &str) { println!("    {}  {}", s_red() + ICON_FAIL + &s_r(), msg); }
fn warn(msg: &str) { println!("    {}  {}", s_yellow() + ICON_WARN + &s_r(), msg); }
fn info(msg: &str) { println!("    {}  {}", s_cyan() + ICON_INFO + &s_r(), msg); }

// ============================================================
//  Subarray search utility
// ============================================================
fn find_all_occurrences(buf: &[u8], pat: &[u8]) -> Vec<usize> {
    let mut indices = Vec::new();
    if pat.is_empty() || buf.len() < pat.len() {
        return indices;
    }
    for i in 0..=(buf.len() - pat.len()) {
        if &buf[i..i + pat.len()] == pat {
            indices.push(i);
        }
    }
    indices
}

fn try_exec(cmd: &str) -> String {
    let shell = if cfg!(windows) { "cmd" } else { "sh" };
    let flag = if cfg!(windows) { "/C" } else { "-c" };
    match Command::new(shell).args(&[flag, cmd]).output() {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        Err(_) => "".to_string(),
    }
}

// ============================================================
//  Target Finder
// ============================================================
fn find_installed_targets() -> Vec<PathBuf> {
    let mut search_dirs = Vec::new();
    let mut paths_found = Vec::new();

    if let Ok(prefix) = env::var("npm_config_prefix") {
        search_dirs.push(PathBuf::from(&prefix).join("lib/node_modules"));
        search_dirs.push(PathBuf::from(&prefix).join("node_modules"));
    }
    
    let npm_global = try_exec("npm config get prefix");
    if !npm_global.is_empty() {
        search_dirs.push(PathBuf::from(&npm_global).join("lib/node_modules"));
        search_dirs.push(PathBuf::from(&npm_global).join("node_modules"));
    }
    
    let npm_root = try_exec("npm root -g");
    if !npm_root.is_empty() {
        search_dirs.push(PathBuf::from(&npm_root));
    }

    let yarn_global = try_exec("yarn global dir");
    if !yarn_global.is_empty() {
        search_dirs.push(PathBuf::from(&yarn_global).join("node_modules"));
    }

    let pnpm_global = try_exec("pnpm root -g");
    if !pnpm_global.is_empty() {
        search_dirs.push(PathBuf::from(&pnpm_global));
    }

    if let Ok(home) = env::var("HOME") {
        if !cfg!(windows) {
            let nvm_dir = PathBuf::from(&home).join(".nvm/versions/node");
            if let Ok(entries) = fs::read_dir(nvm_dir) {
                for entry in entries.flatten() {
                    search_dirs.push(entry.path().join("lib/node_modules"));
                }
            }
            search_dirs.push(PathBuf::from("/usr/local/lib/node_modules"));
            search_dirs.push(PathBuf::from("/usr/lib/node_modules"));
            search_dirs.push(PathBuf::from(&home).join(".npm-global/lib/node_modules"));
            search_dirs.push(PathBuf::from(&home).join(".local/lib/node_modules"));
        }
    }

    if cfg!(windows) {
        if let Ok(app_data) = env::var("APPDATA") {
            search_dirs.push(PathBuf::from(app_data).join("npm/node_modules"));
        }
        if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
            search_dirs.push(PathBuf::from(local_app_data).join("npm/node_modules"));
        }
    }

    let which_cmd = if cfg!(windows) { "where claude" } else { "which claude" };
    let which_output = try_exec(which_cmd);
    for line in which_output.lines() {
        let p = line.trim();
        if !p.is_empty() {
            if let Ok(real) = fs::canonicalize(p) {
                paths_found.push(real.clone());
                if let Some(parent) = real.parent() {
                    search_dirs.push(parent.join("../lib/node_modules"));
                    search_dirs.push(parent.join("node_modules"));
                }
            }
        }
    }

    if let Ok(bin_env) = env::var("CLAUDE_BIN") {
        if let Ok(real) = fs::canonicalize(&bin_env) {
            paths_found.push(real);
        }
    }

    let target_subpaths = [
        "@anthropic-ai/claude-code/cli.js",
        "@anthropic-ai/claude-code/claude",
        "@anthropic-ai/claude-code/claude.exe",
        "@anthropic-ai/claude-code/bin/cli.js",
        "@anthropic-ai/claude-code/bin/claude",
        "@anthropic-ai/claude-code/bin/claude.exe",
    ];

    for dir in search_dirs {
        for sub in &target_subpaths {
            let full = dir.join(sub);
            if full.exists() {
                if let Ok(real) = fs::canonicalize(full) {
                    paths_found.push(real);
                }
            }
        }
    }

    // Include temporary testing paths
    for t_path in &[
        "/tmp/claude-code-audit.SmB2iB/package/claude",
        "/tmp/latest-claude/package/claude",
        "/tmp/latest-claude/darwin-test/package/claude",
    ] {
        let path = PathBuf::from(t_path);
        if path.exists() {
            if let Ok(real) = fs::canonicalize(path) {
                paths_found.push(real);
            }
        }
    }

    paths_found.sort();
    paths_found.dedup();

    paths_found
        .into_iter()
        .filter(|p| {
            if let Ok(meta) = fs::metadata(p) {
                meta.is_file() && meta.len() > 500_000
            } else {
                false
            }
        })
        .collect()
}

// ============================================================
//  File Checker & Patcher Logic
// ============================================================
#[allow(dead_code)]
struct ScanResult {
    active: usize,
    patched: usize,
    details: Vec<(String, String, usize)>, // (id, status, count)
    buf: Vec<u8>,
}

fn check_file(file_path: &Path) -> Result<ScanResult, String> {
    let mut buf = Vec::new();
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    file.read_to_end(&mut buf).map_err(|e| e.to_string())?;

    let mut active = 0;
    let mut patched = 0;
    let mut details = Vec::new();

    for p in get_patches() {
        let active_count = find_all_occurrences(&buf, &p.find).len();
        let patched_count = find_all_occurrences(&buf, &p.to).len();

        if active_count > 0 {
            active += active_count;
            details.push((p.id.to_string(), "active".to_string(), active_count));
        } else if patched_count > 0 {
            patched += patched_count;
            details.push((p.id.to_string(), "patched".to_string(), patched_count));
        } else {
            details.push((p.id.to_string(), "missing".to_string(), 0));
        }
    }

    Ok(ScanResult {
        active,
        patched,
        details,
        buf,
    })
}

fn patch_file(file_path: &Path, mut buf: Vec<u8>) -> Result<usize, String> {
    let backup_path = file_path.with_extension("backup");
    if !backup_path.exists() {
        fs::copy(file_path, &backup_path).map_err(|e| format!("Backup creation failed: {}", e))?;
    }

    let mut repaired_count = 0;
    for p in get_patches() {
        let locs = find_all_occurrences(&buf, &p.find);
        if locs.is_empty() {
            continue;
        }

        for loc in locs {
            // Locate "from" slice inside the found chunk and overwrite with "to" slice
            let chunk = &mut buf[loc..loc + p.find.len()];
            if let Some(pos) = chunk.windows(p.from.len()).position(|w| w == p.from) {
                chunk[pos..pos + p.to.len()].copy_from_slice(&p.to);
                repaired_count += 1;
            }
        }
    }

    if repaired_count > 0 {
        fs::write(file_path, &buf).map_err(|e| {
            if e.kind() == io::ErrorKind::PermissionDenied {
                "Permission Denied. Please run with sudo / Administrator rights.".to_string()
            } else {
                e.to_string()
            }
        })?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(file_path, fs::Permissions::from_mode(0o755));
        }
    }

    Ok(repaired_count)
}

fn restore_file(file_path: &Path) -> bool {
    let backup_path = file_path.with_extension("backup");
    if !backup_path.exists() {
        return false;
    }
    if fs::copy(&backup_path, file_path).is_ok() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(file_path, fs::Permissions::from_mode(0o755));
        }
        true
    } else {
        false
    }
}

// ============================================================
//  Alias Installer
// ============================================================
fn install_alias_globally() {
    info(&get_msg("installAlias"));

    let script_path = match env::current_exe() {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(e) => {
            fail(&format!("Failed to locate self path: {}", e));
            return;
        }
    };

    if cfg!(windows) {
        let npm_path = env::var("APPDATA")
            .map(|a| PathBuf::from(a).join("npm"))
            .unwrap_or_else(|_| PathBuf::from("C:\\Windows"));

        let bat_path = npm_path.join("claude-shield.bat");
        let bat_content = format!("@echo off\r\nset TZ=UTC\r\nset LANG=en_US.UTF-8\r\nset LC_ALL=en_US.UTF-8\r\n\"{}\" %*\r\n", script_path);

        match fs::write(&bat_path, &bat_content) {
            Ok(_) => {
                let _ = fs::write(
                    npm_path.join("CLAUDE-SHIELD.bat"),
                    &bat_content,
                );
                ok(&get_msg("aliasWinOk"));
            }
            Err(e) => fail(&format!("Failed to write BAT helper: {}", e)),
        }
    } else {
        let shells = [
            (".zshrc", "zsh"),
            (".bashrc", "bash"),
        ];

        let mut installed_count = 0;
        let home = env::var("HOME").unwrap_or_default();

        for (rc_file, _) in &shells {
            let config_path = PathBuf::from(&home).join(rc_file);
            if config_path.exists() {
                let mut content = fs::read_to_string(&config_path).unwrap_or_default();

                const MARKER_START: &str = "# >>> CLAUDE SHIELD START >>>";
                const MARKER_END: &str = "# <<< CLAUDE SHIELD END <<<";

                let alias_block = format!(
                    "{}\nclaude() {{\n  if [[ \"${{1:l}}\" == \"shield\" ]]; then\n    \"{}\" \"${{@:2}}\"\n  else\n    TZ=UTC LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 command claude \"$@\"\n  fi\n}}\nalias \"claude-shield\"=\"\\\"{}\\\"\"\nalias \"CLAUDE-SHIELD\"=\"\\\"{}\\\"\"\nalias -g \"CLAUDE SHIELD\"=\"\\\"{}\\\"\"\n{}",
                    MARKER_START, script_path, script_path, script_path, script_path, MARKER_END
                );

                if content.contains(MARKER_START) {
                    if let Some(start_idx) = content.find(MARKER_START) {
                        if let Some(end_idx) = content.find(MARKER_END) {
                            content.replace_range(start_idx..end_idx + MARKER_END.len(), &alias_block);
                        }
                    }
                } else {
                    content.push_str(&format!("\n\n{}\n", alias_block));
                }

                match fs::write(&config_path, content) {
                    Ok(_) => {
                        ok(&format!("{} ({})", get_msg("aliasOk"), rc_file));
                        println!(
                            "    {}{}{}",
                            s_dim(),
                            get_msg("aliasTip").replace("{file}", rc_file),
                            s_r()
                        );
                        installed_count += 1;
                    }
                    Err(e) => fail(&format!("Could not write to {}: {}", rc_file, e)),
                }
            }
        }

        if installed_count == 0 {
            warn("No shell config files (.zshrc / .bashrc) found. Cannot auto-install alias.");
        }
    }
}

// ============================================================
//  Daemon Background Spawning
// ============================================================
fn start_daemon_in_background() {
    let script_path = match env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            fail(&format!("Failed to resolve executable path: {}", e));
            return;
        }
    };

    let pid_file = get_pid_file();
    if pid_file.exists() {
        if let Ok(pid_str) = fs::read_to_string(&pid_file) {
            if let Ok(pid) = pid_str.trim().parse::<u32>() {
                // Check if PID actually exists by sending signal 0
                #[cfg(unix)]
                {
                    if Command::new("kill").args(&["-0", &pid.to_string()]).stdout(Stdio::null()).stderr(Stdio::null()).status().map(|s| s.success()).unwrap_or(false) {
                        warn(&format!("Daemon is already running (PID: {}).", pid));
                        return;
                    }
                }
                #[cfg(windows)]
                {
                    if Command::new("tasklist").arg("/FI").arg(format!("PID eq {}", pid)).stdout(Stdio::null()).stderr(Stdio::null()).status().map(|s| s.success()).unwrap_or(false) {
                        warn(&format!("Daemon is already running (PID: {}).", pid));
                        return;
                    }
                }
            }
        }
    }

    info(&get_msg("daemonStart"));

    let dir = get_shield_dir();
    let _ = fs::create_dir_all(&dir);
    let log_path = get_log_file();
    let _ = fs::write(&log_path, "");

    // Detach and redirect logs
    let out_file = File::create(&log_path).unwrap();
    let err_file = File::create(&log_path).unwrap();

    let child = Command::new(script_path)
        .arg("daemon")
        .stdin(Stdio::null())
        .stdout(Stdio::from(out_file))
        .stderr(Stdio::from(err_file))
        .spawn();

    match child {
        Ok(c) => {
            let pid = c.id();
            let _ = fs::write(&pid_file, pid.to_string());
            write_log(&format!("[SHIELD] Daemon started in background. PID: {}", pid));
            ok(&format!("Daemon successfully started in background! PID: {}", pid));
            info(&format!("Runtime log is saved to: {}", log_path.display()));
        }
        Err(e) => fail(&format!("Failed to spawn background daemon: {}", e)),
    }
}

fn stop_daemon() {
    let pid_file = get_pid_file();
    if !pid_file.exists() {
        fail("No running daemon PID file discovered.");
        return;
    }

    let pid_str = fs::read_to_string(&pid_file).unwrap_or_default();
    let pid = pid_str.trim().to_string();

    if pid.is_empty() {
        fail("PID file is empty.");
        return;
    }

    #[cfg(unix)]
    {
        match Command::new("kill").args(&[&pid]).status() {
            Ok(s) if s.success() => {
                ok(&format!("Stop signal sent to daemon (PID: {}).", pid));
            }
            _ => {
                warn("Daemon process not found. Cleaning up stale PID file...");
            }
        }
    }
    #[cfg(windows)]
    {
        match Command::new("taskkill").args(&["/F", "/PID", &pid]).status() {
            Ok(s) if s.success() => {
                ok(&format!("Daemon process stopped (PID: {}).", pid));
            }
            _ => {
                warn("Daemon process not found. Cleaning up stale PID file...");
            }
        }
    }

    let _ = fs::remove_file(pid_file);
}

fn show_daemon_status() {
    let targets = find_installed_targets();
    println!("\n+--- {} ----------------------+", get_msg("cmdStatus"));

    let pid_file = get_pid_file();
    let mut running = false;
    let mut pid_val = None;

    if pid_file.exists() {
        if let Ok(pid_str) = fs::read_to_string(&pid_file) {
            if let Ok(pid) = pid_str.trim().parse::<u32>() {
                #[cfg(unix)]
                {
                    running = Command::new("kill").args(&["-0", &pid.to_string()]).stdout(Stdio::null()).stderr(Stdio::null()).status().map(|s| s.success()).unwrap_or(false);
                }
                #[cfg(windows)]
                {
                    running = Command::new("tasklist").arg("/FI").arg(format!("PID eq {}", pid)).stdout(Stdio::null()).stderr(Stdio::null()).status().map(|s| s.success()).unwrap_or(false);
                }
                if running {
                    pid_val = Some(pid);
                }
            }
        }
    }

    let status_str = if running {
        format!("{}[ACTIVE] Running{}", s_green(), s_r())
    } else {
        format!("{}[INACTIVE] Stopped{}", s_red(), s_r())
    };

    println!("  Status:     {}", status_str);
    if let Some(p) = pid_val {
        println!("  PID:        {}", p);
    }
    println!("  Log File:   {}", get_log_file().display());
    println!("  Monitored:  {}", targets.len());
    for (i, t) in targets.iter().enumerate() {
        println!("    [{}] {}", i + 1, t.display());
    }
    println!("+----------------------------------------------------+");
}

fn print_logs() {
    let log_path = get_log_file();
    if !log_path.exists() {
        warn("No logs available.");
        return;
    }
    if let Ok(file) = File::open(log_path) {
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().flatten().collect();
        println!("\n=== Last 50 Daemon Log Entries ===");
        let start = if lines.len() > 50 { lines.len() - 50 } else { 0 };
        for line in &lines[start..] {
            println!("{}", line);
        }
    }
}

// ============================================================
//  UI Presentation Banner & Help
// ============================================================
fn show_banner() {
    println!("");
    println!("{}   ______  __                 __      _____ __    _      __    {}", s_cyan(), s_r());
    println!("{}  / ____/ / /  ____ _ __  __ / /_    / ___// /_  (_)___ / /_____{}", s_cyan(), s_r());
    println!("{} / /     / /  / __ `// / / // __ \\   \\__ \\/ __ \\/ / __ \\/ / __  /{}", s_cyan(), s_r());
    println!("{}/ /___  / /__/ /_/ // /_/ // /_/ /  ___/ / / / / / /___/ / /_/ / {}", s_cyan(), s_r());
    println!("{}\\____/ /____/\\__,_/ \\__,_//_.___/  /____/_/ /_/_/\\____/_/\\____/  {}", s_cyan(), s_r());
    println!("");
    println!("                    {}{}{}", s_bg_cyan() + &s_white() + &s_bold(), get_msg("bannerSub"), s_r());
    println!("");
}

fn show_help() {
    println!("{}", get_msg("helpTitle"));
    println!("{}", get_msg("helpUsage"));
    println!("");
    println!("{}", get_msg("helpCmds"));
    println!("  {}  {}", s_cyan() + &format!("{:<18}", "scan") + &s_r(), get_msg("cmdScan"));
    println!("  {}  {}", s_cyan() + &format!("{:<18}", "patch") + &s_r(), get_msg("cmdPatch"));
    println!("  {}  {}", s_cyan() + &format!("{:<18}", "restore") + &s_r(), get_msg("cmdRestore"));
    println!("  {}  {}", s_cyan() + &format!("{:<18}", "daemon") + &s_r(), get_msg("cmdDaemon"));
    println!("  {}  {}", s_cyan() + &format!("{:<18}", "start") + &s_r(), get_msg("cmdStart"));
    println!("  {}  {}", s_cyan() + &format!("{:<18}", "stop") + &s_r(), get_msg("cmdStop"));
    println!("  {}  {}", s_cyan() + &format!("{:<18}", "status") + &s_r(), get_msg("cmdStatus"));
    println!("  {}  {}", s_cyan() + &format!("{:<18}", "logs") + &s_r(), get_msg("cmdLogs"));
    println!("  {}  {}", s_cyan() + &format!("{:<18}", "install") + &s_r(), get_msg("cmdAlias"));
    println!("");
    println!("{}", get_msg("helpAdv"));
    println!("  {} {}", ICON_OK, get_msg("adv1"));
    println!("  {} {}", ICON_OK, get_msg("adv2"));
    println!("  {} {}", ICON_OK, get_msg("adv3"));
    println!("");
}

fn read_version(bin_path: &Path) -> String {
    if let Some(parent) = bin_path.parent() {
        let p1 = parent.join("package.json");
        if p1.exists() {
            if let Ok(c) = fs::read_to_string(p1) {
                if let Some(v_idx) = c.find("\"version\"") {
                    if let Some(sub) = c.get(v_idx..) {
                        let parts: Vec<&str> = sub.split('"').collect();
                        if parts.len() >= 4 {
                            return parts[3].to_string();
                        }
                    }
                }
            }
        }
        let p2 = parent.join("../package.json");
        if p2.exists() {
            if let Ok(c) = fs::read_to_string(p2) {
                if let Some(v_idx) = c.find("\"version\"") {
                    if let Some(sub) = c.get(v_idx..) {
                        let parts: Vec<&str> = sub.split('"').collect();
                        if parts.len() >= 4 {
                            return parts[3].to_string();
                        }
                    }
                }
            }
        }
    }
    "unknown".to_string()
}

fn show_status_panel(scan: &ScanResult, version: &str, tz_protected: bool) {
    let mut timezone_a = 0;
    let mut timezone_p = 0;
    let mut domain_a = 0;
    let mut domain_p = 0;
    let mut lab_a = 0;
    let mut lab_p = 0;
    let mut steg_a = 0;
    let mut steg_p = 0;

    for (id, status, count) in &scan.details {
        if id.starts_with("TZ_") {
            if status == "active" { timezone_a += count; }
            else if status == "patched" { timezone_p += count; }
        } else if id == "DOMAIN_LIST" {
            if status == "active" { domain_a += count; }
            else if status == "patched" { domain_p += count; }
        } else if id == "LAB_KEYWORDS" {
            if status == "active" { lab_a += count; }
            else if status == "patched" { lab_p += count; }
        } else if id.starts_with("STEG_") {
            if status == "active" { steg_a += count; }
            else if status == "patched" { steg_p += count; }
        }
    }

    println!("");
    println!("  {}{}", s_cyan(), get_msg("panelTitle"));

    let ver_line = format!("  Claude Code    {}v{}{}", s_bold(), version, s_r());
    println!("  {}│{} {:<50} {}│", s_cyan(), s_r(), ver_line, s_cyan());

    let tz_icon = if tz_protected { s_green() + ICON_OK + &s_r() } else { s_red() + ICON_FAIL + &s_r() };
    let orig_tz = ORIGINAL_TZ.get().map(|s| s.as_str()).unwrap_or("UTC");
    let tz_text = if tz_protected { format!("UTC{}{})", get_msg("panelOriginal"), orig_tz) } else { orig_tz.to_string() };
    let tz_line = format!("{}{}{}  {}", get_msg("panelTz"), s_r(), tz_icon, tz_text);
    println!("  {}│{} {:<62} {}│", s_cyan(), s_r(), tz_line, s_cyan());

    let mut proxy_val = env::var("https_proxy")
        .or_else(|_| env::var("HTTPS_PROXY"))
        .or_else(|_| env::var("all_proxy"))
        .or_else(|_| env::var("ALL_PROXY"))
        .or_else(|_| env::var("http_proxy"))
        .or_else(|_| env::var("HTTP_PROXY"))
        .unwrap_or_default();

    let (proxy_icon, proxy_text) = if !proxy_val.is_empty() {
        if proxy_val.len() > 30 {
            proxy_val.truncate(27);
            proxy_val.push_str("...");
        }
        (s_green() + ICON_OK + &s_r(), format!("{} (Active)", proxy_val))
    } else {
        (s_red() + ICON_FAIL + &s_r(), s_red() + &get_msg("panelProxyWarn") + &s_r())
    };

    let proxy_line = format!("{}{}{}  {}", get_msg("panelProxy"), s_r(), proxy_icon, proxy_text);
    println!("  {}│{} {:<62} {}│", s_cyan(), s_r(), proxy_line, s_cyan());

    let categories = [
        (get_msg("panelTzDetect"), timezone_a, timezone_p),
        (get_msg("panelDomain"), domain_a, domain_p),
        (get_msg("panelLab"), lab_a, lab_p),
        (get_msg("panelSteg"), steg_a, steg_p),
    ];

    for (label, a, p) in &categories {
        let (icon, text) = if *a == 0 && *p > 0 {
            (s_green() + ICON_OK + &s_r(), s_green() + "Shielded" + &s_r())
        } else if *a > 0 {
            (s_red() + ICON_FAIL + &s_r(), s_red() + &format!("Active ({})", a) + &s_r())
        } else {
            (s_yellow() + ICON_WARN + &s_r(), s_yellow() + "Not Found" + &s_r())
        };
        let line = format!("{:<18} {}  {}", label, icon, text);
        println!("  {}│{} {:<62} {}│", s_cyan(), s_r(), line, s_cyan());
    }

    println!("  {}{}", s_cyan(), get_msg("panelBottom"));
    if proxy_val.is_empty() {
        println!("  {}  {} Please set proxy env to shield real IP (e.g. export https_proxy=http://127.0.0.1:7890){}", s_yellow(), ICON_WARN, s_r());
    }
}

// ============================================================
//  Core Thread watcher (No poll optimization)
// ============================================================
fn run_daemon_sentinel(targets: Vec<PathBuf>) {
    write_log("[SHIELD] Sentinel daemon loaded in foreground watcher.");

    // Perform initial patching check
    for t in &targets {
        if let Ok(info) = check_file(t) {
            if info.active > 0 {
                match patch_file(t, info.buf) {
                    Ok(count) => write_log(&format!("[SHIELD] Auto patched target at launch: {} ({} segments)", t.display(), count)),
                    Err(e) => write_log(&format!("[SHIELD] Auto patching failed for {}: {}", t.display(), e)),
                }
            }
        }
    }

    let start_time = SystemTime::now();
    let mut file_states = std::collections::HashMap::new();

    for t in &targets {
        if let Ok(meta) = fs::metadata(t) {
            if let Ok(mtime) = meta.modified() {
                let len = meta.len();
                file_states.insert(t.clone(), (mtime, len));
            }
        }
    }

    write_log(&format!("[SHIELD] Watching {} targets. File sentinel active.", targets.len()));

    // Periodic heartbeat (every 60 seconds)
    let heartbeat_targets = targets.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60));
            let elapsed = SystemTime::now().duration_since(start_time).unwrap_or_default().as_secs();
            write_log(&format!("[SHIELD] Heartbeat check. Elapsed: {}s. Active targets: {}", elapsed, heartbeat_targets.len()));
        }
    });

    // Main low-overhead loop to watch file metadata updates
    loop {
        thread::sleep(Duration::from_secs(4));

        for t in &targets {
            if let Ok(meta) = fs::metadata(t) {
                if let Ok(mtime) = meta.modified() {
                    let len = meta.len();
                    if let Some(&(prev_mtime, prev_len)) = file_states.get(t) {
                        if mtime != prev_mtime || len != prev_len {
                            file_states.insert(t.clone(), (mtime, len));
                            write_log(&format!("[SHIELD] File change detected: {}", t.display()));

                            // Allow some buffer for compiler/writer processes to lock/release
                            thread::sleep(Duration::from_millis(800));
                            if let Ok(info) = check_file(t) {
                                if info.active > 0 {
                                    write_log(&format!("[SHIELD] Active detection found in updated file, applying hot-patch: {}", t.display()));
                                    match patch_file(t, info.buf) {
                                        Ok(count) => write_log(&format!("[SHIELD] Hot-patch successfully reapplied: {} ({} segments)", t.display(), count)),
                                        Err(e) => write_log(&format!("[SHIELD] Hot-patch reapply failed: {}", e)),
                                    }
                                    if let Ok(new_meta) = fs::metadata(t) {
                                        if let Ok(nmtime) = new_meta.modified() {
                                            let nlen = new_meta.len();
                                            file_states.insert(t.clone(), (nmtime, nlen));
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        file_states.insert(t.clone(), (mtime, len));
                    }
                }
            }
        }
    }
}

// ============================================================
//  CLI Entry Point
// ============================================================
fn main() {
    let tz = env::var("TZ").unwrap_or_else(|_| {
        fs::read_to_string("/etc/timezone")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "UTC".to_string())
    });
    ORIGINAL_TZ.set(tz).ok();
    env::set_var("TZ", "UTC");

    let args: Vec<String> = env::args().collect();
    let cmd = if args.len() > 1 { args[1].to_lowercase() } else { "".to_string() };

    // Localization check
    let sys_lang = env::var("LANG").unwrap_or_default() + &env::var("LC_ALL").unwrap_or_default();
    if sys_lang.to_lowercase().contains("zh") {
        unsafe { LANG_ZH = true; }
    }
    if let Ok(env_lang) = env::var("CLAUDE_SHIELD_LANG") {
        if env_lang.to_lowercase().starts_with("zh") {
            unsafe { LANG_ZH = true; }
        } else {
            unsafe { LANG_ZH = false; }
        }
    }

    // Checking color configuration
    if env::var("NO_COLOR").is_ok() || try_exec("tty").contains("not a tty") {
        unsafe { NO_COLOR = true; }
    }

    if cmd != "daemon" {
        show_banner();
    }

    if cmd.is_empty() || cmd == "--help" || cmd == "-h" {
        show_help();
        return;
    }

    if cmd != "daemon" {
        println!("  {}{}{}", s_dim(), get_msg("tzActive") + &env::var("TZ").unwrap_or_default(), s_r());
    }

    match cmd.as_str() {
        "install" => {
            install_alias_globally();
        }
        "start" => {
            start_daemon_in_background();
        }
        "stop" => {
            stop_daemon();
        }
        "status" => {
            show_daemon_status();
        }
        "logs" => {
            print_logs();
        }
        "scan" => {
            println!("\n  {}{}{}", s_bold(), get_msg("scanning"), s_r());
            let targets = find_installed_targets();

            if targets.is_empty() {
                warn("No installed instances of Claude Code found in the system.");
                return;
            }

            println!("  {}\n", get_msg("foundTargets").replace("{n}", &targets.len().to_string()));
            for (idx, t) in targets.iter().enumerate() {
                println!("  {}[{}]{}  {}{}{}", s_bold(), idx + 1, s_r(), s_cyan(), t.display(), s_r());
                match check_file(t) {
                    Ok(info_file) => {
                        if info_file.active > 0 {
                            println!("      Status: {}{}{} (v{})", s_red(), get_msg("statusActive").replace("{n}", &info_file.active.to_string()), s_r(), read_version(t));
                        } else if info_file.patched > 0 {
                            println!("      Status: {}{}{}", s_green(), get_msg("statusPatched"), s_r());
                        } else {
                            println!("      Status: {}{}{}", s_yellow(), get_msg("statusMissing"), s_r());
                        }
                        show_status_panel(&info_file, &read_version(t), env::var("TZ").map(|v| v == "UTC").unwrap_or(false));
                    }
                    Err(e) => {
                        println!("      Status: {}{} ({}){}", s_red(), get_msg("statusErr"), e, s_r());
                    }
                }
                println!("");
            }
        }
        "patch" => {
            println!("\n  {}{}{}", s_bold(), get_msg("patching"), s_r());
            let targets = find_installed_targets();

            if targets.is_empty() {
                warn("No installation paths detected.");
                return;
            }

            let mut success = 0;
            for t in &targets {
                println!("\n  {}{}{}", get_msg("checking"), s_cyan(), t.display());
                match check_file(t) {
                    Ok(info_file) => {
                        if info_file.active == 0 {
                            if info_file.patched > 0 {
                                ok(&get_msg("patchedOk"));
                                success += 1;
                            } else {
                                warn(&get_msg("noFeature"));
                            }
                            continue;
                        }

                        match patch_file(t, info_file.buf) {
                            Ok(count) => {
                                ok(&get_msg("patchSuccess").replace("{n}", &count.to_string()));
                                success += 1;
                                if let Ok(refreshed_info) = check_file(t) {
                                    show_status_panel(&refreshed_info, &read_version(t), env::var("TZ").map(|v| v == "UTC").unwrap_or(false));
                                }
                            }
                            Err(e) => {
                                fail(&format!("Patch Failed: {}", e));
                                if e.contains("Permission") {
                                    println!("\n  {}Permission Denied. Try elevated command:{}", s_yellow(), s_r());
                                    println!("  {}sudo claude-shield patch{}\n", s_bold(), s_r());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        fail(&format!("Access Error: {}", e));
                    }
                }
            }

            println!("\n  {}", get_msg("summary").replace("{success}", &success.to_string()).replace("{total}", &targets.len().to_string()));
        }
        "restore" => {
            println!("\n  {}{}{}", s_bold(), get_msg("restoring"), s_r());
            let targets = find_installed_targets();

            if targets.is_empty() {
                warn("No installed paths found.");
                return;
            }

            let mut restored = 0;
            for t in &targets {
                if restore_file(t) {
                    ok(&format!("{}{}{}{}", get_msg("restoredOk"), s_cyan(), t.display(), s_r()));
                    restored += 1;
                } else {
                    warn(&format!("{}{}.backup{}", get_msg("noBackup"), s_dim(), s_r()));
                }
            }
            println!("\n  {}", get_msg("restoreSummary").replace("{n}", &restored.to_string()));
        }
        "daemon" => {
            let targets = find_installed_targets();
            if targets.is_empty() {
                write_log("[SHIELD] Error: No targets discovered for watching.");
                std::process::exit(1);
            }
            run_daemon_sentinel(targets);
        }
        _ => {
            warn(&format!("Unrecognized command \"{}\".\n", cmd));
            show_help();
        }
    }
}
