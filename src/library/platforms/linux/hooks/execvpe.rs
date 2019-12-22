use libc::{c_char, c_int};
use std::env;
use crate::platforms::linux;
use crate::common::whitelist;
use crate::common::event;

/*
       int execvpe(const char *path, char *const argv[],
                       char *const envp[]);
*/
hook! {
    unsafe fn hooked_execvpe(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int => execvpe {
		let (program, env) = linux::transform_parameters(path, envp, -1);
        let which_abs_pathbuf = match which::which_in(&program,
                                                      Some(linux::get_env_path()),
                                                      env::current_dir().unwrap()) {
            Err(_why) => {
				*linux::errno_location() = libc::ENOENT;
				return -1 },
            Ok(prog_path) => prog_path
        };
		let abs_prog_str = which_abs_pathbuf.to_str().unwrap();
		let (hexdigest, uid) = linux::get_hash_and_uid(abs_prog_str);
        // Permit/deny execution
        if whitelist::is_whitelisted(abs_prog_str, &env, &hexdigest) {
            event::send_exec_event(uid, abs_prog_str, &hexdigest, true);
            real!(hooked_execvpe)(path, argv, envp)
        } else {
            event::send_exec_event(uid, abs_prog_str, &hexdigest, false);
            *linux::errno_location() = libc::EACCES;
            return -1
        }
    }
}
