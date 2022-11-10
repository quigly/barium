use std::sync::Arc;

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> ash::vk::Bool32 {
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    println!("[Debug][{}][{}] {:?}", severity, ty, message);
    ash::vk::FALSE
}

#[derive(Clone, Copy, Debug)]
pub struct Version
{
	pub major: u32,
	pub minor: u32,
	pub patch: u32
}

impl Version
{
	pub const V1_0: Self = Version::new(1, 0, 0);
	pub const V1_1: Self = Version::new(1, 1, 0);
	pub const V1_2: Self = Version::new(1, 2, 0);
	pub const V1_3: Self = Version::new(1, 3, 0);

	pub const fn new(major: u32, minor: u32, patch: u32) -> Self
	{
		Self { major, minor, patch }
	}

	pub fn make_api_version(&self) -> u32
	{
		ash::vk::make_api_version(0, self.major, self.minor, self.patch)
	}

	pub fn from_u32(version: u32) -> Self
	{
		Self
		{
			major: ash::vk::api_version_major(version),
			minor: ash::vk::api_version_minor(version),
			patch: ash::vk::api_version_patch(version)
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum InstanceCreateError
{
	LibraryLoadFailure,
}

#[derive(Clone, Debug)]
pub struct ApplicationInfo
{
	pub application_name: String,
	pub application_version: Version,
	pub engine_name: String,
	pub engine_version: Version,
	pub api_version: Version,
}

impl Default for ApplicationInfo
{
	fn default() -> Self
	{
		Self
		{
			application_name: String::from("Application"),
			application_version: Version::new(1, 0, 0),
			engine_name: "Engine".to_owned(),
			engine_version: Version::new(1, 0, 0),
			api_version: Version::new(1, 0, 0)
		}
	}
}

impl ApplicationInfo
{
	pub fn build(&self) -> ash::vk::ApplicationInfo
	{
		let application_name = std::ffi::CString::new(self.application_name.clone()).unwrap();
		let engine_name = std::ffi::CString::new(self.engine_name.clone()).unwrap();

		ash::vk::ApplicationInfo::builder()
			.application_name(application_name.as_c_str())
			.application_version(self.application_version.make_api_version())
			.engine_name(engine_name.as_c_str())
			.engine_version(self.engine_version.make_api_version())
            .api_version(self.api_version.make_api_version())
            .build()
	}
}

#[derive(Clone, Debug)]
pub struct InstanceCreateInfo
{
	pub app_info: ApplicationInfo,
	pub extensions: Vec<String>,
	pub layers: Vec<String>
}

#[derive(Clone)]
pub struct Instance
{
	entry: ash::Entry,
	handle: ash::Instance,
	debug_utils: ash::extensions::ext::DebugUtils,
	utils_messenger: ash::vk::DebugUtilsMessengerEXT,
	api_version: Version
}

impl Instance
{
	pub fn new(create_info: &InstanceCreateInfo) -> Result<Self, InstanceCreateError>
	{
		let entry = unsafe { ash::Entry::load() };
		if entry.is_err()
		{
			return Err(InstanceCreateError::LibraryLoadFailure);
		}
		let entry = entry.unwrap();

		let application_info = create_info.app_info.build();

		let mut debug_create_info = ash::vk::DebugUtilsMessengerCreateInfoEXT
        {
            message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                //| vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                //| vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            pfn_user_callback: Some(vulkan_debug_utils_callback),
            ..Default::default()
        };

		let layer_names: Vec<std::ffi::CString> =
			create_info.layers
			.iter()
			.map(|layer_name| std::ffi::CString::new(layer_name.as_str()).unwrap())
			.collect();
		let layer_name_pointers: Vec<*const i8> = layer_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

		let extension_names: Vec<std::ffi::CString> =
			create_info.extensions
			.iter()
			.map(|extension_name| std::ffi::CString::new(extension_name.as_str()).unwrap())
			.collect();
		let extension_name_pointers: Vec<*const i8> = extension_names
            .iter()
            .map(|extension_name| extension_name.as_ptr())
            .collect();

		let instance_create_info = ash::vk::InstanceCreateInfo::builder()
			.push_next(&mut debug_create_info)
			.application_info(&application_info)
			.enabled_extension_names(&extension_name_pointers)
			.enabled_layer_names(&layer_name_pointers);

		let handle = unsafe { entry.create_instance(&instance_create_info, None) };
		if handle.is_err()
		{
			dbg!(handle.err());
			panic!();
		}
		let handle = handle.unwrap();
		
		let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &handle);
        let utils_messenger =
        unsafe { debug_utils.create_debug_utils_messenger(&debug_create_info, None).unwrap() };

		Ok(Self
		{
			entry,
			handle,
			debug_utils,
			utils_messenger,
			api_version: create_info.app_info.api_version
		})
	}

	pub fn enumerate_physical_devices(&self) -> Result<Vec<Arc<PhysicalDevice>>, ()>
	{
		let handles = unsafe { self.handle.enumerate_physical_devices() }.unwrap();

		let mut devices: Vec<Arc<PhysicalDevice>> = Vec::with_capacity(handles.len());

		for handle in handles
		{
			devices.push(Arc::new(PhysicalDevice::from_handle(handle, Arc::new(self.handle.clone()))));
		}

		Ok(devices)
	}

	pub fn create_surface(&self, physical_device: &PhysicalDevice, window: &qpl::Window) -> Result<Surface, ()>
	{
		let handle = window.vk_create_surface(&self.entry, &self.handle, None);
		let loader = ash::extensions::khr::Surface::new(&self.entry, &self.handle);

		Ok(Surface { handle, loader, instance: Arc::new(self.handle.clone()) })
	}

	pub fn create_device(&self, physical_device: Arc<PhysicalDevice>, create_info: &DeviceCreateInfo) -> Result<Device, ()>
	{
		let layer_names: Vec<std::ffi::CString> =
			create_info.layers
			.iter()
			.map(|layer_name| std::ffi::CString::new(layer_name.as_str()).unwrap())
			.collect();
		let layer_name_pointers: Vec<*const i8> = layer_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

		let extension_names: Vec<std::ffi::CString> =
		create_info.extensions
			.iter()
			.map(|extension_name| std::ffi::CString::new(extension_name.as_str()).unwrap())
			.collect();
		let extension_name_pointers: Vec<*const i8> = extension_names
            .iter()
            .map(|extension_name| extension_name.as_ptr())
            .collect();

		let mut queue_create_infos: Vec<ash::vk::DeviceQueueCreateInfo> = Vec::with_capacity(create_info.queue_create_infos.len());
		for info in &create_info.queue_create_infos
		{
			queue_create_infos.push(info.to_ash_type());
		}

		let device_create_info = ash::vk::DeviceCreateInfo::builder()
			.queue_create_infos(&queue_create_infos)
			.enabled_layer_names(&layer_name_pointers)
			.enabled_layer_names(&layer_name_pointers)
			.enabled_features(&create_info.enabled_features)
			.build();

		let handle = unsafe { self.handle.create_device(physical_device.handle, &device_create_info, None) }.unwrap();

		Ok(Device
		{
			handle,
			physical_device: physical_device.clone(),
			api_version: physical_device.api_version
		})
	}

	//pub fn create_device(&self, )

	pub fn create_swapchain(&self, physical_device: &PhysicalDevice, surface: &Surface) -> Result<Swapchain, ()>
	{
		todo!()
	}
}

#[derive(Clone)]
pub struct PhysicalDevice
{
	handle: ash::vk::PhysicalDevice,
	instance: Arc<ash::Instance>,
	features: ash::vk::PhysicalDeviceFeatures,
	properties: ash::vk::PhysicalDeviceProperties,
	memory_properties: ash::vk::PhysicalDeviceMemoryProperties,
	queue_family_properties: Vec<ash::vk::QueueFamilyProperties>,
	api_version: Version,
	extension_properties: Vec<ash::vk::ExtensionProperties>
}

impl PhysicalDevice
{
	pub fn from_handle(handle: ash::vk::PhysicalDevice, instance: Arc<ash::Instance>) -> Self
	{
		let features = unsafe { instance.get_physical_device_features(handle) };
		let properties = unsafe { instance.get_physical_device_properties(handle) };
		let memory_properties = unsafe { instance.get_physical_device_memory_properties(handle) };
		let queue_family_properties = unsafe { instance.get_physical_device_queue_family_properties(handle) };
		let api_version = Version::from_u32(properties.api_version);
		let extension_properties = unsafe { instance.enumerate_device_extension_properties(handle) }.unwrap();

		dbg!(&api_version);

		Self
		{
			handle,
			instance,
			features,
			properties,
			memory_properties,
			queue_family_properties,
			api_version,
			extension_properties
		}
	}

	pub fn get_queue_family_properties(&self) -> Vec<ash::vk::QueueFamilyProperties>
	{
		self.queue_family_properties.clone()
	}

	pub fn get_features(&self) -> ash::vk::PhysicalDeviceFeatures
	{
		self.features.clone()
	}

	pub fn get_properties(&self) -> ash::vk::PhysicalDeviceProperties
	{
		self.properties.clone()
	}

	pub fn get_memory_properties(&self) -> ash::vk::PhysicalDeviceMemoryProperties
	{
		self.memory_properties.clone()
	}

	pub fn is_extension_supported(&self, extension_name: &str) -> bool
	{
		todo!()
	}
}

pub struct Device
{
	handle: ash::Device,
	physical_device: Arc<PhysicalDevice>,
	api_version: Version
}

impl Device
{
	pub fn get_queue(&self, queue_family_index: u32) -> Result<Queue, ()>
	{
		let handle = unsafe { self.handle.get_device_queue(queue_family_index, 0) };
		Ok(Queue { handle, family_index: queue_family_index })
	}

	pub fn get_swapchain_details(&self, surface: &Surface) -> Result<SwapchainDetails, ()>
	{
		let capabilities: ash::vk::SurfaceCapabilitiesKHR = unsafe
		{
			surface.loader.get_physical_device_surface_capabilities(self.physical_device.handle, surface.handle)
		}.unwrap();

		let formats: Vec<ash::vk::SurfaceFormatKHR> = unsafe
		{
			surface.loader.get_physical_device_surface_formats(self.physical_device.handle, surface.handle)
		}.unwrap();

		let modes: Vec<ash::vk::PresentModeKHR> = unsafe
		{
			surface.loader.get_physical_device_surface_present_modes(self.physical_device.handle, surface.handle)
		}.unwrap();

		Ok(SwapchainDetails { capabilities, formats, modes })
	} 
}

#[derive(Clone)]
pub struct Surface
{
	handle: ash::vk::SurfaceKHR,
    loader: ash::extensions::khr::Surface,
	instance: Arc<ash::Instance>
}

#[derive(Clone, Debug)]
pub struct SwapchainDetails
{
	pub capabilities: ash::vk::SurfaceCapabilitiesKHR,
	pub formats: Vec<ash::vk::SurfaceFormatKHR>,
	pub modes: Vec<ash::vk::PresentModeKHR>
}

#[derive(Clone)]
pub struct Swapchain
{
	handle: ash::extensions::khr::Swapchain,
	loader: ash::vk::SwapchainKHR,
	images: Vec<Image>,
	views: Vec<ImageView>
}

#[derive(Clone)]
pub struct DeviceQueueCreateInfo<'a>
{
	pub queue_family_index: u32,
	pub queue_count: u32,
	pub queue_properities: &'a f32
}

impl DeviceQueueCreateInfo<'_>
{
	fn to_ash_type(&self) -> ash::vk::DeviceQueueCreateInfo
	{
		ash::vk::DeviceQueueCreateInfo
		{
			p_queue_priorities: self.queue_properities,
			queue_count: self.queue_count,
			queue_family_index: self.queue_family_index,
			..Default::default()
		}
	}
}

#[derive(Clone)]
pub struct DeviceCreateInfo<'a>
{
	pub layers: Vec<String>,
	pub extensions: Vec<String>,
	pub queue_create_infos: Vec<DeviceQueueCreateInfo<'a>>,
	pub enabled_features: ash::vk::PhysicalDeviceFeatures
}

#[derive(Clone)]
pub struct Queue
{
	handle: ash::vk::Queue,
	family_index: u32
}

#[derive(Clone)]
pub struct Image
{
	handle: ash::vk::Image
}

#[derive(Clone)]
pub struct ImageView
{
	handle: ash::vk::ImageView
}

#[derive(Clone)]
pub struct SwapchainCreateInfoKHR<'a>
{
	pub surface: &'a Surface,
	pub min_image_count: u32,
	pub image_format: ash::vk::Format,
	pub image_color_space: ash::vk::ColorSpaceKHR,
	pub image_extent: ash::vk::Extent2D,
	pub image_array_layers: u32,
	pub image_usage: ash::vk::ImageUsageFlags,
	pub image_sharing_mode: ash::vk::SharingMode,
	pub queue_family_indices: &'a [u32],
	pub pre_transform: ash::vk::SurfaceTransformFlagsKHR,
	pub composite_alpha: ash::vk::CompositeAlphaFlagsKHR,
	pub present_mode: ash::vk::PresentModeKHR,
	pub clipped: bool,
	pub old_swapchain: Option<&'a Swapchain>
}