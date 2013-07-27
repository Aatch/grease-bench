
use sys::io;

mod cgroup;

#[path = "sys/mod.rs"]
pub mod sys;

static env : &'static [&'static str] = &[
    "CC=clang", "CXX=clang++", "TERM=xterm-256color",
    "PATH=/usr/local/bin:/usr/bin:/bin"
];

#[start]
fn start(argc: int, argv: **u8, _cm: *u8) -> int {

    let args : &[*u8] = unsafe {
        std::cast::transmute((argv, argc as uint*std::sys::size_of::<*u8>()))
    };

    let args : ~[~str] = unsafe {
        args.iter().transform(|&p| std::str::raw::from_buf(p)).to_owned_vec()
    };

    let cmdo = args.slice_from(1).connect(" ");
    let cmd : &str = cmdo;
    let c : &str = "-c";

    let mut group = cgroup::Group::new(~"bench", ["cpuacct", "memory"]);

    let mut pipes = (0,0);
    let res = sys::pipe(&mut pipes);
    if res.is_err() {
        sys::fail(fmt!("Can't make pipe (%s)", res.msg()));
    }

    let (read, write) = pipes;

    let pid = sys::fork();
    if pid == 0 {
        io::raw::close(write);

        sys::setuid(1000);
        sys::setgid(1000);

        let mut buf = [0];
        io::raw::read(read, buf);

        io::raw::close(read);
        let res = sys::execve("/bin/sh", [c, cmd], env);

        sys::fail(res.msg());

    } else {
        io::raw::close(read);

        group.set_bool("memory", "memory.use_hierarchy", true);
        group.add_task(pid);

        let mem_usage = group.monitor("memory", "memory.usage_in_bytes");

        let mut logger = Logger::new("mem.csv", mem_usage, ~[]);

        logger.add_comment("Timestamp (ns), memory usage");

        io::raw::write(write, ['0' as u8]);
        io::raw::close(write);

        loop {
            let (_, ret) = sys::waitpid(pid, 1);
            if ret < 0 {
                if -1*ret == sys::err::EINTR {
                    break;
                }
                io::println(sys::err::msg(-1*ret));
            } else if ret > 0 {
                break;
            }

            logger.log();

            //sys::usleep(5000);
        }
    }

    0
}

pub struct Logger {
    file: io::File,
    primary_mon: cgroup::Monitor,
    monitors: ~[cgroup::Monitor],
    prev_val: Option<int>,
    start_time: u64
}

impl Logger {
    pub fn new(log_file: &str, primary_mon: cgroup::Monitor,
                monitors: ~[cgroup::Monitor]) -> Logger {
        Logger {
            file: io::File::open(log_file, "ws").unwrap(),
            primary_mon: primary_mon,
            monitors: monitors,
            prev_val: None,
            start_time: 0
        }
    }

    pub fn add_comment(&self, str: &str) {
        self.file.write_str("# ");
        self.file.write_str(str);
        self.file.write_str("\n");
    }

    pub fn log(&mut self) {
        let val = Some(self.primary_mon.get_int());
        if val != self.prev_val {
            self.prev_val = val;
            self.write_log();
        }
    }

    priv fn write_log(&mut self) {
        let mut tm = sys::getclock();

        if self.start_time == 0 {
            self.start_time = tm;
        }
        tm -= self.start_time;

        self.file.write_str(tm.to_str());
        self.file.write_str(",");
        self.file.write_str(self.prev_val.get().to_str());

        for self.monitors.iter().advance |m| {
            self.file.write_str(",");
            self.file.write_str(m.get_int().to_str());
        }

        self.file.write_str("\n");
    }
}
