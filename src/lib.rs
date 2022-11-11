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
	pub fn new(create_info: &InstanceCreateInfo) -> Result<Arc<Self>, InstanceCreateError>
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

		Ok(Arc::new(Self
		{
			entry,
			handle,
			debug_utils,
			utils_messenger,
			api_version: create_info.app_info.api_version
		}))
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
	pub fn new(physical_device: Arc<PhysicalDevice>, create_info: &DeviceCreateInfo) -> Result<Arc<Self>, ()>
	{
		let layer_names: Vec<std::ffi::CString> = create_info.layers
			.iter()
			.map(|layer_name| std::ffi::CString::new(layer_name.as_str()).unwrap())
			.collect();
		let layer_name_pointers: Vec<*const i8> = layer_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

		let extension_names: Vec<std::ffi::CString> = create_info.extensions
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
			.enabled_extension_names(&extension_name_pointers)
			.enabled_features(&create_info.enabled_features)
			.build();

		let handle = unsafe { physical_device.instance.create_device(physical_device.handle, &device_create_info, None) }.unwrap();

		Ok(Arc::new(Self
		{
			handle,
			physical_device: physical_device.clone(),
			api_version: physical_device.api_version
		}))
	}

	pub fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> Result<Arc<Queue>, ()>
	{
		let handle = unsafe { self.handle.get_device_queue(queue_family_index, queue_index) };
		Queue::from_handle(handle, queue_family_index, queue_index)
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

impl Surface
{
	pub fn new(instance: &Instance, window: &qpl::Window) -> Result<Arc<Self>, ()>
	{
		let handle = window.vk_create_surface(&instance.entry, &instance.handle, None);
		let loader = ash::extensions::khr::Surface::new(&instance.entry, &instance.handle);

		Ok(Arc::new(Surface { handle, loader, instance: Arc::new(instance.handle.clone()) }))
	}
}

#[derive(Clone, Debug)]
pub struct SwapchainDetails
{
	pub capabilities: ash::vk::SurfaceCapabilitiesKHR,
	pub formats: Vec<ash::vk::SurfaceFormatKHR>,
	pub modes: Vec<ash::vk::PresentModeKHR>
}

#[derive(Clone)]
pub struct SwapchainCreateInfoKHR
{
	pub min_image_count: u32,
	pub image_format: ash::vk::Format,
	pub image_color_space: ash::vk::ColorSpaceKHR,
	pub image_extent: ash::vk::Extent2D,
	pub image_array_layers: u32,
	pub image_usage: ash::vk::ImageUsageFlags,
	pub image_sharing_mode: ash::vk::SharingMode,
	pub queue_family_indices: Vec<u32>,
	pub pre_transform: ash::vk::SurfaceTransformFlagsKHR,
	pub composite_alpha: ash::vk::CompositeAlphaFlagsKHR,
	pub present_mode: ash::vk::PresentModeKHR,
	pub clipped: bool,
	pub old_swapchain: Option<Arc<Swapchain>>
}

#[derive(Clone)]
pub struct Swapchain
{
	handle: ash::vk::SwapchainKHR,
	loader: ash::extensions::khr::Swapchain,
	surface: Arc<Surface>
}

impl Swapchain
{
	pub fn new(device: &Arc<Device>, surface: &Arc<Surface>, create_info: &SwapchainCreateInfoKHR) -> Result<Arc<Self>, ()>
	{
		let swapchain_create_info = ash::vk::SwapchainCreateInfoKHR::builder()
			.surface(surface.handle)
			.min_image_count(create_info.min_image_count)
			.image_format(create_info.image_format)
			.image_color_space(create_info.image_color_space)
			.image_extent(create_info.image_extent)
			.image_array_layers(create_info.image_array_layers)
			.image_usage(create_info.image_usage)
			.image_sharing_mode(create_info.image_sharing_mode)
			.queue_family_indices(&create_info.queue_family_indices)
			.pre_transform(create_info.pre_transform)
			.composite_alpha(create_info.composite_alpha)
			.present_mode(create_info.present_mode);
		let loader = ash::extensions::khr::Swapchain::new(&surface.instance, &device.handle);
		let handle = unsafe { loader.create_swapchain(&swapchain_create_info, None).unwrap() };

		Self::from_handle(handle, loader, surface.clone())
	}

	pub fn from_handle(handle: ash::vk::SwapchainKHR, loader: ash::extensions::khr::Swapchain, surface: Arc<Surface>) -> Result<Arc<Self>, ()>
	{
		Ok(Arc::new(Self { handle, loader, surface }))
	}

	pub fn get_images(&self) -> Result<Vec<Arc<Image>>, ()>
	{
		let handles = unsafe { self.loader.get_swapchain_images(self.handle) }.unwrap();
		let mut images: Vec<Arc<Image>> = Vec::with_capacity(handles.len());
		for handle in handles.iter()
		{
			images.push(Arc::new(Image { handle: *handle }));
		}
		Ok(images)
	}
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Queue
{
	handle: ash::vk::Queue,
	family_index: u32,
	index: u32
}

impl Queue
{
	pub fn from_handle(handle: ash::vk::Queue, family_index: u32, index: u32) -> Result<Arc<Self>, ()>
	{
		Ok(Arc::new(Self { handle, family_index, index }))
	}

	pub fn family_index(&self) -> u32 { self.family_index }

	pub fn index(&self) -> u32 { self.index }
}

#[derive(Clone, Debug)]
pub struct Image
{
	handle: ash::vk::Image
}

impl Image
{
	pub fn from_handle(handle: ash::vk::Image) -> Result<Arc<Self>, ()>
	{
		Ok(Arc::new(Self { handle }))
	}
}

#[derive(Clone)]
pub struct ImageViewCreateInfo<'a>
{
	pub image: &'a Arc<Image>,
	pub view_type: ash::vk::ImageViewType,
	pub format: ash::vk::Format,
	pub components: ash::vk::ComponentMapping,
	pub subresource_range: ash::vk::ImageSubresourceRange
}

#[derive(Clone, Debug)]
pub struct ImageView
{
	handle: ash::vk::ImageView
}

impl ImageView
{
	pub fn new(device: &Arc<Device>, create_info: &ImageViewCreateInfo) -> Result<Arc<Self>, ()>
	{
		let image_view_create_info = ash::vk::ImageViewCreateInfo::builder()
			.image(create_info.image.handle)
			.view_type(create_info.view_type)
			.format(create_info.format)
			.components(create_info.components)
			.subresource_range(create_info.subresource_range);

		let handle = unsafe { device.handle.create_image_view(&image_view_create_info, None) }.unwrap();

		Ok(Arc::new(ImageView { handle }))
	}

	pub fn from_handle(handle: ash::vk::ImageView) -> Result<Arc<Self>, ()>
	{
		Ok(Arc::new(Self { handle }))
	}
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum LoadOp
{
	Load,
	Clear,
	DontCare
}

impl LoadOp
{
	fn to_ash_type(&self) -> ash::vk::AttachmentLoadOp
	{
		match *self
		{
			Self::Load => ash::vk::AttachmentLoadOp::LOAD,
			Self::Clear => ash::vk::AttachmentLoadOp::CLEAR,
			Self::DontCare => ash::vk::AttachmentLoadOp::DONT_CARE
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum StoreOp
{
	Store,
	DontCare
}

impl StoreOp
{
	fn to_ash_type(&self) -> ash::vk::AttachmentStoreOp
	{
		match *self
		{
			Self::Store => ash::vk::AttachmentStoreOp::STORE,
			Self::DontCare => ash::vk::AttachmentStoreOp::DONT_CARE
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct AttachmentDescription
{
	pub format: ash::vk::Format,
	pub samples: u32,
	pub load_op: LoadOp,
	pub store_op: StoreOp,
	pub stencil_load_op: LoadOp,
	pub stencil_store_op: StoreOp,
	pub initial_layout: ash::vk::ImageLayout,
	pub final_layout: ash::vk::ImageLayout
}

impl AttachmentDescription
{
	fn to_ash_type(&self) -> ash::vk::AttachmentDescription
	{
		ash::vk::AttachmentDescription::builder()
			.format(self.format)
			.samples(self.sample_count_flags().unwrap())
			.load_op(self.load_op.to_ash_type())
			.store_op(self.store_op.to_ash_type())
			.stencil_load_op(self.stencil_load_op.to_ash_type())
			.stencil_store_op(self.stencil_store_op.to_ash_type())
			.initial_layout(self.initial_layout)
			.final_layout(self.final_layout)
			.build()
	}

	fn sample_count_flags(&self) -> Result<ash::vk::SampleCountFlags, ()>
	{
		match self.samples
		{
			1 => Ok(ash::vk::SampleCountFlags::TYPE_1),
			2 => Ok(ash::vk::SampleCountFlags::TYPE_2),
			4 => Ok(ash::vk::SampleCountFlags::TYPE_4),
			8 => Ok(ash::vk::SampleCountFlags::TYPE_8),
			16 => Ok(ash::vk::SampleCountFlags::TYPE_16),
			32 => Ok(ash::vk::SampleCountFlags::TYPE_32),
			64 => Ok(ash::vk::SampleCountFlags::TYPE_64),
			_ => Err(())
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PipelineBindPoint
{
	Graphics,
	Compute,
	RayTracingKhr
}

impl PipelineBindPoint
{
	fn to_ash_type(&self) -> ash::vk::PipelineBindPoint
	{
		match *self
		{
			Self::Graphics => ash::vk::PipelineBindPoint::GRAPHICS,
			Self::Compute => ash::vk::PipelineBindPoint::COMPUTE,
			Self::RayTracingKhr => ash::vk::PipelineBindPoint::RAY_TRACING_KHR
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct AttachmentReference
{
	pub attachment: u32,
	pub layout: ash::vk::ImageLayout
}

impl AttachmentReference
{
	fn to_ash_type(&self) -> ash::vk::AttachmentReference
	{
		ash::vk::AttachmentReference
		{
			attachment: self.attachment,
			layout: self.layout
		}
	}
}

#[derive(Clone, Debug)]
pub struct SubpassDescription
{
	pub pipeline_bind_point: PipelineBindPoint,
	pub input_attachments: Vec<AttachmentReference>,
	pub color_attachments: Vec<AttachmentReference>,
	pub resolve_attachments: Vec<AttachmentReference>,
	pub depth_stencil_attachment: Option<AttachmentReference>,
	pub preserve_attachments: Vec<u32>
}

impl Default for SubpassDescription
{
	fn default() -> Self
	{
		Self
		{
			pipeline_bind_point: PipelineBindPoint::Graphics,
			input_attachments: Vec::new(),
			color_attachments: Vec::new(),
			resolve_attachments: Vec::new(),
			depth_stencil_attachment: None,
			preserve_attachments: Vec::new()
		}	
	}
}

impl SubpassDescription
{
	fn to_vk_subpass_description(&self) -> VkSubpassDescription
	{
		VkSubpassDescription
		{
			pipeline_bind_point: self.pipeline_bind_point.to_ash_type(),
			input_attachments: self.input_attachments.iter().map(|attachment| attachment.to_ash_type()).collect(),
			color_attachments: self.color_attachments.iter().map(|attachment| attachment.to_ash_type()).collect(),
			resolve_attachments: self.resolve_attachments.iter().map(|attachment| attachment.to_ash_type()).collect(),
			depth_stencil_attachment: match self.depth_stencil_attachment
			{
				Some(depth_stencil_attachment) =>
				{
					Some(depth_stencil_attachment.to_ash_type())
				},
				None => None
			},
			preserve_attachments: self.preserve_attachments.clone(),
		}
	}
}

struct VkSubpassDescription
{
	pub pipeline_bind_point: ash::vk::PipelineBindPoint,
	pub input_attachments: Vec<ash::vk::AttachmentReference>,
	pub color_attachments: Vec<ash::vk::AttachmentReference>,
	pub resolve_attachments: Vec<ash::vk::AttachmentReference>,
	pub depth_stencil_attachment: Option<ash::vk::AttachmentReference>,
	pub preserve_attachments: Vec<u32>
}

impl VkSubpassDescription
{
	fn to_ash_type(&self) -> ash::vk::SubpassDescription
	{
		let mut result = ash::vk::SubpassDescription::builder()
			.pipeline_bind_point(self.pipeline_bind_point)
			.input_attachments(&self.input_attachments)
			.color_attachments(&self.color_attachments)
			.resolve_attachments(&self.resolve_attachments)
			.preserve_attachments(&self.preserve_attachments)
			.build();
		
		match self.depth_stencil_attachment
		{
			Some(depth_stencil_attachment) =>
			{
				result.p_depth_stencil_attachment = &depth_stencil_attachment;
			}
			None => {}
		};

		result
	}
}

#[derive(Clone, Copy, Debug)]
pub struct SubpassDependency
{
	pub src_subpass: u32,
	pub dst_subpass: u32,
	pub src_stage_mask: ash::vk::PipelineStageFlags,
	pub dst_stage_mask: ash::vk::PipelineStageFlags,
	pub src_access_mask: ash::vk::AccessFlags,
	pub dst_access_mask: ash::vk::AccessFlags
}

impl Default for SubpassDependency
{
	fn default() -> Self
	{
		Self
		{
			src_subpass: u32::default(),
			dst_subpass: u32::default(),
			src_stage_mask: ash::vk::PipelineStageFlags::empty(),
			dst_stage_mask: ash::vk::PipelineStageFlags::empty(),
			src_access_mask: ash::vk::AccessFlags::empty(),
			dst_access_mask: ash::vk::AccessFlags::empty()
		}	
	}
}

impl SubpassDependency
{
	fn to_ash_type(&self) -> ash::vk::SubpassDependency
	{
		ash::vk::SubpassDependency::builder()
			.src_subpass(self.src_subpass)
			.dst_subpass(self.dst_subpass)
			.src_stage_mask(self.src_stage_mask)
			.dst_stage_mask(self.dst_stage_mask)
			.src_access_mask(self.src_access_mask)
			.dst_access_mask(self.dst_access_mask)
			.build()
	}
}

#[derive(Clone, Debug)]
pub struct RenderPassCreateInfo
{
	pub attachments: Vec<AttachmentDescription>,
	pub subpasses: Vec<SubpassDescription>,
	pub dependencies: Vec<SubpassDependency>
}

#[derive(Clone, Debug)]
pub struct RenderPass
{
	handle: ash::vk::RenderPass
}

impl RenderPass
{
	pub fn new(device: &Arc<Device>, create_info: &RenderPassCreateInfo) -> Result<Arc<RenderPass>, ()>
	{
		let attachments: Vec<ash::vk::AttachmentDescription> = create_info.attachments.iter().map(|attachment| attachment.to_ash_type()).collect();
		
		let vk_subpasses: Vec<VkSubpassDescription> = create_info.subpasses.iter().map(|subpass| subpass.to_vk_subpass_description()).collect();
		let subpasses: Vec<ash::vk::SubpassDescription> = vk_subpasses.iter().map(|subpass| subpass.to_ash_type()).collect();

		let dependencies: Vec<ash::vk::SubpassDependency> = create_info.dependencies.iter().map(|dependency| dependency.to_ash_type()).collect();

		let renderpass_create_info = ash::vk::RenderPassCreateInfo::builder()
			.attachments(&attachments)
			.subpasses(&subpasses)
			.dependencies(&dependencies);
		let handle = unsafe { device.handle.create_render_pass(&renderpass_create_info, None) }.unwrap();

		Ok(Arc::new(RenderPass { handle }))
	}
}

/*

*/