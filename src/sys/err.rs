
use sys;

pub struct Error {
    priv n:int
}

pub enum ErrorResult<T> {
    Ok(T),
    Err(Error)
}

impl Error {
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        self.n != 0
    }

    #[inline(always)]
    pub fn msg(&self) -> &'static str {
        msg(-1*self.n)
    }

    #[inline(always)]
    pub fn is(&self, code: int) -> bool {
        (-1*self.n) == code
    }
}

impl<T> ErrorResult<T> {
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Ok(d) => d,
            Err(err) => sys::fail(err.msg())
        }
    }

    #[inline]
    pub fn is_error(&self) -> bool {
        match *self {
            Ok(_) => false,
            Err(a) => a.is_err()
        }
    }
}

pub static EPERM           : int =  1;
pub static ENOENT          : int =  2;
pub static ESRCH           : int =  3;
pub static EINTR           : int =  4;
pub static EIO             : int =  5;
pub static ENXIO           : int =  6;
pub static E2BIG           : int =  7;
pub static ENOEXEC         : int =  8;
pub static EBADF           : int =  9;
pub static ECHILD          : int = 10;
pub static EAGAIN          : int = 11;
pub static ENOMEM          : int = 12;
pub static EACCES          : int = 13;
pub static EFAULT          : int = 14;
pub static ENOTBLK         : int = 15;
pub static EBUSY           : int = 16;
pub static EEXIST          : int = 17;
pub static EXDEV           : int = 18;
pub static ENODEV          : int = 19;
pub static ENOTDIR         : int = 20;
pub static EISDIR          : int = 21;
pub static EINVAL          : int = 22;
pub static ENFILE          : int = 23;
pub static EMFILE          : int = 24;
pub static ENOTTY          : int = 25;
pub static ETXTBSY         : int = 26;
pub static EFBIG           : int = 27;
pub static ENOSPC          : int = 28;
pub static ESPIPE          : int = 29;
pub static EROFS           : int = 30;
pub static EMLINK          : int = 31;
pub static EPIPE           : int = 32;
pub static EDOM            : int = 33;
pub static ERANGE          : int = 34;
pub static EDEADLK         : int = 35;
pub static ENAMETOOLONG    : int = 36;
pub static ENOLCK          : int = 37;
pub static ENOSYS          : int = 38;
pub static ENOTEMPTY       : int = 39;
pub static ELOOP           : int = 40;
pub static EWOULDBLOCK     : int = EAGAIN;
pub static ENOMSG          : int = 42;
pub static EIDRM           : int = 43;
pub static ECHRNG          : int = 44;
pub static EL2NSYNC        : int = 45;
pub static EL3HLT          : int = 46;
pub static EL3RST          : int = 47;
pub static ELNRNG          : int = 48;
pub static EUNATCH         : int = 49;
pub static ENOCSI          : int = 50;
pub static EL2HLT          : int = 51;
pub static EBADE           : int = 52;
pub static EBADR           : int = 53;
pub static EXFULL          : int = 54;
pub static ENOANO          : int = 55;
pub static EBADRQC         : int = 56;
pub static EBADSLT         : int = 57;
pub static EDEADLOCK       : int = EDEADLK;
pub static EBFONT          : int = 59;
pub static ENOSTR          : int = 60;
pub static ENODATA         : int = 61;
pub static ETIME           : int = 62;
pub static ENOSR           : int = 63;
pub static ENONET          : int = 64;
pub static ENOPKG          : int = 65;
pub static EREMOTE         : int = 66;
pub static ENOLINK         : int = 67;
pub static EADV            : int = 68;
pub static ESRMNT          : int = 69;
pub static ECOMM           : int = 70;
pub static EPROTO          : int = 71;
pub static EMULTIHOP       : int = 72;
pub static EDOTDOT         : int = 73;
pub static EBADMSG         : int = 74;
pub static EOVERFLOW       : int = 75;
pub static ENOTUNIQ        : int = 76;
pub static EBADFD          : int = 77;
pub static EREMCHG         : int = 78;
pub static ELIBACC         : int = 79;
pub static ELIBBAD         : int = 80;
pub static ELIBSCN         : int = 81;
pub static ELIBMAX         : int = 82;
pub static ELIBEXEC        : int = 83;
pub static EILSEQ          : int = 84;
pub static ERESTART        : int = 85;
pub static ESTRPIPE        : int = 86;
pub static EUSERS          : int = 87;
pub static ENOTSOCK        : int = 88;
pub static EDESTADDRREQ    : int = 89;
pub static EMSGSIZE        : int = 90;
pub static EPROTOTYPE      : int = 91;
pub static ENOPROTOOPT     : int = 92;
pub static EPROTONOSUPPORT : int = 93;
pub static ESOCKTNOSUPPORT : int = 94;
pub static EOPNOTSUPP      : int = 95;
pub static ENOTSUP         : int = EOPNOTSUPP;
pub static EPFNOSUPPORT    : int = 96;
pub static EAFNOSUPPORT    : int = 97;
pub static EADDRINUSE      : int = 98;
pub static EADDRNOTAVAIL   : int = 99;
pub static ENETDOWN        : int = 100;
pub static ENETUNREACH     : int = 101;
pub static ENETRESET       : int = 102;
pub static ECONNABORTED    : int = 103;
pub static ECONNRESET      : int = 104;
pub static ENOBUFS         : int = 105;
pub static EISCONN         : int = 106;
pub static ENOTCONN        : int = 107;
pub static ESHUTDOWN       : int = 108;
pub static ETOOMANYREFS    : int = 109;
pub static ETIMEDOUT       : int = 110;
pub static ECONNREFUSED    : int = 111;
pub static EHOSTDOWN       : int = 112;
pub static EHOSTUNREACH    : int = 113;
pub static EALREADY        : int = 114;
pub static EINPROGRESS     : int = 115;
pub static ESTALE          : int = 116;
pub static EUCLEAN         : int = 117;
pub static ENOTNAM         : int = 118;
pub static ENAVAIL         : int = 119;
pub static EISNAM          : int = 120;
pub static EREMOTEIO       : int = 121;
pub static EDQUOT          : int = 122;
pub static ENOMEDIUM       : int = 123;
pub static EMEDIUMTYPE     : int = 124;
pub static ECANCELED       : int = 125;
pub static ENOKEY          : int = 126;
pub static EKEYEXPIRED     : int = 127;
pub static EKEYREVOKED     : int = 128;
pub static EKEYREJECTED    : int = 129;
pub static EOWNERDEAD      : int = 130;
pub static ENOTRECOVERABLE : int = 131;
pub static ERFKILL         : int = 132;
pub static EHWPOISON       : int = 133;

// The above errors are treated as indexes into
// this array
static ERR_MSG : &'static [&'static str] = &[
    "Success",
    "Operation not permitted",
    "No such file or directory",
    "No such process",
    "Interrupted system call",
    "Input/output error",
    "No such device or address",
    "Argument list too long",
    "Exec format error",
    "Bad file descriptor",
    "No child processes",
    "Resource temporarily unavailable",
    "Cannot allocate memory",
    "Permission denied",
    "Bad address",
    "Block device required",
    "Device or resource busy",
    "File exists",
    "Invalid cross-device link",
    "No such device",
    "Not a directory",
    "Is a directory",
    "Invalid argument",
    "Too many open files in system",
    "Too many open files",
    "Inappropriate ioctl for device",
    "Text file busy",
    "File too large",
    "No space left on device",
    "Illegal seek",
    "Read-only file system",
    "Too many links",
    "Broken pipe",
    "Numerical argument out of domain",
    "Numerical result out of range",
    "Resource deadlock avoided",
    "File name too long",
    "No locks available",
    "Function not implemented",
    "Directory not empty",
    "Too many levels of symbolic links",
    "Unknown error 41",
    "No message of desired type",
    "Identifier removed",
    "Channel number out of range",
    "Level 2 not synchronized",
    "Level 3 halted",
    "Level 3 reset",
    "Link number out of range",
    "Protocol driver not attached",
    "No CSI structure available",
    "Level 2 halted",
    "Invalid exchange",
    "Invalid request descriptor",
    "Exchange full",
    "No anode",
    "Invalid request code",
    "Invalid slot",
    "Unknown error 58",
    "Bad font file format",
    "Device not a stream",
    "No data available",
    "Timer expired",
    "Out of streams resources",
    "Machine is not on the network",
    "Package not installed",
    "Object is remote",
    "Link has been severed",
    "Advertise error",
    "Srmount error",
    "Communication error on send",
    "Protocol error",
    "Multihop attempted",
    "RFS specific error",
    "Bad message",
    "Value too large for defined data type",
    "Name not unique on network",
    "File descriptor in bad state",
    "Remote address changed",
    "Can not access a needed shared library",
    "Accessing a corrupted shared library",
    ".lib section in a.out corrupted",
    "Attempting to link in too many shared libraries",
    "Cannot exec a shared library directly",
    "Invalid or incomplete multibyte or wide character",
    "Interrupted system call should be restarted",
    "Streams pipe error",
    "Too many users",
    "Socket operation on non-socket",
    "Destination address required",
    "Message too long",
    "Protocol wrong type for socket",
    "Protocol not available",
    "Protocol not supported",
    "Socket type not supported",
    "Operation not supported",
    "Protocol family not supported",
    "Address family not supported by protocol",
    "Address already in use",
    "Cannot assign requested address",
    "Network is down",
    "Network is unreachable",
    "Network dropped connection on reset",
    "Software caused connection abort",
    "Connection reset by peer",
    "No buffer space available",
    "Transport endpoint is already connected",
    "Transport endpoint is not connected",
    "Cannot send after transport endpoint shutdown",
    "Too many references: cannot splice",
    "Connection timed out",
    "Connection refused",
    "Host is down",
    "No route to host",
    "Operation already in progress",
    "Operation now in progress",
    "Stale NFS file handle",
    "Structure needs cleaning",
    "Not a XENIX named type file",
    "No XENIX semaphores available",
    "Is a named type file",
    "Remote I/O error",
    "Disk quota exceeded",
    "No medium found",
    "Wrong medium type",
    "Operation canceled",
    "Required key not available",
    "Key has expired",
    "Key has been revoked",
    "Key was rejected by service",
    "Owner died",
    "State not recoverable",
    "Operation not possible due to RF-kill",
    "Memory page has hardware error"
];

#[inline]
pub fn msg(errno: int) -> &'static str {
    use std::cast;
    let errno = errno as uint;
    let len = unsafe {
        let (_, len) : (uint, uint) = cast::transmute(ERR_MSG);
        len
    };

    unsafe {
        if errno < len {
            *ERR_MSG.unsafe_ref(errno)
        } else {
            "Unknown Error"
        }
    }
}
