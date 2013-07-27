
use sys;
use sys::io;
use std::libc::pid_t;

static CGPATH : &'static str = "/sys/fs/cgroup/";

struct Group {
    name: ~str,
    subsys: ~[~str],
    tasks: ~[pid_t]
}

struct Monitor {
    priv file: io::File
}

impl Group {

    pub fn new(name: ~str, subsys: &[&str]) -> Group {
        let mut g = Group {
            name: name,
            subsys: ~[],
            tasks: ~[]
        };

        for subsys.iter().advance |&subsys| {
            g.subsys.push(subsys.to_owned());

            let path = g.get_path(subsys, None);
            let ret = io::mkdir(path, 438);

            if (ret.is_err() && !ret.is(sys::err::EEXIST)) {
                sys::fail(fmt!("Could not create directory (%s)", ret.msg()));
            }

        }

        return g;
    }

    pub fn add_task(&mut self, pid: pid_t) {
        self.tasks.push(pid);
        for self.subsys.iter().advance |sys| {
            let fname = self.get_path(*sys, Some("tasks"));
            let file = io::File::open(fname, "a").unwrap();
            file.write_str(pid.to_str());
            file.write_str("\n");
        }
    }

    pub fn set_bool(&self, subsys: &str, file: &str, val: bool) {
        let val = if val { "1\n" } else { "0\n" };

        let fname = self.get_path(subsys, Some(file));

        let file = io::File::open(fname, "w").unwrap();
        file.write_str(val);
    }

    pub fn get_int(&self, subsys: &str, value: &str) -> int {
        let fname = self.get_path(subsys, Some(value));
        let mut buf = [0u8,..32];

        let file = io::File::open(fname, "r").unwrap();

        let len = file.read_bytes(buf).unwrap() as uint;

        atoi(buf, len)
    }

    pub fn get_str(&self, subsys: &str, value: &str) -> ~str {
        use std::{str,cast};

        let fname = self.get_path(subsys, Some(value));
        let mut buf = [0u8,..256];

        let file = io::File::open(fname, "r").unwrap();
        let len = file.read_bytes(buf).unwrap() as uint;

        unsafe {
            let bufp : *u8 = cast::transmute(&buf);
            str::raw::from_buf_len(bufp, len)
        }
    }

    pub fn monitor(&self, subsys: &str, value: &str) -> Monitor {
        let fname = self.get_path(subsys, Some(value));
        let file = io::File::open(fname, "r").unwrap();
        Monitor { file: file }
    }

    pub fn has_subsys(&self, sys: &str) -> bool {
        self.subsys.iter().any(|s| s.equiv(&sys))
    }

    priv fn get_path(&self, subsys: &str, file: Option<&str>) -> ~str {
        if !self.has_subsys(subsys) {
            sys::fail(fmt!("Group does not have subsys %s", subsys));
        }

        let mut path = CGPATH + subsys + "/" + self.name;
        match file {
            Some(f) => {
                path.push_char('/');
                path.push_str(f);
            }
            None => ()
        }

        path
    }
}

impl Drop for Group {
    pub fn drop(&self) {

        for self.subsys.iter().advance |s| {
            let fname = CGPATH + *s + "/tasks";
            let file = io::File::open(fname, "w").unwrap();
            file.write_str("0\n");
            file.close();
        }

        for self.subsys.iter().advance |s| {
            let dirname = self.get_path(*s, None);
            let err = io::rmdir(dirname);
            if err.is_err() {
                io::println(fmt!("Warning trying to remove %s (%s)", dirname, err.msg()));
            }
        }
    }
}

impl Monitor {
    pub fn get_int(&self) -> int {
        let mut buf = [0u8,..32];

        let len = self.file.read_bytes(buf).unwrap() as uint;
        self.file.reset();
        atoi(buf, len)
    }

    pub fn get_str(&self) -> ~str {
        use std::{str,cast};

        let mut buf = [0u8,..256];
        let len = self.file.read_bytes(buf).unwrap() as uint;
        self.file.reset();

        unsafe {
            let bufp : *u8 = cast::transmute(&buf);
            str::raw::from_buf_len(bufp, len)
        }
    }
}

fn atoi(buf: &[u8], len: uint) -> int {
    let mut accum = 0;
    let mut i = 0;

    let neg = if buf[0] == 45 {
        i += 1;
        -1
    } else {
        1
    };

    while (i < len) {
        let c = buf[i];
        if c < 48 || c > 57 {
            break;
        }
        accum *= 10;
        accum += (buf[i] - 48) as int;
        i += 1;
    }

    return neg * accum;
}
