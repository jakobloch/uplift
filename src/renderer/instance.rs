use ash::{vk, Entry, Instance};
use std::error::Error;
use std::ffi::CString;

pub struct AshInstance {
    pub entry: Entry,
    pub instance: Instance,
}

impl AshInstance {
    pub fn new(app_name: &str) -> Result<Self, Box<dyn Error>> {
        let entry = unsafe { Entry::load()? };
        let app_name = CString::new(app_name)?;
        let engine_name = CString::new("AshEngine")?;

        let app_info = vk::ApplicationInfo::default()
            .application_name(&app_name)
            .application_version(0)
            .engine_name(&engine_name)
            .engine_version(0)
            .api_version(vk::API_VERSION_1_3);

        let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        Ok(AshInstance { entry, instance })
    }

    pub fn destroy(&self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

impl Drop for AshInstance {
    fn drop(&mut self) {
        self.destroy();
    }
}
