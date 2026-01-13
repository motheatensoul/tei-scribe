use super::{RelaxNgValidator, XsdValidator};
use crate::errors::{Result, SagaError};
use crate::validator::relaxng::RelaxNgSchema;
use crate::validator::ValidationResult;
use std::collections::HashMap;
use std::panic;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use tokio::sync::oneshot;

pub struct ValidationActor {
    rx: Receiver<ValidationRequest>,
    rng_cache: HashMap<PathBuf, RelaxNgSchema>,
    // TODO: Add XSD cache when we implement XSD reuse
}

pub enum ValidationRequest {
    Xsd {
        xml: String,
        schema_path: PathBuf,
        reply: oneshot::Sender<Result<ValidationResult>>,
    },
    XsdString {
        xml: String,
        schema_content: String,
        schema_name: String,
        reply: oneshot::Sender<Result<ValidationResult>>,
    },
    RelaxNg {
        xml: String,
        schema_path: PathBuf,
        reply: oneshot::Sender<Result<ValidationResult>>,
    },
    RelaxNgString {
        xml: String,
        schema_content: String,
        schema_name: String,
        reply: oneshot::Sender<Result<ValidationResult>>,
    },
}

#[derive(Clone)]
pub struct ValidationSender(pub Sender<ValidationRequest>);

impl ValidationActor {
    pub fn spawn() -> ValidationSender {
        let (tx, rx) = channel();

        thread::Builder::new()
            .name("validation-actor".into())
            .spawn(move || {
                let mut actor = ValidationActor {
                    rx,
                    rng_cache: HashMap::new(),
                };
                actor.run();
            })
            .expect("Failed to spawn validation actor thread");

        ValidationSender(tx)
    }

    fn run(&mut self) {
        while let Ok(req) = self.rx.recv() {
            let start = std::time::Instant::now();
            
            match req {
                ValidationRequest::Xsd { xml, schema_path, reply } => {
                    log::info!("Starting XSD validation against {:?}", schema_path);
                    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                        XsdValidator::validate(&xml, &schema_path)
                            .map_err(SagaError::Validation)
                    }));
                    
                    match result {
                        Ok(res) => {
                            log::info!("XSD validation took {:?}", start.elapsed());
                            let _ = reply.send(res);
                        }
                        Err(_) => {
                            log::error!("XSD validation panicked");
                            let _ = reply.send(Err(SagaError::Validation("XSD validation panicked".to_string())));
                        }
                    }
                }
                ValidationRequest::XsdString { xml, schema_content, schema_name, reply } => {
                    log::info!("Starting XSD string validation: {}", schema_name);
                    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                        XsdValidator::validate_with_schema_string(&xml, &schema_content, &schema_name)
                            .map_err(SagaError::Validation)
                    }));
                    
                    match result {
                        Ok(res) => {
                            log::info!("XSD string validation took {:?}", start.elapsed());
                            let _ = reply.send(res);
                        }
                        Err(_) => {
                            log::error!("XSD string validation panicked");
                            let _ = reply.send(Err(SagaError::Validation("XSD string validation panicked".to_string())));
                        }
                    }
                }
                ValidationRequest::RelaxNg { xml, schema_path, reply } => {
                    log::info!("Starting RelaxNG validation against {:?}", schema_path);
                    
                    // Check cache first
                    if !self.rng_cache.contains_key(&schema_path) {
                        log::info!("Cache miss for RelaxNG schema: {:?}", schema_path);
                        match RelaxNgSchema::parse_file(&schema_path) {
                            Ok(schema) => {
                                self.rng_cache.insert(schema_path.clone(), schema);
                            }
                            Err(e) => {
                                let _ = reply.send(Err(SagaError::Validation(e)));
                                continue;
                            }
                        }
                    } else {
                        log::info!("Cache hit for RelaxNG schema: {:?}", schema_path);
                    }

                    // Get schema from cache
                    if let Some(schema) = self.rng_cache.get(&schema_path) {
                        let schema_name = schema_path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                        
                        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                            schema.validate(&xml, &schema_name)
                                .map_err(SagaError::Validation)
                        }));
                        
                        match result {
                            Ok(res) => {
                                log::info!("RelaxNG validation took {:?}", start.elapsed());
                                let _ = reply.send(res);
                            }
                            Err(_) => {
                                log::error!("RelaxNG validation panicked");
                                let _ = reply.send(Err(SagaError::Validation("RelaxNG validation panicked".to_string())));
                            }
                        }
                    }
                }
                ValidationRequest::RelaxNgString { xml, schema_content, schema_name, reply } => {
                    log::info!("Starting RelaxNG string validation: {}", schema_name);
                    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                        RelaxNgValidator::validate_with_schema_string(&xml, &schema_content, &schema_name)
                             .map_err(SagaError::Validation)
                    }));
                    
                    match result {
                        Ok(res) => {
                            log::info!("RelaxNG string validation took {:?}", start.elapsed());
                            let _ = reply.send(res);
                        }
                        Err(_) => {
                            log::error!("RelaxNG string validation panicked");
                            let _ = reply.send(Err(SagaError::Validation("RelaxNG string validation panicked".to_string())));
                        }
                    }
                }
            }
        }
    }
}
