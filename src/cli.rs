#![allow(deprecated)]

use {
    clap::{crate_authors, crate_version, App, Arg},
    std::{
        net::{SocketAddr, ToSocketAddrs},
        path::{Path, PathBuf},
    },
};

pub fn generate_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("blog-serve")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        /* .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .max_values(64)
                .takes_value(false)
                .help("Sets level of debug output"),
        ) */
        .arg(
            Arg::with_name("name")
                .long("name")
                .help("Set server's name"),
        )
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .value_name("IP/HOST")
                .requires("port")
                .help("Set the IP / resolvable host to bind"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .requires("addr")
                .validator(|v| v.parse::<u16>().map(|_| ()).map_err(|e| format!("{}", e)))
                .help("Set port to bind"),
        )
        .arg(
            Arg::with_name("bind")
                .long("bind")
                .value_name("HOST:PORT")
                .conflicts_with("addr")
                .default_value("0.0.0.0:8080")
                .help("Set both IP/HOST and PORT to bind"),
        )
        .arg(
            Arg::with_name("base")
                .long("base")
                .value_name("DIR")
                .required(true)
                .validator(|v| match Path::new(v.as_str()).is_dir() {
                    true => Ok(()),
                    false => Err(format!("'{}' does not exist or is not a directory", v)),
                })
                .help("Path to directory containing index.html and related files"),
        )
}

pub struct ProgramArgs {
    bind: TCPBind,
    server_name: String,
    base_dir: PathBuf,
}

impl ProgramArgs {
    pub fn init<'a, 'b>(cli: App<'a, 'b>) -> Self {
        let store = cli.get_matches();

        let bind: TCPBind = match (
            store.value_of("bind"),
            store.value_of("addr"),
            store.value_of("port"),
        ) {
            (Some(bind), _, _) => TCPBind::Simple(Box::from(bind)),
            (None, Some(addr), Some(port)) => {
                TCPBind::Complex((Box::from(addr), port.parse::<u16>().unwrap()))
            }
            _ => unreachable!(),
        };

        let server_name = store.value_of("name").unwrap_or("httpd").to_string();

        let base_dir = PathBuf::from(store.value_of("base").unwrap());

        Self {
            bind,
            server_name,
            base_dir,
        }
    }

    pub fn bind(&self) -> &impl ToSocketAddrs {
        &self.bind
    }

    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir.as_path()
    }
}

enum TCPBind {
    Simple(Box<str>),
    Complex((Box<str>, u16)),
}

impl ToSocketAddrs for TCPBind {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        match self {
            Self::Simple(s) => s.to_socket_addrs(),
            Self::Complex((s, p)) => (s.as_ref(), *p).to_socket_addrs(),
        }
    }
}
