use hyperion_abi::sys::fs::FileDesc;
use hyperion_abi::sys::fs::FileOpenFlags;
use hyperion_abi::sys::pipe;
use hyperion_abi::sys::LaunchConfig;

use crate::ffi::OsStr;
use crate::fmt;
use crate::io;
use crate::marker::PhantomData;
use crate::num::NonZeroI32;
use crate::os::hyperion::map_sys_err;
use crate::path::Path;
use crate::sys::fs::File;
use crate::sys::fs::OpenOptions;
use crate::sys::pipe::AnonPipe;
use crate::sys::unsupported;
use crate::sys_common::AsInner;
use crate::sys_common::{
    process::{CommandEnv, CommandEnvs},
    FromInner,
};

pub use crate::ffi::OsString as EnvKey;

////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

pub struct Command {
    env: CommandEnv,

    program: String,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>,
    args: Vec<String>,
}

// passed back to std::process with the pipes connected to the child, if any
// were requested
pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

// FIXME: This should be a unit struct, so we can always construct it
// The value here should be never used, since we cannot spawn processes.
#[derive(Clone, Copy)]
pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
}

impl Command {
    pub fn new(program: &OsStr) -> Command {
        Command {
            env: Default::default(),

            program: program.to_str().expect("program name should be UTF-8").to_string(),
            stdin: None,
            stdout: None,
            stderr: None,
            args: Vec::new(),
        }
    }

    pub fn arg(&mut self, arg: &OsStr) {
        self.args.push(arg.to_str().expect("cli args should be UTF-8").to_string());
    }

    pub fn env_mut(&mut self) -> &mut CommandEnv {
        &mut self.env
    }

    pub fn cwd(&mut self, _dir: &OsStr) {
        panic!("hyperion doesn't have os enforced working directories");
    }

    pub fn stdin(&mut self, stdin: Stdio) {
        self.stdin = Some(stdin);
    }

    pub fn stdout(&mut self, stdout: Stdio) {
        self.stdout = Some(stdout);
    }

    pub fn stderr(&mut self, stderr: Stdio) {
        self.stderr = Some(stderr);
    }

    pub fn get_program(&self) -> &OsStr {
        panic!("unsupported")
    }

    pub fn get_args(&self) -> CommandArgs<'_> {
        CommandArgs { _p: PhantomData }
    }

    pub fn get_envs(&self) -> CommandEnvs<'_> {
        self.env.iter()
    }

    pub fn get_current_dir(&self) -> Option<&Path> {
        None
    }

    pub fn spawn(
        &mut self,
        default: Stdio,
        _needs_stdin: bool,
    ) -> io::Result<(Process, StdioPipes)> {
        let stdin = self.stdin.unwrap_or(default);
        let stdout = self.stdout.unwrap_or(default);
        let stderr = self.stderr.unwrap_or(default);

        struct LazyNull(Option<File>);

        impl LazyNull {
            fn get(&mut self) -> FileDesc {
                *self
                    .0
                    .get_or_insert_with(|| {
                        File::open(
                            "/dev/null".as_ref(),
                            &OpenOptions::from_flags(FileOpenFlags::READ_WRITE),
                        )
                        .unwrap()
                    })
                    .as_inner()
            }
        }

        let mut null = LazyNull(None);

        let mut pipes = StdioPipes { stdin: None, stdout: None, stderr: None };

        let stdin = match stdin {
            Stdio::Inherit => FileDesc(0),
            Stdio::Null => null.get(),
            Stdio::MakePipe => {
                let [r, w] = pipe().unwrap();
                pipes.stdin = Some(AnonPipe::from_fd(r));
                w
            }
        };
        let stdout = match stdout {
            Stdio::Inherit => FileDesc(1),
            Stdio::Null => null.get(),
            Stdio::MakePipe => {
                let [r, w] = pipe().unwrap();
                pipes.stdout = Some(AnonPipe::from_fd(w));
                r
            }
        };
        let stderr = match stderr {
            Stdio::Inherit => FileDesc(2),
            Stdio::Null => null.get(),
            Stdio::MakePipe => {
                let [r, w] = pipe().unwrap();
                pipes.stderr = Some(AnonPipe::from_fd(w));
                r
            }
        };

        let args: Vec<&str> = self.args.iter().map(|s| s.as_str()).collect();

        let pid: usize = hyperion_abi::sys::system_with(
            self.program.as_str(),
            &args,
            LaunchConfig { stdin, stdout, stderr },
        )
        .map_err(map_sys_err)?;

        Ok((Process(pid), pipes))
    }

    pub fn output(&mut self) -> io::Result<(ExitStatus, Vec<u8>, Vec<u8>)> {
        unsupported()
    }
}

impl From<AnonPipe> for Stdio {
    fn from(pipe: AnonPipe) -> Stdio {
        pipe.diverge()
    }
}

impl From<io::Stdout> for Stdio {
    fn from(_: io::Stdout) -> Stdio {
        // FIXME: This is wrong.
        // Instead, the Stdio we have here should be a unit struct.
        panic!("unsupported")
    }
}

impl From<io::Stderr> for Stdio {
    fn from(_: io::Stderr) -> Stdio {
        // FIXME: This is wrong.
        // Instead, the Stdio we have here should be a unit struct.
        panic!("unsupported")
    }
}

impl From<File> for Stdio {
    fn from(_file: File) -> Stdio {
        // FIXME: This is wrong.
        // Instead, the Stdio we have here should be a unit struct.
        panic!("unsupported")
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct ExitStatus(i64);

impl ExitStatus {
    pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
        Ok(())
    }

    pub fn code(&self) -> Option<i32> {
        Some(0)
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<dummy exit status>")
    }
}

pub struct ExitStatusError(!);

impl Clone for ExitStatusError {
    fn clone(&self) -> ExitStatusError {
        self.0
    }
}

impl Copy for ExitStatusError {}

impl PartialEq for ExitStatusError {
    fn eq(&self, _other: &ExitStatusError) -> bool {
        self.0
    }
}

impl Eq for ExitStatusError {}

impl fmt::Debug for ExitStatusError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl Into<ExitStatus> for ExitStatusError {
    fn into(self) -> ExitStatus {
        self.0
    }
}

impl ExitStatusError {
    pub fn code(self) -> Option<NonZeroI32> {
        self.0
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitCode(i64);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(0);
    pub const FAILURE: ExitCode = ExitCode(1);

    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        Self(code as _)
    }
}

impl From<i64> for ExitCode {
    fn from(code: i64) -> Self {
        Self(code as _)
    }
}

pub trait ExitCodeExt {
    fn from_raw(raw: i64) -> Self;
}

impl ExitCodeExt for crate::process::ExitCode {
    fn from_raw(raw: i64) -> Self {
        crate::process::ExitCode::from_inner(From::from(raw))
    }
}

pub struct Process(usize);

impl Process {
    pub fn id(&self) -> u32 {
        self.0 as u32
    }

    pub fn kill(&mut self) -> io::Result<()> {
        todo!()
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        // FIXME: this is the most hacky way to wait for a process to close
        // hyperion doesn't have a wait syscall yet
        let path = format!("/proc/{}/status", self.id());

        // lmao, just spin on the /proc/<id> directory, this is pure evil
        while crate::fs::File::open(&path).is_ok() {
            hyperion_abi::sys::yield_now()
        }

        Ok(ExitStatus(0))
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        self.wait().map(Some)
    }
}

pub struct CommandArgs<'a> {
    _p: PhantomData<&'a ()>,
}

impl<'a> Iterator for CommandArgs<'a> {
    type Item = &'a OsStr;
    fn next(&mut self) -> Option<&'a OsStr> {
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl<'a> ExactSizeIterator for CommandArgs<'a> {}

impl<'a> fmt::Debug for CommandArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().finish()
    }
}
