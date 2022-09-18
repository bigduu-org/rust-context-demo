use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime::Runtime;
use crate::configuration::context_configuration::ContextConfiguration;
use crate::context::{ApplicationContext, DefaultApplicationContext};

static ATOMIC_ID: AtomicUsize = AtomicUsize::new(1);


#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ApplicationBuilder {
    configuration: Option<ContextConfiguration>,
}

impl ApplicationBuilder {
    pub fn builder() -> &'static ApplicationBuilder {
        &ApplicationBuilder {
            configuration: None
        }
    }

    pub fn code_builder(&self) -> CodeApplicationBuilder {
        CodeApplicationBuilder {
            ..Default::default()
        }
    }

    pub fn resource_builder(&self) -> ResourceApplicationBuilder {
        let configuration = ContextConfiguration::load_properties();
        ResourceApplicationBuilder {
            configuration
        }
    }

    pub fn build(&self) -> Box<DefaultApplicationContext> {
        let configuration = ContextConfiguration::load_properties();
        let _ = init_program(&configuration);
        Box::new(DefaultApplicationContext { configuration })
    }
}

#[derive(Debug, Default)]
pub struct ResourceApplicationBuilder {
    configuration: ContextConfiguration,
}

impl ResourceApplicationBuilder {
    pub fn build(&self) -> (Box<DefaultApplicationContext>, Runtime) {
        let configuration = self.configuration.clone();
        let runtime = init_program(&configuration);
        (Box::new(DefaultApplicationContext {
            configuration,
        }), runtime)
    }
}


#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct CodeApplicationBuilder {
    configuration: ContextConfiguration,
}

fn init_log4rs(configuration: &ContextConfiguration) {
    log4rs::init_file(
        configuration.properties.bigduu.log4rs.resources.clone(),
        Default::default(),
    ).expect("Init log4rs fail")
}

fn gen_thread_fn() -> String {
    let _id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
    let thread_name = format!("thread-pool-{}", _id);
    thread_name
}

fn build_tokio_runtime(_configuration: &ContextConfiguration) -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(_configuration.properties.bigduu.tokio.workers as usize)
        .thread_name_fn(gen_thread_fn)
        .enable_all()
        .build()
        .expect("Create runtime error")
}

fn init_program(configuration: &ContextConfiguration) -> Runtime {
    init_log4rs(configuration);
    build_tokio_runtime(configuration)
}
