use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, exit};
use std::thread;
use std::time::{Duration, Instant};

const UEFI_TARGET: &str = "x86_64-unknown-uefi";
const KERNEL_PACKAGE: &str = "kernel";
const KERNEL_BINARY_NAME: &str = "kernel.efi";
const EFI_BOOT_PATH: &str = "EFI/BOOT/BOOTX64.EFI";
const STARTUP_SCRIPT_PATH: &str = "startup.nsh";
const DEFAULT_MEMORY_MB: &str = "256M";
const DEFAULT_TEST_TIMEOUT_SECS: u64 = 10;
const SUCCESS_MARKER: &str = "rustos: hello from UEFI";

fn main() {
    let mut args = env::args_os();
    let _program = args.next();

    let Some(command) = args.next() else {
        print_help();
        return;
    };

    let result = match command.to_string_lossy().as_ref() {
        "check" => cmd_check(),
        "ci" => cmd_ci(),
        "fmt" => cmd_fmt(),
        "lint" => cmd_lint(),
        "run" => cmd_run(args.collect(), false),
        "test" => cmd_test(args.collect()),
        "test-qemu" => cmd_run(args.collect(), true),
        "test-unit" => cmd_test_unit(),
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        other => Err(format!("unknown xtask command: {other}")),
    };

    if let Err(message) = result {
        eprintln!("error: {message}");
        exit(1);
    }
}

fn cmd_check() -> Result<(), String> {
    run_command("cargo", ["check", "--workspace", "--all-targets"])
}

fn cmd_ci() -> Result<(), String> {
    cmd_fmt()?;
    cmd_lint()?;
    cmd_check()?;
    cmd_test_unit()?;
    run_command(
        "cargo",
        [
            "build",
            "--package",
            KERNEL_PACKAGE,
            "--target",
            UEFI_TARGET,
        ],
    )
}

fn cmd_fmt() -> Result<(), String> {
    run_command("cargo", ["fmt", "--all", "--", "--check"])
}

fn cmd_lint() -> Result<(), String> {
    run_command(
        "cargo",
        [
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )
}

fn cmd_run(extra_args: Vec<OsString>, bounded_test_mode: bool) -> Result<(), String> {
    ensure_command_available("qemu-system-x86_64")?;

    let kernel_efi = build_efi()?;
    let workspace_root = workspace_root()?;
    let artifacts_dir = workspace_root.join("artifacts");
    let image_dir = artifacts_dir.join("efi-root");
    recreate_directory(&image_dir)?;
    install_boot_file(&kernel_efi, &image_dir)?;
    install_startup_script(&image_dir)?;

    let firmware_code = find_firmware_code()?;
    let firmware_vars = prepare_firmware_vars(&artifacts_dir)?;

    if bounded_test_mode {
        run_qemu_bounded(&firmware_code, &firmware_vars, &image_dir, extra_args)
    } else {
        run_qemu(&firmware_code, &firmware_vars, &image_dir, extra_args)
    }
}

fn cmd_test(extra_args: Vec<OsString>) -> Result<(), String> {
    cmd_test_unit()?;
    cmd_run(extra_args, true)
}

fn cmd_test_unit() -> Result<(), String> {
    run_command(
        "cargo",
        ["test", "--workspace", "--exclude", KERNEL_PACKAGE],
    )
}

fn build_efi() -> Result<PathBuf, String> {
    run_command(
        "cargo",
        [
            "build",
            "--package",
            KERNEL_PACKAGE,
            "--target",
            UEFI_TARGET,
        ],
    )?;

    let path = workspace_root()?
        .join("target")
        .join(UEFI_TARGET)
        .join("debug")
        .join(KERNEL_BINARY_NAME);

    if path.is_file() {
        Ok(path)
    } else {
        Err(format!(
            "expected EFI binary was not found at {}",
            path.display()
        ))
    }
}

fn install_boot_file(kernel_efi: &Path, image_dir: &Path) -> Result<(), String> {
    let boot_path = image_dir.join(EFI_BOOT_PATH);
    let Some(parent) = boot_path.parent() else {
        return Err(String::from("failed to determine EFI boot directory"));
    };

    fs::create_dir_all(parent).map_err(|error| {
        format!(
            "failed to create EFI boot directory {}: {error}",
            parent.display()
        )
    })?;

    fs::copy(kernel_efi, &boot_path).map_err(|error| {
        format!(
            "failed to copy {} to {}: {error}",
            kernel_efi.display(),
            boot_path.display()
        )
    })?;

    Ok(())
}

fn install_startup_script(image_dir: &Path) -> Result<(), String> {
    let startup_script = image_dir.join(STARTUP_SCRIPT_PATH);
    let script_contents = "fs0:\r\nEFI\\BOOT\\BOOTX64.EFI\r\n";

    fs::write(&startup_script, script_contents).map_err(|error| {
        format!(
            "failed to write startup script {}: {error}",
            startup_script.display()
        )
    })
}

fn find_firmware_code() -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("RUSTOS_UEFI_CODE") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Ok(path);
        }

        return Err(format!(
            "RUSTOS_UEFI_CODE is set, but the file does not exist: {}",
            path.display()
        ));
    }

    let candidates = [
        "/opt/homebrew/share/qemu/edk2-x86_64-code.fd",
        "/usr/local/share/qemu/edk2-x86_64-code.fd",
        "/usr/share/qemu/edk2-x86_64-code.fd",
        "/usr/share/OVMF/OVMF_CODE.fd",
        "/usr/share/edk2/x64/OVMF_CODE.fd",
    ];

    find_first_existing_file(&candidates).ok_or_else(|| {
        String::from(
            "failed to find UEFI firmware code file. set RUSTOS_UEFI_CODE to the firmware code file path",
        )
    })
}

fn prepare_firmware_vars(artifacts_dir: &Path) -> Result<PathBuf, String> {
    let source = find_firmware_vars_source()?;
    let destination = artifacts_dir.join("OVMF_VARS.fd");

    if destination.exists() {
        return Ok(destination);
    }

    fs::create_dir_all(artifacts_dir).map_err(|error| {
        format!(
            "failed to create artifacts directory {}: {error}",
            artifacts_dir.display()
        )
    })?;

    fs::copy(&source, &destination).map_err(|error| {
        format!(
            "failed to copy firmware vars file from {} to {}: {error}",
            source.display(),
            destination.display()
        )
    })?;

    Ok(destination)
}

fn find_firmware_vars_source() -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("RUSTOS_UEFI_VARS") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Ok(path);
        }

        return Err(format!(
            "RUSTOS_UEFI_VARS is set, but the file does not exist: {}",
            path.display()
        ));
    }

    let candidates = [
        "/opt/homebrew/share/qemu/edk2-i386-vars.fd",
        "/usr/local/share/qemu/edk2-i386-vars.fd",
        "/usr/share/qemu/edk2-i386-vars.fd",
        "/usr/share/OVMF/OVMF_VARS.fd",
        "/usr/share/edk2/x64/OVMF_VARS.fd",
    ];

    find_first_existing_file(&candidates).ok_or_else(|| {
        String::from(
            "failed to find UEFI firmware vars file. set RUSTOS_UEFI_VARS to the firmware vars file path",
        )
    })
}

fn find_first_existing_file(candidates: &[&str]) -> Option<PathBuf> {
    candidates
        .iter()
        .map(PathBuf::from)
        .find(|path| path.is_file())
}

fn run_qemu(
    firmware_code: &Path,
    firmware_vars: &Path,
    image_dir: &Path,
    extra_args: Vec<OsString>,
) -> Result<(), String> {
    let mut command = qemu_command(firmware_code, firmware_vars, image_dir, extra_args);
    let status = command
        .status()
        .map_err(|error| format!("failed to start qemu-system-x86_64: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("qemu-system-x86_64 exited with status: {status}"))
    }
}

fn run_qemu_bounded(
    firmware_code: &Path,
    firmware_vars: &Path,
    image_dir: &Path,
    extra_args: Vec<OsString>,
) -> Result<(), String> {
    let timeout = env::var("RUSTOS_QEMU_TIMEOUT_SECS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_TEST_TIMEOUT_SECS);

    let mut command = qemu_command(firmware_code, firmware_vars, image_dir, extra_args);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .map_err(|error| format!("failed to start qemu-system-x86_64: {error}"))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| String::from("failed to capture qemu stdout"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| String::from("failed to capture qemu stderr"))?;

    let stdout_handle = thread::spawn(move || read_stream(stdout));
    let stderr_handle = thread::spawn(move || read_stream(stderr));

    let deadline = Instant::now() + Duration::from_secs(timeout);

    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("failed to wait for qemu-system-x86_64: {error}"))?
        {
            let stdout_output = stdout_handle
                .join()
                .map_err(|_| String::from("failed to join qemu stdout reader"))??;
            let stderr_output = stderr_handle
                .join()
                .map_err(|_| String::from("failed to join qemu stderr reader"))??;

            print!("{stdout_output}");
            eprint!("{stderr_output}");

            if stdout_output.contains(SUCCESS_MARKER) || stderr_output.contains(SUCCESS_MARKER) {
                return Ok(());
            }

            if status.success() {
                return Err(String::from(
                    "qemu exited before the expected boot marker was observed",
                ));
            }

            return Err(format!("qemu-system-x86_64 exited with status: {status}"));
        }

        if Instant::now() >= deadline {
            child
                .kill()
                .map_err(|error| format!("failed to stop qemu-system-x86_64: {error}"))?;
            let _ = child.wait();

            let stdout_output = stdout_handle
                .join()
                .map_err(|_| String::from("failed to join qemu stdout reader"))??;
            let stderr_output = stderr_handle
                .join()
                .map_err(|_| String::from("failed to join qemu stderr reader"))??;

            print!("{stdout_output}");
            eprint!("{stderr_output}");

            if stdout_output.contains(SUCCESS_MARKER) || stderr_output.contains(SUCCESS_MARKER) {
                return Ok(());
            }

            return Err(format!(
                "qemu timed out after {timeout} seconds before the expected boot marker was observed"
            ));
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn qemu_command(
    firmware_code: &Path,
    firmware_vars: &Path,
    image_dir: &Path,
    extra_args: Vec<OsString>,
) -> Command {
    let mut command = Command::new("qemu-system-x86_64");
    command.args([
        OsStr::new("-machine"),
        OsStr::new("q35"),
        OsStr::new("-m"),
        OsStr::new(DEFAULT_MEMORY_MB),
        OsStr::new("-serial"),
        OsStr::new("stdio"),
        OsStr::new("-display"),
        OsStr::new("none"),
    ]);

    command.arg("-drive");
    command.arg(format!(
        "if=pflash,format=raw,readonly=on,file={}",
        firmware_code.display()
    ));
    command.arg("-drive");
    command.arg(format!(
        "if=pflash,format=raw,file={}",
        firmware_vars.display()
    ));
    command.arg("-drive");
    command.arg(format!("format=raw,file=fat:rw:{}", image_dir.display()));
    command.args(extra_args);
    command
}

fn read_stream<R: Read>(mut reader: R) -> Result<String, String> {
    let mut output = String::new();
    reader
        .read_to_string(&mut output)
        .map_err(|error| format!("failed to read qemu output: {error}"))?;
    Ok(output)
}

fn recreate_directory(path: &Path) -> Result<(), String> {
    if path.exists() {
        fs::remove_dir_all(path)
            .map_err(|error| format!("failed to remove directory {}: {error}", path.display()))?;
    }

    fs::create_dir_all(path)
        .map_err(|error| format!("failed to create directory {}: {error}", path.display()))
}

fn ensure_command_available(program: &str) -> Result<(), String> {
    if command_exists(program) {
        Ok(())
    } else {
        Err(format!(
            "required command not found: {program}. install it and ensure it is available in PATH"
        ))
    }
}

fn command_exists(program: &str) -> bool {
    let Some(paths) = env::var_os("PATH") else {
        return false;
    };

    env::split_paths(&paths).any(|directory| executable_exists(&directory, program))
}

fn executable_exists(directory: &Path, program: &str) -> bool {
    let candidate = directory.join(program);
    if candidate.is_file() {
        return true;
    }

    #[cfg(windows)]
    {
        let candidate = directory.join(format!("{program}.exe"));
        candidate.is_file()
    }

    #[cfg(not(windows))]
    {
        false
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| String::from("failed to determine workspace root"))
}

fn run_command<I, S>(program: &str, args: I) -> Result<(), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|error| format!("failed to start {program}: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} exited with status: {status}"))
    }
}

fn print_help() {
    eprintln!("rustos xtask");
    eprintln!();
    eprintln!("usage:");
    eprintln!("  cargo run -p xtask -- <command>");
    eprintln!();
    eprintln!("commands:");
    eprintln!("  check      run cargo check for the workspace");
    eprintln!("  ci         run the CI-friendly local validation sequence");
    eprintln!("  fmt        check formatting with rustfmt");
    eprintln!("  lint       run clippy with warnings denied");
    eprintln!("  run        build the UEFI binary and launch it with qemu");
    eprintln!("  test       run unit tests, then run qemu in bounded test mode");
    eprintln!("  test-qemu  run qemu in bounded test mode and exit automatically");
    eprintln!("  test-unit  run host-side unit tests for workspace crates");
    eprintln!();
    eprintln!("run requirements:");
    eprintln!("  - qemu-system-x86_64 must be installed");
    eprintln!("  - UEFI firmware files must be installed");
    eprintln!("  - set RUSTOS_UEFI_CODE to override the firmware code file");
    eprintln!("  - set RUSTOS_UEFI_VARS to override the firmware vars file");
    eprintln!("  - set RUSTOS_QEMU_TIMEOUT_SECS to change the bounded test timeout");
}
