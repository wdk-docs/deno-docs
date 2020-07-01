// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use crate::op_error::OpError;

#[cfg(not(unix))]
const SIGINT: i32 = 2;
#[cfg(not(unix))]
const SIGKILL: i32 = 9;
#[cfg(not(unix))]
const SIGTERM: i32 = 15;

#[cfg(not(unix))]
use winapi::{
  shared::minwindef::DWORD,
  um::{
    handleapi::CloseHandle,
    processthreadsapi::{OpenProcess, TerminateProcess},
    winnt::PROCESS_TERMINATE,
  },
};

#[cfg(unix)]
pub fn kill(pid: i32, signo: i32) -> Result<(), OpError> {
  use nix::sys::signal::{kill as unix_kill, Signal};
  use nix::unistd::Pid;
  use std::convert::TryFrom;
  let sig = Signal::try_from(signo)?;
  unix_kill(Pid::from_raw(pid), Option::Some(sig)).map_err(OpError::from)
}

#[cfg(not(unix))]
pub fn kill(pid: i32, signal: i32) -> Result<(), OpError> {
  match signal {
    SIGINT | SIGKILL | SIGTERM => {
      if pid <= 0 {
        return Err(OpError::type_error("unsupported pid".to_string()));
      }
      unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid as DWORD);
        if handle.is_null() {
          return Err(OpError::from(std::io::Error::last_os_error()));
        }
        if TerminateProcess(handle, 1) == 0 {
          CloseHandle(handle);
          return Err(OpError::from(std::io::Error::last_os_error()));
        }
        if CloseHandle(handle) == 0 {
          return Err(OpError::from(std::io::Error::last_os_error()));
        }
      }
    }
    _ => {
      return Err(OpError::type_error("unsupported signal".to_string()));
    }
  }
  Ok(())
}
