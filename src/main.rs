use std::{process, env::{self}, path::Path};

use nix::{unistd::{fork, ForkResult, chroot}, sys::{wait::wait}, sched::{unshare, CloneFlags}, env::clearenv, mount::{mount, MsFlags}};

const MOUNT_DIRS: [&str; 2] = ["lib", "lib64"];
const POD_DIR: &str = "./root";

fn handle_unshare() {
    let unshare_result = unshare(
        CloneFlags::CLONE_NEWUTS |
        CloneFlags::CLONE_NEWPID |
        CloneFlags::CLONE_NEWNS |
        CloneFlags::CLONE_NEWUSER |
        CloneFlags::CLONE_NEWIPC |
        CloneFlags::CLONE_NEWCGROUP
    );

    if let Err(e) = unshare_result {
        panic!("Unable to unshare: {}", e);
    }
}

fn clear_env() {
    unsafe {
        if let Err(e) = clearenv() {
            panic!("Unable to clear env: {}", e);
        }
    };
}

fn sandbox_filesystem() {
    if let Err(e) = chroot(POD_DIR) {
        panic!("Unable to change root: {}", e);
    }
    
    if let Err(e) = std::env::set_current_dir("/") {
        panic!("Unable to change current dir: {}", e);
    }
}

fn mount_libs() {
    let flags = MsFlags::MS_BIND | MsFlags::MS_RDONLY | MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOATIME | MsFlags::MS_NODIRATIME;
    let result: Result<Vec<()>, nix::Error> = MOUNT_DIRS.iter().map(|p| mount::<Path, Path, Path, Path>(Some(&Path::new("/").join(p)), &Path::new(POD_DIR).join(p), None, flags, None)).collect();

    if let Err(e) = result {
        panic!("Unable to mount: {}", e);
    }
}

fn exec_wrappper(args: &[String]) {
    clear_env();
    sandbox_filesystem();

    let c = args.get(1).unwrap();
    let err = exec::execvp(c, &args[1..]);
    println!("ERR {}", err);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing arg");
        return;
    }

    mount_libs();
    handle_unshare();
    let fork_result = fork();

    if let Ok(r) = fork_result {
        match r {
            ForkResult::Parent { child } => println!("\n\n[CHILD PID] {}:\n", child),
            ForkResult::Child => {
                println!("\n\n[CHILD RUNNING] {}:\n", process::id());
                exec_wrappper(&args);
            },
        }
    } else {
        println!("Unable to fork");
        return;
    }

    match wait() {
        Err (_) => println!("\n\nWait failed"),  
        Ok (status) => println!("\n\nWait {:?}", status),
    };

    println!("\n\n[Parent finished]");
    // TODO: Unmount libs?
}
