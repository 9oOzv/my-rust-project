use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use std::sync::Arc;

fn create_instance() -> Arc<Instance> {
    let library = VulkanLibrary::new()
        .expect("no local Vulkan library/DLL");
    let instance =
        Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::empty(),
                ..Default::default()
            }
        )
        .expect("failed to create instance");
    return instance;
}

fn find_devices(instance: Arc<Instance>) {
    let devices = instance
        .physical_devices()
        .expect("failed to find physical devices");
    return devices
}


fn main() {
    let instance = create_instance();
}
