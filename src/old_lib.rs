use napi::{
  CallContext, Env, JsBoolean, JsObject, JsString, JsUndefined, Property,
  Result as NapiResult,
};

use virt::{connect::Connect, domain::Domain};

// Add them when we need them
// use log::{error, info, warn};
use napi_derive::js_function;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Libvirt {
  conn: Option<String>,
  connection: Option<Connect>
}

#[napi]
impl Libvirt {
  pub fn new() -> Self {
    Self { conn: None, connection: None }
  }

  #[napi]
  pub fn connect(&mut self, uri: String) -> Result<(), napi::Error> {
    // disconnect current connection if is not None
    if let Some(ref mut conn) = self.connection {
        if let Err(e) = conn.close() {
            eprintln!("Failed to close existing connection: {}", e);
        }
        self.connection = None;
    }
    match Connect::open(&uri) {
      Ok(con_result) => {
        self.conn = Some(uri.to_string());
        self.connection = Some(con_result);
        Ok(())
      }
      Err(e) => Err(napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to connect: {}", e),
      )),
    }
  }

  #[napi]
  pub fn list_domains(&self) -> Result<Vec<String>, napi::Error> {
    let connection_result = self.get_connection();
    match connection_result {
        Ok(connection) => {
            let domains_result = connection.list_defined_domains();
            match domains_result {
                Ok(list) => Ok(list),
                Err(err) => {
                    Err(napi::Error::new(
                        napi::Status::GenericFailure,
                        format!("Error: {}", err)
                    ))
                }
            }
        },
        Err(err) => Err(err)
    }
  }

  pub fn get_connection(&self) -> Result<&Connect, napi::Error> {
    match &self.connection {
        Some(con) => {
            if con.is_alive().unwrap_or(false) {
                return Ok(con)
            } else {
                Err(napi::Error::new(
                    napi::Status::GenericFailure,
                    format!("Connection is not alive"),
                ))
            }
        },
        None => Err(napi::Error::new(
            napi::Status::GenericFailure,
            format!("Connection not opened"),
        )),
    }
  }

  pub fn get_dommain(&self, name: String) -> Result<Domain, napi::Error> {
    let conn = self.get_connection();
    match conn {
      Ok(connection) => {
        let machine = Domain::lookup_by_name(&connection, &name.to_owned());
        match machine {
          Ok(result) => Ok(result),
          Err(err) => {
            Err(napi::Error::new(
              napi::Status::GenericFailure,
              format!("Operation failed: {}", err)
            ))
          }
        }
      }
      Err(error) => Err(error),
    }
  }
}

#[js_function(0)]
pub fn libvirt_constructor(ctx: CallContext) -> NapiResult<JsUndefined> {
  let mut this: JsObject = ctx.this_unchecked();
  ctx.env.wrap(&mut this, Libvirt::new())?;
  ctx.env.get_undefined()
}

#[js_function(2)]
pub fn libvirt_connect(ctx: CallContext) -> NapiResult<JsBoolean> { // Change return type to JsBoolean
  let this: JsObject = ctx.this_unchecked();
  let lib: &mut Libvirt = ctx.env.unwrap(&this)?;
  let uri = ctx.get::<JsString>(0)?;
  let uri_str = uri.into_utf8()?.as_str()?.to_owned();
  let connect_result = lib.connect(uri_str);
  match connect_result {
    Ok(_) => ctx.env.get_boolean(true), // Convert Rust bool to JsBoolean
    Err(e) => Err(e),
  }
}

#[js_function(1)]
pub fn libvirt_list_machines(ctx: CallContext) -> NapiResult<JsObject> {
  let this: JsObject = ctx.this_unchecked();
  let lib: &mut Libvirt = ctx.env.unwrap(&this)?;
  let machines = lib.list_domains();
  match machines {
    Ok(domains) => {
      let mut array = ctx.env.create_array_with_length(domains.len())?;
      for (i, domain) in domains.iter().enumerate() {
        let js_string = ctx.env.create_string(&domain)?;
        array.set_element(i as u32, js_string)?;
      }
      Ok(array)
    }
    Err(e) => Err(napi::Error::new(
      napi::Status::GenericFailure,
      format!("Failed to list domains: {}", e),
    )),
  }
}
#[js_function(1)] // machine_name:str
pub fn libvirt_suspend(ctx: CallContext) -> NapiResult<JsBoolean> {
  let this: JsObject = ctx.this_unchecked();
  let lib: &mut Libvirt = ctx.env.unwrap(&this)?;
  let name = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
  let machine_result = lib.get_dommain(name);
  match machine_result {
    Ok(machine) => {
        let result = machine.suspend();
        match result {
            Ok(_) => {
                ctx.env.get_boolean(true)
            },
            Err(err) => {
                Err(napi::Error::new(
                    napi::Status::GenericFailure,
                    format!("Operation failed: {}", err)
                ))
            }
        }
    },
    Err(err) => {
        Err(napi::Error::new(
            napi::Status::GenericFailure,
            format!("Machine not found: {}", err)
        ))
    }
  }
}

#[js_function(1)] // machine_string
pub fn libvirt_power_resume(ctx: CallContext) -> NapiResult<JsBoolean> {
    let this: JsObject = ctx.this_unchecked();
  let lib: &mut Libvirt = ctx.env.unwrap(&this)?;
  let name = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
  let machine_result = lib.get_dommain(name);
  match machine_result {
    Ok(machine) => {
        let result = machine.resume();
        match result {
            Ok(_) => {
                ctx.env.get_boolean(true)
            },
            Err(err) => {
                Err(napi::Error::new(
                    napi::Status::GenericFailure,
                    format!("Operation failed: {}", err)
                ))
            }
        }
    },
    Err(err) => {
        Err(napi::Error::new(
            napi::Status::GenericFailure,
            format!("Machine not found: {}", err)
        ))
    }
  }
}

#[js_function(2)] //machine_name:str, acpi: bool param, optional, default false
pub fn libvirt_power_on(ctx: CallContext) -> NapiResult<JsBoolean> {
    let this: JsObject = ctx.this_unchecked();
  let lib: &mut Libvirt = ctx.env.unwrap(&this)?;
  let name = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
  let machine_result = lib.get_dommain(name);
  match machine_result {
    Ok(machine) => {
        let result = machine.create();
        match result {
            Ok(_) => {
                ctx.env.get_boolean(true)
            },
            Err(err) => {
                Err(napi::Error::new(
                    napi::Status::GenericFailure,
                    format!("Operation failed: {}", err)
                ))
            }
        }
    },
    Err(err) => {
        Err(napi::Error::new(
            napi::Status::GenericFailure,
            format!("Machine not found: {}", err)
        ))
    }
  }
}

#[js_function(2)] //machine_name:str, acpi: bool param, optional, default false
pub fn libvirt_power_off(ctx: CallContext) -> NapiResult<JsBoolean> {
  let this: JsObject = ctx.this_unchecked();
  let lib: &mut Libvirt = ctx.env.unwrap(&this)?;
  let name = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
  let acpi_js: JsBoolean = ctx.get::<JsBoolean>(1)?; // Ensure to use the correct argument index
  let acpi: bool = acpi_js.get_value()?; // Correct method to convert JsBoolean to bool
  let machine_result = lib.get_dommain(name);
  match machine_result {
    Ok(machine) => {
        if acpi {
            match machine.shutdown() {
                Ok(_) => ctx.env.get_boolean(true),
                Err(err) => {
                    Err(napi::Error::new(
                        napi::Status::GenericFailure,
                        format!("Operation failed: {}", err)
                    ))
                }
            }
        } else {
            match machine.destroy() {
                Ok(_) => ctx.env.get_boolean(true),
                Err(err) => {
                    Err(napi::Error::new(
                        napi::Status::GenericFailure,
                        format!("Operation failed: {}", err)
                    ))
                }
            }
        }
    },
    Err(err) => {
        Err(napi::Error::new(
            napi::Status::GenericFailure,
            format!("Machine not found: {}", err)
        ))
    }
  }
}

#[js_function(2)] // xml:str
pub fn libvirt_define_xml(ctx: CallContext) -> NapiResult<JsBoolean> {
  let this: JsObject = ctx.this_unchecked();
  let lib: &mut Libvirt = ctx.env.unwrap(&this)?;
  let xml = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
  let connection_result = lib.get_connection();
  match connection_result {
    Ok(connection) => {
        let result = Domain::define_xml(&connection, &xml.to_owned()); // Adjusted to two arguments
        match result {
            Ok(_) => ctx.env.get_boolean(true),
            Err(err) => {
                Err(napi::Error::new(
                    napi::Status::GenericFailure,
                    format!("Operation failed: {}", err)
                ))
            }
        }
    },
    Err(err) => {
        Err(err)
    }
  }
}

#[js_function(1)] // machine_name:str
pub fn libvirt_destroy(ctx: CallContext) -> NapiResult<JsBoolean> {
    let this: JsObject = ctx.this_unchecked();
    let lib: &mut Libvirt = ctx.env.unwrap(&this)?;
    let name = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
    let machine_result = lib.get_dommain(name);
    match machine_result {
      Ok(machine) => {
          let result = machine.undefine();
          match result {
              Ok(_) => {
                  ctx.env.get_boolean(true)
              },
              Err(err) => {
                  Err(napi::Error::new(
                      napi::Status::GenericFailure,
                      format!("Operation failed: {}", err)
                  ))
              }
          }
      },
      Err(err) => {
          Err(napi::Error::new(
              napi::Status::GenericFailure,
              format!("Machine not found: {}", err)
          ))
      }
    }
}

#[js_function(1)] // machine_name:str
pub fn libvirt_get_domain_info(ctx: CallContext) -> NapiResult<JsObject> {
    let this: JsObject = ctx.this_unchecked();
    let lib: &mut Libvirt = ctx.env.unwrap(&this)?;
    let name = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
    let machine_result = lib.get_dommain(name);
    match machine_result {
      Ok(machine) => {

        let mut info_object = ctx.env.create_object()?;
        let id = machine.get_id().unwrap_or(0);
        let name = machine.get_name().unwrap_or("Unkown".to_string());
        let state = machine.get_state().unwrap_or((0, 0)).0;
        let memory = machine.get_max_memory().unwrap_or(0);
        let vcpus = machine.get_max_vcpus().unwrap_or(0);
        let os_type = machine.get_os_type().unwrap_or("Unkown".to_string());
        let hostname = machine.get_hostname(0).unwrap_or("Unkown".to_string());
        let uuid = machine.get_uuid_string().unwrap_or("Unknown".to_string());
        let is_active = machine.is_active().unwrap_or(false);
        
        // Create the hash with all the above info
        info_object.set_named_property("id", ctx.env.create_uint32(id)?)?;
        info_object.set_named_property("name", ctx.env.create_string(&name)?)?;
        info_object.set_named_property("state", ctx.env.create_uint32(state as u32)?)?;
        info_object.set_named_property("memory", ctx.env.create_double(memory as f64)?)?; // NOTE: Not sure about the casting
        info_object.set_named_property("vcpus", ctx.env.create_double(vcpus as f64)?)?; // NOTE: Not sure about the casting
        info_object.set_named_property("os_type", ctx.env.create_string(&os_type)?)?;
        info_object.set_named_property("hostname", ctx.env.create_string(&hostname)?)?;
        info_object.set_named_property("uuid", ctx.env.create_string(&uuid)?)?;
        info_object.set_named_property("is_active", ctx.env.get_boolean(is_active))?;
        Ok(info_object)
      },
      Err(err) => {
          Err(napi::Error::new(
              napi::Status::GenericFailure,
              format!("Machine not found: {}", err)
          ))
      }
    }
}


// Add the export code to create the class Libvirt with all the instance methods
#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<(), napi::Error> {
    let libvirt_class = env.define_class(
        "LibvirtInternal",
        libvirt_constructor,
        &[
            Property::new("connect")?.with_method(libvirt_connect),
            Property::new("listMachines")?.with_method(libvirt_list_machines),
            Property::new("suspendMachine")?.with_method(libvirt_suspend),
            Property::new("getDomainInfo")?.with_method(libvirt_get_domain_info),
            Property::new("powerOn")?.with_method(libvirt_power_resume),
            Property::new("powerOff")?.with_method(libvirt_power_off),
            Property::new("defineXML")?.with_method(libvirt_define_xml),
        ],
    )?;
    exports.set_named_property("Libvirt", libvirt_class)?;
    
    Ok(())
}
