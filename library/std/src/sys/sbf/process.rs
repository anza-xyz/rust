use crate::ffi::{CString, OsStr};
use crate::fmt;
use crate::io;
use crate::num::NonZeroI32;
use crate::path::Path;
use crate::sys::fs::File;
use crate::sys::pipe::AnonPipe;
use crate::sys::{unsupported, Void};
use crate::sys_common::process::{CommandEnv, CommandEnvs};

pub use crate::ffi::OsString as EnvKey;

////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

pub struct Command {
    env: CommandEnv,
    args: Vec<CString>,
}

// passed back to std::process with the pipes connected to the child, if any
// were requested
pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
}

impl Command {
    pub fn new(_program: &OsStr) -> Command {
        Command {
            env: Default::default(),
            args: vec![CString::new("").unwrap()]
        }
    }

    pub fn arg(&mut self, _arg: &OsStr) {
    }

    pub fn env_mut(&mut self) -> &mut CommandEnv {
        &mut self.env
    }

    pub fn cwd(&mut self, _dir: &OsStr) {
    }

    pub fn stdin(&mut self, _stdin: Stdio) {
    }

    pub fn stdout(&mut self, _stdout: Stdio) {
    }

    pub fn stderr(&mut self, _stderr: Stdio) {
    }

    pub fn spawn(&mut self, _default: Stdio, _needs_stdin: bool)
        -> io::Result<(Process, StdioPipes)> {
        unsupported()
    }

    pub fn output(&mut self) -> io::Result<(ExitStatus, Vec<u8>, Vec<u8>)> {
        unsupported()
    }

    pub fn get_args(&self) -> CommandArgs<'_> {
        let mut iter = self.args.iter();
        iter.next();
        CommandArgs { iter }
    }

    pub fn get_envs(&self) -> CommandEnvs<'_> {
        self.env.iter()
    }

    pub fn get_current_dir(&self) -> Option<&Path> {
        Some(Path::new(OsStr::new("")))
    }

    pub fn get_program(&self) -> &OsStr {
        OsStr::new("")
    }
}

impl From<AnonPipe> for Stdio {
    fn from(pipe: AnonPipe) -> Stdio {
        pipe.diverge()
    }
}

impl From<File> for Stdio {
    fn from(file: File) -> Stdio {
        file.diverge()
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub struct ExitStatus(Void);

impl ExitStatus {
    pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
        match self.0 {}
    }

    pub fn code(&self) -> Option<i32> {
        match self.0 {}
    }
}

impl Clone for ExitStatus {
    fn clone(&self) -> ExitStatus {
        match self.0 {}
    }
}

impl Copy for ExitStatus {}

impl PartialEq for ExitStatus {
    fn eq(&self, _other: &ExitStatus) -> bool {
        match self.0 {}
    }
}

impl Eq for ExitStatus {
}

impl fmt::Debug for ExitStatus {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatusError(ExitStatus);

impl Into<ExitStatus> for ExitStatusError {
    fn into(self) -> ExitStatus {
        match self.0 {}
    }
}

impl ExitStatusError {
    pub fn code(self) -> Option<NonZeroI32> {
        match self.0 {}
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitCode(u8);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(0 as _);
    pub const FAILURE: ExitCode = ExitCode(1 as _);

    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        Self(code)
    }
}

pub struct Process(Void);

impl Process {
    pub fn id(&self) -> u32 {
        match self.0 {}
    }

    pub fn kill(&mut self) -> io::Result<()> {
        match self.0 {}
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        match self.0 {}
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        match self.0 {}
    }
}

pub struct CommandArgs<'a> {
    iter: crate::slice::Iter<'a, CString>,
}

impl<'a> Iterator for CommandArgs<'a> {
    type Item = &'a OsStr;
    fn next(&mut self) -> Option<&'a OsStr> {
        self.iter.next().map(|_| OsStr::new(""))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for CommandArgs<'a> {
    fn len(&self) -> usize {
        self.iter.len()
    }
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

impl<'a> fmt::Debug for CommandArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter.clone()).finish()
    }
}
