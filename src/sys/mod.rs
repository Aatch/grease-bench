use std::cast;
use std::libc::{pid_t};

pub mod io;
pub mod sig;
pub mod err;

#[inline(always)]
pub unsafe fn syscall0(n: int) -> int {
    let mut ret : int = 0;

    asm!("syscall" : "={rax}"(ret) : "{rax}"(n) : "rcx", "r11", "memory" : "volatile");

    return ret;
}

#[inline(always)]
pub unsafe fn syscall1(n: int, a1: int) -> int {
    let mut ret : int = 0;

    asm!("syscall" : "={rax}"(ret) : "{rax}"(n), "{rdi}"(a1) : "rcx", "r11", "memory" : "volatile");

    return ret;
}

#[inline(always)]
pub unsafe fn syscall2(n: int, a1: int, a2: int) -> int {
    let mut ret : int = 0;

    asm!("syscall" : "={rax}"(ret) : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2) : "rcx", "r11", "memory" : "volatile");

    return ret;
}

#[inline(always)]
pub unsafe fn syscall3(n: int, a1: int, a2: int, a3: int) -> int {

    let mut ret : int = 0;

    asm!("syscall" : "={rax}"(ret) : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3) : "rcx", "r11", "memory" : "volatile");

    return ret;
}

#[inline(always)]
pub unsafe fn syscall4(n: int, a1: int, a2: int, a3: int, a4: int) -> int {

    let mut ret : int = 0;

    asm!("syscall" : "={rax}"(ret) : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3), "{r10}"(a4)
                   : "rcx", "r11", "memory" : "volatile");

    return ret;
}

#[inline(always)]
pub unsafe fn syscall5(n: int, a1: int, a2: int, a3: int, a4: int, a5: int) -> int {

    let mut ret : int = 0;

    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3), "{r10}"(a4), "{r8}"(a5)
                   : "rcx", "r11", "memory" : "volatile");

    return ret;
}

#[inline]
pub fn fork() -> pid_t {
    unsafe {
        syscall0(n::FORK) as pid_t
    }
}

#[inline]
pub fn usleep(micros: uint) {
    unsafe {
        let secs = micros / 1000000;
        let nanos = (micros % 1000000) * 1000;
        let ts : (int, int) = (secs as int, nanos as int);
        let tsp : int = cast::transmute(&ts);

        syscall2(n::NANOSLEEP, tsp, 0);
    }
}

#[inline]
pub fn pause() {
    unsafe {
        syscall0(n::PAUSE);
    }
}

#[inline]
pub fn waitpid(pid: pid_t, options: int) -> (int, int) {
    unsafe {
        let mut stat = -1;
        let statp : *mut int = &mut stat;

        let ret = syscall4(n::WAIT4, pid as int, statp as int, options, 0);

        (stat, ret)
    }
}

#[inline]
pub fn kill(pid: pid_t, sig: int) -> int {
    unsafe {
        syscall2(n::KILL, pid as int, sig as int)
    }
}

#[inlne]
pub fn abort() -> ! {
    raise(sig::ABRT);
    raise(sig::KILL);
    loop {}
}

#[inline]
pub fn raise(sig: int) -> int {
    unsafe {
        let tid = syscall0(n::GETTID);
        let pid = syscall0(n::GETPID);

        syscall3(n::TGKILL, pid, tid, sig)
    }
}

#[inline]
pub fn fail(msg: &str) -> ! {
    io::print("Failure: ");
    io::println(msg);
    abort()
}

#[inline]
pub fn getuid() -> int {
    unsafe {
        syscall0(n::GETUID)
    }
}

#[inline]
pub fn setuid(uid: int) {
    unsafe {
        syscall1(n::SETUID, uid);
    }
}

#[inline]
pub fn setgid(gid: int) {
    unsafe {
        syscall1(n::SETGID, gid);
    }
}

#[inline]
pub fn execve(filename: &str, args: &[&str], envs: &[&str]) -> err::Error {
    use std::libc::c_char;
    unsafe {
        let file : *c_char = filename.as_c_str(|s| s);
        let mut argv = ~[file];
        for args.iter().transform(|s| s.as_c_str(|r| r)).advance |p| {
            argv.push(p)
        }

        argv.push(0 as *c_char);

        let mut envp : ~[*c_char] = envs.iter().transform(|s| s.as_c_str(|r| r)).collect();
        envp.push(0 as *c_char);

        let argv_raw : **c_char = &argv[0];
        let envp_raw : **c_char = &envp[0];

        cast::transmute(syscall3(n::EXECVE, file as int, argv_raw as int, envp_raw as int))
    }
}

#[inline]
pub fn pipe(pipefd: &mut (int,int)) -> err::Error {
    unsafe {
        let mut p = (-1i32, -1i32);
        let pipefdp : int = cast::transmute(&mut p);

        let ret = cast::transmute(syscall1(n::PIPE, pipefdp));

        let (r,w) = p;
        *pipefd = (r as int, w as int);

        ret
    }
}

#[inline]
pub fn getclock() -> u64 {
    unsafe {
        let mut ts : (uint, uint) = (0,0);
        let tsp : *mut int = cast::transmute(&mut ts);
        syscall2(n::CLOCK_GETTIME, 1, tsp as int);

        let (s, ns) = ts;

        (s as u64 *1_000_000_000) + ns as u64
    }
}

pub mod n {
    pub static READ                    : int = 0;
    pub static WRITE                   : int = 1;
    pub static OPEN                    : int = 2;
    pub static CLOSE                   : int = 3;
    pub static STAT                    : int = 4;
    pub static FSTAT                   : int = 5;
    pub static LSTAT                   : int = 6;
    pub static POLL                    : int = 7;
    pub static LSEEK                   : int = 8;
    pub static MMAP                    : int = 9;
    pub static MPROTECT                : int = 10;
    pub static MUNMAP                  : int = 11;
    pub static BRK                     : int = 12;
    pub static RT_SIGACTION            : int = 13;
    pub static RT_SIGPROCMASK          : int = 14;
    pub static RT_SIGRETURN            : int = 15;
    pub static IOCTL                   : int = 16;
    pub static PREAD64                 : int = 17;
    pub static PWRITE64                : int = 18;
    pub static READV                   : int = 19;
    pub static WRITEV                  : int = 20;
    pub static ACCESS                  : int = 21;
    pub static PIPE                    : int = 22;
    pub static SELECT                  : int = 23;
    pub static SCHED_YIELD             : int = 24;
    pub static MREMAP                  : int = 25;
    pub static MSYNC                   : int = 26;
    pub static MINCORE                 : int = 27;
    pub static MADVISE                 : int = 28;
    pub static SHMGET                  : int = 29;
    pub static SHMAT                   : int = 30;
    pub static SHMCTL                  : int = 31;
    pub static DUP                     : int = 32;
    pub static DUP2                    : int = 33;
    pub static PAUSE                   : int = 34;
    pub static NANOSLEEP               : int = 35;
    pub static GETITIMER               : int = 36;
    pub static ALARM                   : int = 37;
    pub static SETITIMER               : int = 38;
    pub static GETPID                  : int = 39;
    pub static SENDFILE                : int = 40;
    pub static SOCKET                  : int = 41;
    pub static CONNECT                 : int = 42;
    pub static ACCEPT                  : int = 43;
    pub static SENDTO                  : int = 44;
    pub static RECVFROM                : int = 45;
    pub static SENDMSG                 : int = 46;
    pub static RECVMSG                 : int = 47;
    pub static SHUTDOWN                : int = 48;
    pub static BIND                    : int = 49;
    pub static LISTEN                  : int = 50;
    pub static GETSOCKNAME             : int = 51;
    pub static GETPEERNAME             : int = 52;
    pub static SOCKETPAIR              : int = 53;
    pub static SETSOCKOPT              : int = 54;
    pub static GETSOCKOPT              : int = 55;
    pub static CLONE                   : int = 56;
    pub static FORK                    : int = 57;
    pub static VFORK                   : int = 58;
    pub static EXECVE                  : int = 59;
    pub static EXIT                    : int = 60;
    pub static WAIT4                   : int = 61;
    pub static KILL                    : int = 62;
    pub static UNAME                   : int = 63;
    pub static SEMGET                  : int = 64;
    pub static SEMOP                   : int = 65;
    pub static SEMCTL                  : int = 66;
    pub static SHMDT                   : int = 67;
    pub static MSGGET                  : int = 68;
    pub static MSGSND                  : int = 69;
    pub static MSGRCV                  : int = 70;
    pub static MSGCTL                  : int = 71;
    pub static FCNTL                   : int = 72;
    pub static FLOCK                   : int = 73;
    pub static FSYNC                   : int = 74;
    pub static FDATASYNC               : int = 75;
    pub static TRUNCATE                : int = 76;
    pub static FTRUNCATE               : int = 77;
    pub static GETDENTS                : int = 78;
    pub static GETCWD                  : int = 79;
    pub static CHDIR                   : int = 80;
    pub static FCHDIR                  : int = 81;
    pub static RENAME                  : int = 82;
    pub static MKDIR                   : int = 83;
    pub static RMDIR                   : int = 84;
    pub static CREAT                   : int = 85;
    pub static LINK                    : int = 86;
    pub static UNLINK                  : int = 87;
    pub static SYMLINK                 : int = 88;
    pub static READLINK                : int = 89;
    pub static CHMOD                   : int = 90;
    pub static FCHMOD                  : int = 91;
    pub static CHOWN                   : int = 92;
    pub static FCHOWN                  : int = 93;
    pub static LCHOWN                  : int = 94;
    pub static UMASK                   : int = 95;
    pub static GETTIMEOFDAY            : int = 96;
    pub static GETRLIMIT               : int = 97;
    pub static GETRUSAGE               : int = 98;
    pub static SYSINFO                 : int = 99;
    pub static TIMES                   : int = 100;
    pub static PTRACE                  : int = 101;
    pub static GETUID                  : int = 102;
    pub static SYSLOG                  : int = 103;
    pub static GETGID                  : int = 104;
    pub static SETUID                  : int = 105;
    pub static SETGID                  : int = 106;
    pub static GETEUID                 : int = 107;
    pub static GETEGID                 : int = 108;
    pub static SETPGID                 : int = 109;
    pub static GETPPID                 : int = 110;
    pub static GETPGRP                 : int = 111;
    pub static SETSID                  : int = 112;
    pub static SETREUID                : int = 113;
    pub static SETREGID                : int = 114;
    pub static GETGROUPS               : int = 115;
    pub static SETGROUPS               : int = 116;
    pub static SETRESUID               : int = 117;
    pub static GETRESUID               : int = 118;
    pub static SETRESGID               : int = 119;
    pub static GETRESGID               : int = 120;
    pub static GETPGID                 : int = 121;
    pub static SETFSUID                : int = 122;
    pub static SETFSGID                : int = 123;
    pub static GETSID                  : int = 124;
    pub static CAPGET                  : int = 125;
    pub static CAPSET                  : int = 126;
    pub static RT_SIGPENDING           : int = 127;
    pub static RT_SIGTIMEDWAIT         : int = 128;
    pub static RT_SIGQUEUEINFO         : int = 129;
    pub static RT_SIGSUSPEND           : int = 130;
    pub static SIGALTSTACK             : int = 131;
    pub static UTIME                   : int = 132;
    pub static MKNOD                   : int = 133;
    pub static USELIB                  : int = 134;
    pub static PERSONALITY             : int = 135;
    pub static USTAT                   : int = 136;
    pub static STATFS                  : int = 137;
    pub static FSTATFS                 : int = 138;
    pub static SYSFS                   : int = 139;
    pub static GETPRIORITY             : int = 140;
    pub static SETPRIORITY             : int = 141;
    pub static SCHED_SETPARAM          : int = 142;
    pub static SCHED_GETPARAM          : int = 143;
    pub static SCHED_SETSCHEDULER      : int = 144;
    pub static SCHED_GETSCHEDULER      : int = 145;
    pub static SCHED_GET_PRIORITY_MAX  : int = 146;
    pub static SCHED_GET_PRIORITY_MIN  : int = 147;
    pub static SCHED_RR_GET_INTERVAL   : int = 148;
    pub static MLOCK                   : int = 149;
    pub static MUNLOCK                 : int = 150;
    pub static MLOCKALL                : int = 151;
    pub static MUNLOCKALL              : int = 152;
    pub static VHANGUP                 : int = 153;
    pub static MODIFY_LDT              : int = 154;
    pub static PIVOT_ROOT              : int = 155;
    pub static _SYSCTL                 : int = 156;
    pub static PRCTL                   : int = 157;
    pub static ARCH_PRCTL              : int = 158;
    pub static ADJTIMEX                : int = 159;
    pub static SETRLIMIT               : int = 160;
    pub static CHROOT                  : int = 161;
    pub static SYNC                    : int = 162;
    pub static ACCT                    : int = 163;
    pub static SETTIMEOFDAY            : int = 164;
    pub static MOUNT                   : int = 165;
    pub static UMOUNT2                 : int = 166;
    pub static SWAPON                  : int = 167;
    pub static SWAPOFF                 : int = 168;
    pub static REBOOT                  : int = 169;
    pub static SETHOSTNAME             : int = 170;
    pub static SETDOMAINNAME           : int = 171;
    pub static IOPL                    : int = 172;
    pub static IOPERM                  : int = 173;
    pub static CREATE_MODULE           : int = 174;
    pub static INIT_MODULE             : int = 175;
    pub static DELETE_MODULE           : int = 176;
    pub static GET_KERNEL_SYMS         : int = 177;
    pub static QUERY_MODULE            : int = 178;
    pub static QUOTACTL                : int = 179;
    pub static NFSSERVCTL              : int = 180;
    pub static GETPMSG                 : int = 181;
    pub static PUTPMSG                 : int = 182;
    pub static AFS_SYSCALL             : int = 183;
    pub static TUXCALL                 : int = 184;
    pub static SECURITY                : int = 185;
    pub static GETTID                  : int = 186;
    pub static READAHEAD               : int = 187;
    pub static SETXATTR                : int = 188;
    pub static LSETXATTR               : int = 189;
    pub static FSETXATTR               : int = 190;
    pub static GETXATTR                : int = 191;
    pub static LGETXATTR               : int = 192;
    pub static FGETXATTR               : int = 193;
    pub static LISTXATTR               : int = 194;
    pub static LLISTXATTR              : int = 195;
    pub static FLISTXATTR              : int = 196;
    pub static REMOVEXATTR             : int = 197;
    pub static LREMOVEXATTR            : int = 198;
    pub static FREMOVEXATTR            : int = 199;
    pub static TKILL                   : int = 200;
    pub static TIME                    : int = 201;
    pub static FUTEX                   : int = 202;
    pub static SCHED_SETAFFINITY       : int = 203;
    pub static SCHED_GETAFFINITY       : int = 204;
    pub static SET_THREAD_AREA         : int = 205;
    pub static IO_SETUP                : int = 206;
    pub static IO_DESTROY              : int = 207;
    pub static IO_GETEVENTS            : int = 208;
    pub static IO_SUBMIT               : int = 209;
    pub static IO_CANCEL               : int = 210;
    pub static GET_THREAD_AREA         : int = 211;
    pub static LOOKUP_DCOOKIE          : int = 212;
    pub static EPOLL_CREATE            : int = 213;
    pub static EPOLL_CTL_OLD           : int = 214;
    pub static EPOLL_WAIT_OLD          : int = 215;
    pub static REMAP_FILE_PAGES        : int = 216;
    pub static GETDENTS64              : int = 217;
    pub static SET_TID_ADDRESS         : int = 218;
    pub static RESTART_SYSCALL         : int = 219;
    pub static SEMTIMEDOP              : int = 220;
    pub static FADVISE64               : int = 221;
    pub static TIMER_CREATE            : int = 222;
    pub static TIMER_SETTIME           : int = 223;
    pub static TIMER_GETTIME           : int = 224;
    pub static TIMER_GETOVERRUN        : int = 225;
    pub static TIMER_DELETE            : int = 226;
    pub static CLOCK_SETTIME           : int = 227;
    pub static CLOCK_GETTIME           : int = 228;
    pub static CLOCK_GETRES            : int = 229;
    pub static CLOCK_NANOSLEEP         : int = 230;
    pub static EXIT_GROUP              : int = 231;
    pub static EPOLL_WAIT              : int = 232;
    pub static EPOLL_CTL               : int = 233;
    pub static TGKILL                  : int = 234;
    pub static UTIMES                  : int = 235;
    pub static VSERVER                 : int = 236;
    pub static MBIND                   : int = 237;
    pub static SET_MEMPOLICY           : int = 238;
    pub static GET_MEMPOLICY           : int = 239;
    pub static MQ_OPEN                 : int = 240;
    pub static MQ_UNLINK               : int = 241;
    pub static MQ_TIMEDSEND            : int = 242;
    pub static MQ_TIMEDRECEIVE         : int = 243;
    pub static MQ_NOTIFY               : int = 244;
    pub static MQ_GETSETATTR           : int = 245;
    pub static KEXEC_LOAD              : int = 246;
    pub static WAITID                  : int = 247;
    pub static ADD_KEY                 : int = 248;
    pub static REQUEST_KEY             : int = 249;
    pub static KEYCTL                  : int = 250;
    pub static IOPRIO_SET              : int = 251;
    pub static IOPRIO_GET              : int = 252;
    pub static INOTIFY_INIT            : int = 253;
    pub static INOTIFY_ADD_WATCH       : int = 254;
    pub static INOTIFY_RM_WATCH        : int = 255;
    pub static MIGRATE_PAGES           : int = 256;
    pub static OPENAT                  : int = 257;
    pub static MKDIRAT                 : int = 258;
    pub static MKNODAT                 : int = 259;
    pub static FCHOWNAT                : int = 260;
    pub static FUTIMESAT               : int = 261;
    pub static NEWFSTATAT              : int = 262;
    pub static UNLINKAT                : int = 263;
    pub static RENAMEAT                : int = 264;
    pub static LINKAT                  : int = 265;
    pub static SYMLINKAT               : int = 266;
    pub static READLINKAT              : int = 267;
    pub static FCHMODAT                : int = 268;
    pub static FACCESSAT               : int = 269;
    pub static PSELECT6                : int = 270;
    pub static PPOLL                   : int = 271;
    pub static UNSHARE                 : int = 272;
    pub static SET_ROBUST_LIST         : int = 273;
    pub static GET_ROBUST_LIST         : int = 274;
    pub static SPLICE                  : int = 275;
    pub static TEE                     : int = 276;
    pub static SYNC_FILE_RANGE         : int = 277;
    pub static VMSPLICE                : int = 278;
    pub static MOVE_PAGES              : int = 279;
    pub static UTIMENSAT               : int = 280;
    pub static EPOLL_PWAIT             : int = 281;
    pub static SIGNALFD                : int = 282;
    pub static TIMERFD_CREATE          : int = 283;
    pub static EVENTFD                 : int = 284;
    pub static FALLOCATE               : int = 285;
    pub static TIMERFD_SETTIME         : int = 286;
    pub static TIMERFD_GETTIME         : int = 287;
    pub static ACCEPT4                 : int = 288;
    pub static SIGNALFD4               : int = 289;
    pub static EVENTFD2                : int = 290;
    pub static EPOLL_CREATE1           : int = 291;
    pub static DUP3                    : int = 292;
    pub static PIPE2                   : int = 293;
    pub static INOTIFY_INIT1           : int = 294;
    pub static PREADV                  : int = 295;
    pub static PWRITEV                 : int = 296;
    pub static RT_TGSIGQUEUEINFO       : int = 297;
    pub static PERF_EVENT_OPEN         : int = 298;
    pub static RECVMMSG                : int = 299;
    pub static FANOTIFY_INIT           : int = 300;
    pub static FANOTIFY_MARK           : int = 301;
    pub static PRLIMIT64               : int = 302;
    pub static NAME_TO_HANDLE_AT       : int = 303;
    pub static OPEN_BY_HANDLE_AT       : int = 304;
    pub static CLOCK_ADJTIME           : int = 305;
    pub static SYNCFS                  : int = 306;
    pub static SENDMMSG                : int = 307;
    pub static SETNS                   : int = 308;
    pub static GETCPU                  : int = 309;
    pub static PROCESS_VM_READV        : int = 310;
    pub static PROCESS_VM_WRITEV       : int = 311;
    pub static KCMP                    : int = 312;
    pub static FINIT_MODULE            : int = 313;
}
