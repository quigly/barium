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

	pub fn handle(&self) -> ash::Device
	{
		self.handle.clone()
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
	const DEFAULT_ATTACHMENT: Self = AttachmentReference
	{
		attachment: 0,
		layout: ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
	};

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
pub struct SubpassDescription<'a>
{
	pub pipeline_bind_point: PipelineBindPoint,
	pub input_attachments: Option<&'a [AttachmentReference]>,
	pub color_attachments: &'a [AttachmentReference],
	pub resolve_attachments: Option<&'a [AttachmentReference]>,
	pub depth_stencil_attachment: Option<AttachmentReference>,
	pub preserve_attachments: Option<&'a [u32]>
}

impl Default for SubpassDescription<'_>
{
	fn default() -> Self
	{
		Self
		{
			pipeline_bind_point: PipelineBindPoint::Graphics,
			input_attachments: None,
			color_attachments: &[AttachmentReference::DEFAULT_ATTACHMENT],
			resolve_attachments: None,
			depth_stencil_attachment: None,
			preserve_attachments: None
		}	
	}
}

impl SubpassDescription<'_>
{
	fn to_vk_subpass_description(&self) -> VkSubpassDescription
	{
		VkSubpassDescription
		{
			pipeline_bind_point: self.pipeline_bind_point.to_ash_type(),
			input_attachments: match self.input_attachments
			{
				Some(input_attachments) =>
				{
					Some(input_attachments.iter().map(|attachment| attachment.to_ash_type()).collect())
				},
				None => { None }
			},
			color_attachments: self.color_attachments.iter().map(|attachment| attachment.to_ash_type()).collect(),
			resolve_attachments: match self.resolve_attachments
			{
				Some(resolve_attachments) =>
				{
					Some(resolve_attachments.iter().map(|attachment| attachment.to_ash_type()).collect())
				},
				None => { None }
			},
			depth_stencil_attachment: match self.depth_stencil_attachment
			{
				Some(depth_stencil_attachment) =>
				{
					Some(depth_stencil_attachment.to_ash_type())
				},
				None => None
			},
			preserve_attachments: match self.preserve_attachments
			{
				Some(preserve_attachments) =>
				{
					Some(preserve_attachments.to_vec())
				},
				None => { None }
			},
		}
	}
}

struct VkSubpassDescription
{
	pub pipeline_bind_point: ash::vk::PipelineBindPoint,
	pub input_attachments: Option<Vec<ash::vk::AttachmentReference>>,
	pub color_attachments: Vec<ash::vk::AttachmentReference>,
	pub resolve_attachments: Option<Vec<ash::vk::AttachmentReference>>,
	pub depth_stencil_attachment: Option<ash::vk::AttachmentReference>,
	pub preserve_attachments: Option<Vec<u32>>
}

impl VkSubpassDescription
{
	fn to_ash_type(&self) -> ash::vk::SubpassDescription
	{
		let mut result = ash::vk::SubpassDescription::builder()
			.pipeline_bind_point(self.pipeline_bind_point)
			.color_attachments(&self.color_attachments);
		
		if let Some(input_attachments) = &self.input_attachments
		{
			result = result.input_attachments(&input_attachments);
		}

		if let Some(resolve_attachments) = &self.resolve_attachments
		{
			result = result.resolve_attachments(&resolve_attachments);
		}

		if let Some(depth_stencil_attachment) = &self.depth_stencil_attachment
		{
			result = result.depth_stencil_attachment(&depth_stencil_attachment);
		}

		if let Some(preserve_attachments) = &self.preserve_attachments
		{
			result = result.preserve_attachments(&preserve_attachments);
		}

		result.build()
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
pub struct RenderPassCreateInfo<'a>
{
	pub attachments: &'a [AttachmentDescription],
	pub subpasses: &'a [SubpassDescription<'a>],
	pub dependencies: &'a [SubpassDependency]
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

#[derive(Clone, Debug)]
pub struct FramebufferCreateInfo<'a>
{
	pub renderpass: &'a RenderPass,
	pub attachments: Vec<&'a Arc<ImageView>>,
	pub width: u32,
	pub height: u32,
	pub layers: u32
}

#[derive(Clone, Debug)]
pub struct Framebuffer
{
	handle: ash::vk::Framebuffer
}

impl Framebuffer
{
	pub fn new(device: &Arc<Device>, create_info: &FramebufferCreateInfo) -> Result<Arc<Self>, ()>
	{
		let attachments: Vec<ash::vk::ImageView> = create_info.attachments.iter().map(|attachment| attachment.handle.clone()).collect();

		let framebuffer_create_info = ash::vk::FramebufferCreateInfo::builder()
			.attachments(&attachments)
			.render_pass(create_info.renderpass.handle)
			.width(create_info.width)
			.height(create_info.height)
			.layers(create_info.layers);
		
		let handle = unsafe { device.handle.create_framebuffer(&framebuffer_create_info, None) }.unwrap();
	
		Ok(Arc::new(Framebuffer { handle }))
	}
}

#[derive(Clone, Copy, Debug)]
pub struct ShaderModuleCreateInfo<'a>
{
	pub code: &'a [u8]
}

#[derive(Clone, Debug)]
pub struct ShaderModule
{
	handle: ash::vk::ShaderModule
}

impl ShaderModule
{
	pub fn from_handle(handle: ash::vk::ShaderModule) -> Result<Arc<Self>, ()>
	{
		Ok(Arc::new(Self { handle }))
	}

	pub fn new(device: &Arc<Device>, create_info: &ShaderModuleCreateInfo) -> Result<Arc<Self>, ()>
	{
		assert!((create_info.code.len() % 4) == 0);

		let code: &[u32] = unsafe { std::slice::from_raw_parts(create_info.code.as_ptr() as *const _, create_info.code.len() / std::mem::size_of::<u32>()) };

		let shader_module_create_info = ash::vk::ShaderModuleCreateInfo::builder()
			.code(code);

		let handle = unsafe { device.handle.create_shader_module(&shader_module_create_info, None) }.unwrap();

		Self::from_handle(handle)
	}
}

#[derive(Clone, Copy, Debug)]
pub enum PrimitiveTopology
{
	PointList,
	LineList,
	LineStrip,
	TriangleList,
	TriangleStrip,
	TriangleFan,
	LineListWithAdjacency,
	LineStripWithAdjacency,
	TriangleListWithAdjacency,
	TriangleStripWithAdjacency,
	PatchList
}

impl PrimitiveTopology
{
	fn to_ash_type(&self) -> ash::vk::PrimitiveTopology
	{
		match *self
		{
			Self::PointList => ash::vk::PrimitiveTopology::POINT_LIST,
			Self::LineList => ash::vk::PrimitiveTopology::LINE_LIST,
			Self::LineStrip => ash::vk::PrimitiveTopology::LINE_STRIP,
			Self::TriangleList => ash::vk::PrimitiveTopology::TRIANGLE_LIST,
			Self::TriangleStrip => ash::vk::PrimitiveTopology::TRIANGLE_STRIP,
			Self::TriangleFan => ash::vk::PrimitiveTopology::TRIANGLE_FAN,
			Self::LineListWithAdjacency => ash::vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY,
			Self::LineStripWithAdjacency => ash::vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY,
			Self::TriangleListWithAdjacency => ash::vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY,
			Self::TriangleStripWithAdjacency => ash::vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY,
			Self::PatchList => ash::vk::PrimitiveTopology::PATCH_LIST
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PipelineInputAssemblyStateCreateInfo
{
	pub topology: PrimitiveTopology,
	pub primitive_restart_enable: bool
}

impl Default for PipelineInputAssemblyStateCreateInfo
{
	fn default() -> Self
	{
		Self
		{
			topology: PrimitiveTopology::TriangleList,
			primitive_restart_enable: false
		}	
	}
}

impl PipelineInputAssemblyStateCreateInfo
{
	fn to_ash_type(&self) -> ash::vk::PipelineInputAssemblyStateCreateInfo
	{
		ash::vk::PipelineInputAssemblyStateCreateInfo::builder()
			.topology(self.topology.to_ash_type())
			.primitive_restart_enable(self.primitive_restart_enable)
			.build()
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PipelineShaderStageCreateInfo<'a>
{
	pub stage: ash::vk::ShaderStageFlags,
	pub module: &'a Arc<ShaderModule>,
	pub name: &'a str,
}

#[derive(Clone, Copy, Debug)]
pub struct Viewport
{
	pub origin: [f32; 2],
	pub dimensions: [f32; 2],
	pub depth: [f32; 2]
}

impl Viewport
{
	fn to_ash_type(&self) -> ash::vk::Viewport
	{
		ash::vk::Viewport
		{
			x: self.origin[0],
			y: self.origin[1],
			width: self.dimensions[0],
			height: self.dimensions[1],
			min_depth: self.depth[0],
			max_depth: self.depth[1]
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Scissor
{
	pub origin: [i32; 2],
	pub dimensions: [u32; 2]
}

impl Scissor
{
	fn to_ash_type(&self) -> ash::vk::Rect2D
	{
		ash::vk::Rect2D
		{
			offset: ash::vk::Offset2D { x: self.origin[0], y: self.origin[1] },
			extent: ash::vk::Extent2D { width: self.dimensions[0], height: self.dimensions[1] }
		}
	}
}

#[derive(Clone, Debug)]
pub struct PipelineViewportStateCreateInfo<'a>
{
	pub viewports: &'a [Viewport],
	pub scissors: &'a [Scissor]
}

#[derive(Clone, Copy, Debug)]
pub enum VertexInputRate
{
	Vertex,
	Instance
}

impl VertexInputRate
{
	fn to_ash_type(&self) -> ash::vk::VertexInputRate
	{
		match *self
		{
			Self::Vertex => ash::vk::VertexInputRate::VERTEX,
			Self::Instance => ash::vk::VertexInputRate::INSTANCE
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct VertexInputBindingDescription
{
	pub binding: u32,
	pub stride: u32,
	pub input_rate: VertexInputRate
}

impl VertexInputBindingDescription
{
	fn to_ash_type(&self) -> ash::vk::VertexInputBindingDescription
	{
		ash::vk::VertexInputBindingDescription
		{
			binding: self.binding,
			stride: self.stride,
			input_rate: self.input_rate.to_ash_type()
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct VertexInputAttributeDescription
{
	pub location: u32,
	pub binding: u32,
	pub format: ash::vk::Format,
	pub offset: u32
}

impl VertexInputAttributeDescription
{
	fn to_ash_type(&self) -> ash::vk::VertexInputAttributeDescription
	{
		ash::vk::VertexInputAttributeDescription
		{
			location: self.location,
			binding: self.binding,
			format: self.format,
			offset: self.offset
		}
	}
}

#[derive(Clone, Debug)]
pub struct PipelineVertexInputStateCreateInfo
{
	pub vertex_binding_descriptions: Vec<VertexInputBindingDescription>,
	pub vertex_attribute_descriptions: Vec<VertexInputAttributeDescription>
}

impl Default for PipelineVertexInputStateCreateInfo
{
	fn default() -> Self
	{
		Self
		{
			vertex_binding_descriptions: Vec::new(),
			vertex_attribute_descriptions: Vec::new()
		}	
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PipelineTessellationStateCreateInfo
{
	pub patch_control_points: u32
}

impl PipelineTessellationStateCreateInfo
{
	fn to_ash_type(&self) -> ash::vk::PipelineTessellationStateCreateInfo
	{
		ash::vk::PipelineTessellationStateCreateInfo::builder()
			.patch_control_points(self.patch_control_points)
			.build()
	}
}

#[derive(Clone, Copy, Debug)]
pub enum PolygonMode
{
	Fill,
	Line,
	Point
}

impl PolygonMode
{
	fn to_ash_type(&self) -> ash::vk::PolygonMode
	{
		match *self
		{
			Self::Fill => ash::vk::PolygonMode::FILL,
			Self::Line => ash::vk::PolygonMode::LINE,
			Self::Point => ash::vk::PolygonMode::POINT
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum CullMode
{
	None,
	Front,
	Back,
	FrontAndBack
}

impl CullMode
{
	fn to_ash_type(&self) -> ash::vk::CullModeFlags
	{
		match *self
		{
			Self::None => ash::vk::CullModeFlags::NONE,
			Self::Front => ash::vk::CullModeFlags::FRONT,
			Self::Back => ash::vk::CullModeFlags::BACK,
			Self::FrontAndBack => ash::vk::CullModeFlags::FRONT_AND_BACK
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum FrontFace
{
	CounterClockwise,
	Clockwise
}

impl FrontFace
{
	fn to_ash_type(&self) -> ash::vk::FrontFace
	{
		match *self
		{
			Self::CounterClockwise => ash::vk::FrontFace::COUNTER_CLOCKWISE,
			Self::Clockwise => ash::vk::FrontFace::CLOCKWISE
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PipelineRasterizationStateCreateInfo
{
	pub depth_clamp_enable: bool,
	pub rasterizer_discard_enable: bool,
	pub polygon_mode: PolygonMode,
	pub cull_mode: CullMode,
	pub front_face: FrontFace,
	pub depth_bias_enable: bool,
	pub depth_bias_constant_factor: f32,
	pub depth_bias_clamp: f32,
	pub depth_bias_slope_factor: f32,
	pub line_width: f32
}

impl Default for PipelineRasterizationStateCreateInfo
{
	fn default() -> Self
	{
		Self
		{
			depth_clamp_enable: false,
			rasterizer_discard_enable: false,
			polygon_mode: PolygonMode::Fill,
			cull_mode: CullMode::None,
			front_face: FrontFace::CounterClockwise,
			depth_bias_enable: false,
			depth_bias_constant_factor: 0.0,
			depth_bias_clamp: 0.0,
			depth_bias_slope_factor: 0.0,
			line_width: 1.0,
		}
	}
}

impl PipelineRasterizationStateCreateInfo
{
	fn to_ash_type(&self) -> ash::vk::PipelineRasterizationStateCreateInfo
	{
		ash::vk::PipelineRasterizationStateCreateInfo::builder()
			.depth_clamp_enable(self.depth_clamp_enable)
			.rasterizer_discard_enable(self.rasterizer_discard_enable)
			.polygon_mode(self.polygon_mode.to_ash_type())
			.cull_mode(self.cull_mode.to_ash_type())
			.front_face(self.front_face.to_ash_type())
			.depth_bias_enable(self.depth_bias_enable)
			.depth_bias_constant_factor(self.depth_bias_constant_factor)
			.depth_bias_clamp(self.depth_bias_clamp)
			.depth_bias_slope_factor(self.depth_bias_slope_factor)
			.line_width(self.line_width)
			.build()
	}
}

#[derive(Clone, Debug)]
pub struct PipelineMultisampleStateCreateInfo
{
	pub rasterization_samples: u32,
	pub sample_shading_enable: bool,
	pub min_sample_shading: f32,
	pub sample_mask: Vec<u32>,
	pub alpha_to_coverage_enable: bool,
	pub alpha_to_one_enable: bool
}

impl Default for PipelineMultisampleStateCreateInfo
{
	fn default() -> Self
	{
		Self
		{
			rasterization_samples: 1,
			sample_shading_enable: false,
			min_sample_shading: 1.0,
			sample_mask: Vec::new(),
			alpha_to_coverage_enable: false,
			alpha_to_one_enable: false
		}
	}
}

impl PipelineMultisampleStateCreateInfo
{
	fn to_ash_type(&self) -> ash::vk::PipelineMultisampleStateCreateInfo
	{
		ash::vk::PipelineMultisampleStateCreateInfo::builder()
			.rasterization_samples(self.get_rasterization_samples())
			.sample_shading_enable(self.sample_shading_enable)
			.min_sample_shading(self.min_sample_shading)
			.sample_mask(&self.sample_mask)
			.alpha_to_coverage_enable(self.alpha_to_coverage_enable)
			.alpha_to_one_enable(self.alpha_to_one_enable)
			.build()
	}

	fn get_rasterization_samples(&self) -> ash::vk::SampleCountFlags
	{
		let mut flags: u32 = 0;

		match self.rasterization_samples
		{
			1 =>  { flags |= ash::vk::SampleCountFlags::TYPE_1.as_raw() }
			2 =>  { flags |= ash::vk::SampleCountFlags::TYPE_2.as_raw() }
			4 =>  { flags |= ash::vk::SampleCountFlags::TYPE_4.as_raw() }
			8 =>  { flags |= ash::vk::SampleCountFlags::TYPE_8.as_raw() }
			16 => { flags |= ash::vk::SampleCountFlags::TYPE_16.as_raw() }
			32 => { flags |= ash::vk::SampleCountFlags::TYPE_32.as_raw() }
			64 => { flags |= ash::vk::SampleCountFlags::TYPE_64.as_raw() }
			_ => { panic!("Invalid sample count! [{}]", self.rasterization_samples) }
		};

		ash::vk::SampleCountFlags::from_raw(flags)
	}
}

#[derive(Clone, Copy, Debug)]
pub enum CompareOp
{
	Never,
	Less,
	Equal,
	LessOrEqual,
	Greater,
	NotEqual,
	GreaterOrEqual,
	Always
}

impl CompareOp
{
	fn to_ash_type(&self) -> ash::vk::CompareOp
	{
		match *self
		{
			Self::Never => ash::vk::CompareOp::NEVER,
			Self::Less => ash::vk::CompareOp::LESS,
			Self::Equal => ash::vk::CompareOp::EQUAL,
			Self::LessOrEqual => ash::vk::CompareOp::LESS_OR_EQUAL,
			Self::Greater => ash::vk::CompareOp::GREATER,
			Self::NotEqual => ash::vk::CompareOp::NOT_EQUAL,
			Self::GreaterOrEqual => ash::vk::CompareOp::GREATER_OR_EQUAL,
			Self::Always => ash::vk::CompareOp::ALWAYS
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum StencilOp
{
	Keep,
	Zero,
	Replace,
	IncrementAndClamp,
	DecrementAndClamp,
	Invert,
	IncrementAndWrap,
	DecrementAndWrap
}

impl StencilOp
{
	fn to_ash_type(&self) -> ash::vk::StencilOp
	{
		match *self
		{
			Self::Keep => ash::vk::StencilOp::KEEP,
			Self::Zero => ash::vk::StencilOp::ZERO,
			Self::Replace => ash::vk::StencilOp::REPLACE,
			Self::IncrementAndClamp => ash::vk::StencilOp::INCREMENT_AND_CLAMP,
			Self::DecrementAndClamp => ash::vk::StencilOp::DECREMENT_AND_CLAMP,
			Self::Invert => ash::vk::StencilOp::INVERT,
			Self::IncrementAndWrap => ash::vk::StencilOp::INCREMENT_AND_WRAP,
			Self::DecrementAndWrap => ash::vk::StencilOp::DECREMENT_AND_WRAP
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct StencilOpState
{
	pub fail_op: StencilOp,
	pub pass_op: StencilOp,
	pub depth_fail_op: StencilOp,
	pub compare_op: CompareOp,
	pub compare_mask: u32,
	pub write_mask: u32,
	pub reference: u32
}

impl Default for StencilOpState
{
	fn default() -> Self
	{
		Self
		{
			fail_op: StencilOp::Zero,
			pass_op: StencilOp::Zero,
			depth_fail_op: StencilOp::Zero,
			compare_op: CompareOp::Never,
			compare_mask: 0,
			write_mask: 0,
			reference: 0
		}
	}
}

impl StencilOpState
{
	fn to_ash_type(&self) -> ash::vk::StencilOpState
	{
		ash::vk::StencilOpState
		{
			fail_op: self.fail_op.to_ash_type(),
			pass_op: self.pass_op.to_ash_type(),
			depth_fail_op: self.depth_fail_op.to_ash_type(),
			compare_op: self.compare_op.to_ash_type(),
			compare_mask: self.compare_mask,
			write_mask: self.write_mask,
			reference: self.reference
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PipelineDepthStencilStateCreateInfo
{
	pub depth_test_enable: bool,
	pub depth_write_enable: bool,
	pub depth_compare_op: CompareOp,
	pub depth_bounds_test_enable: bool,
	pub stencil_test_enable: bool,
	pub front: StencilOpState,
	pub back: StencilOpState,
	pub min_depth_bounds: f32,
	pub max_depth_bounds: f32
}

impl Default for PipelineDepthStencilStateCreateInfo
{
	fn default() -> Self
	{
		Self
		{
			depth_test_enable: false,
			depth_write_enable: false,
			depth_compare_op: CompareOp::Less,
			depth_bounds_test_enable: false,
			stencil_test_enable: false,
			front: StencilOpState::default(),
			back: StencilOpState::default(),
			min_depth_bounds: 0.0,
			max_depth_bounds: 1.0
		}
	}
}

impl PipelineDepthStencilStateCreateInfo
{
	fn to_ash_type(&self) -> ash::vk::PipelineDepthStencilStateCreateInfo
	{
		ash::vk::PipelineDepthStencilStateCreateInfo::builder()
			.depth_test_enable(self.depth_test_enable)	
			.depth_write_enable(self.depth_write_enable)
			.depth_compare_op(self.depth_compare_op.to_ash_type())
			.depth_bounds_test_enable(self.depth_bounds_test_enable)
			.stencil_test_enable(self.stencil_test_enable)
			.front(self.front.to_ash_type())
			.back(self.back.to_ash_type())
			.min_depth_bounds(self.min_depth_bounds)
			.max_depth_bounds(self.max_depth_bounds)
			.build()
	}
}

#[derive(Clone, Copy, Debug)]
pub enum LogicOp
{
	Clear,
	And,
	AndReverse,
	Copy,
	AndInverted,
	NoOp,
	Xor,
	Or,
	Nor,
	Equivalent,
	Invert,
	OrReverse,
	CopyInverted,
	OrInverted,
	Nand,
	Set
}

impl LogicOp
{
	fn to_ash_type(&self) -> ash::vk::LogicOp
	{
		match *self
		{
			Self::Clear => ash::vk::LogicOp::CLEAR,
			Self::And => ash::vk::LogicOp::AND,
			Self::AndReverse => ash::vk::LogicOp::AND_REVERSE,
			Self::Copy => ash::vk::LogicOp::COPY,
			Self::AndInverted => ash::vk::LogicOp::AND_INVERTED,
			Self::NoOp => ash::vk::LogicOp::NO_OP,
			Self::Xor => ash::vk::LogicOp::XOR,
			Self::Or => ash::vk::LogicOp::OR,
			Self::Nor => ash::vk::LogicOp::NOR,
			Self::Equivalent => ash::vk::LogicOp::EQUIVALENT,
			Self::Invert => ash::vk::LogicOp::INVERT,
			Self::OrReverse => ash::vk::LogicOp::OR_REVERSE,
			Self::CopyInverted => ash::vk::LogicOp::COPY_INVERTED,
			Self::OrInverted => ash::vk::LogicOp::OR_INVERTED,
			Self::Nand => ash::vk::LogicOp::NAND,
			Self::Set => ash::vk::LogicOp::SET,
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum BlendFactor
{
	Zero,
	One,
	SrcColor,
	OneMinusSrcColor,
	DstColor,
	OneMinusDstColor,
	SrcAlpha,
	OneMinusSrcAlpha,
	DstAlpha,
	OneMinusDstAlpha,
	ConstantColor,
	OneMinusConstantColor,
	ConstantAlpha,
	OneMinusConstantAlpha,
	SrcAlphaSaturate,
	Src1Color,
	OneMinusSrc1Color,
	Src1Alpha,
	OneMinusSrc1Alpha
}

impl BlendFactor
{
	fn to_ash_type(&self) -> ash::vk::BlendFactor
	{
		match *self
		{
			Self::Zero => ash::vk::BlendFactor::ZERO,
			Self::One => ash::vk::BlendFactor::ONE,
			Self::SrcColor => ash::vk::BlendFactor::SRC_COLOR,
			Self::OneMinusSrcColor => ash::vk::BlendFactor::ONE_MINUS_SRC_COLOR,
			Self::DstColor => ash::vk::BlendFactor::DST_COLOR,
			Self::OneMinusDstColor => ash::vk::BlendFactor::ONE_MINUS_DST_COLOR,
			Self::SrcAlpha => ash::vk::BlendFactor::SRC_ALPHA,
			Self::OneMinusSrcAlpha => ash::vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
			Self::DstAlpha => ash::vk::BlendFactor::DST_ALPHA,
			Self::OneMinusDstAlpha => ash::vk::BlendFactor::ONE_MINUS_DST_ALPHA,
			Self::ConstantColor => ash::vk::BlendFactor::CONSTANT_COLOR,
			Self::OneMinusConstantColor => ash::vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
			Self::ConstantAlpha => ash::vk::BlendFactor::CONSTANT_ALPHA,
			Self::OneMinusConstantAlpha => ash::vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA,
			Self::SrcAlphaSaturate => ash::vk::BlendFactor::SRC_ALPHA_SATURATE,
			Self::Src1Color => ash::vk::BlendFactor::SRC1_COLOR,
			Self::OneMinusSrc1Color => ash::vk::BlendFactor::ONE_MINUS_SRC1_COLOR,
			Self::Src1Alpha => ash::vk::BlendFactor::SRC1_ALPHA,
			Self::OneMinusSrc1Alpha => ash::vk::BlendFactor::ONE_MINUS_SRC1_ALPHA
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum BlendOp
{
	Add,
	Subtract,
	ReverseSubtract,
	Min,
	Max
}

impl BlendOp
{
	fn to_ash_type(&self) -> ash::vk::BlendOp
	{
		match *self
		{
			Self::Add => ash::vk::BlendOp::ADD,
			Self::Subtract => ash::vk::BlendOp::SUBTRACT,
			Self::ReverseSubtract => ash::vk::BlendOp::REVERSE_SUBTRACT,
			Self::Min => ash::vk::BlendOp::MIN,
			Self::Max => ash::vk::BlendOp::MAX
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct ColorComponents
{
	pub r: bool,
	pub g: bool,
	pub b: bool,
	pub a: bool
}

impl Default for ColorComponents
{
	fn default() -> Self
	{
		Self
		{
			r: true, g: true, b: true, a: true
		}
	}
}

impl ColorComponents
{
	pub fn empty() -> Self
	{
		Self
		{
			r: false, g: false, b: false, a: false
		}
	}

	fn to_ash_type(&self) -> ash::vk::ColorComponentFlags
	{
		let mut flags: u32 = 0;

		if self.r { flags |= ash::vk::ColorComponentFlags::R.as_raw(); }
		if self.g { flags |= ash::vk::ColorComponentFlags::G.as_raw(); }
		if self.b { flags |= ash::vk::ColorComponentFlags::B.as_raw(); }
		if self.a { flags |= ash::vk::ColorComponentFlags::A.as_raw(); }

		ash::vk::ColorComponentFlags::from_raw(flags)
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PipelineColorBlendAttachmentState
{
	pub blend_enable: bool,
	pub src_color_blend_factor: BlendFactor,
	pub dst_color_blend_factor: BlendFactor,
	pub color_blend_op: BlendOp,
	pub src_alpha_blend_factor: BlendFactor,
	pub dst_alpha_blend_factor: BlendFactor,
	pub alpha_blend_op: BlendOp,
	pub color_write_mask: ColorComponents
}

impl PipelineColorBlendAttachmentState
{
	fn to_ash_type(&self) -> ash::vk::PipelineColorBlendAttachmentState
	{
		ash::vk::PipelineColorBlendAttachmentState
		{
			blend_enable: self.blend_enable as u32,
			src_color_blend_factor: self.src_color_blend_factor.to_ash_type(),
			dst_color_blend_factor: self.dst_color_blend_factor.to_ash_type(),
			color_blend_op: self.color_blend_op.to_ash_type(),
			src_alpha_blend_factor: self.src_alpha_blend_factor.to_ash_type(),
			dst_alpha_blend_factor: self.dst_alpha_blend_factor.to_ash_type(),
			alpha_blend_op: self.alpha_blend_op.to_ash_type(),
			color_write_mask: self.color_write_mask.to_ash_type()
		}
	}
}

#[derive(Clone, Debug)]
pub struct PipelineColorBlendStateCreateInfo<'a>
{
	pub logic_op_enable: bool,
	pub logic_op: LogicOp,
	pub attachments: &'a [PipelineColorBlendAttachmentState],
	pub blend_constants: [f32; 4]
}

#[derive(Clone, Copy, Debug)]
pub enum DynamicState
{
	Viewport,
	Scissor,
	LineWidth,
	DepthBias,
	BlendConstants,
	DepthBounds,
	StencilCompareMask,
	StencilWriteMask,
	StencilReference
}

impl DynamicState
{
	fn to_ash_type(&self) -> ash::vk::DynamicState
	{
		match *self
		{
			Self::Viewport => ash::vk::DynamicState::VIEWPORT,
			Self::Scissor => ash::vk::DynamicState::SCISSOR,
			Self::LineWidth => ash::vk::DynamicState::LINE_WIDTH,
			Self::DepthBias => ash::vk::DynamicState::DEPTH_BIAS,
			Self::BlendConstants => ash::vk::DynamicState::BLEND_CONSTANTS,
			Self::DepthBounds => ash::vk::DynamicState::DEPTH_BOUNDS,
			Self::StencilCompareMask => ash::vk::DynamicState::STENCIL_COMPARE_MASK,
			Self::StencilWriteMask => ash::vk::DynamicState::STENCIL_WRITE_MASK,
			Self::StencilReference => ash::vk::DynamicState::STENCIL_REFERENCE
		}
	}
}

#[derive(Clone, Debug)]
pub struct PipelineDynamicStateCreateInfo<'a>
{
	pub dynamic_states: &'a [DynamicState]
}

impl PipelineDynamicStateCreateInfo<'_>
{
	fn fill_ash_types(&self, dynamic_states: &mut Vec<ash::vk::DynamicState>)
	{
		for state in self.dynamic_states
		{
			dynamic_states.push(state.to_ash_type());
		}
	}
}

pub struct GraphicsPipelineCreateInfo<'a>
{
	pub stages: &'a [PipelineShaderStageCreateInfo<'a>],
	pub vertex_input_state: &'a PipelineVertexInputStateCreateInfo,
	pub input_assembly_state: &'a PipelineInputAssemblyStateCreateInfo,
	pub tessellation_state: &'a PipelineTessellationStateCreateInfo,
	pub viewport_state: &'a PipelineViewportStateCreateInfo<'a>,
	pub rasterization_state: &'a PipelineRasterizationStateCreateInfo,
	pub multisample_state: &'a PipelineMultisampleStateCreateInfo,
	pub depth_stencil_state: &'a PipelineDepthStencilStateCreateInfo,
	pub color_blend_state: &'a PipelineColorBlendStateCreateInfo<'a>,
	pub dynamic_state: &'a PipelineDynamicStateCreateInfo<'a>,
	pub layout: &'a PipelineLayout,
	pub renderpass: &'a RenderPass,
	pub subpass: u32,
	pub base_pipeline: Option<&'a Pipeline>,
	pub base_pipeline_index: Option<i32>
}

pub struct PipelineLayout
{
	handle: ash::vk::PipelineLayout
}

impl PipelineLayout
{
	pub fn from_handle(handle: ash::vk::PipelineLayout) -> Result<Arc<Self>, ()>
	{
		Ok(Arc::new(Self { handle }))
	}
}

pub struct Pipeline
{
	handle: ash::vk::Pipeline
}

impl Pipeline
{
	pub fn from_handle(handle: ash::vk::Pipeline) -> Result<Arc<Self>, ()>
	{
		Ok(Arc::new(Self { handle }))
	}

	pub fn new(device: &Arc<Device>, create_info: &GraphicsPipelineCreateInfo) -> Result<Arc<Self>, ()>
	{
		/* VkPipelineShaderStageCreateInfo */
		let mut stages: Vec<ash::vk::PipelineShaderStageCreateInfo> = Vec::with_capacity(create_info.stages.len());
		let mut stage_names: Vec<std::ffi::CString> = Vec::with_capacity(create_info.stages.len());
		for barium_stage in create_info.stages
		{
			stage_names.push(std::ffi::CString::new(barium_stage.name).unwrap());

			stages.push(ash::vk::PipelineShaderStageCreateInfo::builder()
				.stage(barium_stage.stage)
				.module(barium_stage.module.handle)
				.name(stage_names.last().unwrap().as_c_str())
				.build());
		}

		/* VkPipelineVertexInputStateCreateInfo */
		let vertex_binding_descriptions: Vec<ash::vk::VertexInputBindingDescription> = create_info.vertex_input_state.vertex_binding_descriptions.iter().map(|state| state.to_ash_type()).collect();
		let vertex_attribute_descriptions: Vec<ash::vk::VertexInputAttributeDescription> = create_info.vertex_input_state.vertex_attribute_descriptions.iter().map(|state| state.to_ash_type()).collect();
		let vertex_input_state = ash::vk::PipelineVertexInputStateCreateInfo::builder()
			.vertex_binding_descriptions(&vertex_binding_descriptions)
			.vertex_attribute_descriptions(&vertex_attribute_descriptions)
			.build();

		/* VkPipelineInputAssemblyStateCreateInfo */
		let input_assembly_state = create_info.input_assembly_state.to_ash_type();

		/* VkPipelineTessellationStateCreateInfo */
		let tessellation_state = create_info.tessellation_state.to_ash_type();

		/* VkPipelineViewportStateCreateInfo */
		let viewports: Vec<ash::vk::Viewport> = create_info.viewport_state.viewports.iter().map(|state| state.to_ash_type()).collect();
		let scissors: Vec<ash::vk::Rect2D> = create_info.viewport_state.scissors.iter().map(|state| state.to_ash_type()).collect();
		let viewport_state = ash::vk::PipelineViewportStateCreateInfo::builder()
			.viewports(&viewports)
			.scissors(&scissors)
			.build();

		/* VkPipelineRasterizationStateCreateInfo */
		let rasterization_state = create_info.rasterization_state.to_ash_type();

		/* VkPipelineMultisampleStateCreateInfo */
		let multisample_state = create_info.multisample_state.to_ash_type();

		/* VkPipelineDepthStencilStateCreateInfo */
		let depth_stencil_state = create_info.depth_stencil_state.to_ash_type();

		/* VkPipelineColorBlendStateCreateInfo */
		let color_blend_state_attachments: Vec<ash::vk::PipelineColorBlendAttachmentState> = create_info.color_blend_state.attachments.iter().map(|state| state.to_ash_type()).collect();
		let color_blend_state = ash::vk::PipelineColorBlendStateCreateInfo::builder()
			.logic_op_enable(create_info.color_blend_state.logic_op_enable)
			.logic_op(create_info.color_blend_state.logic_op.to_ash_type())
			.attachments(&color_blend_state_attachments)
			.blend_constants(create_info.color_blend_state.blend_constants)
			.build();

		/* VkPipelineDynamicStateCreateInfo */
		let dynamic_state_states: Vec<ash::vk::DynamicState> = create_info.dynamic_state.dynamic_states.iter().map(|state| state.to_ash_type()).collect();
		let dynamic_state = ash::vk::PipelineDynamicStateCreateInfo::builder()
			.dynamic_states(&dynamic_state_states)
			.build();

		let graphics_pipeline_create_info = ash::vk::GraphicsPipelineCreateInfo::builder()
			.stages(&stages)	
			.stages(&stages).vertex_input_state(&vertex_input_state)
			.input_assembly_state(&input_assembly_state)
			.tessellation_state(&tessellation_state)
			.viewport_state(&viewport_state)
			.rasterization_state(&rasterization_state)
			.multisample_state(&multisample_state)
			.depth_stencil_state(&depth_stencil_state)
			.color_blend_state(&color_blend_state)
			.dynamic_state(&dynamic_state)
			.layout(create_info.layout.handle)
			.render_pass(create_info.renderpass.handle)
			.subpass(create_info.subpass)
			.base_pipeline_handle(match create_info.base_pipeline
			{
				Some(base_pipeline) => { base_pipeline.handle },
				None => { ash::vk::Pipeline::null() }
			})
			.base_pipeline_index(create_info.base_pipeline_index.unwrap_or(-1));

		let handle = unsafe { device.handle.create_graphics_pipelines(ash::vk::PipelineCache::null(), &[graphics_pipeline_create_info.build()], None) }.unwrap();

		Self::from_handle(handle[0])
	}
}

#[derive(Clone, Copy, Debug)]
pub struct CommandPoolCreateInfo
{
	pub queue_family_index: u32,
	pub transient: bool
}

impl CommandPoolCreateInfo
{
	fn to_ash_type(&self) -> ash::vk::CommandPoolCreateInfo
	{
		ash::vk::CommandPoolCreateInfo::builder()
			.queue_family_index(self.queue_family_index)
			.flags(if self.transient
			{
				ash::vk::CommandPoolCreateFlags::TRANSIENT
			}
			else
			{
				ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER
			})
			.build()
	}
}

#[derive(Clone, Debug)]
pub struct CommandPool
{
	handle: ash::vk::CommandPool,
	queue_family_index: u32
}

impl CommandPool
{
	pub fn new(device: &Arc<Device>, create_info: &CommandPoolCreateInfo) -> Result<Arc<Self>, ()>
	{
		let command_buffer_create_info = create_info.to_ash_type();
		let handle = unsafe { device.handle.create_command_pool(&command_buffer_create_info, None) }.unwrap();
		Ok(Arc::new(Self { handle, queue_family_index: create_info.queue_family_index }))
	}
}

#[derive(Clone, Copy, Debug)]
pub struct CommandBufferAllocateInfo<'a>
{
	pub command_pool: &'a Arc<CommandPool>,
	pub primary: bool,
	pub count: u32
}

impl CommandBufferAllocateInfo<'_>
{
	fn to_ash_type(&self) -> ash::vk::CommandBufferAllocateInfo
	{
		ash::vk::CommandBufferAllocateInfo::builder()
			.command_pool(self.command_pool.handle)
			.level(if self.primary { ash::vk::CommandBufferLevel::PRIMARY } else { ash::vk::CommandBufferLevel::SECONDARY })
			.command_buffer_count(self.count)
			.build()
	}
}

pub struct CommandBuffer
{
	handle: ash::vk::CommandBuffer
}

impl CommandBuffer
{
	pub fn from_handle(handle: ash::vk::CommandBuffer) -> Result<Arc<Self>, ()>
	{
		Ok(Arc::new(Self { handle }))
	}

	pub fn allocate_buffers(device: &Arc<Device>, allocate_info: &CommandBufferAllocateInfo) -> Result<Vec<Arc<CommandBuffer>>, ()>
	{
		let command_buffer_allocate_info = allocate_info.to_ash_type();
		let handles = unsafe { device.handle.allocate_command_buffers(&command_buffer_allocate_info) }.unwrap();
		let command_buffers: Vec<Arc<CommandBuffer>> = handles.iter().map(|handle| Arc::new(Self { handle: *handle })).collect();
		Ok(command_buffers)
	}
}