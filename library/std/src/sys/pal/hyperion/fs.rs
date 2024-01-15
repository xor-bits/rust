use hyperion_syscall::fs::{FileDesc, FileOpenFlags};
use hyperion_syscall::open;

use crate::ffi::OsString;
use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, SeekFrom};
use crate::path::{Path, PathBuf};
use crate::sys::time::SystemTime;
use crate::sys::unsupported;

use super::io::map_sys_err;

pub struct File(FileDesc);

pub struct FileAttr(!);

pub struct ReadDir(!);

pub struct DirEntry(!);

#[derive(Clone, Debug)]
pub struct OpenOptions {
    flags: FileOpenFlags,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {}

pub struct FilePermissions(!);

pub struct FileType(!);

#[derive(Debug)]
pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.0
    }

    pub fn perm(&self) -> FilePermissions {
        self.0
    }

    pub fn file_type(&self) -> FileType {
        self.0
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        self.0
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        self.0
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        self.0
    }
}

impl Clone for FileAttr {
    fn clone(&self) -> FileAttr {
        self.0
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.0
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
        self.0
    }
}

impl Clone for FilePermissions {
    fn clone(&self) -> FilePermissions {
        self.0
    }
}

impl PartialEq for FilePermissions {
    fn eq(&self, _other: &FilePermissions) -> bool {
        self.0
    }
}

impl Eq for FilePermissions {}

impl fmt::Debug for FilePermissions {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0
    }

    pub fn is_file(&self) -> bool {
        self.0
    }

    pub fn is_symlink(&self) -> bool {
        self.0
    }
}

impl Clone for FileType {
    fn clone(&self) -> FileType {
        self.0
    }
}

impl Copy for FileType {}

impl PartialEq for FileType {
    fn eq(&self, _other: &FileType) -> bool {
        self.0
    }
}

impl Eq for FileType {}

impl Hash for FileType {
    fn hash<H: Hasher>(&self, _h: &mut H) {
        self.0
    }
}

impl fmt::Debug for FileType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        self.0
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.0
    }

    pub fn file_name(&self) -> OsString {
        self.0
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        self.0
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        self.0
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions { flags: FileOpenFlags::empty() }
    }

    pub fn read(&mut self, read: bool) {
        self.flags.set(FileOpenFlags::READ, read);
    }

    pub fn write(&mut self, write: bool) {
        self.flags.set(FileOpenFlags::WRITE, write);
    }

    pub fn append(&mut self, append: bool) {
        self.flags.set(FileOpenFlags::APPEND, append);
    }

    pub fn truncate(&mut self, truncate: bool) {
        self.flags.set(FileOpenFlags::TRUNC, truncate);
    }

    pub fn create(&mut self, create: bool) {
        self.flags.set(FileOpenFlags::CREATE, create);
    }

    pub fn create_new(&mut self, create_new: bool) {
        self.flags.set(FileOpenFlags::CREATE_NEW, create_new);
    }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        if opts.flags.intersection(FileOpenFlags::READ_WRITE).is_empty() {
            return Err(io::const_io_error!(
                io::ErrorKind::InvalidInput,
                "the path should be UTF-8"
            ));
        }

        let path = path.canonicalize()?;
        let Some(path) = path.to_str() else {
            return Err(io::const_io_error!(
                io::ErrorKind::InvalidFilename,
                "the path should be UTF-8"
            ));
        };

        open(path, opts.flags, 0).map(File).map_err(map_sys_err)
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        Err(io::const_io_error!(io::ErrorKind::Unsupported, "file_attr unsupported"))
    }

    pub fn fsync(&self) -> io::Result<()> {
        Err(io::const_io_error!(io::ErrorKind::Unsupported, "fsync unsupported"))
    }

    pub fn datasync(&self) -> io::Result<()> {
        Err(io::const_io_error!(io::ErrorKind::Unsupported, "datasync unsupported"))
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        Err(io::const_io_error!(io::ErrorKind::Unsupported, "truncate unsupported"))
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        hyperion_syscall::read(self.0, buf).map_err(map_sys_err)
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        Err(io::const_io_error!(io::ErrorKind::Unsupported, "read_vectored unsupported"))
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_>) -> io::Result<()> {
        let read =
            hyperion_syscall::read_uninit(self.0, cursor.uninit_mut()).map_err(map_sys_err)?;
        unsafe { cursor.advance(read) };
        Ok(())
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        hyperion_syscall::write(self.0, buf).map_err(map_sys_err)
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        Err(io::const_io_error!(io::ErrorKind::Unsupported, "write_vectored unsupported"))
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let (offs, origin) = match pos {
            SeekFrom::Start(offs) => (offs as _, hyperion_syscall::fs::Seek::SET),
            SeekFrom::End(offs) => (offs as _, hyperion_syscall::fs::Seek::END),
            SeekFrom::Current(offs) => (offs as _, hyperion_syscall::fs::Seek::CUR),
        };
        hyperion_syscall::seek(self.0, offs, origin.0).map_err(map_sys_err)?;
        let mut meta = hyperion_syscall::fs::Metadata::zeroed();
        hyperion_syscall::metadata(self.0, &mut meta).map_err(map_sys_err)?;
        Ok(meta.position as _)
    }

    pub fn duplicate(&self) -> io::Result<File> {
        // hyperion_syscall::dup(self.0, )
        Err(io::const_io_error!(io::ErrorKind::Unsupported, "duplicate unsupported"))
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        Err(io::const_io_error!(io::ErrorKind::Unsupported, "set_permissions unsupported"))
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        Err(io::const_io_error!(io::ErrorKind::Unsupported, "set_times unsupported"))
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("File").field(&self.0).finish()
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported()
}

pub fn unlink(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}

pub fn set_perm(_p: &Path, perm: FilePermissions) -> io::Result<()> {
    match perm.0 {}
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}

pub fn try_exists(_path: &Path) -> io::Result<bool> {
    unsupported()
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    if p.has_root() { Ok(p.into()) } else { Ok(Path::new("/").join(p)) }
}

pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    unsupported()
}
