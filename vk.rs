macro_rules! non_dispatchable_handle
{
	($name:ident) =>
	{
		#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
		pub struct $name(pub u64);
	}
}

macro_rules! dispatchable_handle
{
	($name:ident) =>
	{
		#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
		pub struct $name(pub *mut ());
	}
}
type VkSampleMask = u32;
type VkBool32 = u32;
type VkFlags = u32;
type VkFlags64 = u64;
type VkDeviceSize = u64;
type VkDeviceAddress = u64;
type VkFramebufferCreateFlags = VkFlags;
type VkQueryPoolCreateFlags = VkFlags;
type VkRenderPassCreateFlags = VkFlags;
type VkSamplerCreateFlags = VkFlags;
type VkPipelineLayoutCreateFlags = VkFlags;
type VkPipelineCacheCreateFlags = VkFlags;
type VkPipelineDepthStencilStateCreateFlags = VkFlags;
type VkPipelineDynamicStateCreateFlags = VkFlags;
type VkPipelineColorBlendStateCreateFlags = VkFlags;
type VkPipelineMultisampleStateCreateFlags = VkFlags;
type VkPipelineRasterizationStateCreateFlags = VkFlags;
type VkPipelineViewportStateCreateFlags = VkFlags;
type VkPipelineTessellationStateCreateFlags = VkFlags;
type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
type VkPipelineVertexInputStateCreateFlags = VkFlags;
type VkPipelineShaderStageCreateFlags = VkFlags;
type VkDescriptorSetLayoutCreateFlags = VkFlags;
type VkBufferViewCreateFlags = VkFlags;
type VkInstanceCreateFlags = VkFlags;
type VkDeviceCreateFlags = VkFlags;
type VkDeviceQueueCreateFlags = VkFlags;
type VkQueueFlags = VkFlags;
type VkMemoryPropertyFlags = VkFlags;
type VkMemoryHeapFlags = VkFlags;
type VkAccessFlags = VkFlags;
type VkBufferUsageFlags = VkFlags;
type VkBufferCreateFlags = VkFlags;
type VkShaderStageFlags = VkFlags;
type VkImageUsageFlags = VkFlags;
type VkImageCreateFlags = VkFlags;
type VkImageViewCreateFlags = VkFlags;
type VkPipelineCreateFlags = VkFlags;
type VkColorComponentFlags = VkFlags;
type VkFenceCreateFlags = VkFlags;
type VkSemaphoreCreateFlags = VkFlags;
type VkFormatFeatureFlags = VkFlags;
type VkQueryControlFlags = VkFlags;
type VkQueryResultFlags = VkFlags;
type VkShaderModuleCreateFlags = VkFlags;
type VkEventCreateFlags = VkFlags;
type VkCommandPoolCreateFlags = VkFlags;
type VkCommandPoolResetFlags = VkFlags;
type VkCommandBufferResetFlags = VkFlags;
type VkCommandBufferUsageFlags = VkFlags;
type VkQueryPipelineStatisticFlags = VkFlags;
type VkMemoryMapFlags = VkFlags;
type VkImageAspectFlags = VkFlags;
type VkSparseMemoryBindFlags = VkFlags;
type VkSparseImageFormatFlags = VkFlags;
type VkSubpassDescriptionFlags = VkFlags;
type VkPipelineStageFlags = VkFlags;
type VkSampleCountFlags = VkFlags;
type VkAttachmentDescriptionFlags = VkFlags;
type VkStencilFaceFlags = VkFlags;
type VkCullModeFlags = VkFlags;
type VkDescriptorPoolCreateFlags = VkFlags;
type VkDescriptorPoolResetFlags = VkFlags;
type VkDependencyFlags = VkFlags;
type VkSubgroupFeatureFlags = VkFlags;
type VkIndirectCommandsLayoutUsageFlagsNV = VkFlags;
type VkIndirectStateFlagsNV = VkFlags;
type VkGeometryFlagsKHR = VkFlags;
type VkGeometryFlagsNV = VkGeometryFlagsKHR;
type VkGeometryInstanceFlagsKHR = VkFlags;
type VkGeometryInstanceFlagsNV = VkGeometryInstanceFlagsKHR;
type VkBuildAccelerationStructureFlagsKHR = VkFlags;
type VkBuildAccelerationStructureFlagsNV = VkBuildAccelerationStructureFlagsKHR;
type VkPrivateDataSlotCreateFlags = VkFlags;
type VkPrivateDataSlotCreateFlagsEXT = VkPrivateDataSlotCreateFlags;
type VkAccelerationStructureCreateFlagsKHR = VkFlags;
type VkDescriptorUpdateTemplateCreateFlags = VkFlags;
type VkDescriptorUpdateTemplateCreateFlagsKHR = VkDescriptorUpdateTemplateCreateFlags;
type VkPipelineCreationFeedbackFlags = VkFlags;
type VkPipelineCreationFeedbackFlagsEXT = VkPipelineCreationFeedbackFlags;
type VkPerformanceCounterDescriptionFlagsKHR = VkFlags;
type VkAcquireProfilingLockFlagsKHR = VkFlags;
type VkSemaphoreWaitFlags = VkFlags;
type VkSemaphoreWaitFlagsKHR = VkSemaphoreWaitFlags;
type VkPipelineCompilerControlFlagsAMD = VkFlags;
type VkShaderCorePropertiesFlagsAMD = VkFlags;
type VkDeviceDiagnosticsConfigFlagsNV = VkFlags;
type VkAccessFlags2 = VkFlags64;
type VkAccessFlags2KHR = VkAccessFlags2;
type VkPipelineStageFlags2 = VkFlags64;
type VkPipelineStageFlags2KHR = VkPipelineStageFlags2;
type VkAccelerationStructureMotionInfoFlagsNV = VkFlags;
type VkAccelerationStructureMotionInstanceFlagsNV = VkFlags;
type VkFormatFeatureFlags2 = VkFlags64;
type VkFormatFeatureFlags2KHR = VkFormatFeatureFlags2;
type VkRenderingFlags = VkFlags;
type VkMemoryDecompressionMethodFlagsNV = VkFlags64;
type VkRenderingFlagsKHR = VkRenderingFlags;
type VkBuildMicromapFlagsEXT = VkFlags;
type VkMicromapCreateFlagsEXT = VkFlags;
type VkCompositeAlphaFlagsKHR = VkFlags;
type VkDisplayPlaneAlphaFlagsKHR = VkFlags;
type VkSurfaceTransformFlagsKHR = VkFlags;
type VkSwapchainCreateFlagsKHR = VkFlags;
type VkDisplayModeCreateFlagsKHR = VkFlags;
type VkDisplaySurfaceCreateFlagsKHR = VkFlags;
type VkAndroidSurfaceCreateFlagsKHR = VkFlags;
type VkViSurfaceCreateFlagsNN = VkFlags;
type VkWaylandSurfaceCreateFlagsKHR = VkFlags;
type VkWin32SurfaceCreateFlagsKHR = VkFlags;
type VkXlibSurfaceCreateFlagsKHR = VkFlags;
type VkXcbSurfaceCreateFlagsKHR = VkFlags;
type VkDirectFBSurfaceCreateFlagsEXT = VkFlags;
type VkIOSSurfaceCreateFlagsMVK = VkFlags;
type VkMacOSSurfaceCreateFlagsMVK = VkFlags;
type VkMetalSurfaceCreateFlagsEXT = VkFlags;
type VkImagePipeSurfaceCreateFlagsFUCHSIA = VkFlags;
type VkStreamDescriptorSurfaceCreateFlagsGGP = VkFlags;
type VkHeadlessSurfaceCreateFlagsEXT = VkFlags;
type VkScreenSurfaceCreateFlagsQNX = VkFlags;
type VkPeerMemoryFeatureFlags = VkFlags;
type VkPeerMemoryFeatureFlagsKHR = VkPeerMemoryFeatureFlags;
type VkMemoryAllocateFlags = VkFlags;
type VkMemoryAllocateFlagsKHR = VkMemoryAllocateFlags;
type VkDeviceGroupPresentModeFlagsKHR = VkFlags;
type VkDebugReportFlagsEXT = VkFlags;
type VkCommandPoolTrimFlags = VkFlags;
type VkCommandPoolTrimFlagsKHR = VkCommandPoolTrimFlags;
type VkExternalMemoryHandleTypeFlagsNV = VkFlags;
type VkExternalMemoryFeatureFlagsNV = VkFlags;
type VkExternalMemoryHandleTypeFlags = VkFlags;
type VkExternalMemoryHandleTypeFlagsKHR = VkExternalMemoryHandleTypeFlags;
type VkExternalMemoryFeatureFlags = VkFlags;
type VkExternalMemoryFeatureFlagsKHR = VkExternalMemoryFeatureFlags;
type VkExternalSemaphoreHandleTypeFlags = VkFlags;
type VkExternalSemaphoreHandleTypeFlagsKHR = VkExternalSemaphoreHandleTypeFlags;
type VkExternalSemaphoreFeatureFlags = VkFlags;
type VkExternalSemaphoreFeatureFlagsKHR = VkExternalSemaphoreFeatureFlags;
type VkSemaphoreImportFlags = VkFlags;
type VkSemaphoreImportFlagsKHR = VkSemaphoreImportFlags;
type VkExternalFenceHandleTypeFlags = VkFlags;
type VkExternalFenceHandleTypeFlagsKHR = VkExternalFenceHandleTypeFlags;
type VkExternalFenceFeatureFlags = VkFlags;
type VkExternalFenceFeatureFlagsKHR = VkExternalFenceFeatureFlags;
type VkFenceImportFlags = VkFlags;
type VkFenceImportFlagsKHR = VkFenceImportFlags;
type VkSurfaceCounterFlagsEXT = VkFlags;
type VkPipelineViewportSwizzleStateCreateFlagsNV = VkFlags;
type VkPipelineDiscardRectangleStateCreateFlagsEXT = VkFlags;
type VkPipelineCoverageToColorStateCreateFlagsNV = VkFlags;
type VkPipelineCoverageModulationStateCreateFlagsNV = VkFlags;
type VkPipelineCoverageReductionStateCreateFlagsNV = VkFlags;
type VkValidationCacheCreateFlagsEXT = VkFlags;
type VkDebugUtilsMessageSeverityFlagsEXT = VkFlags;
type VkDebugUtilsMessageTypeFlagsEXT = VkFlags;
type VkDebugUtilsMessengerCreateFlagsEXT = VkFlags;
type VkDebugUtilsMessengerCallbackDataFlagsEXT = VkFlags;
type VkDeviceMemoryReportFlagsEXT = VkFlags;
type VkPipelineRasterizationConservativeStateCreateFlagsEXT = VkFlags;
type VkDescriptorBindingFlags = VkFlags;
type VkDescriptorBindingFlagsEXT = VkDescriptorBindingFlags;
type VkConditionalRenderingFlagsEXT = VkFlags;
type VkResolveModeFlags = VkFlags;
type VkResolveModeFlagsKHR = VkResolveModeFlags;
type VkPipelineRasterizationStateStreamCreateFlagsEXT = VkFlags;
type VkPipelineRasterizationDepthClipStateCreateFlagsEXT = VkFlags;
type VkSwapchainImageUsageFlagsANDROID = VkFlags;
type VkToolPurposeFlags = VkFlags;
type VkToolPurposeFlagsEXT = VkToolPurposeFlags;
type VkSubmitFlags = VkFlags;
type VkSubmitFlagsKHR = VkSubmitFlags;
type VkImageFormatConstraintsFlagsFUCHSIA = VkFlags;
type VkImageConstraintsInfoFlagsFUCHSIA = VkFlags;
type VkGraphicsPipelineLibraryFlagsEXT = VkFlags;
type VkImageCompressionFlagsEXT = VkFlags;
type VkImageCompressionFixedRateFlagsEXT = VkFlags;
type VkExportMetalObjectTypeFlagsEXT = VkFlags;
type VkDeviceAddressBindingFlagsEXT = VkFlags;
type VkVideoCodecOperationFlagsKHR = VkFlags;
type VkVideoCapabilityFlagsKHR = VkFlags;
type VkVideoSessionCreateFlagsKHR = VkFlags;
type VkVideoSessionParametersCreateFlagsKHR = VkFlags;
type VkVideoBeginCodingFlagsKHR = VkFlags;
type VkVideoEndCodingFlagsKHR = VkFlags;
type VkVideoCodingControlFlagsKHR = VkFlags;
type VkVideoDecodeUsageFlagsKHR = VkFlags;
type VkVideoDecodeCapabilityFlagsKHR = VkFlags;
type VkVideoDecodeFlagsKHR = VkFlags;
type VkVideoDecodeH264PictureLayoutFlagsEXT = VkFlags;
type VkVideoEncodeFlagsKHR = VkFlags;
type VkVideoEncodeUsageFlagsKHR = VkFlags;
type VkVideoEncodeContentFlagsKHR = VkFlags;
type VkVideoEncodeCapabilityFlagsKHR = VkFlags;
type VkVideoEncodeRateControlFlagsKHR = VkFlags;
type VkVideoEncodeRateControlModeFlagsKHR = VkFlags;
type VkVideoChromaSubsamplingFlagsKHR = VkFlags;
type VkVideoComponentBitDepthFlagsKHR = VkFlags;
type VkVideoEncodeH264CapabilityFlagsEXT = VkFlags;
type VkVideoEncodeH264InputModeFlagsEXT = VkFlags;
type VkVideoEncodeH264OutputModeFlagsEXT = VkFlags;
type VkVideoEncodeH265CapabilityFlagsEXT = VkFlags;
type VkVideoEncodeH265InputModeFlagsEXT = VkFlags;
type VkVideoEncodeH265OutputModeFlagsEXT = VkFlags;
type VkVideoEncodeH265CtbSizeFlagsEXT = VkFlags;
type VkVideoEncodeH265TransformBlockSizeFlagsEXT = VkFlags;
dispatchable_handle!(VkInstance);
dispatchable_handle!(VkPhysicalDevice);
dispatchable_handle!(VkDevice);
dispatchable_handle!(VkQueue);
dispatchable_handle!(VkCommandBuffer);
non_dispatchable_handle!(VkDeviceMemory);
non_dispatchable_handle!(VkCommandPool);
non_dispatchable_handle!(VkBuffer);
non_dispatchable_handle!(VkBufferView);
non_dispatchable_handle!(VkImage);
non_dispatchable_handle!(VkImageView);
non_dispatchable_handle!(VkShaderModule);
non_dispatchable_handle!(VkPipeline);
non_dispatchable_handle!(VkPipelineLayout);
non_dispatchable_handle!(VkSampler);
non_dispatchable_handle!(VkDescriptorSet);
non_dispatchable_handle!(VkDescriptorSetLayout);
non_dispatchable_handle!(VkDescriptorPool);
non_dispatchable_handle!(VkFence);
non_dispatchable_handle!(VkSemaphore);
non_dispatchable_handle!(VkEvent);
non_dispatchable_handle!(VkQueryPool);
non_dispatchable_handle!(VkFramebuffer);
non_dispatchable_handle!(VkRenderPass);
non_dispatchable_handle!(VkPipelineCache);
non_dispatchable_handle!(VkIndirectCommandsLayoutNV);
non_dispatchable_handle!(VkDescriptorUpdateTemplate);
type VkDescriptorUpdateTemplateKHR = VkDescriptorUpdateTemplate;
non_dispatchable_handle!(VkSamplerYcbcrConversion);
type VkSamplerYcbcrConversionKHR = VkSamplerYcbcrConversion;
non_dispatchable_handle!(VkValidationCacheEXT);
non_dispatchable_handle!(VkAccelerationStructureKHR);
non_dispatchable_handle!(VkAccelerationStructureNV);
non_dispatchable_handle!(VkPerformanceConfigurationINTEL);
non_dispatchable_handle!(VkBufferCollectionFUCHSIA);
non_dispatchable_handle!(VkDeferredOperationKHR);
non_dispatchable_handle!(VkPrivateDataSlot);
type VkPrivateDataSlotEXT = VkPrivateDataSlot;
non_dispatchable_handle!(VkCuModuleNVX);
non_dispatchable_handle!(VkCuFunctionNVX);
non_dispatchable_handle!(VkOpticalFlowSessionNV);
non_dispatchable_handle!(VkMicromapEXT);
non_dispatchable_handle!(VkDisplayKHR);
non_dispatchable_handle!(VkDisplayModeKHR);
non_dispatchable_handle!(VkSurfaceKHR);
non_dispatchable_handle!(VkSwapchainKHR);
non_dispatchable_handle!(VkDebugReportCallbackEXT);
non_dispatchable_handle!(VkDebugUtilsMessengerEXT);
non_dispatchable_handle!(VkVideoSessionKHR);
non_dispatchable_handle!(VkVideoSessionParametersKHR);
pub struct VkBaseOutStructure
{
	pub sType: VkStructureType,
	pub pNext: *mut VkBaseOutStructure
}
pub struct VkBaseInStructure
{
	pub sType: VkStructureType,
	pub pNext: *const VkBaseInStructure
}
pub struct VkOffset2D
{
	pub x: i32,
	pub y: i32
}
pub struct VkOffset3D
{
	pub x: i32,
	pub y: i32,
	pub z: i32
}
pub struct VkExtent2D
{
	pub width: u32,
	pub height: u32
}
pub struct VkExtent3D
{
	pub width: u32,
	pub height: u32,
	pub depth: u32
}
pub struct VkViewport
{
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
	pub minDepth: f32,
	pub maxDepth: f32
}
pub struct VkRect2D
{
	pub offset: VkOffset2D,
	pub extent: VkExtent2D
}
pub struct VkClearRect
{
	pub rect: VkRect2D,
	pub baseArrayLayer: u32,
	pub layerCount: u32
}
pub struct VkComponentMapping
{
	pub r: VkComponentSwizzle,
	pub g: VkComponentSwizzle,
	pub b: VkComponentSwizzle,
	pub a: VkComponentSwizzle
}
pub struct VkPhysicalDeviceProperties
{
	pub apiVersion: u32,
	pub driverVersion: u32,
	pub vendorID: u32,
	pub deviceID: u32,
	pub deviceType: VkPhysicalDeviceType,
	pub deviceName: [i8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
	pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
	pub limits: VkPhysicalDeviceLimits,
	pub sparseProperties: VkPhysicalDeviceSparseProperties
}
pub struct VkExtensionProperties
{
	pub extensionName: [i8; VK_MAX_EXTENSION_NAME_SIZE],
	pub specVersion: u32
}
pub struct VkLayerProperties
{
	pub layerName: [i8; VK_MAX_EXTENSION_NAME_SIZE],
	pub specVersion: u32,
	pub implementationVersion: u32,
	pub description: [i8; VK_MAX_DESCRIPTION_SIZE]
}
pub struct VkApplicationInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pApplicationName: *const i8,
	pub applicationVersion: u32,
	pub pEngineName: *const i8,
	pub engineVersion: u32,
	pub apiVersion: u32
}
pub struct VkAllocationCallbacks
{
	pub pUserData: *mut c_void,
	pub pfnAllocation: PFN_vkAllocationFunction,
	pub pfnReallocation: PFN_vkReallocationFunction,
	pub pfnFree: PFN_vkFreeFunction,
	pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
	pub pfnInternalFree: PFN_vkInternalFreeNotification
}
pub struct VkDeviceQueueCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDeviceQueueCreateFlags,
	pub queueFamilyIndex: u32,
	pub queueCount: u32,
	pub pQueuePriorities: *const f32
}
pub struct VkDeviceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDeviceCreateFlags,
	pub queueCreateInfoCount: u32,
	pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
	pub enabledLayerCount: u32,
	pub ppEnabledLayerNames: *const *const i8,
	pub enabledExtensionCount: u32,
	pub ppEnabledExtensionNames: *const *const i8,
	pub pEnabledFeatures: *const VkPhysicalDeviceFeatures
}
pub struct VkInstanceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkInstanceCreateFlags,
	pub pApplicationInfo: *const VkApplicationInfo,
	pub enabledLayerCount: u32,
	pub ppEnabledLayerNames: *const *const i8,
	pub enabledExtensionCount: u32,
	pub ppEnabledExtensionNames: *const *const i8
}
pub struct VkQueueFamilyProperties
{
	pub queueFlags: VkQueueFlags,
	pub queueCount: u32,
	pub timestampValidBits: u32,
	pub minImageTransferGranularity: VkExtent3D
}
pub struct VkPhysicalDeviceMemoryProperties
{
	pub memoryTypeCount: u32,
	pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
	pub memoryHeapCount: u32,
	pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS]
}
pub struct VkMemoryAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub allocationSize: VkDeviceSize,
	pub memoryTypeIndex: u32
}
pub struct VkMemoryRequirements
{
	pub size: VkDeviceSize,
	pub alignment: VkDeviceSize,
	pub memoryTypeBits: u32
}
pub struct VkSparseImageFormatProperties
{
	pub aspectMask: VkImageAspectFlags,
	pub imageGranularity: VkExtent3D,
	pub flags: VkSparseImageFormatFlags
}
pub struct VkSparseImageMemoryRequirements
{
	pub formatProperties: VkSparseImageFormatProperties,
	pub imageMipTailFirstLod: u32,
	pub imageMipTailSize: VkDeviceSize,
	pub imageMipTailOffset: VkDeviceSize,
	pub imageMipTailStride: VkDeviceSize
}
pub struct VkMemoryType
{
	pub propertyFlags: VkMemoryPropertyFlags,
	pub heapIndex: u32
}
pub struct VkMemoryHeap
{
	pub size: VkDeviceSize,
	pub flags: VkMemoryHeapFlags
}
pub struct VkMappedMemoryRange
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub memory: VkDeviceMemory,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize
}
pub struct VkFormatProperties
{
	pub linearTilingFeatures: VkFormatFeatureFlags,
	pub optimalTilingFeatures: VkFormatFeatureFlags,
	pub bufferFeatures: VkFormatFeatureFlags
}
pub struct VkImageFormatProperties
{
	pub maxExtent: VkExtent3D,
	pub maxMipLevels: u32,
	pub maxArrayLayers: u32,
	pub sampleCounts: VkSampleCountFlags,
	pub maxResourceSize: VkDeviceSize
}
pub struct VkDescriptorBufferInfo
{
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub range: VkDeviceSize
}
pub struct VkDescriptorImageInfo
{
	pub sampler: VkSampler,
	pub imageView: VkImageView,
	pub imageLayout: VkImageLayout
}
pub struct VkWriteDescriptorSet
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub dstSet: VkDescriptorSet,
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32,
	pub descriptorType: VkDescriptorType,
	pub pImageInfo: *const VkDescriptorImageInfo,
	pub pBufferInfo: *const VkDescriptorBufferInfo,
	pub pTexelBufferView: *const VkBufferView
}
pub struct VkCopyDescriptorSet
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcSet: VkDescriptorSet,
	pub srcBinding: u32,
	pub srcArrayElement: u32,
	pub dstSet: VkDescriptorSet,
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32
}
pub struct VkBufferCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkBufferCreateFlags,
	pub size: VkDeviceSize,
	pub usage: VkBufferUsageFlags,
	pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32
}
pub struct VkBufferViewCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkBufferViewCreateFlags,
	pub buffer: VkBuffer,
	pub format: VkFormat,
	pub offset: VkDeviceSize,
	pub range: VkDeviceSize
}
pub struct VkImageSubresource
{
	pub aspectMask: VkImageAspectFlags,
	pub mipLevel: u32,
	pub arrayLayer: u32
}
pub struct VkImageSubresourceLayers
{
	pub aspectMask: VkImageAspectFlags,
	pub mipLevel: u32,
	pub baseArrayLayer: u32,
	pub layerCount: u32
}
pub struct VkImageSubresourceRange
{
	pub aspectMask: VkImageAspectFlags,
	pub baseMipLevel: u32,
	pub levelCount: u32,
	pub baseArrayLayer: u32,
	pub layerCount: u32
}
pub struct VkMemoryBarrier
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags
}
pub struct VkBufferMemoryBarrier
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize
}
pub struct VkImageMemoryBarrier
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub oldLayout: VkImageLayout,
	pub newLayout: VkImageLayout,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub image: VkImage,
	pub subresourceRange: VkImageSubresourceRange
}
pub struct VkImageCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkImageCreateFlags,
	pub imageType: VkImageType,
	pub format: VkFormat,
	pub extent: VkExtent3D,
	pub mipLevels: u32,
	pub arrayLayers: u32,
	pub samples: VkSampleCountFlagBits,
	pub tiling: VkImageTiling,
	pub usage: VkImageUsageFlags,
	pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32,
	pub initialLayout: VkImageLayout
}
pub struct VkSubresourceLayout
{
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub rowPitch: VkDeviceSize,
	pub arrayPitch: VkDeviceSize,
	pub depthPitch: VkDeviceSize
}
pub struct VkImageViewCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkImageViewCreateFlags,
	pub image: VkImage,
	pub viewType: VkImageViewType,
	pub format: VkFormat,
	pub components: VkComponentMapping,
	pub subresourceRange: VkImageSubresourceRange
}
pub struct VkBufferCopy
{
	pub srcOffset: VkDeviceSize,
	pub dstOffset: VkDeviceSize,
	pub size: VkDeviceSize
}
pub struct VkSparseMemoryBind
{
	pub resourceOffset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub flags: VkSparseMemoryBindFlags
}
pub struct VkSparseImageMemoryBind
{
	pub subresource: VkImageSubresource,
	pub offset: VkOffset3D,
	pub extent: VkExtent3D,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub flags: VkSparseMemoryBindFlags
}
pub struct VkSparseBufferMemoryBindInfo
{
	pub buffer: VkBuffer,
	pub bindCount: u32,
	pub pBinds: *const VkSparseMemoryBind
}
pub struct VkSparseImageOpaqueMemoryBindInfo
{
	pub image: VkImage,
	pub bindCount: u32,
	pub pBinds: *const VkSparseMemoryBind
}
pub struct VkSparseImageMemoryBindInfo
{
	pub image: VkImage,
	pub bindCount: u32,
	pub pBinds: *const VkSparseImageMemoryBind
}
pub struct VkBindSparseInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: *const VkSemaphore,
	pub bufferBindCount: u32,
	pub pBufferBinds: *const VkSparseBufferMemoryBindInfo,
	pub imageOpaqueBindCount: u32,
	pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
	pub imageBindCount: u32,
	pub pImageBinds: *const VkSparseImageMemoryBindInfo,
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphores: *const VkSemaphore
}
pub struct VkImageCopy
{
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D
}
pub struct VkImageBlit
{
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffsets: [VkOffset3D; 2],
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffsets: [VkOffset3D; 2]
}
pub struct VkBufferImageCopy
{
	pub bufferOffset: VkDeviceSize,
	pub bufferRowLength: u32,
	pub bufferImageHeight: u32,
	pub imageSubresource: VkImageSubresourceLayers,
	pub imageOffset: VkOffset3D,
	pub imageExtent: VkExtent3D
}
pub struct VkCopyMemoryIndirectCommandNV
{
	pub srcAddress: VkDeviceAddress,
	pub dstAddress: VkDeviceAddress,
	pub size: VkDeviceSize
}
pub struct VkCopyMemoryToImageIndirectCommandNV
{
	pub srcAddress: VkDeviceAddress,
	pub bufferRowLength: u32,
	pub bufferImageHeight: u32,
	pub imageSubresource: VkImageSubresourceLayers,
	pub imageOffset: VkOffset3D,
	pub imageExtent: VkExtent3D
}
pub struct VkImageResolve
{
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D
}
pub struct VkShaderModuleCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkShaderModuleCreateFlags,
	pub codeSize: usize,
	pub pCode: *const u32
}
pub struct VkDescriptorSetLayoutBinding
{
	pub binding: u32,
	pub descriptorType: VkDescriptorType,
	pub descriptorCount: u32,
	pub stageFlags: VkShaderStageFlags,
	pub pImmutableSamplers: *const VkSampler
}
pub struct VkDescriptorSetLayoutCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDescriptorSetLayoutCreateFlags,
	pub bindingCount: u32,
	pub pBindings: *const VkDescriptorSetLayoutBinding
}
pub struct VkDescriptorPoolSize
{
	pub kind: VkDescriptorType,
	pub descriptorCount: u32
}
pub struct VkDescriptorPoolCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDescriptorPoolCreateFlags,
	pub maxSets: u32,
	pub poolSizeCount: u32,
	pub pPoolSizes: *const VkDescriptorPoolSize
}
pub struct VkDescriptorSetAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub descriptorPool: VkDescriptorPool,
	pub descriptorSetCount: u32,
	pub pSetLayouts: *const VkDescriptorSetLayout
}
pub struct VkSpecializationMapEntry
{
	pub constantID: u32,
	pub offset: u32,
	pub size: usize
}
pub struct VkSpecializationInfo
{
	pub mapEntryCount: u32,
	pub pMapEntries: *const VkSpecializationMapEntry,
	pub dataSize: usize,
	pub pData: *const c_void
}
pub struct VkPipelineShaderStageCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineShaderStageCreateFlags,
	pub stage: VkShaderStageFlagBits,
	pub module: VkShaderModule,
	pub pName: *const i8,
	pub pSpecializationInfo: *const VkSpecializationInfo
}
pub struct VkComputePipelineCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineCreateFlags,
	pub stage: VkPipelineShaderStageCreateInfo,
	pub layout: VkPipelineLayout,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32
}
pub struct VkVertexInputBindingDescription
{
	pub binding: u32,
	pub stride: u32,
	pub inputRate: VkVertexInputRate
}
pub struct VkVertexInputAttributeDescription
{
	pub location: u32,
	pub binding: u32,
	pub format: VkFormat,
	pub offset: u32
}
pub struct VkPipelineVertexInputStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineVertexInputStateCreateFlags,
	pub vertexBindingDescriptionCount: u32,
	pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
	pub vertexAttributeDescriptionCount: u32,
	pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription
}
pub struct VkPipelineInputAssemblyStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineInputAssemblyStateCreateFlags,
	pub topology: VkPrimitiveTopology,
	pub primitiveRestartEnable: VkBool32
}
pub struct VkPipelineTessellationStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineTessellationStateCreateFlags,
	pub patchControlPoints: u32
}
pub struct VkPipelineViewportStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineViewportStateCreateFlags,
	pub viewportCount: u32,
	pub pViewports: *const VkViewport,
	pub scissorCount: u32,
	pub pScissors: *const VkRect2D
}
pub struct VkPipelineRasterizationStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineRasterizationStateCreateFlags,
	pub depthClampEnable: VkBool32,
	pub rasterizerDiscardEnable: VkBool32,
	pub polygonMode: VkPolygonMode,
	pub cullMode: VkCullModeFlags,
	pub frontFace: VkFrontFace,
	pub depthBiasEnable: VkBool32,
	pub depthBiasConstantFactor: f32,
	pub depthBiasClamp: f32,
	pub depthBiasSlopeFactor: f32,
	pub lineWidth: f32
}
pub struct VkPipelineMultisampleStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineMultisampleStateCreateFlags,
	pub rasterizationSamples: VkSampleCountFlagBits,
	pub sampleShadingEnable: VkBool32,
	pub minSampleShading: f32,
	pub pSampleMask: *const VkSampleMask,
	pub alphaToCoverageEnable: VkBool32,
	pub alphaToOneEnable: VkBool32
}
pub struct VkPipelineColorBlendAttachmentState
{
	pub blendEnable: VkBool32,
	pub srcColorBlendFactor: VkBlendFactor,
	pub dstColorBlendFactor: VkBlendFactor,
	pub colorBlendOp: VkBlendOp,
	pub srcAlphaBlendFactor: VkBlendFactor,
	pub dstAlphaBlendFactor: VkBlendFactor,
	pub alphaBlendOp: VkBlendOp,
	pub colorWriteMask: VkColorComponentFlags
}
pub struct VkPipelineColorBlendStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineColorBlendStateCreateFlags,
	pub logicOpEnable: VkBool32,
	pub logicOp: VkLogicOp,
	pub attachmentCount: u32,
	pub pAttachments: *const VkPipelineColorBlendAttachmentState,
	pub blendConstants: [f32; 4]
}
pub struct VkPipelineDynamicStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineDynamicStateCreateFlags,
	pub dynamicStateCount: u32,
	pub pDynamicStates: *const VkDynamicState
}
pub struct VkStencilOpState
{
	pub failOp: VkStencilOp,
	pub passOp: VkStencilOp,
	pub depthFailOp: VkStencilOp,
	pub compareOp: VkCompareOp,
	pub compareMask: u32,
	pub writeMask: u32,
	pub reference: u32
}
pub struct VkPipelineDepthStencilStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineDepthStencilStateCreateFlags,
	pub depthTestEnable: VkBool32,
	pub depthWriteEnable: VkBool32,
	pub depthCompareOp: VkCompareOp,
	pub depthBoundsTestEnable: VkBool32,
	pub stencilTestEnable: VkBool32,
	pub front: VkStencilOpState,
	pub back: VkStencilOpState,
	pub minDepthBounds: f32,
	pub maxDepthBounds: f32
}
pub struct VkGraphicsPipelineCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineCreateFlags,
	pub stageCount: u32,
	pub pStages: *const VkPipelineShaderStageCreateInfo,
	pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
	pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
	pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
	pub pViewportState: *const VkPipelineViewportStateCreateInfo,
	pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
	pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
	pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
	pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
	pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
	pub layout: VkPipelineLayout,
	pub renderPass: VkRenderPass,
	pub subpass: u32,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32
}
pub struct VkPipelineCacheCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineCacheCreateFlags,
	pub initialDataSize: usize,
	pub pInitialData: *const c_void
}
pub struct VkPipelineCacheHeaderVersionOne
{
	pub headerSize: u32,
	pub headerVersion: VkPipelineCacheHeaderVersion,
	pub vendorID: u32,
	pub deviceID: u32,
	pub pipelineCacheUUID: [u8; VK_UUID_SIZE]
}
pub struct VkPushConstantRange
{
	pub stageFlags: VkShaderStageFlags,
	pub offset: u32,
	pub size: u32
}
pub struct VkPipelineLayoutCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineLayoutCreateFlags,
	pub setLayoutCount: u32,
	pub pSetLayouts: *const VkDescriptorSetLayout,
	pub pushConstantRangeCount: u32,
	pub pPushConstantRanges: *const VkPushConstantRange
}
pub struct VkSamplerCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkSamplerCreateFlags,
	pub magFilter: VkFilter,
	pub minFilter: VkFilter,
	pub mipmapMode: VkSamplerMipmapMode,
	pub addressModeU: VkSamplerAddressMode,
	pub addressModeV: VkSamplerAddressMode,
	pub addressModeW: VkSamplerAddressMode,
	pub mipLodBias: f32,
	pub anisotropyEnable: VkBool32,
	pub maxAnisotropy: f32,
	pub compareEnable: VkBool32,
	pub compareOp: VkCompareOp,
	pub minLod: f32,
	pub maxLod: f32,
	pub borderColor: VkBorderColor,
	pub unnormalizedCoordinates: VkBool32
}
pub struct VkCommandPoolCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkCommandPoolCreateFlags,
	pub queueFamilyIndex: u32
}
pub struct VkCommandBufferAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub commandPool: VkCommandPool,
	pub level: VkCommandBufferLevel,
	pub commandBufferCount: u32
}
pub struct VkCommandBufferInheritanceInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub renderPass: VkRenderPass,
	pub subpass: u32,
	pub framebuffer: VkFramebuffer,
	pub occlusionQueryEnable: VkBool32,
	pub queryFlags: VkQueryControlFlags,
	pub pipelineStatistics: VkQueryPipelineStatisticFlags
}
pub struct VkCommandBufferBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkCommandBufferUsageFlags,
	pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo
}
pub struct VkRenderPassBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub renderPass: VkRenderPass,
	pub framebuffer: VkFramebuffer,
	pub renderArea: VkRect2D,
	pub clearValueCount: u32,
	pub pClearValues: *const VkClearValue
}
pub struct VkClearDepthStencilValue
{
	pub depth: f32,
	pub stencil: u32
}
pub struct VkClearAttachment
{
	pub aspectMask: VkImageAspectFlags,
	pub colorAttachment: u32,
	pub clearValue: VkClearValue
}
pub struct VkAttachmentDescription
{
	pub flags: VkAttachmentDescriptionFlags,
	pub format: VkFormat,
	pub samples: VkSampleCountFlagBits,
	pub loadOp: VkAttachmentLoadOp,
	pub storeOp: VkAttachmentStoreOp,
	pub stencilLoadOp: VkAttachmentLoadOp,
	pub stencilStoreOp: VkAttachmentStoreOp,
	pub initialLayout: VkImageLayout,
	pub finalLayout: VkImageLayout
}
pub struct VkAttachmentReference
{
	pub attachment: u32,
	pub layout: VkImageLayout
}
pub struct VkSubpassDescription
{
	pub flags: VkSubpassDescriptionFlags,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub inputAttachmentCount: u32,
	pub pInputAttachments: *const VkAttachmentReference,
	pub colorAttachmentCount: u32,
	pub pColorAttachments: *const VkAttachmentReference,
	pub pResolveAttachments: *const VkAttachmentReference,
	pub pDepthStencilAttachment: *const VkAttachmentReference,
	pub preserveAttachmentCount: u32,
	pub pPreserveAttachments: *const u32
}
pub struct VkSubpassDependency
{
	pub srcSubpass: u32,
	pub dstSubpass: u32,
	pub srcStageMask: VkPipelineStageFlags,
	pub dstStageMask: VkPipelineStageFlags,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub dependencyFlags: VkDependencyFlags
}
pub struct VkRenderPassCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkRenderPassCreateFlags,
	pub attachmentCount: u32,
	pub pAttachments: *const VkAttachmentDescription,
	pub subpassCount: u32,
	pub pSubpasses: *const VkSubpassDescription,
	pub dependencyCount: u32,
	pub pDependencies: *const VkSubpassDependency
}
pub struct VkEventCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkEventCreateFlags
}
pub struct VkFenceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkFenceCreateFlags
}
pub struct VkPhysicalDeviceFeatures
{
	pub robustBufferAccess: VkBool32,
	pub fullDrawIndexUint32: VkBool32,
	pub imageCubeArray: VkBool32,
	pub independentBlend: VkBool32,
	pub geometryShader: VkBool32,
	pub tessellationShader: VkBool32,
	pub sampleRateShading: VkBool32,
	pub dualSrcBlend: VkBool32,
	pub logicOp: VkBool32,
	pub multiDrawIndirect: VkBool32,
	pub drawIndirectFirstInstance: VkBool32,
	pub depthClamp: VkBool32,
	pub depthBiasClamp: VkBool32,
	pub fillModeNonSolid: VkBool32,
	pub depthBounds: VkBool32,
	pub wideLines: VkBool32,
	pub largePoints: VkBool32,
	pub alphaToOne: VkBool32,
	pub multiViewport: VkBool32,
	pub samplerAnisotropy: VkBool32,
	pub textureCompressionETC2: VkBool32,
	pub textureCompressionASTC_LDR: VkBool32,
	pub textureCompressionBC: VkBool32,
	pub occlusionQueryPrecise: VkBool32,
	pub pipelineStatisticsQuery: VkBool32,
	pub vertexPipelineStoresAndAtomics: VkBool32,
	pub fragmentStoresAndAtomics: VkBool32,
	pub shaderTessellationAndGeometryPointSize: VkBool32,
	pub shaderImageGatherExtended: VkBool32,
	pub shaderStorageImageExtendedFormats: VkBool32,
	pub shaderStorageImageMultisample: VkBool32,
	pub shaderStorageImageReadWithoutFormat: VkBool32,
	pub shaderStorageImageWriteWithoutFormat: VkBool32,
	pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
	pub shaderSampledImageArrayDynamicIndexing: VkBool32,
	pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageImageArrayDynamicIndexing: VkBool32,
	pub shaderClipDistance: VkBool32,
	pub shaderCullDistance: VkBool32,
	pub shaderFloat64: VkBool32,
	pub shaderInt64: VkBool32,
	pub shaderInt16: VkBool32,
	pub shaderResourceResidency: VkBool32,
	pub shaderResourceMinLod: VkBool32,
	pub sparseBinding: VkBool32,
	pub sparseResidencyBuffer: VkBool32,
	pub sparseResidencyImage2D: VkBool32,
	pub sparseResidencyImage3D: VkBool32,
	pub sparseResidency2Samples: VkBool32,
	pub sparseResidency4Samples: VkBool32,
	pub sparseResidency8Samples: VkBool32,
	pub sparseResidency16Samples: VkBool32,
	pub sparseResidencyAliased: VkBool32,
	pub variableMultisampleRate: VkBool32,
	pub inheritedQueries: VkBool32
}
pub struct VkPhysicalDeviceSparseProperties
{
	pub residencyStandard2DBlockShape: VkBool32,
	pub residencyStandard2DMultisampleBlockShape: VkBool32,
	pub residencyStandard3DBlockShape: VkBool32,
	pub residencyAlignedMipSize: VkBool32,
	pub residencyNonResidentStrict: VkBool32
}
pub struct VkPhysicalDeviceLimits
{
	pub maxImageDimension1D: u32,
	pub maxImageDimension2D: u32,
	pub maxImageDimension3D: u32,
	pub maxImageDimensionCube: u32,
	pub maxImageArrayLayers: u32,
	pub maxTexelBufferElements: u32,
	pub maxUniformBufferRange: u32,
	pub maxStorageBufferRange: u32,
	pub maxPushConstantsSize: u32,
	pub maxMemoryAllocationCount: u32,
	pub maxSamplerAllocationCount: u32,
	pub bufferImageGranularity: VkDeviceSize,
	pub sparseAddressSpaceSize: VkDeviceSize,
	pub maxBoundDescriptorSets: u32,
	pub maxPerStageDescriptorSamplers: u32,
	pub maxPerStageDescriptorUniformBuffers: u32,
	pub maxPerStageDescriptorStorageBuffers: u32,
	pub maxPerStageDescriptorSampledImages: u32,
	pub maxPerStageDescriptorStorageImages: u32,
	pub maxPerStageDescriptorInputAttachments: u32,
	pub maxPerStageResources: u32,
	pub maxDescriptorSetSamplers: u32,
	pub maxDescriptorSetUniformBuffers: u32,
	pub maxDescriptorSetUniformBuffersDynamic: u32,
	pub maxDescriptorSetStorageBuffers: u32,
	pub maxDescriptorSetStorageBuffersDynamic: u32,
	pub maxDescriptorSetSampledImages: u32,
	pub maxDescriptorSetStorageImages: u32,
	pub maxDescriptorSetInputAttachments: u32,
	pub maxVertexInputAttributes: u32,
	pub maxVertexInputBindings: u32,
	pub maxVertexInputAttributeOffset: u32,
	pub maxVertexInputBindingStride: u32,
	pub maxVertexOutputComponents: u32,
	pub maxTessellationGenerationLevel: u32,
	pub maxTessellationPatchSize: u32,
	pub maxTessellationControlPerVertexInputComponents: u32,
	pub maxTessellationControlPerVertexOutputComponents: u32,
	pub maxTessellationControlPerPatchOutputComponents: u32,
	pub maxTessellationControlTotalOutputComponents: u32,
	pub maxTessellationEvaluationInputComponents: u32,
	pub maxTessellationEvaluationOutputComponents: u32,
	pub maxGeometryShaderInvocations: u32,
	pub maxGeometryInputComponents: u32,
	pub maxGeometryOutputComponents: u32,
	pub maxGeometryOutputVertices: u32,
	pub maxGeometryTotalOutputComponents: u32,
	pub maxFragmentInputComponents: u32,
	pub maxFragmentOutputAttachments: u32,
	pub maxFragmentDualSrcAttachments: u32,
	pub maxFragmentCombinedOutputResources: u32,
	pub maxComputeSharedMemorySize: u32,
	pub maxComputeWorkGroupCount: [u32; 3],
	pub maxComputeWorkGroupInvocations: u32,
	pub maxComputeWorkGroupSize: [u32; 3],
	pub subPixelPrecisionBits: u32,
	pub subTexelPrecisionBits: u32,
	pub mipmapPrecisionBits: u32,
	pub maxDrawIndexedIndexValue: u32,
	pub maxDrawIndirectCount: u32,
	pub maxSamplerLodBias: f32,
	pub maxSamplerAnisotropy: f32,
	pub maxViewports: u32,
	pub maxViewportDimensions: [u32; 2],
	pub viewportBoundsRange: [f32; 2],
	pub viewportSubPixelBits: u32,
	pub minMemoryMapAlignment: usize,
	pub minTexelBufferOffsetAlignment: VkDeviceSize,
	pub minUniformBufferOffsetAlignment: VkDeviceSize,
	pub minStorageBufferOffsetAlignment: VkDeviceSize,
	pub minTexelOffset: i32,
	pub maxTexelOffset: u32,
	pub minTexelGatherOffset: i32,
	pub maxTexelGatherOffset: u32,
	pub minInterpolationOffset: f32,
	pub maxInterpolationOffset: f32,
	pub subPixelInterpolationOffsetBits: u32,
	pub maxFramebufferWidth: u32,
	pub maxFramebufferHeight: u32,
	pub maxFramebufferLayers: u32,
	pub framebufferColorSampleCounts: VkSampleCountFlags,
	pub framebufferDepthSampleCounts: VkSampleCountFlags,
	pub framebufferStencilSampleCounts: VkSampleCountFlags,
	pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
	pub maxColorAttachments: u32,
	pub sampledImageColorSampleCounts: VkSampleCountFlags,
	pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
	pub sampledImageDepthSampleCounts: VkSampleCountFlags,
	pub sampledImageStencilSampleCounts: VkSampleCountFlags,
	pub storageImageSampleCounts: VkSampleCountFlags,
	pub maxSampleMaskWords: u32,
	pub timestampComputeAndGraphics: VkBool32,
	pub timestampPeriod: f32,
	pub maxClipDistances: u32,
	pub maxCullDistances: u32,
	pub maxCombinedClipAndCullDistances: u32,
	pub discreteQueuePriorities: u32,
	pub pointSizeRange: [f32; 2],
	pub lineWidthRange: [f32; 2],
	pub pointSizeGranularity: f32,
	pub lineWidthGranularity: f32,
	pub strictLines: VkBool32,
	pub standardSampleLocations: VkBool32,
	pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
	pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
	pub nonCoherentAtomSize: VkDeviceSize
}
pub struct VkSemaphoreCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkSemaphoreCreateFlags
}
pub struct VkQueryPoolCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkQueryPoolCreateFlags,
	pub queryType: VkQueryType,
	pub queryCount: u32,
	pub pipelineStatistics: VkQueryPipelineStatisticFlags
}
pub struct VkFramebufferCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkFramebufferCreateFlags,
	pub renderPass: VkRenderPass,
	pub attachmentCount: u32,
	pub pAttachments: *const VkImageView,
	pub width: u32,
	pub height: u32,
	pub layers: u32
}
pub struct VkDrawIndirectCommand
{
	pub vertexCount: u32,
	pub instanceCount: u32,
	pub firstVertex: u32,
	pub firstInstance: u32
}
pub struct VkDrawIndexedIndirectCommand
{
	pub indexCount: u32,
	pub instanceCount: u32,
	pub firstIndex: u32,
	pub vertexOffset: i32,
	pub firstInstance: u32
}
pub struct VkDispatchIndirectCommand
{
	pub x: u32,
	pub y: u32,
	pub z: u32
}
pub struct VkMultiDrawInfoEXT
{
	pub firstVertex: u32,
	pub vertexCount: u32
}
pub struct VkMultiDrawIndexedInfoEXT
{
	pub firstIndex: u32,
	pub indexCount: u32,
	pub vertexOffset: i32
}
pub struct VkSubmitInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: *const VkSemaphore,
	pub pWaitDstStageMask: *const VkPipelineStageFlags,
	pub commandBufferCount: u32,
	pub pCommandBuffers: *const VkCommandBuffer,
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphores: *const VkSemaphore
}
pub struct VkDisplayPropertiesKHR
{
	pub display: VkDisplayKHR,
	pub displayName: *const i8,
	pub physicalDimensions: VkExtent2D,
	pub physicalResolution: VkExtent2D,
	pub supportedTransforms: VkSurfaceTransformFlagsKHR,
	pub planeReorderPossible: VkBool32,
	pub persistentContent: VkBool32
}
pub struct VkDisplayPlanePropertiesKHR
{
	pub currentDisplay: VkDisplayKHR,
	pub currentStackIndex: u32
}
pub struct VkDisplayModeParametersKHR
{
	pub visibleRegion: VkExtent2D,
	pub refreshRate: u32
}
pub struct VkDisplayModePropertiesKHR
{
	pub displayMode: VkDisplayModeKHR,
	pub parameters: VkDisplayModeParametersKHR
}
pub struct VkDisplayModeCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDisplayModeCreateFlagsKHR,
	pub parameters: VkDisplayModeParametersKHR
}
pub struct VkDisplayPlaneCapabilitiesKHR
{
	pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
	pub minSrcPosition: VkOffset2D,
	pub maxSrcPosition: VkOffset2D,
	pub minSrcExtent: VkExtent2D,
	pub maxSrcExtent: VkExtent2D,
	pub minDstPosition: VkOffset2D,
	pub maxDstPosition: VkOffset2D,
	pub minDstExtent: VkExtent2D,
	pub maxDstExtent: VkExtent2D
}
pub struct VkDisplaySurfaceCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDisplaySurfaceCreateFlagsKHR,
	pub displayMode: VkDisplayModeKHR,
	pub planeIndex: u32,
	pub planeStackIndex: u32,
	pub transform: VkSurfaceTransformFlagBitsKHR,
	pub globalAlpha: f32,
	pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
	pub imageExtent: VkExtent2D
}
pub struct VkDisplayPresentInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcRect: VkRect2D,
	pub dstRect: VkRect2D,
	pub persistent: VkBool32
}
pub struct VkSurfaceCapabilitiesKHR
{
	pub minImageCount: u32,
	pub maxImageCount: u32,
	pub currentExtent: VkExtent2D,
	pub minImageExtent: VkExtent2D,
	pub maxImageExtent: VkExtent2D,
	pub maxImageArrayLayers: u32,
	pub supportedTransforms: VkSurfaceTransformFlagsKHR,
	pub currentTransform: VkSurfaceTransformFlagBitsKHR,
	pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
	pub supportedUsageFlags: VkImageUsageFlags
}
pub struct VkAndroidSurfaceCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkAndroidSurfaceCreateFlagsKHR,
	pub window: *mut ANativeWindow
}
pub struct VkViSurfaceCreateInfoNN
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkViSurfaceCreateFlagsNN,
	pub window: *mut c_void
}
pub struct VkWaylandSurfaceCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkWaylandSurfaceCreateFlagsKHR,
	pub display: *mut wl_display,
	pub surface: *mut wl_surface
}
pub struct VkWin32SurfaceCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkWin32SurfaceCreateFlagsKHR,
	pub hinstance: HINSTANCE,
	pub hwnd: HWND
}
pub struct VkXlibSurfaceCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkXlibSurfaceCreateFlagsKHR,
	pub dpy: *mut Display,
	pub window: Window
}
pub struct VkXcbSurfaceCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkXcbSurfaceCreateFlagsKHR,
	pub connection: *mut xcb_connection_t,
	pub window: xcb_window_t
}
pub struct VkDirectFBSurfaceCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDirectFBSurfaceCreateFlagsEXT,
	pub dfb: *mut IDirectFB,
	pub surface: *mut IDirectFBSurface
}
pub struct VkImagePipeSurfaceCreateInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkImagePipeSurfaceCreateFlagsFUCHSIA,
	pub imagePipeHandle: zx_handle_t
}
pub struct VkStreamDescriptorSurfaceCreateInfoGGP
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkStreamDescriptorSurfaceCreateFlagsGGP,
	pub streamDescriptor: GgpStreamDescriptor
}
pub struct VkScreenSurfaceCreateInfoQNX
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkScreenSurfaceCreateFlagsQNX,
	pub context: *mut _screen_context,
	pub window: *mut _screen_window
}
pub struct VkSurfaceFormatKHR
{
	pub format: VkFormat,
	pub colorSpace: VkColorSpaceKHR
}
pub struct VkSwapchainCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkSwapchainCreateFlagsKHR,
	pub surface: VkSurfaceKHR,
	pub minImageCount: u32,
	pub imageFormat: VkFormat,
	pub imageColorSpace: VkColorSpaceKHR,
	pub imageExtent: VkExtent2D,
	pub imageArrayLayers: u32,
	pub imageUsage: VkImageUsageFlags,
	pub imageSharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32,
	pub preTransform: VkSurfaceTransformFlagBitsKHR,
	pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
	pub presentMode: VkPresentModeKHR,
	pub clipped: VkBool32,
	pub oldSwapchain: VkSwapchainKHR
}
pub struct VkPresentInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: *const VkSemaphore,
	pub swapchainCount: u32,
	pub pSwapchains: *const VkSwapchainKHR,
	pub pImageIndices: *const u32,
	pub pResults: *mut VkResult
}
pub struct VkDebugReportCallbackCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDebugReportFlagsEXT,
	pub pfnCallback: PFN_vkDebugReportCallbackEXT,
	pub pUserData: *mut c_void
}
pub struct VkValidationFlagsEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub disabledValidationCheckCount: u32,
	pub pDisabledValidationChecks: *const VkValidationCheckEXT
}
pub struct VkValidationFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub enabledValidationFeatureCount: u32,
	pub pEnabledValidationFeatures: *const VkValidationFeatureEnableEXT,
	pub disabledValidationFeatureCount: u32,
	pub pDisabledValidationFeatures: *const VkValidationFeatureDisableEXT
}
pub struct VkPipelineRasterizationStateRasterizationOrderAMD
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub rasterizationOrder: VkRasterizationOrderAMD
}
pub struct VkDebugMarkerObjectNameInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub objectType: VkDebugReportObjectTypeEXT,
	pub object: uint64_t,
	pub pObjectName: *const i8
}
pub struct VkDebugMarkerObjectTagInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub objectType: VkDebugReportObjectTypeEXT,
	pub object: uint64_t,
	pub tagName: uint64_t,
	pub tagSize: usize,
	pub pTag: *const c_void
}
pub struct VkDebugMarkerMarkerInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pMarkerName: *const i8,
	pub color: [f32; 4]
}
pub struct VkDedicatedAllocationImageCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub dedicatedAllocation: VkBool32
}
pub struct VkDedicatedAllocationBufferCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub dedicatedAllocation: VkBool32
}
pub struct VkDedicatedAllocationMemoryAllocateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub image: VkImage,
	pub buffer: VkBuffer
}
pub struct VkExternalImageFormatPropertiesNV
{
	pub imageFormatProperties: VkImageFormatProperties,
	pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV,
	pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
	pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV
}
pub struct VkExternalMemoryImageCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleTypes: VkExternalMemoryHandleTypeFlagsNV
}
pub struct VkExportMemoryAllocateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleTypes: VkExternalMemoryHandleTypeFlagsNV
}
pub struct VkImportMemoryWin32HandleInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleType: VkExternalMemoryHandleTypeFlagsNV,
	pub handle: HANDLE
}
pub struct VkExportMemoryWin32HandleInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pAttributes: *const SECURITY_ATTRIBUTES,
	pub dwAccess: DWORD
}
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub acquireCount: u32,
	pub pAcquireSyncs: *const VkDeviceMemory,
	pub pAcquireKeys: *const uint64_t,
	pub pAcquireTimeoutMilliseconds: *const u32,
	pub releaseCount: u32,
	pub pReleaseSyncs: *const VkDeviceMemory,
	pub pReleaseKeys: *const uint64_t
}
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub deviceGeneratedCommands: VkBool32
}
pub struct VkDevicePrivateDataCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub privateDataSlotRequestCount: u32
}
type VkDevicePrivateDataCreateInfoEXT = VkDevicePrivateDataCreateInfo;
pub struct VkPrivateDataSlotCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPrivateDataSlotCreateFlags
}
type VkPrivateDataSlotCreateInfoEXT = VkPrivateDataSlotCreateInfo;
pub struct VkPhysicalDevicePrivateDataFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub privateData: VkBool32
}
type VkPhysicalDevicePrivateDataFeaturesEXT = VkPhysicalDevicePrivateDataFeatures;
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxGraphicsShaderGroupCount: u32,
	pub maxIndirectSequenceCount: u32,
	pub maxIndirectCommandsTokenCount: u32,
	pub maxIndirectCommandsStreamCount: u32,
	pub maxIndirectCommandsTokenOffset: u32,
	pub maxIndirectCommandsStreamStride: u32,
	pub minSequencesCountBufferOffsetAlignment: u32,
	pub minSequencesIndexBufferOffsetAlignment: u32,
	pub minIndirectCommandsBufferOffsetAlignment: u32
}
pub struct VkPhysicalDeviceMultiDrawPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxMultiDrawCount: u32
}
pub struct VkGraphicsShaderGroupCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub stageCount: u32,
	pub pStages: *const VkPipelineShaderStageCreateInfo,
	pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
	pub pTessellationState: *const VkPipelineTessellationStateCreateInfo
}
pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub groupCount: u32,
	pub pGroups: *const VkGraphicsShaderGroupCreateInfoNV,
	pub pipelineCount: u32,
	pub pPipelines: *const VkPipeline
}
pub struct VkBindShaderGroupIndirectCommandNV
{
	pub groupIndex: u32
}
pub struct VkBindIndexBufferIndirectCommandNV
{
	pub bufferAddress: VkDeviceAddress,
	pub size: u32,
	pub indexType: VkIndexType
}
pub struct VkBindVertexBufferIndirectCommandNV
{
	pub bufferAddress: VkDeviceAddress,
	pub size: u32,
	pub stride: u32
}
pub struct VkSetStateFlagsIndirectCommandNV
{
	pub data: u32
}
pub struct VkIndirectCommandsStreamNV
{
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize
}
pub struct VkIndirectCommandsLayoutTokenNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub tokenType: VkIndirectCommandsTokenTypeNV,
	pub stream: u32,
	pub offset: u32,
	pub vertexBindingUnit: u32,
	pub vertexDynamicStride: VkBool32,
	pub pushconstantPipelineLayout: VkPipelineLayout,
	pub pushconstantShaderStageFlags: VkShaderStageFlags,
	pub pushconstantOffset: u32,
	pub pushconstantSize: u32,
	pub indirectStateFlags: VkIndirectStateFlagsNV,
	pub indexTypeCount: u32,
	pub pIndexTypes: *const VkIndexType,
	pub pIndexTypeValues: *const u32
}
pub struct VkIndirectCommandsLayoutCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkIndirectCommandsLayoutUsageFlagsNV,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub tokenCount: u32,
	pub pTokens: *const VkIndirectCommandsLayoutTokenNV,
	pub streamCount: u32,
	pub pStreamStrides: *const u32
}
pub struct VkGeneratedCommandsInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub pipeline: VkPipeline,
	pub indirectCommandsLayout: VkIndirectCommandsLayoutNV,
	pub streamCount: u32,
	pub pStreams: *const VkIndirectCommandsStreamNV,
	pub sequencesCount: u32,
	pub preprocessBuffer: VkBuffer,
	pub preprocessOffset: VkDeviceSize,
	pub preprocessSize: VkDeviceSize,
	pub sequencesCountBuffer: VkBuffer,
	pub sequencesCountOffset: VkDeviceSize,
	pub sequencesIndexBuffer: VkBuffer,
	pub sequencesIndexOffset: VkDeviceSize
}
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub pipeline: VkPipeline,
	pub indirectCommandsLayout: VkIndirectCommandsLayoutNV,
	pub maxSequencesCount: u32
}
pub struct VkPhysicalDeviceFeatures2
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub features: VkPhysicalDeviceFeatures
}
type VkPhysicalDeviceFeatures2KHR = VkPhysicalDeviceFeatures2;
pub struct VkPhysicalDeviceProperties2
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub properties: VkPhysicalDeviceProperties
}
type VkPhysicalDeviceProperties2KHR = VkPhysicalDeviceProperties2;
pub struct VkFormatProperties2
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub formatProperties: VkFormatProperties
}
type VkFormatProperties2KHR = VkFormatProperties2;
pub struct VkImageFormatProperties2
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub imageFormatProperties: VkImageFormatProperties
}
type VkImageFormatProperties2KHR = VkImageFormatProperties2;
pub struct VkPhysicalDeviceImageFormatInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub format: VkFormat,
	pub kind: VkImageType,
	pub tiling: VkImageTiling,
	pub usage: VkImageUsageFlags,
	pub flags: VkImageCreateFlags
}
type VkPhysicalDeviceImageFormatInfo2KHR = VkPhysicalDeviceImageFormatInfo2;
pub struct VkQueueFamilyProperties2
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub queueFamilyProperties: VkQueueFamilyProperties
}
type VkQueueFamilyProperties2KHR = VkQueueFamilyProperties2;
pub struct VkPhysicalDeviceMemoryProperties2
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryProperties: VkPhysicalDeviceMemoryProperties
}
type VkPhysicalDeviceMemoryProperties2KHR = VkPhysicalDeviceMemoryProperties2;
pub struct VkSparseImageFormatProperties2
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub properties: VkSparseImageFormatProperties
}
type VkSparseImageFormatProperties2KHR = VkSparseImageFormatProperties2;
pub struct VkPhysicalDeviceSparseImageFormatInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub format: VkFormat,
	pub kind: VkImageType,
	pub samples: VkSampleCountFlagBits,
	pub usage: VkImageUsageFlags,
	pub tiling: VkImageTiling
}
type VkPhysicalDeviceSparseImageFormatInfo2KHR = VkPhysicalDeviceSparseImageFormatInfo2;
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxPushDescriptors: u32
}
pub struct VkConformanceVersion
{
	pub major: u8,
	pub minor: u8,
	pub subminor: u8,
	pub patch: u8
}
type VkConformanceVersionKHR = VkConformanceVersion;
pub struct VkPhysicalDeviceDriverProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub driverID: VkDriverId,
	pub driverName: [i8; VK_MAX_DRIVER_NAME_SIZE],
	pub driverInfo: [i8; VK_MAX_DRIVER_INFO_SIZE],
	pub conformanceVersion: VkConformanceVersion
}
type VkPhysicalDeviceDriverPropertiesKHR = VkPhysicalDeviceDriverProperties;
pub struct VkPresentRegionsKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub swapchainCount: u32,
	pub pRegions: *const VkPresentRegionKHR
}
pub struct VkPresentRegionKHR
{
	pub rectangleCount: u32,
	pub pRectangles: *const VkRectLayerKHR
}
pub struct VkRectLayerKHR
{
	pub offset: VkOffset2D,
	pub extent: VkExtent2D,
	pub layer: u32
}
pub struct VkPhysicalDeviceVariablePointersFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub variablePointersStorageBuffer: VkBool32,
	pub variablePointers: VkBool32
}
type VkPhysicalDeviceVariablePointersFeaturesKHR = VkPhysicalDeviceVariablePointersFeatures;
type VkPhysicalDeviceVariablePointerFeaturesKHR = VkPhysicalDeviceVariablePointersFeatures;
type VkPhysicalDeviceVariablePointerFeatures = VkPhysicalDeviceVariablePointersFeatures;
pub struct VkExternalMemoryProperties
{
	pub externalMemoryFeatures: VkExternalMemoryFeatureFlags,
	pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlags,
	pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlags
}
type VkExternalMemoryPropertiesKHR = VkExternalMemoryProperties;
pub struct VkPhysicalDeviceExternalImageFormatInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleType: VkExternalMemoryHandleTypeFlagBits
}
type VkPhysicalDeviceExternalImageFormatInfoKHR = VkPhysicalDeviceExternalImageFormatInfo;
pub struct VkExternalImageFormatProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub externalMemoryProperties: VkExternalMemoryProperties
}
type VkExternalImageFormatPropertiesKHR = VkExternalImageFormatProperties;
pub struct VkPhysicalDeviceExternalBufferInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkBufferCreateFlags,
	pub usage: VkBufferUsageFlags,
	pub handleType: VkExternalMemoryHandleTypeFlagBits
}
type VkPhysicalDeviceExternalBufferInfoKHR = VkPhysicalDeviceExternalBufferInfo;
pub struct VkExternalBufferProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub externalMemoryProperties: VkExternalMemoryProperties
}
type VkExternalBufferPropertiesKHR = VkExternalBufferProperties;
pub struct VkPhysicalDeviceIDProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub deviceUUID: [u8; VK_UUID_SIZE],
	pub driverUUID: [u8; VK_UUID_SIZE],
	pub deviceLUID: [u8; VK_LUID_SIZE],
	pub deviceNodeMask: u32,
	pub deviceLUIDValid: VkBool32
}
type VkPhysicalDeviceIDPropertiesKHR = VkPhysicalDeviceIDProperties;
pub struct VkExternalMemoryImageCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleTypes: VkExternalMemoryHandleTypeFlags
}
type VkExternalMemoryImageCreateInfoKHR = VkExternalMemoryImageCreateInfo;
pub struct VkExternalMemoryBufferCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleTypes: VkExternalMemoryHandleTypeFlags
}
type VkExternalMemoryBufferCreateInfoKHR = VkExternalMemoryBufferCreateInfo;
pub struct VkExportMemoryAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleTypes: VkExternalMemoryHandleTypeFlags
}
type VkExportMemoryAllocateInfoKHR = VkExportMemoryAllocateInfo;
pub struct VkImportMemoryWin32HandleInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
	pub handle: HANDLE,
	pub name: LPCWSTR
}
pub struct VkExportMemoryWin32HandleInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pAttributes: *const SECURITY_ATTRIBUTES,
	pub dwAccess: DWORD,
	pub name: LPCWSTR
}
pub struct VkImportMemoryZirconHandleInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
	pub handle: zx_handle_t
}
pub struct VkMemoryZirconHandlePropertiesFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryTypeBits: u32
}
pub struct VkMemoryGetZirconHandleInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub memory: VkDeviceMemory,
	pub handleType: VkExternalMemoryHandleTypeFlagBits
}
pub struct VkMemoryWin32HandlePropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryTypeBits: u32
}
pub struct VkMemoryGetWin32HandleInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub memory: VkDeviceMemory,
	pub handleType: VkExternalMemoryHandleTypeFlagBits
}
pub struct VkImportMemoryFdInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
	pub fd: int
}
pub struct VkMemoryFdPropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryTypeBits: u32
}
pub struct VkMemoryGetFdInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub memory: VkDeviceMemory,
	pub handleType: VkExternalMemoryHandleTypeFlagBits
}
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub acquireCount: u32,
	pub pAcquireSyncs: *const VkDeviceMemory,
	pub pAcquireKeys: *const uint64_t,
	pub pAcquireTimeouts: *const u32,
	pub releaseCount: u32,
	pub pReleaseSyncs: *const VkDeviceMemory,
	pub pReleaseKeys: *const uint64_t
}
pub struct VkPhysicalDeviceExternalSemaphoreInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits
}
type VkPhysicalDeviceExternalSemaphoreInfoKHR = VkPhysicalDeviceExternalSemaphoreInfo;
pub struct VkExternalSemaphoreProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlags,
	pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlags,
	pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlags
}
type VkExternalSemaphorePropertiesKHR = VkExternalSemaphoreProperties;
pub struct VkExportSemaphoreCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleTypes: VkExternalSemaphoreHandleTypeFlags
}
type VkExportSemaphoreCreateInfoKHR = VkExportSemaphoreCreateInfo;
pub struct VkImportSemaphoreWin32HandleInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub semaphore: VkSemaphore,
	pub flags: VkSemaphoreImportFlags,
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
	pub handle: HANDLE,
	pub name: LPCWSTR
}
pub struct VkExportSemaphoreWin32HandleInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pAttributes: *const SECURITY_ATTRIBUTES,
	pub dwAccess: DWORD,
	pub name: LPCWSTR
}
pub struct VkD3D12FenceSubmitInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub waitSemaphoreValuesCount: u32,
	pub pWaitSemaphoreValues: *const uint64_t,
	pub signalSemaphoreValuesCount: u32,
	pub pSignalSemaphoreValues: *const uint64_t
}
pub struct VkSemaphoreGetWin32HandleInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub semaphore: VkSemaphore,
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits
}
pub struct VkImportSemaphoreFdInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub semaphore: VkSemaphore,
	pub flags: VkSemaphoreImportFlags,
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
	pub fd: int
}
pub struct VkSemaphoreGetFdInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub semaphore: VkSemaphore,
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits
}
pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub semaphore: VkSemaphore,
	pub flags: VkSemaphoreImportFlags,
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
	pub zirconHandle: zx_handle_t
}
pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub semaphore: VkSemaphore,
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits
}
pub struct VkPhysicalDeviceExternalFenceInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleType: VkExternalFenceHandleTypeFlagBits
}
type VkPhysicalDeviceExternalFenceInfoKHR = VkPhysicalDeviceExternalFenceInfo;
pub struct VkExternalFenceProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlags,
	pub compatibleHandleTypes: VkExternalFenceHandleTypeFlags,
	pub externalFenceFeatures: VkExternalFenceFeatureFlags
}
type VkExternalFencePropertiesKHR = VkExternalFenceProperties;
pub struct VkExportFenceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleTypes: VkExternalFenceHandleTypeFlags
}
type VkExportFenceCreateInfoKHR = VkExportFenceCreateInfo;
pub struct VkImportFenceWin32HandleInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub fence: VkFence,
	pub flags: VkFenceImportFlags,
	pub handleType: VkExternalFenceHandleTypeFlagBits,
	pub handle: HANDLE,
	pub name: LPCWSTR
}
pub struct VkExportFenceWin32HandleInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pAttributes: *const SECURITY_ATTRIBUTES,
	pub dwAccess: DWORD,
	pub name: LPCWSTR
}
pub struct VkFenceGetWin32HandleInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub fence: VkFence,
	pub handleType: VkExternalFenceHandleTypeFlagBits
}
pub struct VkImportFenceFdInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub fence: VkFence,
	pub flags: VkFenceImportFlags,
	pub handleType: VkExternalFenceHandleTypeFlagBits,
	pub fd: int
}
pub struct VkFenceGetFdInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub fence: VkFence,
	pub handleType: VkExternalFenceHandleTypeFlagBits
}
pub struct VkPhysicalDeviceMultiviewFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub multiview: VkBool32,
	pub multiviewGeometryShader: VkBool32,
	pub multiviewTessellationShader: VkBool32
}
type VkPhysicalDeviceMultiviewFeaturesKHR = VkPhysicalDeviceMultiviewFeatures;
pub struct VkPhysicalDeviceMultiviewProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxMultiviewViewCount: u32,
	pub maxMultiviewInstanceIndex: u32
}
type VkPhysicalDeviceMultiviewPropertiesKHR = VkPhysicalDeviceMultiviewProperties;
pub struct VkRenderPassMultiviewCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub subpassCount: u32,
	pub pViewMasks: *const u32,
	pub dependencyCount: u32,
	pub pViewOffsets: *const i32,
	pub correlationMaskCount: u32,
	pub pCorrelationMasks: *const u32
}
type VkRenderPassMultiviewCreateInfoKHR = VkRenderPassMultiviewCreateInfo;
pub struct VkSurfaceCapabilities2EXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub minImageCount: u32,
	pub maxImageCount: u32,
	pub currentExtent: VkExtent2D,
	pub minImageExtent: VkExtent2D,
	pub maxImageExtent: VkExtent2D,
	pub maxImageArrayLayers: u32,
	pub supportedTransforms: VkSurfaceTransformFlagsKHR,
	pub currentTransform: VkSurfaceTransformFlagBitsKHR,
	pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
	pub supportedUsageFlags: VkImageUsageFlags,
	pub supportedSurfaceCounters: VkSurfaceCounterFlagsEXT
}
pub struct VkDisplayPowerInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub powerState: VkDisplayPowerStateEXT
}
pub struct VkDeviceEventInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub deviceEvent: VkDeviceEventTypeEXT
}
pub struct VkDisplayEventInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub displayEvent: VkDisplayEventTypeEXT
}
pub struct VkSwapchainCounterCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub surfaceCounters: VkSurfaceCounterFlagsEXT
}
pub struct VkPhysicalDeviceGroupProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub physicalDeviceCount: u32,
	pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE],
	pub subsetAllocation: VkBool32
}
type VkPhysicalDeviceGroupPropertiesKHR = VkPhysicalDeviceGroupProperties;
pub struct VkMemoryAllocateFlagsInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkMemoryAllocateFlags,
	pub deviceMask: u32
}
type VkMemoryAllocateFlagsInfoKHR = VkMemoryAllocateFlagsInfo;
pub struct VkBindBufferMemoryInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub buffer: VkBuffer,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize
}
type VkBindBufferMemoryInfoKHR = VkBindBufferMemoryInfo;
pub struct VkBindBufferMemoryDeviceGroupInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub deviceIndexCount: u32,
	pub pDeviceIndices: *const u32
}
type VkBindBufferMemoryDeviceGroupInfoKHR = VkBindBufferMemoryDeviceGroupInfo;
pub struct VkBindImageMemoryInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub image: VkImage,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize
}
type VkBindImageMemoryInfoKHR = VkBindImageMemoryInfo;
pub struct VkBindImageMemoryDeviceGroupInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub deviceIndexCount: u32,
	pub pDeviceIndices: *const u32,
	pub splitInstanceBindRegionCount: u32,
	pub pSplitInstanceBindRegions: *const VkRect2D
}
type VkBindImageMemoryDeviceGroupInfoKHR = VkBindImageMemoryDeviceGroupInfo;
pub struct VkDeviceGroupRenderPassBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub deviceMask: u32,
	pub deviceRenderAreaCount: u32,
	pub pDeviceRenderAreas: *const VkRect2D
}
type VkDeviceGroupRenderPassBeginInfoKHR = VkDeviceGroupRenderPassBeginInfo;
pub struct VkDeviceGroupCommandBufferBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub deviceMask: u32
}
type VkDeviceGroupCommandBufferBeginInfoKHR = VkDeviceGroupCommandBufferBeginInfo;
pub struct VkDeviceGroupSubmitInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphoreDeviceIndices: *const u32,
	pub commandBufferCount: u32,
	pub pCommandBufferDeviceMasks: *const u32,
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphoreDeviceIndices: *const u32
}
type VkDeviceGroupSubmitInfoKHR = VkDeviceGroupSubmitInfo;
pub struct VkDeviceGroupBindSparseInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub resourceDeviceIndex: u32,
	pub memoryDeviceIndex: u32
}
type VkDeviceGroupBindSparseInfoKHR = VkDeviceGroupBindSparseInfo;
pub struct VkDeviceGroupPresentCapabilitiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE],
	pub modes: VkDeviceGroupPresentModeFlagsKHR
}
pub struct VkImageSwapchainCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub swapchain: VkSwapchainKHR
}
pub struct VkBindImageMemorySwapchainInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub swapchain: VkSwapchainKHR,
	pub imageIndex: u32
}
pub struct VkAcquireNextImageInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub swapchain: VkSwapchainKHR,
	pub timeout: uint64_t,
	pub semaphore: VkSemaphore,
	pub fence: VkFence,
	pub deviceMask: u32
}
pub struct VkDeviceGroupPresentInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub swapchainCount: u32,
	pub pDeviceMasks: *const u32,
	pub mode: VkDeviceGroupPresentModeFlagBitsKHR
}
pub struct VkDeviceGroupDeviceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub physicalDeviceCount: u32,
	pub pPhysicalDevices: *const VkPhysicalDevice
}
type VkDeviceGroupDeviceCreateInfoKHR = VkDeviceGroupDeviceCreateInfo;
pub struct VkDeviceGroupSwapchainCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub modes: VkDeviceGroupPresentModeFlagsKHR
}
pub struct VkDescriptorUpdateTemplateEntry
{
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32,
	pub descriptorType: VkDescriptorType,
	pub offset: usize,
	pub stride: usize
}
type VkDescriptorUpdateTemplateEntryKHR = VkDescriptorUpdateTemplateEntry;
pub struct VkDescriptorUpdateTemplateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDescriptorUpdateTemplateCreateFlags,
	pub descriptorUpdateEntryCount: u32,
	pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntry,
	pub templateType: VkDescriptorUpdateTemplateType,
	pub descriptorSetLayout: VkDescriptorSetLayout,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub pipelineLayout: VkPipelineLayout,
	pub set: u32
}
type VkDescriptorUpdateTemplateCreateInfoKHR = VkDescriptorUpdateTemplateCreateInfo;
pub struct VkXYColorEXT
{
	pub x: f32,
	pub y: f32
}
pub struct VkPhysicalDevicePresentIdFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub presentId: VkBool32
}
pub struct VkPresentIdKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub swapchainCount: u32,
	pub pPresentIds: *const uint64_t
}
pub struct VkPhysicalDevicePresentWaitFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub presentWait: VkBool32
}
pub struct VkHdrMetadataEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub displayPrimaryRed: VkXYColorEXT,
	pub displayPrimaryGreen: VkXYColorEXT,
	pub displayPrimaryBlue: VkXYColorEXT,
	pub whitePoint: VkXYColorEXT,
	pub maxLuminance: f32,
	pub minLuminance: f32,
	pub maxContentLightLevel: f32,
	pub maxFrameAverageLightLevel: f32
}
pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub localDimmingSupport: VkBool32
}
pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub localDimmingEnable: VkBool32
}
pub struct VkRefreshCycleDurationGOOGLE
{
	pub refreshDuration: uint64_t
}
pub struct VkPastPresentationTimingGOOGLE
{
	pub presentID: u32,
	pub desiredPresentTime: uint64_t,
	pub actualPresentTime: uint64_t,
	pub earliestPresentTime: uint64_t,
	pub presentMargin: uint64_t
}
pub struct VkPresentTimesInfoGOOGLE
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub swapchainCount: u32,
	pub pTimes: *const VkPresentTimeGOOGLE
}
pub struct VkPresentTimeGOOGLE
{
	pub presentID: u32,
	pub desiredPresentTime: uint64_t
}
pub struct VkIOSSurfaceCreateInfoMVK
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkIOSSurfaceCreateFlagsMVK,
	pub pView: *const c_void
}
pub struct VkMacOSSurfaceCreateInfoMVK
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkMacOSSurfaceCreateFlagsMVK,
	pub pView: *const c_void
}
pub struct VkMetalSurfaceCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkMetalSurfaceCreateFlagsEXT,
	pub pLayer: *const CAMetalLayer
}
pub struct VkViewportWScalingNV
{
	pub xcoeff: f32,
	pub ycoeff: f32
}
pub struct VkPipelineViewportWScalingStateCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub viewportWScalingEnable: VkBool32,
	pub viewportCount: u32,
	pub pViewportWScalings: *const VkViewportWScalingNV
}
pub struct VkViewportSwizzleNV
{
	pub x: VkViewportCoordinateSwizzleNV,
	pub y: VkViewportCoordinateSwizzleNV,
	pub z: VkViewportCoordinateSwizzleNV,
	pub w: VkViewportCoordinateSwizzleNV
}
pub struct VkPipelineViewportSwizzleStateCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
	pub viewportCount: u32,
	pub pViewportSwizzles: *const VkViewportSwizzleNV
}
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxDiscardRectangles: u32
}
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
	pub discardRectangleMode: VkDiscardRectangleModeEXT,
	pub discardRectangleCount: u32,
	pub pDiscardRectangles: *const VkRect2D
}
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub perViewPositionAllComponents: VkBool32
}
pub struct VkInputAttachmentAspectReference
{
	pub subpass: u32,
	pub inputAttachmentIndex: u32,
	pub aspectMask: VkImageAspectFlags
}
type VkInputAttachmentAspectReferenceKHR = VkInputAttachmentAspectReference;
pub struct VkRenderPassInputAttachmentAspectCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub aspectReferenceCount: u32,
	pub pAspectReferences: *const VkInputAttachmentAspectReference
}
type VkRenderPassInputAttachmentAspectCreateInfoKHR = VkRenderPassInputAttachmentAspectCreateInfo;
pub struct VkPhysicalDeviceSurfaceInfo2KHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub surface: VkSurfaceKHR
}
pub struct VkSurfaceCapabilities2KHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub surfaceCapabilities: VkSurfaceCapabilitiesKHR
}
pub struct VkSurfaceFormat2KHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub surfaceFormat: VkSurfaceFormatKHR
}
pub struct VkDisplayProperties2KHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub displayProperties: VkDisplayPropertiesKHR
}
pub struct VkDisplayPlaneProperties2KHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub displayPlaneProperties: VkDisplayPlanePropertiesKHR
}
pub struct VkDisplayModeProperties2KHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub displayModeProperties: VkDisplayModePropertiesKHR
}
pub struct VkDisplayPlaneInfo2KHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub mode: VkDisplayModeKHR,
	pub planeIndex: u32
}
pub struct VkDisplayPlaneCapabilities2KHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub capabilities: VkDisplayPlaneCapabilitiesKHR
}
pub struct VkSharedPresentSurfaceCapabilitiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub sharedPresentSupportedUsageFlags: VkImageUsageFlags
}
pub struct VkPhysicalDevice16BitStorageFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub storageBuffer16BitAccess: VkBool32,
	pub uniformAndStorageBuffer16BitAccess: VkBool32,
	pub storagePushConstant16: VkBool32,
	pub storageInputOutput16: VkBool32
}
type VkPhysicalDevice16BitStorageFeaturesKHR = VkPhysicalDevice16BitStorageFeatures;
pub struct VkPhysicalDeviceSubgroupProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub subgroupSize: u32,
	pub supportedStages: VkShaderStageFlags,
	pub supportedOperations: VkSubgroupFeatureFlags,
	pub quadOperationsInAllStages: VkBool32
}
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderSubgroupExtendedTypes: VkBool32
}
type VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR = VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures;
pub struct VkBufferMemoryRequirementsInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub buffer: VkBuffer
}
type VkBufferMemoryRequirementsInfo2KHR = VkBufferMemoryRequirementsInfo2;
pub struct VkDeviceBufferMemoryRequirements
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pCreateInfo: *const VkBufferCreateInfo
}
type VkDeviceBufferMemoryRequirementsKHR = VkDeviceBufferMemoryRequirements;
pub struct VkImageMemoryRequirementsInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub image: VkImage
}
type VkImageMemoryRequirementsInfo2KHR = VkImageMemoryRequirementsInfo2;
pub struct VkImageSparseMemoryRequirementsInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub image: VkImage
}
type VkImageSparseMemoryRequirementsInfo2KHR = VkImageSparseMemoryRequirementsInfo2;
pub struct VkDeviceImageMemoryRequirements
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pCreateInfo: *const VkImageCreateInfo,
	pub planeAspect: VkImageAspectFlagBits
}
type VkDeviceImageMemoryRequirementsKHR = VkDeviceImageMemoryRequirements;
pub struct VkMemoryRequirements2
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryRequirements: VkMemoryRequirements
}
type VkMemoryRequirements2KHR = VkMemoryRequirements2;
pub struct VkSparseImageMemoryRequirements2
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryRequirements: VkSparseImageMemoryRequirements
}
type VkSparseImageMemoryRequirements2KHR = VkSparseImageMemoryRequirements2;
pub struct VkPhysicalDevicePointClippingProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub pointClippingBehavior: VkPointClippingBehavior
}
type VkPhysicalDevicePointClippingPropertiesKHR = VkPhysicalDevicePointClippingProperties;
pub struct VkMemoryDedicatedRequirements
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub prefersDedicatedAllocation: VkBool32,
	pub requiresDedicatedAllocation: VkBool32
}
type VkMemoryDedicatedRequirementsKHR = VkMemoryDedicatedRequirements;
pub struct VkMemoryDedicatedAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub image: VkImage,
	pub buffer: VkBuffer
}
type VkMemoryDedicatedAllocateInfoKHR = VkMemoryDedicatedAllocateInfo;
pub struct VkImageViewUsageCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub usage: VkImageUsageFlags
}
type VkImageViewUsageCreateInfoKHR = VkImageViewUsageCreateInfo;
pub struct VkPipelineTessellationDomainOriginStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub domainOrigin: VkTessellationDomainOrigin
}
type VkPipelineTessellationDomainOriginStateCreateInfoKHR = VkPipelineTessellationDomainOriginStateCreateInfo;
pub struct VkSamplerYcbcrConversionInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub conversion: VkSamplerYcbcrConversion
}
type VkSamplerYcbcrConversionInfoKHR = VkSamplerYcbcrConversionInfo;
pub struct VkSamplerYcbcrConversionCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub format: VkFormat,
	pub ycbcrModel: VkSamplerYcbcrModelConversion,
	pub ycbcrRange: VkSamplerYcbcrRange,
	pub components: VkComponentMapping,
	pub xChromaOffset: VkChromaLocation,
	pub yChromaOffset: VkChromaLocation,
	pub chromaFilter: VkFilter,
	pub forceExplicitReconstruction: VkBool32
}
type VkSamplerYcbcrConversionCreateInfoKHR = VkSamplerYcbcrConversionCreateInfo;
pub struct VkBindImagePlaneMemoryInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub planeAspect: VkImageAspectFlagBits
}
type VkBindImagePlaneMemoryInfoKHR = VkBindImagePlaneMemoryInfo;
pub struct VkImagePlaneMemoryRequirementsInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub planeAspect: VkImageAspectFlagBits
}
type VkImagePlaneMemoryRequirementsInfoKHR = VkImagePlaneMemoryRequirementsInfo;
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub samplerYcbcrConversion: VkBool32
}
type VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR = VkPhysicalDeviceSamplerYcbcrConversionFeatures;
pub struct VkSamplerYcbcrConversionImageFormatProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub combinedImageSamplerDescriptorCount: u32
}
type VkSamplerYcbcrConversionImageFormatPropertiesKHR = VkSamplerYcbcrConversionImageFormatProperties;
pub struct VkTextureLODGatherFormatPropertiesAMD
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub supportsTextureGatherLODBiasAMD: VkBool32
}
pub struct VkConditionalRenderingBeginInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub flags: VkConditionalRenderingFlagsEXT
}
pub struct VkProtectedSubmitInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub protectedSubmit: VkBool32
}
pub struct VkPhysicalDeviceProtectedMemoryFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub protectedMemory: VkBool32
}
pub struct VkPhysicalDeviceProtectedMemoryProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub protectedNoFault: VkBool32
}
pub struct VkDeviceQueueInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDeviceQueueCreateFlags,
	pub queueFamilyIndex: u32,
	pub queueIndex: u32
}
pub struct VkPipelineCoverageToColorStateCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
	pub coverageToColorEnable: VkBool32,
	pub coverageToColorLocation: u32
}
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub filterMinmaxSingleComponentFormats: VkBool32,
	pub filterMinmaxImageComponentMapping: VkBool32
}
type VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT = VkPhysicalDeviceSamplerFilterMinmaxProperties;
pub struct VkSampleLocationEXT
{
	pub x: f32,
	pub y: f32
}
pub struct VkSampleLocationsInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub sampleLocationsPerPixel: VkSampleCountFlagBits,
	pub sampleLocationGridSize: VkExtent2D,
	pub sampleLocationsCount: u32,
	pub pSampleLocations: *const VkSampleLocationEXT
}
pub struct VkAttachmentSampleLocationsEXT
{
	pub attachmentIndex: u32,
	pub sampleLocationsInfo: VkSampleLocationsInfoEXT
}
pub struct VkSubpassSampleLocationsEXT
{
	pub subpassIndex: u32,
	pub sampleLocationsInfo: VkSampleLocationsInfoEXT
}
pub struct VkRenderPassSampleLocationsBeginInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub attachmentInitialSampleLocationsCount: u32,
	pub pAttachmentInitialSampleLocations: *const VkAttachmentSampleLocationsEXT,
	pub postSubpassSampleLocationsCount: u32,
	pub pPostSubpassSampleLocations: *const VkSubpassSampleLocationsEXT
}
pub struct VkPipelineSampleLocationsStateCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub sampleLocationsEnable: VkBool32,
	pub sampleLocationsInfo: VkSampleLocationsInfoEXT
}
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub sampleLocationSampleCounts: VkSampleCountFlags,
	pub maxSampleLocationGridSize: VkExtent2D,
	pub sampleLocationCoordinateRange: [f32; 2],
	pub sampleLocationSubPixelBits: u32,
	pub variableSampleLocations: VkBool32
}
pub struct VkMultisamplePropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxSampleLocationGridSize: VkExtent2D
}
pub struct VkSamplerReductionModeCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub reductionMode: VkSamplerReductionMode
}
type VkSamplerReductionModeCreateInfoEXT = VkSamplerReductionModeCreateInfo;
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub advancedBlendCoherentOperations: VkBool32
}
pub struct VkPhysicalDeviceMultiDrawFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub multiDraw: VkBool32
}
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub advancedBlendMaxColorAttachments: u32,
	pub advancedBlendIndependentBlend: VkBool32,
	pub advancedBlendNonPremultipliedSrcColor: VkBool32,
	pub advancedBlendNonPremultipliedDstColor: VkBool32,
	pub advancedBlendCorrelatedOverlap: VkBool32,
	pub advancedBlendAllOperations: VkBool32
}
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcPremultiplied: VkBool32,
	pub dstPremultiplied: VkBool32,
	pub blendOverlap: VkBlendOverlapEXT
}
pub struct VkPhysicalDeviceInlineUniformBlockFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub inlineUniformBlock: VkBool32,
	pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32
}
type VkPhysicalDeviceInlineUniformBlockFeaturesEXT = VkPhysicalDeviceInlineUniformBlockFeatures;
pub struct VkPhysicalDeviceInlineUniformBlockProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxInlineUniformBlockSize: u32,
	pub maxPerStageDescriptorInlineUniformBlocks: u32,
	pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
	pub maxDescriptorSetInlineUniformBlocks: u32,
	pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32
}
type VkPhysicalDeviceInlineUniformBlockPropertiesEXT = VkPhysicalDeviceInlineUniformBlockProperties;
pub struct VkWriteDescriptorSetInlineUniformBlock
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub dataSize: u32,
	pub pData: *const c_void
}
type VkWriteDescriptorSetInlineUniformBlockEXT = VkWriteDescriptorSetInlineUniformBlock;
pub struct VkDescriptorPoolInlineUniformBlockCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub maxInlineUniformBlockBindings: u32
}
type VkDescriptorPoolInlineUniformBlockCreateInfoEXT = VkDescriptorPoolInlineUniformBlockCreateInfo;
pub struct VkPipelineCoverageModulationStateCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
	pub coverageModulationMode: VkCoverageModulationModeNV,
	pub coverageModulationTableEnable: VkBool32,
	pub coverageModulationTableCount: u32,
	pub pCoverageModulationTable: *const f32
}
pub struct VkImageFormatListCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub viewFormatCount: u32,
	pub pViewFormats: *const VkFormat
}
type VkImageFormatListCreateInfoKHR = VkImageFormatListCreateInfo;
pub struct VkValidationCacheCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkValidationCacheCreateFlagsEXT,
	pub initialDataSize: usize,
	pub pInitialData: *const c_void
}
pub struct VkShaderModuleValidationCacheCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub validationCache: VkValidationCacheEXT
}
pub struct VkPhysicalDeviceMaintenance3Properties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxPerSetDescriptors: u32,
	pub maxMemoryAllocationSize: VkDeviceSize
}
type VkPhysicalDeviceMaintenance3PropertiesKHR = VkPhysicalDeviceMaintenance3Properties;
pub struct VkPhysicalDeviceMaintenance4Features
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maintenance4: VkBool32
}
type VkPhysicalDeviceMaintenance4FeaturesKHR = VkPhysicalDeviceMaintenance4Features;
pub struct VkPhysicalDeviceMaintenance4Properties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxBufferSize: VkDeviceSize
}
type VkPhysicalDeviceMaintenance4PropertiesKHR = VkPhysicalDeviceMaintenance4Properties;
pub struct VkDescriptorSetLayoutSupport
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub supported: VkBool32
}
type VkDescriptorSetLayoutSupportKHR = VkDescriptorSetLayoutSupport;
pub struct VkPhysicalDeviceShaderDrawParametersFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderDrawParameters: VkBool32
}
type VkPhysicalDeviceShaderDrawParameterFeatures = VkPhysicalDeviceShaderDrawParametersFeatures;
pub struct VkPhysicalDeviceShaderFloat16Int8Features
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderFloat16: VkBool32,
	pub shaderInt8: VkBool32
}
type VkPhysicalDeviceShaderFloat16Int8FeaturesKHR = VkPhysicalDeviceShaderFloat16Int8Features;
type VkPhysicalDeviceFloat16Int8FeaturesKHR = VkPhysicalDeviceShaderFloat16Int8Features;
pub struct VkPhysicalDeviceFloatControlsProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
	pub roundingModeIndependence: VkShaderFloatControlsIndependence,
	pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
	pub shaderDenormPreserveFloat16: VkBool32,
	pub shaderDenormPreserveFloat32: VkBool32,
	pub shaderDenormPreserveFloat64: VkBool32,
	pub shaderDenormFlushToZeroFloat16: VkBool32,
	pub shaderDenormFlushToZeroFloat32: VkBool32,
	pub shaderDenormFlushToZeroFloat64: VkBool32,
	pub shaderRoundingModeRTEFloat16: VkBool32,
	pub shaderRoundingModeRTEFloat32: VkBool32,
	pub shaderRoundingModeRTEFloat64: VkBool32,
	pub shaderRoundingModeRTZFloat16: VkBool32,
	pub shaderRoundingModeRTZFloat32: VkBool32,
	pub shaderRoundingModeRTZFloat64: VkBool32
}
type VkPhysicalDeviceFloatControlsPropertiesKHR = VkPhysicalDeviceFloatControlsProperties;
pub struct VkPhysicalDeviceHostQueryResetFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub hostQueryReset: VkBool32
}
type VkPhysicalDeviceHostQueryResetFeaturesEXT = VkPhysicalDeviceHostQueryResetFeatures;
pub struct VkNativeBufferUsage2ANDROID
{
	pub consumer: uint64_t,
	pub producer: uint64_t
}
pub struct VkNativeBufferANDROID
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handle: *const c_void,
	pub stride: int,
	pub format: int,
	pub usage: int,
	pub usage2: VkNativeBufferUsage2ANDROID
}
pub struct VkSwapchainImageCreateInfoANDROID
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub usage: VkSwapchainImageUsageFlagsANDROID
}
pub struct VkPhysicalDevicePresentationPropertiesANDROID
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub sharedImage: VkBool32
}
pub struct VkShaderResourceUsageAMD
{
	pub numUsedVgprs: u32,
	pub numUsedSgprs: u32,
	pub ldsSizePerLocalWorkGroup: u32,
	pub ldsUsageSizeInBytes: usize,
	pub scratchMemUsageInBytes: usize
}
pub struct VkShaderStatisticsInfoAMD
{
	pub shaderStageMask: VkShaderStageFlags,
	pub resourceUsage: VkShaderResourceUsageAMD,
	pub numPhysicalVgprs: u32,
	pub numPhysicalSgprs: u32,
	pub numAvailableVgprs: u32,
	pub numAvailableSgprs: u32,
	pub computeWorkGroupSize: [u32; 3]
}
pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub globalPriority: VkQueueGlobalPriorityKHR
}
type VkDeviceQueueGlobalPriorityCreateInfoEXT = VkDeviceQueueGlobalPriorityCreateInfoKHR;
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub globalPriorityQuery: VkBool32
}
type VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT = VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR;
pub struct VkQueueFamilyGlobalPriorityPropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub priorityCount: u32,
	pub priorities: [VkQueueGlobalPriorityKHR; VK_MAX_GLOBAL_PRIORITY_SIZE_KHR]
}
type VkQueueFamilyGlobalPriorityPropertiesEXT = VkQueueFamilyGlobalPriorityPropertiesKHR;
pub struct VkDebugUtilsObjectNameInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub objectType: VkObjectType,
	pub objectHandle: uint64_t,
	pub pObjectName: *const i8
}
pub struct VkDebugUtilsObjectTagInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub objectType: VkObjectType,
	pub objectHandle: uint64_t,
	pub tagName: uint64_t,
	pub tagSize: usize,
	pub pTag: *const c_void
}
pub struct VkDebugUtilsLabelEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pLabelName: *const i8,
	pub color: [f32; 4]
}
pub struct VkDebugUtilsMessengerCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
	pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
	pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
	pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
	pub pUserData: *mut c_void
}
pub struct VkDebugUtilsMessengerCallbackDataEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
	pub pMessageIdName: *const i8,
	pub messageIdNumber: i32,
	pub pMessage: *const i8,
	pub queueLabelCount: u32,
	pub pQueueLabels: *const VkDebugUtilsLabelEXT,
	pub cmdBufLabelCount: u32,
	pub pCmdBufLabels: *const VkDebugUtilsLabelEXT,
	pub objectCount: u32,
	pub pObjects: *const VkDebugUtilsObjectNameInfoEXT
}
pub struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub deviceMemoryReport: VkBool32
}
pub struct VkDeviceDeviceMemoryReportCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDeviceMemoryReportFlagsEXT,
	pub pfnUserCallback: PFN_vkDeviceMemoryReportCallbackEXT,
	pub pUserData: *mut c_void
}
pub struct VkDeviceMemoryReportCallbackDataEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub flags: VkDeviceMemoryReportFlagsEXT,
	pub kind: VkDeviceMemoryReportEventTypeEXT,
	pub memoryObjectId: uint64_t,
	pub size: VkDeviceSize,
	pub objectType: VkObjectType,
	pub objectHandle: uint64_t,
	pub heapIndex: u32
}
pub struct VkImportMemoryHostPointerInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
	pub pHostPointer: *mut c_void
}
pub struct VkMemoryHostPointerPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryTypeBits: u32
}
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub minImportedHostPointerAlignment: VkDeviceSize
}
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub primitiveOverestimationSize: f32,
	pub maxExtraPrimitiveOverestimationSize: f32,
	pub extraPrimitiveOverestimationSizeGranularity: f32,
	pub primitiveUnderestimation: VkBool32,
	pub conservativePointAndLineRasterization: VkBool32,
	pub degenerateTrianglesRasterized: VkBool32,
	pub degenerateLinesRasterized: VkBool32,
	pub fullyCoveredFragmentShaderInputVariable: VkBool32,
	pub conservativeRasterizationPostDepthCoverage: VkBool32
}
pub struct VkCalibratedTimestampInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub timeDomain: VkTimeDomainEXT
}
pub struct VkPhysicalDeviceShaderCorePropertiesAMD
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderEngineCount: u32,
	pub shaderArraysPerEngineCount: u32,
	pub computeUnitsPerShaderArray: u32,
	pub simdPerComputeUnit: u32,
	pub wavefrontsPerSimd: u32,
	pub wavefrontSize: u32,
	pub sgprsPerSimd: u32,
	pub minSgprAllocation: u32,
	pub maxSgprAllocation: u32,
	pub sgprAllocationGranularity: u32,
	pub vgprsPerSimd: u32,
	pub minVgprAllocation: u32,
	pub maxVgprAllocation: u32,
	pub vgprAllocationGranularity: u32
}
pub struct VkPhysicalDeviceShaderCoreProperties2AMD
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderCoreFeatures: VkShaderCorePropertiesFlagsAMD,
	pub activeComputeUnitCount: u32
}
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
	pub conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
	pub extraPrimitiveOverestimationSize: f32
}
pub struct VkPhysicalDeviceDescriptorIndexingFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
	pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
	pub descriptorBindingPartiallyBound: VkBool32,
	pub descriptorBindingVariableDescriptorCount: VkBool32,
	pub runtimeDescriptorArray: VkBool32
}
type VkPhysicalDeviceDescriptorIndexingFeaturesEXT = VkPhysicalDeviceDescriptorIndexingFeatures;
pub struct VkPhysicalDeviceDescriptorIndexingProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxUpdateAfterBindDescriptorsInAllPools: u32,
	pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
	pub robustBufferAccessUpdateAfterBind: VkBool32,
	pub quadDivergentImplicitLod: VkBool32,
	pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
	pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
	pub maxPerStageUpdateAfterBindResources: u32,
	pub maxDescriptorSetUpdateAfterBindSamplers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
	pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
	pub maxDescriptorSetUpdateAfterBindInputAttachments: u32
}
type VkPhysicalDeviceDescriptorIndexingPropertiesEXT = VkPhysicalDeviceDescriptorIndexingProperties;
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub bindingCount: u32,
	pub pBindingFlags: *const VkDescriptorBindingFlags
}
type VkDescriptorSetLayoutBindingFlagsCreateInfoEXT = VkDescriptorSetLayoutBindingFlagsCreateInfo;
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub descriptorSetCount: u32,
	pub pDescriptorCounts: *const u32
}
type VkDescriptorSetVariableDescriptorCountAllocateInfoEXT = VkDescriptorSetVariableDescriptorCountAllocateInfo;
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxVariableDescriptorCount: u32
}
type VkDescriptorSetVariableDescriptorCountLayoutSupportEXT = VkDescriptorSetVariableDescriptorCountLayoutSupport;
pub struct VkAttachmentDescription2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkAttachmentDescriptionFlags,
	pub format: VkFormat,
	pub samples: VkSampleCountFlagBits,
	pub loadOp: VkAttachmentLoadOp,
	pub storeOp: VkAttachmentStoreOp,
	pub stencilLoadOp: VkAttachmentLoadOp,
	pub stencilStoreOp: VkAttachmentStoreOp,
	pub initialLayout: VkImageLayout,
	pub finalLayout: VkImageLayout
}
type VkAttachmentDescription2KHR = VkAttachmentDescription2;
pub struct VkAttachmentReference2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub attachment: u32,
	pub layout: VkImageLayout,
	pub aspectMask: VkImageAspectFlags
}
type VkAttachmentReference2KHR = VkAttachmentReference2;
pub struct VkSubpassDescription2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkSubpassDescriptionFlags,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub viewMask: u32,
	pub inputAttachmentCount: u32,
	pub pInputAttachments: *const VkAttachmentReference2,
	pub colorAttachmentCount: u32,
	pub pColorAttachments: *const VkAttachmentReference2,
	pub pResolveAttachments: *const VkAttachmentReference2,
	pub pDepthStencilAttachment: *const VkAttachmentReference2,
	pub preserveAttachmentCount: u32,
	pub pPreserveAttachments: *const u32
}
type VkSubpassDescription2KHR = VkSubpassDescription2;
pub struct VkSubpassDependency2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcSubpass: u32,
	pub dstSubpass: u32,
	pub srcStageMask: VkPipelineStageFlags,
	pub dstStageMask: VkPipelineStageFlags,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub dependencyFlags: VkDependencyFlags,
	pub viewOffset: i32
}
type VkSubpassDependency2KHR = VkSubpassDependency2;
pub struct VkRenderPassCreateInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkRenderPassCreateFlags,
	pub attachmentCount: u32,
	pub pAttachments: *const VkAttachmentDescription2,
	pub subpassCount: u32,
	pub pSubpasses: *const VkSubpassDescription2,
	pub dependencyCount: u32,
	pub pDependencies: *const VkSubpassDependency2,
	pub correlatedViewMaskCount: u32,
	pub pCorrelatedViewMasks: *const u32
}
type VkRenderPassCreateInfo2KHR = VkRenderPassCreateInfo2;
pub struct VkSubpassBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub contents: VkSubpassContents
}
type VkSubpassBeginInfoKHR = VkSubpassBeginInfo;
pub struct VkSubpassEndInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void
}
type VkSubpassEndInfoKHR = VkSubpassEndInfo;
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub timelineSemaphore: VkBool32
}
type VkPhysicalDeviceTimelineSemaphoreFeaturesKHR = VkPhysicalDeviceTimelineSemaphoreFeatures;
pub struct VkPhysicalDeviceTimelineSemaphoreProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxTimelineSemaphoreValueDifference: uint64_t
}
type VkPhysicalDeviceTimelineSemaphorePropertiesKHR = VkPhysicalDeviceTimelineSemaphoreProperties;
pub struct VkSemaphoreTypeCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub semaphoreType: VkSemaphoreType,
	pub initialValue: uint64_t
}
type VkSemaphoreTypeCreateInfoKHR = VkSemaphoreTypeCreateInfo;
pub struct VkTimelineSemaphoreSubmitInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub waitSemaphoreValueCount: u32,
	pub pWaitSemaphoreValues: *const uint64_t,
	pub signalSemaphoreValueCount: u32,
	pub pSignalSemaphoreValues: *const uint64_t
}
type VkTimelineSemaphoreSubmitInfoKHR = VkTimelineSemaphoreSubmitInfo;
pub struct VkSemaphoreWaitInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkSemaphoreWaitFlags,
	pub semaphoreCount: u32,
	pub pSemaphores: *const VkSemaphore,
	pub pValues: *const uint64_t
}
type VkSemaphoreWaitInfoKHR = VkSemaphoreWaitInfo;
pub struct VkSemaphoreSignalInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub semaphore: VkSemaphore,
	pub value: uint64_t
}
type VkSemaphoreSignalInfoKHR = VkSemaphoreSignalInfo;
pub struct VkVertexInputBindingDivisorDescriptionEXT
{
	pub binding: u32,
	pub divisor: u32
}
pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub vertexBindingDivisorCount: u32,
	pub pVertexBindingDivisors: *const VkVertexInputBindingDivisorDescriptionEXT
}
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxVertexAttribDivisor: u32
}
pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub pciDomain: u32,
	pub pciBus: u32,
	pub pciDevice: u32,
	pub pciFunction: u32
}
pub struct VkImportAndroidHardwareBufferInfoANDROID
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub buffer: *mut AHardwareBuffer
}
pub struct VkAndroidHardwareBufferUsageANDROID
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub androidHardwareBufferUsage: uint64_t
}
pub struct VkAndroidHardwareBufferPropertiesANDROID
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub allocationSize: VkDeviceSize,
	pub memoryTypeBits: u32
}
pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub memory: VkDeviceMemory
}
pub struct VkAndroidHardwareBufferFormatPropertiesANDROID
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub format: VkFormat,
	pub externalFormat: uint64_t,
	pub formatFeatures: VkFormatFeatureFlags,
	pub samplerYcbcrConversionComponents: VkComponentMapping,
	pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
	pub suggestedYcbcrRange: VkSamplerYcbcrRange,
	pub suggestedXChromaOffset: VkChromaLocation,
	pub suggestedYChromaOffset: VkChromaLocation
}
pub struct VkCommandBufferInheritanceConditionalRenderingInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub conditionalRenderingEnable: VkBool32
}
pub struct VkExternalFormatANDROID
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub externalFormat: uint64_t
}
pub struct VkPhysicalDevice8BitStorageFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub storageBuffer8BitAccess: VkBool32,
	pub uniformAndStorageBuffer8BitAccess: VkBool32,
	pub storagePushConstant8: VkBool32
}
type VkPhysicalDevice8BitStorageFeaturesKHR = VkPhysicalDevice8BitStorageFeatures;
pub struct VkPhysicalDeviceConditionalRenderingFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub conditionalRendering: VkBool32,
	pub inheritedConditionalRendering: VkBool32
}
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub vulkanMemoryModel: VkBool32,
	pub vulkanMemoryModelDeviceScope: VkBool32,
	pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32
}
type VkPhysicalDeviceVulkanMemoryModelFeaturesKHR = VkPhysicalDeviceVulkanMemoryModelFeatures;
pub struct VkPhysicalDeviceShaderAtomicInt64Features
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderBufferInt64Atomics: VkBool32,
	pub shaderSharedInt64Atomics: VkBool32
}
type VkPhysicalDeviceShaderAtomicInt64FeaturesKHR = VkPhysicalDeviceShaderAtomicInt64Features;
pub struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderBufferFloat32Atomics: VkBool32,
	pub shaderBufferFloat32AtomicAdd: VkBool32,
	pub shaderBufferFloat64Atomics: VkBool32,
	pub shaderBufferFloat64AtomicAdd: VkBool32,
	pub shaderSharedFloat32Atomics: VkBool32,
	pub shaderSharedFloat32AtomicAdd: VkBool32,
	pub shaderSharedFloat64Atomics: VkBool32,
	pub shaderSharedFloat64AtomicAdd: VkBool32,
	pub shaderImageFloat32Atomics: VkBool32,
	pub shaderImageFloat32AtomicAdd: VkBool32,
	pub sparseImageFloat32Atomics: VkBool32,
	pub sparseImageFloat32AtomicAdd: VkBool32
}
pub struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderBufferFloat16Atomics: VkBool32,
	pub shaderBufferFloat16AtomicAdd: VkBool32,
	pub shaderBufferFloat16AtomicMinMax: VkBool32,
	pub shaderBufferFloat32AtomicMinMax: VkBool32,
	pub shaderBufferFloat64AtomicMinMax: VkBool32,
	pub shaderSharedFloat16Atomics: VkBool32,
	pub shaderSharedFloat16AtomicAdd: VkBool32,
	pub shaderSharedFloat16AtomicMinMax: VkBool32,
	pub shaderSharedFloat32AtomicMinMax: VkBool32,
	pub shaderSharedFloat64AtomicMinMax: VkBool32,
	pub shaderImageFloat32AtomicMinMax: VkBool32,
	pub sparseImageFloat32AtomicMinMax: VkBool32
}
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub vertexAttributeInstanceRateDivisor: VkBool32,
	pub vertexAttributeInstanceRateZeroDivisor: VkBool32
}
pub struct VkQueueFamilyCheckpointPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub checkpointExecutionStageMask: VkPipelineStageFlags
}
pub struct VkCheckpointDataNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub stage: VkPipelineStageFlagBits,
	pub pCheckpointMarker: *mut c_void
}
pub struct VkPhysicalDeviceDepthStencilResolveProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub supportedDepthResolveModes: VkResolveModeFlags,
	pub supportedStencilResolveModes: VkResolveModeFlags,
	pub independentResolveNone: VkBool32,
	pub independentResolve: VkBool32
}
type VkPhysicalDeviceDepthStencilResolvePropertiesKHR = VkPhysicalDeviceDepthStencilResolveProperties;
pub struct VkSubpassDescriptionDepthStencilResolve
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub depthResolveMode: VkResolveModeFlagBits,
	pub stencilResolveMode: VkResolveModeFlagBits,
	pub pDepthStencilResolveAttachment: *const VkAttachmentReference2
}
type VkSubpassDescriptionDepthStencilResolveKHR = VkSubpassDescriptionDepthStencilResolve;
pub struct VkImageViewASTCDecodeModeEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub decodeMode: VkFormat
}
pub struct VkPhysicalDeviceASTCDecodeFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub decodeModeSharedExponent: VkBool32
}
pub struct VkPhysicalDeviceTransformFeedbackFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub transformFeedback: VkBool32,
	pub geometryStreams: VkBool32
}
pub struct VkPhysicalDeviceTransformFeedbackPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxTransformFeedbackStreams: u32,
	pub maxTransformFeedbackBuffers: u32,
	pub maxTransformFeedbackBufferSize: VkDeviceSize,
	pub maxTransformFeedbackStreamDataSize: u32,
	pub maxTransformFeedbackBufferDataSize: u32,
	pub maxTransformFeedbackBufferDataStride: u32,
	pub transformFeedbackQueries: VkBool32,
	pub transformFeedbackStreamsLinesTriangles: VkBool32,
	pub transformFeedbackRasterizationStreamSelect: VkBool32,
	pub transformFeedbackDraw: VkBool32
}
pub struct VkPipelineRasterizationStateStreamCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineRasterizationStateStreamCreateFlagsEXT,
	pub rasterizationStream: u32
}
pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub representativeFragmentTest: VkBool32
}
pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub representativeFragmentTestEnable: VkBool32
}
pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub exclusiveScissor: VkBool32
}
pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub exclusiveScissorCount: u32,
	pub pExclusiveScissors: *const VkRect2D
}
pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub cornerSampledImage: VkBool32
}
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub computeDerivativeGroupQuads: VkBool32,
	pub computeDerivativeGroupLinear: VkBool32
}
type VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV = VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR;
pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub imageFootprint: VkBool32
}
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub dedicatedAllocationImageAliasing: VkBool32
}
pub struct VkPhysicalDeviceCopyMemoryIndirectFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub indirectCopy: VkBool32
}
pub struct VkPhysicalDeviceCopyMemoryIndirectPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub supportedQueues: VkQueueFlags
}
pub struct VkPhysicalDeviceMemoryDecompressionFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryDecompression: VkBool32
}
pub struct VkPhysicalDeviceMemoryDecompressionPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub decompressionMethods: VkMemoryDecompressionMethodFlagsNV,
	pub maxDecompressionIndirectCount: uint64_t
}
pub struct VkShadingRatePaletteNV
{
	pub shadingRatePaletteEntryCount: u32,
	pub pShadingRatePaletteEntries: *const VkShadingRatePaletteEntryNV
}
pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub shadingRateImageEnable: VkBool32,
	pub viewportCount: u32,
	pub pShadingRatePalettes: *const VkShadingRatePaletteNV
}
pub struct VkPhysicalDeviceShadingRateImageFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shadingRateImage: VkBool32,
	pub shadingRateCoarseSampleOrder: VkBool32
}
pub struct VkPhysicalDeviceShadingRateImagePropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shadingRateTexelSize: VkExtent2D,
	pub shadingRatePaletteSize: u32,
	pub shadingRateMaxCoarseSamples: u32
}
pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub invocationMask: VkBool32
}
pub struct VkCoarseSampleLocationNV
{
	pub pixelX: u32,
	pub pixelY: u32,
	pub sample: u32
}
pub struct VkCoarseSampleOrderCustomNV
{
	pub shadingRate: VkShadingRatePaletteEntryNV,
	pub sampleCount: u32,
	pub sampleLocationCount: u32,
	pub pSampleLocations: *const VkCoarseSampleLocationNV
}
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub sampleOrderType: VkCoarseSampleOrderTypeNV,
	pub customSampleOrderCount: u32,
	pub pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV
}
pub struct VkPhysicalDeviceMeshShaderFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub taskShader: VkBool32,
	pub meshShader: VkBool32
}
pub struct VkPhysicalDeviceMeshShaderPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxDrawMeshTasksCount: u32,
	pub maxTaskWorkGroupInvocations: u32,
	pub maxTaskWorkGroupSize: [u32; 3],
	pub maxTaskTotalMemorySize: u32,
	pub maxTaskOutputCount: u32,
	pub maxMeshWorkGroupInvocations: u32,
	pub maxMeshWorkGroupSize: [u32; 3],
	pub maxMeshTotalMemorySize: u32,
	pub maxMeshOutputVertices: u32,
	pub maxMeshOutputPrimitives: u32,
	pub maxMeshMultiviewViewCount: u32,
	pub meshOutputPerVertexGranularity: u32,
	pub meshOutputPerPrimitiveGranularity: u32
}
pub struct VkDrawMeshTasksIndirectCommandNV
{
	pub taskCount: u32,
	pub firstTask: u32
}
pub struct VkPhysicalDeviceMeshShaderFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub taskShader: VkBool32,
	pub meshShader: VkBool32,
	pub multiviewMeshShader: VkBool32,
	pub primitiveFragmentShadingRateMeshShader: VkBool32,
	pub meshShaderQueries: VkBool32
}
pub struct VkPhysicalDeviceMeshShaderPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxTaskWorkGroupTotalCount: u32,
	pub maxTaskWorkGroupCount: [u32; 3],
	pub maxTaskWorkGroupInvocations: u32,
	pub maxTaskWorkGroupSize: [u32; 3],
	pub maxTaskPayloadSize: u32,
	pub maxTaskSharedMemorySize: u32,
	pub maxTaskPayloadAndSharedMemorySize: u32,
	pub maxMeshWorkGroupTotalCount: u32,
	pub maxMeshWorkGroupCount: [u32; 3],
	pub maxMeshWorkGroupInvocations: u32,
	pub maxMeshWorkGroupSize: [u32; 3],
	pub maxMeshSharedMemorySize: u32,
	pub maxMeshPayloadAndSharedMemorySize: u32,
	pub maxMeshOutputMemorySize: u32,
	pub maxMeshPayloadAndOutputMemorySize: u32,
	pub maxMeshOutputComponents: u32,
	pub maxMeshOutputVertices: u32,
	pub maxMeshOutputPrimitives: u32,
	pub maxMeshOutputLayers: u32,
	pub maxMeshMultiviewViewCount: u32,
	pub meshOutputPerVertexGranularity: u32,
	pub meshOutputPerPrimitiveGranularity: u32,
	pub maxPreferredTaskWorkGroupInvocations: u32,
	pub maxPreferredMeshWorkGroupInvocations: u32,
	pub prefersLocalInvocationVertexOutput: VkBool32,
	pub prefersLocalInvocationPrimitiveOutput: VkBool32,
	pub prefersCompactVertexOutput: VkBool32,
	pub prefersCompactPrimitiveOutput: VkBool32
}
pub struct VkDrawMeshTasksIndirectCommandEXT
{
	pub groupCountX: u32,
	pub groupCountY: u32,
	pub groupCountZ: u32
}
pub struct VkRayTracingShaderGroupCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub kind: VkRayTracingShaderGroupTypeKHR,
	pub generalShader: u32,
	pub closestHitShader: u32,
	pub anyHitShader: u32,
	pub intersectionShader: u32
}
pub struct VkRayTracingShaderGroupCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub kind: VkRayTracingShaderGroupTypeKHR,
	pub generalShader: u32,
	pub closestHitShader: u32,
	pub anyHitShader: u32,
	pub intersectionShader: u32,
	pub pShaderGroupCaptureReplayHandle: *const c_void
}
pub struct VkRayTracingPipelineCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineCreateFlags,
	pub stageCount: u32,
	pub pStages: *const VkPipelineShaderStageCreateInfo,
	pub groupCount: u32,
	pub pGroups: *const VkRayTracingShaderGroupCreateInfoNV,
	pub maxRecursionDepth: u32,
	pub layout: VkPipelineLayout,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32
}
pub struct VkRayTracingPipelineCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineCreateFlags,
	pub stageCount: u32,
	pub pStages: *const VkPipelineShaderStageCreateInfo,
	pub groupCount: u32,
	pub pGroups: *const VkRayTracingShaderGroupCreateInfoKHR,
	pub maxPipelineRayRecursionDepth: u32,
	pub pLibraryInfo: *const VkPipelineLibraryCreateInfoKHR,
	pub pLibraryInterface: *const VkRayTracingPipelineInterfaceCreateInfoKHR,
	pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
	pub layout: VkPipelineLayout,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32
}
pub struct VkGeometryTrianglesNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub vertexData: VkBuffer,
	pub vertexOffset: VkDeviceSize,
	pub vertexCount: u32,
	pub vertexStride: VkDeviceSize,
	pub vertexFormat: VkFormat,
	pub indexData: VkBuffer,
	pub indexOffset: VkDeviceSize,
	pub indexCount: u32,
	pub indexType: VkIndexType,
	pub transformData: VkBuffer,
	pub transformOffset: VkDeviceSize
}
pub struct VkGeometryAABBNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub aabbData: VkBuffer,
	pub numAABBs: u32,
	pub stride: u32,
	pub offset: VkDeviceSize
}
pub struct VkGeometryDataNV
{
	pub triangles: VkGeometryTrianglesNV,
	pub aabbs: VkGeometryAABBNV
}
pub struct VkGeometryNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub geometryType: VkGeometryTypeKHR,
	pub geometry: VkGeometryDataNV,
	pub flags: VkGeometryFlagsKHR
}
pub struct VkAccelerationStructureInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub kind: VkAccelerationStructureTypeNV,
	pub flags: VkBuildAccelerationStructureFlagsNV,
	pub instanceCount: u32,
	pub geometryCount: u32,
	pub pGeometries: *const VkGeometryNV
}
pub struct VkAccelerationStructureCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub compactedSize: VkDeviceSize,
	pub info: VkAccelerationStructureInfoNV
}
pub struct VkBindAccelerationStructureMemoryInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub accelerationStructure: VkAccelerationStructureNV,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub deviceIndexCount: u32,
	pub pDeviceIndices: *const u32
}
pub struct VkWriteDescriptorSetAccelerationStructureKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub accelerationStructureCount: u32,
	pub pAccelerationStructures: *const VkAccelerationStructureKHR
}
pub struct VkWriteDescriptorSetAccelerationStructureNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub accelerationStructureCount: u32,
	pub pAccelerationStructures: *const VkAccelerationStructureNV
}
pub struct VkAccelerationStructureMemoryRequirementsInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub kind: VkAccelerationStructureMemoryRequirementsTypeNV,
	pub accelerationStructure: VkAccelerationStructureNV
}
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub accelerationStructure: VkBool32,
	pub accelerationStructureCaptureReplay: VkBool32,
	pub accelerationStructureIndirectBuild: VkBool32,
	pub accelerationStructureHostCommands: VkBool32,
	pub descriptorBindingAccelerationStructureUpdateAfterBind: VkBool32
}
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub rayTracingPipeline: VkBool32,
	pub rayTracingPipelineShaderGroupHandleCaptureReplay: VkBool32,
	pub rayTracingPipelineShaderGroupHandleCaptureReplayMixed: VkBool32,
	pub rayTracingPipelineTraceRaysIndirect: VkBool32,
	pub rayTraversalPrimitiveCulling: VkBool32
}
pub struct VkPhysicalDeviceRayQueryFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub rayQuery: VkBool32
}
pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxGeometryCount: uint64_t,
	pub maxInstanceCount: uint64_t,
	pub maxPrimitiveCount: uint64_t,
	pub maxPerStageDescriptorAccelerationStructures: u32,
	pub maxPerStageDescriptorUpdateAfterBindAccelerationStructures: u32,
	pub maxDescriptorSetAccelerationStructures: u32,
	pub maxDescriptorSetUpdateAfterBindAccelerationStructures: u32,
	pub minAccelerationStructureScratchOffsetAlignment: u32
}
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderGroupHandleSize: u32,
	pub maxRayRecursionDepth: u32,
	pub maxShaderGroupStride: u32,
	pub shaderGroupBaseAlignment: u32,
	pub shaderGroupHandleCaptureReplaySize: u32,
	pub maxRayDispatchInvocationCount: u32,
	pub shaderGroupHandleAlignment: u32,
	pub maxRayHitAttributeSize: u32
}
pub struct VkPhysicalDeviceRayTracingPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderGroupHandleSize: u32,
	pub maxRecursionDepth: u32,
	pub maxShaderGroupStride: u32,
	pub shaderGroupBaseAlignment: u32,
	pub maxGeometryCount: uint64_t,
	pub maxInstanceCount: uint64_t,
	pub maxTriangleCount: uint64_t,
	pub maxDescriptorSetAccelerationStructures: u32
}
pub struct VkStridedDeviceAddressRegionKHR
{
	pub deviceAddress: VkDeviceAddress,
	pub stride: VkDeviceSize,
	pub size: VkDeviceSize
}
pub struct VkTraceRaysIndirectCommandKHR
{
	pub width: u32,
	pub height: u32,
	pub depth: u32
}
pub struct VkTraceRaysIndirectCommand2KHR
{
	pub raygenShaderRecordAddress: VkDeviceAddress,
	pub raygenShaderRecordSize: VkDeviceSize,
	pub missShaderBindingTableAddress: VkDeviceAddress,
	pub missShaderBindingTableSize: VkDeviceSize,
	pub missShaderBindingTableStride: VkDeviceSize,
	pub hitShaderBindingTableAddress: VkDeviceAddress,
	pub hitShaderBindingTableSize: VkDeviceSize,
	pub hitShaderBindingTableStride: VkDeviceSize,
	pub callableShaderBindingTableAddress: VkDeviceAddress,
	pub callableShaderBindingTableSize: VkDeviceSize,
	pub callableShaderBindingTableStride: VkDeviceSize,
	pub width: u32,
	pub height: u32,
	pub depth: u32
}
pub struct VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub rayTracingMaintenance1: VkBool32,
	pub rayTracingPipelineTraceRaysIndirect2: VkBool32
}
pub struct VkDrmFormatModifierPropertiesListEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub drmFormatModifierCount: u32,
	pub pDrmFormatModifierProperties: *mut VkDrmFormatModifierPropertiesEXT
}
pub struct VkDrmFormatModifierPropertiesEXT
{
	pub drmFormatModifier: uint64_t,
	pub drmFormatModifierPlaneCount: u32,
	pub drmFormatModifierTilingFeatures: VkFormatFeatureFlags
}
pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub drmFormatModifier: uint64_t,
	pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32
}
pub struct VkImageDrmFormatModifierListCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub drmFormatModifierCount: u32,
	pub pDrmFormatModifiers: *const uint64_t
}
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub drmFormatModifier: uint64_t,
	pub drmFormatModifierPlaneCount: u32,
	pub pPlaneLayouts: *const VkSubresourceLayout
}
pub struct VkImageDrmFormatModifierPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub drmFormatModifier: uint64_t
}
pub struct VkImageStencilUsageCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub stencilUsage: VkImageUsageFlags
}
type VkImageStencilUsageCreateInfoEXT = VkImageStencilUsageCreateInfo;
pub struct VkDeviceMemoryOverallocationCreateInfoAMD
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub overallocationBehavior: VkMemoryOverallocationBehaviorAMD
}
pub struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub fragmentDensityMap: VkBool32,
	pub fragmentDensityMapDynamic: VkBool32,
	pub fragmentDensityMapNonSubsampledImages: VkBool32
}
pub struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub fragmentDensityMapDeferred: VkBool32
}
pub struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub fragmentDensityMapOffset: VkBool32
}
pub struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub minFragmentDensityTexelSize: VkExtent2D,
	pub maxFragmentDensityTexelSize: VkExtent2D,
	pub fragmentDensityInvocations: VkBool32
}
pub struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub subsampledLoads: VkBool32,
	pub subsampledCoarseReconstructionEarlyAccess: VkBool32,
	pub maxSubsampledArrayLayers: u32,
	pub maxDescriptorSetSubsampledSamplers: u32
}
pub struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub fragmentDensityOffsetGranularity: VkExtent2D
}
pub struct VkRenderPassFragmentDensityMapCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub fragmentDensityMapAttachment: VkAttachmentReference
}
pub struct VkSubpassFragmentDensityMapOffsetEndInfoQCOM
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub fragmentDensityOffsetCount: u32,
	pub pFragmentDensityOffsets: *const VkOffset2D
}
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub scalarBlockLayout: VkBool32
}
type VkPhysicalDeviceScalarBlockLayoutFeaturesEXT = VkPhysicalDeviceScalarBlockLayoutFeatures;
pub struct VkSurfaceProtectedCapabilitiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub supportsProtected: VkBool32
}
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub uniformBufferStandardLayout: VkBool32
}
type VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR = VkPhysicalDeviceUniformBufferStandardLayoutFeatures;
pub struct VkPhysicalDeviceDepthClipEnableFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub depthClipEnable: VkBool32
}
pub struct VkPipelineRasterizationDepthClipStateCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineRasterizationDepthClipStateCreateFlagsEXT,
	pub depthClipEnable: VkBool32
}
pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub heapBudget: [VkDeviceSize; VK_MAX_MEMORY_HEAPS],
	pub heapUsage: [VkDeviceSize; VK_MAX_MEMORY_HEAPS]
}
pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryPriority: VkBool32
}
pub struct VkMemoryPriorityAllocateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub priority: f32
}
pub struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub pageableDeviceLocalMemory: VkBool32
}
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub bufferDeviceAddress: VkBool32,
	pub bufferDeviceAddressCaptureReplay: VkBool32,
	pub bufferDeviceAddressMultiDevice: VkBool32
}
type VkPhysicalDeviceBufferDeviceAddressFeaturesKHR = VkPhysicalDeviceBufferDeviceAddressFeatures;
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub bufferDeviceAddress: VkBool32,
	pub bufferDeviceAddressCaptureReplay: VkBool32,
	pub bufferDeviceAddressMultiDevice: VkBool32
}
type VkPhysicalDeviceBufferAddressFeaturesEXT = VkPhysicalDeviceBufferDeviceAddressFeaturesEXT;
pub struct VkBufferDeviceAddressInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub buffer: VkBuffer
}
type VkBufferDeviceAddressInfoKHR = VkBufferDeviceAddressInfo;
type VkBufferDeviceAddressInfoEXT = VkBufferDeviceAddressInfo;
pub struct VkBufferOpaqueCaptureAddressCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub opaqueCaptureAddress: uint64_t
}
type VkBufferOpaqueCaptureAddressCreateInfoKHR = VkBufferOpaqueCaptureAddressCreateInfo;
pub struct VkBufferDeviceAddressCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub deviceAddress: VkDeviceAddress
}
pub struct VkPhysicalDeviceImageViewImageFormatInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub imageViewType: VkImageViewType
}
pub struct VkFilterCubicImageViewImageFormatPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub filterCubic: VkBool32,
	pub filterCubicMinmax: VkBool32
}
pub struct VkPhysicalDeviceImagelessFramebufferFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub imagelessFramebuffer: VkBool32
}
type VkPhysicalDeviceImagelessFramebufferFeaturesKHR = VkPhysicalDeviceImagelessFramebufferFeatures;
pub struct VkFramebufferAttachmentsCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub attachmentImageInfoCount: u32,
	pub pAttachmentImageInfos: *const VkFramebufferAttachmentImageInfo
}
type VkFramebufferAttachmentsCreateInfoKHR = VkFramebufferAttachmentsCreateInfo;
pub struct VkFramebufferAttachmentImageInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkImageCreateFlags,
	pub usage: VkImageUsageFlags,
	pub width: u32,
	pub height: u32,
	pub layerCount: u32,
	pub viewFormatCount: u32,
	pub pViewFormats: *const VkFormat
}
type VkFramebufferAttachmentImageInfoKHR = VkFramebufferAttachmentImageInfo;
pub struct VkRenderPassAttachmentBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub attachmentCount: u32,
	pub pAttachments: *const VkImageView
}
type VkRenderPassAttachmentBeginInfoKHR = VkRenderPassAttachmentBeginInfo;
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub textureCompressionASTC_HDR: VkBool32
}
type VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT = VkPhysicalDeviceTextureCompressionASTCHDRFeatures;
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub cooperativeMatrix: VkBool32,
	pub cooperativeMatrixRobustBufferAccess: VkBool32
}
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub cooperativeMatrixSupportedStages: VkShaderStageFlags
}
pub struct VkCooperativeMatrixPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub MSize: u32,
	pub NSize: u32,
	pub KSize: u32,
	pub AType: VkComponentTypeNV,
	pub BType: VkComponentTypeNV,
	pub CType: VkComponentTypeNV,
	pub DType: VkComponentTypeNV,
	pub scope: VkScopeNV
}
pub struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub ycbcrImageArrays: VkBool32
}
pub struct VkImageViewHandleInfoNVX
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub imageView: VkImageView,
	pub descriptorType: VkDescriptorType,
	pub sampler: VkSampler
}
pub struct VkImageViewAddressPropertiesNVX
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub deviceAddress: VkDeviceAddress,
	pub size: VkDeviceSize
}
pub struct VkPresentFrameTokenGGP
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub frameToken: GgpFrameToken
}
pub struct VkPipelineCreationFeedback
{
	pub flags: VkPipelineCreationFeedbackFlags,
	pub duration: uint64_t
}
type VkPipelineCreationFeedbackEXT = VkPipelineCreationFeedback;
pub struct VkPipelineCreationFeedbackCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pPipelineCreationFeedback: *mut VkPipelineCreationFeedback,
	pub pipelineStageCreationFeedbackCount: u32,
	pub pPipelineStageCreationFeedbacks: *mut VkPipelineCreationFeedback
}
type VkPipelineCreationFeedbackCreateInfoEXT = VkPipelineCreationFeedbackCreateInfo;
pub struct VkSurfaceFullScreenExclusiveInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub fullScreenExclusive: VkFullScreenExclusiveEXT
}
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub hmonitor: HMONITOR
}
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub fullScreenExclusiveSupported: VkBool32
}
pub struct VkPhysicalDevicePresentBarrierFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub presentBarrier: VkBool32
}
pub struct VkSurfaceCapabilitiesPresentBarrierNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub presentBarrierSupported: VkBool32
}
pub struct VkSwapchainPresentBarrierCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub presentBarrierEnable: VkBool32
}
pub struct VkPhysicalDevicePerformanceQueryFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub performanceCounterQueryPools: VkBool32,
	pub performanceCounterMultipleQueryPools: VkBool32
}
pub struct VkPhysicalDevicePerformanceQueryPropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub allowCommandBufferQueryCopies: VkBool32
}
pub struct VkPerformanceCounterKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub unit: VkPerformanceCounterUnitKHR,
	pub scope: VkPerformanceCounterScopeKHR,
	pub storage: VkPerformanceCounterStorageKHR,
	pub uuid: [u8; VK_UUID_SIZE]
}
pub struct VkPerformanceCounterDescriptionKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub flags: VkPerformanceCounterDescriptionFlagsKHR,
	pub name: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub category: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub description: [i8; VK_MAX_DESCRIPTION_SIZE]
}
pub struct VkQueryPoolPerformanceCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub queueFamilyIndex: u32,
	pub counterIndexCount: u32,
	pub pCounterIndices: *const u32
}
pub struct VkAcquireProfilingLockInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkAcquireProfilingLockFlagsKHR,
	pub timeout: uint64_t
}
pub struct VkPerformanceQuerySubmitInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub counterPassIndex: u32
}
pub struct VkHeadlessSurfaceCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkHeadlessSurfaceCreateFlagsEXT
}
pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub coverageReductionMode: VkBool32
}
pub struct VkPipelineCoverageReductionStateCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkPipelineCoverageReductionStateCreateFlagsNV,
	pub coverageReductionMode: VkCoverageReductionModeNV
}
pub struct VkFramebufferMixedSamplesCombinationNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub coverageReductionMode: VkCoverageReductionModeNV,
	pub rasterizationSamples: VkSampleCountFlagBits,
	pub depthStencilSamples: VkSampleCountFlags,
	pub colorSamples: VkSampleCountFlags
}
pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderIntegerFunctions2: VkBool32
}
pub struct VkPerformanceValueINTEL
{
	pub kind: VkPerformanceValueTypeINTEL,
	pub data: VkPerformanceValueDataINTEL
}
pub struct VkInitializePerformanceApiInfoINTEL
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pUserData: *mut c_void
}
pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub performanceCountersSampling: VkQueryPoolSamplingModeINTEL
}
type VkQueryPoolCreateInfoINTEL = VkQueryPoolPerformanceQueryCreateInfoINTEL;
pub struct VkPerformanceMarkerInfoINTEL
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub marker: uint64_t
}
pub struct VkPerformanceStreamMarkerInfoINTEL
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub marker: u32
}
pub struct VkPerformanceOverrideInfoINTEL
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub kind: VkPerformanceOverrideTypeINTEL,
	pub enable: VkBool32,
	pub parameter: uint64_t
}
pub struct VkPerformanceConfigurationAcquireInfoINTEL
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub kind: VkPerformanceConfigurationTypeINTEL
}
pub struct VkPhysicalDeviceShaderClockFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderSubgroupClock: VkBool32,
	pub shaderDeviceClock: VkBool32
}
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub indexTypeUint8: VkBool32
}
pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderSMCount: u32,
	pub shaderWarpsPerSM: u32
}
pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderSMBuiltins: VkBool32
}
pub struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub fragmentShaderSampleInterlock: VkBool32,
	pub fragmentShaderPixelInterlock: VkBool32,
	pub fragmentShaderShadingRateInterlock: VkBool32
}
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub separateDepthStencilLayouts: VkBool32
}
type VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR = VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures;
pub struct VkAttachmentReferenceStencilLayout
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub stencilLayout: VkImageLayout
}
pub struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub primitiveTopologyListRestart: VkBool32,
	pub primitiveTopologyPatchListRestart: VkBool32
}
type VkAttachmentReferenceStencilLayoutKHR = VkAttachmentReferenceStencilLayout;
pub struct VkAttachmentDescriptionStencilLayout
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub stencilInitialLayout: VkImageLayout,
	pub stencilFinalLayout: VkImageLayout
}
type VkAttachmentDescriptionStencilLayoutKHR = VkAttachmentDescriptionStencilLayout;
pub struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub pipelineExecutableInfo: VkBool32
}
pub struct VkPipelineInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pipeline: VkPipeline
}
type VkPipelineInfoEXT = VkPipelineInfoKHR;
pub struct VkPipelineExecutablePropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub stages: VkShaderStageFlags,
	pub name: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub description: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub subgroupSize: u32
}
pub struct VkPipelineExecutableInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pipeline: VkPipeline,
	pub executableIndex: u32
}
pub struct VkPipelineExecutableStatisticKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub name: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub description: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub format: VkPipelineExecutableStatisticFormatKHR,
	pub value: VkPipelineExecutableStatisticValueKHR
}
pub struct VkPipelineExecutableInternalRepresentationKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub name: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub description: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub isText: VkBool32,
	pub dataSize: usize,
	pub pData: *mut c_void
}
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderDemoteToHelperInvocation: VkBool32
}
type VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT = VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures;
pub struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub texelBufferAlignment: VkBool32
}
pub struct VkPhysicalDeviceTexelBufferAlignmentProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
	pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
	pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
	pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32
}
type VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT = VkPhysicalDeviceTexelBufferAlignmentProperties;
pub struct VkPhysicalDeviceSubgroupSizeControlFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub subgroupSizeControl: VkBool32,
	pub computeFullSubgroups: VkBool32
}
type VkPhysicalDeviceSubgroupSizeControlFeaturesEXT = VkPhysicalDeviceSubgroupSizeControlFeatures;
pub struct VkPhysicalDeviceSubgroupSizeControlProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub minSubgroupSize: u32,
	pub maxSubgroupSize: u32,
	pub maxComputeWorkgroupSubgroups: u32,
	pub requiredSubgroupSizeStages: VkShaderStageFlags
}
type VkPhysicalDeviceSubgroupSizeControlPropertiesEXT = VkPhysicalDeviceSubgroupSizeControlProperties;
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub requiredSubgroupSize: u32
}
type VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT = VkPipelineShaderStageRequiredSubgroupSizeCreateInfo;
pub struct VkSubpassShadingPipelineCreateInfoHUAWEI
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub renderPass: VkRenderPass,
	pub subpass: u32
}
pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxSubpassShadingWorkgroupSizeAspectRatio: u32
}
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub opaqueCaptureAddress: uint64_t
}
type VkMemoryOpaqueCaptureAddressAllocateInfoKHR = VkMemoryOpaqueCaptureAddressAllocateInfo;
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub memory: VkDeviceMemory
}
type VkDeviceMemoryOpaqueCaptureAddressInfoKHR = VkDeviceMemoryOpaqueCaptureAddressInfo;
pub struct VkPhysicalDeviceLineRasterizationFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub rectangularLines: VkBool32,
	pub bresenhamLines: VkBool32,
	pub smoothLines: VkBool32,
	pub stippledRectangularLines: VkBool32,
	pub stippledBresenhamLines: VkBool32,
	pub stippledSmoothLines: VkBool32
}
pub struct VkPhysicalDeviceLineRasterizationPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub lineSubPixelPrecisionBits: u32
}
pub struct VkPipelineRasterizationLineStateCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub lineRasterizationMode: VkLineRasterizationModeEXT,
	pub stippledLineEnable: VkBool32,
	pub lineStippleFactor: u32,
	pub lineStipplePattern: u16
}
pub struct VkPhysicalDevicePipelineCreationCacheControlFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub pipelineCreationCacheControl: VkBool32
}
type VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT = VkPhysicalDevicePipelineCreationCacheControlFeatures;
pub struct VkPhysicalDeviceVulkan11Features
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub storageBuffer16BitAccess: VkBool32,
	pub uniformAndStorageBuffer16BitAccess: VkBool32,
	pub storagePushConstant16: VkBool32,
	pub storageInputOutput16: VkBool32,
	pub multiview: VkBool32,
	pub multiviewGeometryShader: VkBool32,
	pub multiviewTessellationShader: VkBool32,
	pub variablePointersStorageBuffer: VkBool32,
	pub variablePointers: VkBool32,
	pub protectedMemory: VkBool32,
	pub samplerYcbcrConversion: VkBool32,
	pub shaderDrawParameters: VkBool32
}
pub struct VkPhysicalDeviceVulkan11Properties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub deviceUUID: [u8; VK_UUID_SIZE],
	pub driverUUID: [u8; VK_UUID_SIZE],
	pub deviceLUID: [u8; VK_LUID_SIZE],
	pub deviceNodeMask: u32,
	pub deviceLUIDValid: VkBool32,
	pub subgroupSize: u32,
	pub subgroupSupportedStages: VkShaderStageFlags,
	pub subgroupSupportedOperations: VkSubgroupFeatureFlags,
	pub subgroupQuadOperationsInAllStages: VkBool32,
	pub pointClippingBehavior: VkPointClippingBehavior,
	pub maxMultiviewViewCount: u32,
	pub maxMultiviewInstanceIndex: u32,
	pub protectedNoFault: VkBool32,
	pub maxPerSetDescriptors: u32,
	pub maxMemoryAllocationSize: VkDeviceSize
}
pub struct VkPhysicalDeviceVulkan12Features
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub samplerMirrorClampToEdge: VkBool32,
	pub drawIndirectCount: VkBool32,
	pub storageBuffer8BitAccess: VkBool32,
	pub uniformAndStorageBuffer8BitAccess: VkBool32,
	pub storagePushConstant8: VkBool32,
	pub shaderBufferInt64Atomics: VkBool32,
	pub shaderSharedInt64Atomics: VkBool32,
	pub shaderFloat16: VkBool32,
	pub shaderInt8: VkBool32,
	pub descriptorIndexing: VkBool32,
	pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
	pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
	pub descriptorBindingPartiallyBound: VkBool32,
	pub descriptorBindingVariableDescriptorCount: VkBool32,
	pub runtimeDescriptorArray: VkBool32,
	pub samplerFilterMinmax: VkBool32,
	pub scalarBlockLayout: VkBool32,
	pub imagelessFramebuffer: VkBool32,
	pub uniformBufferStandardLayout: VkBool32,
	pub shaderSubgroupExtendedTypes: VkBool32,
	pub separateDepthStencilLayouts: VkBool32,
	pub hostQueryReset: VkBool32,
	pub timelineSemaphore: VkBool32,
	pub bufferDeviceAddress: VkBool32,
	pub bufferDeviceAddressCaptureReplay: VkBool32,
	pub bufferDeviceAddressMultiDevice: VkBool32,
	pub vulkanMemoryModel: VkBool32,
	pub vulkanMemoryModelDeviceScope: VkBool32,
	pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
	pub shaderOutputViewportIndex: VkBool32,
	pub shaderOutputLayer: VkBool32,
	pub subgroupBroadcastDynamicId: VkBool32
}
pub struct VkPhysicalDeviceVulkan12Properties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub driverID: VkDriverId,
	pub driverName: [i8; VK_MAX_DRIVER_NAME_SIZE],
	pub driverInfo: [i8; VK_MAX_DRIVER_INFO_SIZE],
	pub conformanceVersion: VkConformanceVersion,
	pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
	pub roundingModeIndependence: VkShaderFloatControlsIndependence,
	pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
	pub shaderDenormPreserveFloat16: VkBool32,
	pub shaderDenormPreserveFloat32: VkBool32,
	pub shaderDenormPreserveFloat64: VkBool32,
	pub shaderDenormFlushToZeroFloat16: VkBool32,
	pub shaderDenormFlushToZeroFloat32: VkBool32,
	pub shaderDenormFlushToZeroFloat64: VkBool32,
	pub shaderRoundingModeRTEFloat16: VkBool32,
	pub shaderRoundingModeRTEFloat32: VkBool32,
	pub shaderRoundingModeRTEFloat64: VkBool32,
	pub shaderRoundingModeRTZFloat16: VkBool32,
	pub shaderRoundingModeRTZFloat32: VkBool32,
	pub shaderRoundingModeRTZFloat64: VkBool32,
	pub maxUpdateAfterBindDescriptorsInAllPools: u32,
	pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
	pub robustBufferAccessUpdateAfterBind: VkBool32,
	pub quadDivergentImplicitLod: VkBool32,
	pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
	pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
	pub maxPerStageUpdateAfterBindResources: u32,
	pub maxDescriptorSetUpdateAfterBindSamplers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
	pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
	pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
	pub supportedDepthResolveModes: VkResolveModeFlags,
	pub supportedStencilResolveModes: VkResolveModeFlags,
	pub independentResolveNone: VkBool32,
	pub independentResolve: VkBool32,
	pub filterMinmaxSingleComponentFormats: VkBool32,
	pub filterMinmaxImageComponentMapping: VkBool32,
	pub maxTimelineSemaphoreValueDifference: uint64_t,
	pub framebufferIntegerColorSampleCounts: VkSampleCountFlags
}
pub struct VkPhysicalDeviceVulkan13Features
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub robustImageAccess: VkBool32,
	pub inlineUniformBlock: VkBool32,
	pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
	pub pipelineCreationCacheControl: VkBool32,
	pub privateData: VkBool32,
	pub shaderDemoteToHelperInvocation: VkBool32,
	pub shaderTerminateInvocation: VkBool32,
	pub subgroupSizeControl: VkBool32,
	pub computeFullSubgroups: VkBool32,
	pub synchronization2: VkBool32,
	pub textureCompressionASTC_HDR: VkBool32,
	pub shaderZeroInitializeWorkgroupMemory: VkBool32,
	pub dynamicRendering: VkBool32,
	pub shaderIntegerDotProduct: VkBool32,
	pub maintenance4: VkBool32
}
pub struct VkPhysicalDeviceVulkan13Properties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub minSubgroupSize: u32,
	pub maxSubgroupSize: u32,
	pub maxComputeWorkgroupSubgroups: u32,
	pub requiredSubgroupSizeStages: VkShaderStageFlags,
	pub maxInlineUniformBlockSize: u32,
	pub maxPerStageDescriptorInlineUniformBlocks: u32,
	pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
	pub maxDescriptorSetInlineUniformBlocks: u32,
	pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
	pub maxInlineUniformTotalSize: u32,
	pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct8BitSignedAccelerated: VkBool32,
	pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct16BitSignedAccelerated: VkBool32,
	pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct32BitSignedAccelerated: VkBool32,
	pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct64BitSignedAccelerated: VkBool32,
	pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
	pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
	pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
	pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
	pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
	pub maxBufferSize: VkDeviceSize
}
pub struct VkPipelineCompilerControlCreateInfoAMD
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub compilerControlFlags: VkPipelineCompilerControlFlagsAMD
}
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub deviceCoherentMemory: VkBool32
}
pub struct VkPhysicalDeviceToolProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub name: [i8; VK_MAX_EXTENSION_NAME_SIZE],
	pub version: [i8; VK_MAX_EXTENSION_NAME_SIZE],
	pub purposes: VkToolPurposeFlags,
	pub description: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub layer: [i8; VK_MAX_EXTENSION_NAME_SIZE]
}
type VkPhysicalDeviceToolPropertiesEXT = VkPhysicalDeviceToolProperties;
pub struct VkSamplerCustomBorderColorCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub customBorderColor: VkClearColorValue,
	pub format: VkFormat
}
pub struct VkPhysicalDeviceCustomBorderColorPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxCustomBorderColorSamplers: u32
}
pub struct VkPhysicalDeviceCustomBorderColorFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub customBorderColors: VkBool32,
	pub customBorderColorWithoutFormat: VkBool32
}
pub struct VkSamplerBorderColorComponentMappingCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub components: VkComponentMapping,
	pub srgb: VkBool32
}
pub struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub borderColorSwizzle: VkBool32,
	pub borderColorSwizzleFromImage: VkBool32
}
pub struct VkAccelerationStructureGeometryTrianglesDataKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub vertexFormat: VkFormat,
	pub vertexData: VkDeviceOrHostAddressConstKHR,
	pub vertexStride: VkDeviceSize,
	pub maxVertex: u32,
	pub indexType: VkIndexType,
	pub indexData: VkDeviceOrHostAddressConstKHR,
	pub transformData: VkDeviceOrHostAddressConstKHR
}
pub struct VkAccelerationStructureGeometryAabbsDataKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub data: VkDeviceOrHostAddressConstKHR,
	pub stride: VkDeviceSize
}
pub struct VkAccelerationStructureGeometryInstancesDataKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub arrayOfPointers: VkBool32,
	pub data: VkDeviceOrHostAddressConstKHR
}
pub struct VkAccelerationStructureGeometryKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub geometryType: VkGeometryTypeKHR,
	pub geometry: VkAccelerationStructureGeometryDataKHR,
	pub flags: VkGeometryFlagsKHR
}
pub struct VkAccelerationStructureBuildGeometryInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub kind: VkAccelerationStructureTypeKHR,
	pub flags: VkBuildAccelerationStructureFlagsKHR,
	pub mode: VkBuildAccelerationStructureModeKHR,
	pub srcAccelerationStructure: VkAccelerationStructureKHR,
	pub dstAccelerationStructure: VkAccelerationStructureKHR,
	pub geometryCount: u32,
	pub pGeometries: *const VkAccelerationStructureGeometryKHR,
	pub ppGeometries: *const *const VkAccelerationStructureGeometryKHR,
	pub scratchData: VkDeviceOrHostAddressKHR
}
pub struct VkAccelerationStructureBuildRangeInfoKHR
{
	pub primitiveCount: u32,
	pub primitiveOffset: u32,
	pub firstVertex: u32,
	pub transformOffset: u32
}
pub struct VkAccelerationStructureCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub createFlags: VkAccelerationStructureCreateFlagsKHR,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub kind: VkAccelerationStructureTypeKHR,
	pub deviceAddress: VkDeviceAddress
}
pub struct VkAabbPositionsKHR
{
	pub minX: f32,
	pub minY: f32,
	pub minZ: f32,
	pub maxX: f32,
	pub maxY: f32,
	pub maxZ: f32
}
type VkAabbPositionsNV = VkAabbPositionsKHR;
pub struct VkTransformMatrixKHR
{
	pub matrix: [f32; 34]
}
type VkTransformMatrixNV = VkTransformMatrixKHR;
type VkAccelerationStructureInstanceNV = VkAccelerationStructureInstanceKHR;
pub struct VkAccelerationStructureDeviceAddressInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub accelerationStructure: VkAccelerationStructureKHR
}
pub struct VkAccelerationStructureVersionInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pVersionData: *const u8
}
pub struct VkCopyAccelerationStructureInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub src: VkAccelerationStructureKHR,
	pub dst: VkAccelerationStructureKHR,
	pub mode: VkCopyAccelerationStructureModeKHR
}
pub struct VkCopyAccelerationStructureToMemoryInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub src: VkAccelerationStructureKHR,
	pub dst: VkDeviceOrHostAddressKHR,
	pub mode: VkCopyAccelerationStructureModeKHR
}
pub struct VkCopyMemoryToAccelerationStructureInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub src: VkDeviceOrHostAddressConstKHR,
	pub dst: VkAccelerationStructureKHR,
	pub mode: VkCopyAccelerationStructureModeKHR
}
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub maxPipelineRayPayloadSize: u32,
	pub maxPipelineRayHitAttributeSize: u32
}
pub struct VkPipelineLibraryCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub libraryCount: u32,
	pub pLibraries: *const VkPipeline
}
pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub extendedDynamicState: VkBool32
}
pub struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub extendedDynamicState2: VkBool32,
	pub extendedDynamicState2LogicOp: VkBool32,
	pub extendedDynamicState2PatchControlPoints: VkBool32
}
pub struct VkPhysicalDeviceExtendedDynamicState3FeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub extendedDynamicState3TessellationDomainOrigin: VkBool32,
	pub extendedDynamicState3DepthClampEnable: VkBool32,
	pub extendedDynamicState3PolygonMode: VkBool32,
	pub extendedDynamicState3RasterizationSamples: VkBool32,
	pub extendedDynamicState3SampleMask: VkBool32,
	pub extendedDynamicState3AlphaToCoverageEnable: VkBool32,
	pub extendedDynamicState3AlphaToOneEnable: VkBool32,
	pub extendedDynamicState3LogicOpEnable: VkBool32,
	pub extendedDynamicState3ColorBlendEnable: VkBool32,
	pub extendedDynamicState3ColorBlendEquation: VkBool32,
	pub extendedDynamicState3ColorWriteMask: VkBool32,
	pub extendedDynamicState3RasterizationStream: VkBool32,
	pub extendedDynamicState3ConservativeRasterizationMode: VkBool32,
	pub extendedDynamicState3ExtraPrimitiveOverestimationSize: VkBool32,
	pub extendedDynamicState3DepthClipEnable: VkBool32,
	pub extendedDynamicState3SampleLocationsEnable: VkBool32,
	pub extendedDynamicState3ColorBlendAdvanced: VkBool32,
	pub extendedDynamicState3ProvokingVertexMode: VkBool32,
	pub extendedDynamicState3LineRasterizationMode: VkBool32,
	pub extendedDynamicState3LineStippleEnable: VkBool32,
	pub extendedDynamicState3DepthClipNegativeOneToOne: VkBool32,
	pub extendedDynamicState3ViewportWScalingEnable: VkBool32,
	pub extendedDynamicState3ViewportSwizzle: VkBool32,
	pub extendedDynamicState3CoverageToColorEnable: VkBool32,
	pub extendedDynamicState3CoverageToColorLocation: VkBool32,
	pub extendedDynamicState3CoverageModulationMode: VkBool32,
	pub extendedDynamicState3CoverageModulationTableEnable: VkBool32,
	pub extendedDynamicState3CoverageModulationTable: VkBool32,
	pub extendedDynamicState3CoverageReductionMode: VkBool32,
	pub extendedDynamicState3RepresentativeFragmentTestEnable: VkBool32,
	pub extendedDynamicState3ShadingRateImageEnable: VkBool32
}
pub struct VkPhysicalDeviceExtendedDynamicState3PropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub dynamicPrimitiveTopologyUnrestricted: VkBool32
}
pub struct VkColorBlendEquationEXT
{
	pub srcColorBlendFactor: VkBlendFactor,
	pub dstColorBlendFactor: VkBlendFactor,
	pub colorBlendOp: VkBlendOp,
	pub srcAlphaBlendFactor: VkBlendFactor,
	pub dstAlphaBlendFactor: VkBlendFactor,
	pub alphaBlendOp: VkBlendOp
}
pub struct VkColorBlendAdvancedEXT
{
	pub advancedBlendOp: VkBlendOp,
	pub srcPremultiplied: VkBool32,
	pub dstPremultiplied: VkBool32,
	pub blendOverlap: VkBlendOverlapEXT,
	pub clampResults: VkBool32
}
pub struct VkRenderPassTransformBeginInfoQCOM
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub transform: VkSurfaceTransformFlagBitsKHR
}
pub struct VkCopyCommandTransformInfoQCOM
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub transform: VkSurfaceTransformFlagBitsKHR
}
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub transform: VkSurfaceTransformFlagBitsKHR,
	pub renderArea: VkRect2D
}
pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub diagnosticsConfig: VkBool32
}
pub struct VkDeviceDiagnosticsConfigCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDeviceDiagnosticsConfigFlagsNV
}
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderZeroInitializeWorkgroupMemory: VkBool32
}
type VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR = VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
pub struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderSubgroupUniformControlFlow: VkBool32
}
pub struct VkPhysicalDeviceRobustness2FeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub robustBufferAccess2: VkBool32,
	pub robustImageAccess2: VkBool32,
	pub nullDescriptor: VkBool32
}
pub struct VkPhysicalDeviceRobustness2PropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub robustStorageBufferAccessSizeAlignment: VkDeviceSize,
	pub robustUniformBufferAccessSizeAlignment: VkDeviceSize
}
pub struct VkPhysicalDeviceImageRobustnessFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub robustImageAccess: VkBool32
}
type VkPhysicalDeviceImageRobustnessFeaturesEXT = VkPhysicalDeviceImageRobustnessFeatures;
pub struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub workgroupMemoryExplicitLayout: VkBool32,
	pub workgroupMemoryExplicitLayoutScalarBlockLayout: VkBool32,
	pub workgroupMemoryExplicitLayout8BitAccess: VkBool32,
	pub workgroupMemoryExplicitLayout16BitAccess: VkBool32
}
pub struct VkPhysicalDevicePortabilitySubsetFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub constantAlphaColorBlendFactors: VkBool32,
	pub events: VkBool32,
	pub imageViewFormatReinterpretation: VkBool32,
	pub imageViewFormatSwizzle: VkBool32,
	pub imageView2DOn3DImage: VkBool32,
	pub multisampleArrayImage: VkBool32,
	pub mutableComparisonSamplers: VkBool32,
	pub pointPolygons: VkBool32,
	pub samplerMipLodBias: VkBool32,
	pub separateStencilMaskRef: VkBool32,
	pub shaderSampleRateInterpolationFunctions: VkBool32,
	pub tessellationIsolines: VkBool32,
	pub tessellationPointMode: VkBool32,
	pub triangleFans: VkBool32,
	pub vertexAttributeAccessBeyondStride: VkBool32
}
pub struct VkPhysicalDevicePortabilitySubsetPropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub minVertexInputBindingStrideAlignment: u32
}
pub struct VkPhysicalDevice4444FormatsFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub formatA4R4G4B4: VkBool32,
	pub formatA4B4G4R4: VkBool32
}
pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub subpassShading: VkBool32
}
pub struct VkBufferCopy2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcOffset: VkDeviceSize,
	pub dstOffset: VkDeviceSize,
	pub size: VkDeviceSize
}
type VkBufferCopy2KHR = VkBufferCopy2;
pub struct VkImageCopy2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D
}
type VkImageCopy2KHR = VkImageCopy2;
pub struct VkImageBlit2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffsets: [VkOffset3D; 2],
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffsets: [VkOffset3D; 2]
}
type VkImageBlit2KHR = VkImageBlit2;
pub struct VkBufferImageCopy2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub bufferOffset: VkDeviceSize,
	pub bufferRowLength: u32,
	pub bufferImageHeight: u32,
	pub imageSubresource: VkImageSubresourceLayers,
	pub imageOffset: VkOffset3D,
	pub imageExtent: VkExtent3D
}
type VkBufferImageCopy2KHR = VkBufferImageCopy2;
pub struct VkImageResolve2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D
}
type VkImageResolve2KHR = VkImageResolve2;
pub struct VkCopyBufferInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcBuffer: VkBuffer,
	pub dstBuffer: VkBuffer,
	pub regionCount: u32,
	pub pRegions: *const VkBufferCopy2
}
type VkCopyBufferInfo2KHR = VkCopyBufferInfo2;
pub struct VkCopyImageInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcImage: VkImage,
	pub srcImageLayout: VkImageLayout,
	pub dstImage: VkImage,
	pub dstImageLayout: VkImageLayout,
	pub regionCount: u32,
	pub pRegions: *const VkImageCopy2
}
type VkCopyImageInfo2KHR = VkCopyImageInfo2;
pub struct VkBlitImageInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcImage: VkImage,
	pub srcImageLayout: VkImageLayout,
	pub dstImage: VkImage,
	pub dstImageLayout: VkImageLayout,
	pub regionCount: u32,
	pub pRegions: *const VkImageBlit2,
	pub filter: VkFilter
}
type VkBlitImageInfo2KHR = VkBlitImageInfo2;
pub struct VkCopyBufferToImageInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcBuffer: VkBuffer,
	pub dstImage: VkImage,
	pub dstImageLayout: VkImageLayout,
	pub regionCount: u32,
	pub pRegions: *const VkBufferImageCopy2
}
type VkCopyBufferToImageInfo2KHR = VkCopyBufferToImageInfo2;
pub struct VkCopyImageToBufferInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcImage: VkImage,
	pub srcImageLayout: VkImageLayout,
	pub dstBuffer: VkBuffer,
	pub regionCount: u32,
	pub pRegions: *const VkBufferImageCopy2
}
type VkCopyImageToBufferInfo2KHR = VkCopyImageToBufferInfo2;
pub struct VkResolveImageInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcImage: VkImage,
	pub srcImageLayout: VkImageLayout,
	pub dstImage: VkImage,
	pub dstImageLayout: VkImageLayout,
	pub regionCount: u32,
	pub pRegions: *const VkImageResolve2
}
type VkResolveImageInfo2KHR = VkResolveImageInfo2;
pub struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderImageInt64Atomics: VkBool32,
	pub sparseImageInt64Atomics: VkBool32
}
pub struct VkFragmentShadingRateAttachmentInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pFragmentShadingRateAttachment: *const VkAttachmentReference2,
	pub shadingRateAttachmentTexelSize: VkExtent2D
}
pub struct VkPipelineFragmentShadingRateStateCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub fragmentSize: VkExtent2D,
	pub combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2]
}
pub struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub pipelineFragmentShadingRate: VkBool32,
	pub primitiveFragmentShadingRate: VkBool32,
	pub attachmentFragmentShadingRate: VkBool32
}
pub struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub minFragmentShadingRateAttachmentTexelSize: VkExtent2D,
	pub maxFragmentShadingRateAttachmentTexelSize: VkExtent2D,
	pub maxFragmentShadingRateAttachmentTexelSizeAspectRatio: u32,
	pub primitiveFragmentShadingRateWithMultipleViewports: VkBool32,
	pub layeredShadingRateAttachments: VkBool32,
	pub fragmentShadingRateNonTrivialCombinerOps: VkBool32,
	pub maxFragmentSize: VkExtent2D,
	pub maxFragmentSizeAspectRatio: u32,
	pub maxFragmentShadingRateCoverageSamples: u32,
	pub maxFragmentShadingRateRasterizationSamples: VkSampleCountFlagBits,
	pub fragmentShadingRateWithShaderDepthStencilWrites: VkBool32,
	pub fragmentShadingRateWithSampleMask: VkBool32,
	pub fragmentShadingRateWithShaderSampleMask: VkBool32,
	pub fragmentShadingRateWithConservativeRasterization: VkBool32,
	pub fragmentShadingRateWithFragmentShaderInterlock: VkBool32,
	pub fragmentShadingRateWithCustomSampleLocations: VkBool32,
	pub fragmentShadingRateStrictMultiplyCombiner: VkBool32
}
pub struct VkPhysicalDeviceFragmentShadingRateKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub sampleCounts: VkSampleCountFlags,
	pub fragmentSize: VkExtent2D
}
pub struct VkPhysicalDeviceShaderTerminateInvocationFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderTerminateInvocation: VkBool32
}
type VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR = VkPhysicalDeviceShaderTerminateInvocationFeatures;
pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub fragmentShadingRateEnums: VkBool32,
	pub supersampleFragmentShadingRates: VkBool32,
	pub noInvocationFragmentShadingRates: VkBool32
}
pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxFragmentShadingRateInvocationCount: VkSampleCountFlagBits
}
pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub shadingRateType: VkFragmentShadingRateTypeNV,
	pub shadingRate: VkFragmentShadingRateNV,
	pub combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2]
}
pub struct VkAccelerationStructureBuildSizesInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub accelerationStructureSize: VkDeviceSize,
	pub updateScratchSize: VkDeviceSize,
	pub buildScratchSize: VkDeviceSize
}
pub struct VkPhysicalDeviceImage2DViewOf3DFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub image2DViewOf3D: VkBool32,
	pub sampler2DViewOf3D: VkBool32
}
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub mutableDescriptorType: VkBool32
}
type VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE = VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT;
pub struct VkMutableDescriptorTypeListEXT
{
	pub descriptorTypeCount: u32,
	pub pDescriptorTypes: *const VkDescriptorType
}
type VkMutableDescriptorTypeListVALVE = VkMutableDescriptorTypeListEXT;
pub struct VkMutableDescriptorTypeCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub mutableDescriptorTypeListCount: u32,
	pub pMutableDescriptorTypeLists: *const VkMutableDescriptorTypeListEXT
}
type VkMutableDescriptorTypeCreateInfoVALVE = VkMutableDescriptorTypeCreateInfoEXT;
pub struct VkPhysicalDeviceDepthClipControlFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub depthClipControl: VkBool32
}
pub struct VkPipelineViewportDepthClipControlCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub negativeOneToOne: VkBool32
}
pub struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub vertexInputDynamicState: VkBool32
}
pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub externalMemoryRDMA: VkBool32
}
pub struct VkVertexInputBindingDescription2EXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub binding: u32,
	pub stride: u32,
	pub inputRate: VkVertexInputRate,
	pub divisor: u32
}
pub struct VkVertexInputAttributeDescription2EXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub location: u32,
	pub binding: u32,
	pub format: VkFormat,
	pub offset: u32
}
pub struct VkPhysicalDeviceColorWriteEnableFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub colorWriteEnable: VkBool32
}
pub struct VkPipelineColorWriteCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub attachmentCount: u32,
	pub pColorWriteEnables: *const VkBool32
}
pub struct VkMemoryBarrier2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcStageMask: VkPipelineStageFlags2,
	pub srcAccessMask: VkAccessFlags2,
	pub dstStageMask: VkPipelineStageFlags2,
	pub dstAccessMask: VkAccessFlags2
}
type VkMemoryBarrier2KHR = VkMemoryBarrier2;
pub struct VkImageMemoryBarrier2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcStageMask: VkPipelineStageFlags2,
	pub srcAccessMask: VkAccessFlags2,
	pub dstStageMask: VkPipelineStageFlags2,
	pub dstAccessMask: VkAccessFlags2,
	pub oldLayout: VkImageLayout,
	pub newLayout: VkImageLayout,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub image: VkImage,
	pub subresourceRange: VkImageSubresourceRange
}
type VkImageMemoryBarrier2KHR = VkImageMemoryBarrier2;
pub struct VkBufferMemoryBarrier2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub srcStageMask: VkPipelineStageFlags2,
	pub srcAccessMask: VkAccessFlags2,
	pub dstStageMask: VkPipelineStageFlags2,
	pub dstAccessMask: VkAccessFlags2,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize
}
type VkBufferMemoryBarrier2KHR = VkBufferMemoryBarrier2;
pub struct VkDependencyInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub dependencyFlags: VkDependencyFlags,
	pub memoryBarrierCount: u32,
	pub pMemoryBarriers: *const VkMemoryBarrier2,
	pub bufferMemoryBarrierCount: u32,
	pub pBufferMemoryBarriers: *const VkBufferMemoryBarrier2,
	pub imageMemoryBarrierCount: u32,
	pub pImageMemoryBarriers: *const VkImageMemoryBarrier2
}
type VkDependencyInfoKHR = VkDependencyInfo;
pub struct VkSemaphoreSubmitInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub semaphore: VkSemaphore,
	pub value: uint64_t,
	pub stageMask: VkPipelineStageFlags2,
	pub deviceIndex: u32
}
type VkSemaphoreSubmitInfoKHR = VkSemaphoreSubmitInfo;
pub struct VkCommandBufferSubmitInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub commandBuffer: VkCommandBuffer,
	pub deviceMask: u32
}
type VkCommandBufferSubmitInfoKHR = VkCommandBufferSubmitInfo;
pub struct VkSubmitInfo2
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkSubmitFlags,
	pub waitSemaphoreInfoCount: u32,
	pub pWaitSemaphoreInfos: *const VkSemaphoreSubmitInfo,
	pub commandBufferInfoCount: u32,
	pub pCommandBufferInfos: *const VkCommandBufferSubmitInfo,
	pub signalSemaphoreInfoCount: u32,
	pub pSignalSemaphoreInfos: *const VkSemaphoreSubmitInfo
}
type VkSubmitInfo2KHR = VkSubmitInfo2;
pub struct VkQueueFamilyCheckpointProperties2NV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub checkpointExecutionStageMask: VkPipelineStageFlags2
}
pub struct VkCheckpointData2NV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub stage: VkPipelineStageFlags2,
	pub pCheckpointMarker: *mut c_void
}
pub struct VkPhysicalDeviceSynchronization2Features
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub synchronization2: VkBool32
}
type VkPhysicalDeviceSynchronization2FeaturesKHR = VkPhysicalDeviceSynchronization2Features;
pub struct VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub primitivesGeneratedQuery: VkBool32,
	pub primitivesGeneratedQueryWithRasterizerDiscard: VkBool32,
	pub primitivesGeneratedQueryWithNonZeroStreams: VkBool32
}
pub struct VkPhysicalDeviceLegacyDitheringFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub legacyDithering: VkBool32
}
pub struct VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub multisampledRenderToSingleSampled: VkBool32
}
pub struct VkSubpassResolvePerformanceQueryEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub optimal: VkBool32
}
pub struct VkMultisampledRenderToSingleSampledInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub multisampledRenderToSingleSampledEnable: VkBool32,
	pub rasterizationSamples: VkSampleCountFlagBits
}
pub struct VkPhysicalDevicePipelineProtectedAccessFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub pipelineProtectedAccess: VkBool32
}
pub struct VkQueueFamilyVideoPropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub videoCodecOperations: VkVideoCodecOperationFlagsKHR
}
pub struct VkQueueFamilyQueryResultStatusPropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub queryResultStatusSupport: VkBool32
}
pub struct VkVideoProfileListInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub profileCount: u32,
	pub pProfiles: *const VkVideoProfileInfoKHR
}
pub struct VkPhysicalDeviceVideoFormatInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub imageUsage: VkImageUsageFlags
}
pub struct VkVideoFormatPropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub format: VkFormat,
	pub componentMapping: VkComponentMapping,
	pub imageCreateFlags: VkImageCreateFlags,
	pub imageType: VkImageType,
	pub imageTiling: VkImageTiling,
	pub imageUsageFlags: VkImageUsageFlags
}
pub struct VkVideoProfileInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub videoCodecOperation: VkVideoCodecOperationFlagBitsKHR,
	pub chromaSubsampling: VkVideoChromaSubsamplingFlagsKHR,
	pub lumaBitDepth: VkVideoComponentBitDepthFlagsKHR,
	pub chromaBitDepth: VkVideoComponentBitDepthFlagsKHR
}
pub struct VkVideoCapabilitiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub flags: VkVideoCapabilityFlagsKHR,
	pub minBitstreamBufferOffsetAlignment: VkDeviceSize,
	pub minBitstreamBufferSizeAlignment: VkDeviceSize,
	pub pictureAccessGranularity: VkExtent2D,
	pub minCodedExtent: VkExtent2D,
	pub maxCodedExtent: VkExtent2D,
	pub maxDpbSlots: u32,
	pub maxActiveReferencePictures: u32,
	pub stdHeaderVersion: VkExtensionProperties
}
pub struct VkVideoSessionMemoryRequirementsKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryBindIndex: u32,
	pub memoryRequirements: VkMemoryRequirements
}
pub struct VkBindVideoSessionMemoryInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub memoryBindIndex: u32,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub memorySize: VkDeviceSize
}
pub struct VkVideoPictureResourceInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub codedOffset: VkOffset2D,
	pub codedExtent: VkExtent2D,
	pub baseArrayLayer: u32,
	pub imageViewBinding: VkImageView
}
pub struct VkVideoReferenceSlotInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub slotIndex: i32,
	pub pPictureResource: *const VkVideoPictureResourceInfoKHR
}
pub struct VkVideoDecodeCapabilitiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub flags: VkVideoDecodeCapabilityFlagsKHR
}
pub struct VkVideoDecodeUsageInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub videoUsageHints: VkVideoDecodeUsageFlagsKHR
}
pub struct VkVideoDecodeInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkVideoDecodeFlagsKHR,
	pub srcBuffer: VkBuffer,
	pub srcBufferOffset: VkDeviceSize,
	pub srcBufferRange: VkDeviceSize,
	pub dstPictureResource: VkVideoPictureResourceInfoKHR,
	pub pSetupReferenceSlot: *const VkVideoReferenceSlotInfoKHR,
	pub referenceSlotCount: u32,
	pub pReferenceSlots: *const VkVideoReferenceSlotInfoKHR
}
pub struct VkVideoDecodeH264ProfileInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub stdProfileIdc: StdVideoH264ProfileIdc,
	pub pictureLayout: VkVideoDecodeH264PictureLayoutFlagBitsEXT
}
pub struct VkVideoDecodeH264CapabilitiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxLevelIdc: StdVideoH264LevelIdc,
	pub fieldOffsetGranularity: VkOffset2D
}
pub struct VkVideoDecodeH264SessionParametersAddInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub stdSPSCount: u32,
	pub pStdSPSs: *const StdVideoH264SequenceParameterSet,
	pub stdPPSCount: u32,
	pub pStdPPSs: *const StdVideoH264PictureParameterSet
}
pub struct VkVideoDecodeH264SessionParametersCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub maxStdSPSCount: u32,
	pub maxStdPPSCount: u32,
	pub pParametersAddInfo: *const VkVideoDecodeH264SessionParametersAddInfoEXT
}
pub struct VkVideoDecodeH264PictureInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pStdPictureInfo: *const StdVideoDecodeH264PictureInfo,
	pub sliceCount: u32,
	pub pSliceOffsets: *const u32
}
pub struct VkVideoDecodeH264DpbSlotInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pStdReferenceInfo: *const StdVideoDecodeH264ReferenceInfo
}
pub struct VkVideoDecodeH265ProfileInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub stdProfileIdc: StdVideoH265ProfileIdc
}
pub struct VkVideoDecodeH265CapabilitiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxLevelIdc: StdVideoH265LevelIdc
}
pub struct VkVideoDecodeH265SessionParametersAddInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub stdVPSCount: u32,
	pub pStdVPSs: *const StdVideoH265VideoParameterSet,
	pub stdSPSCount: u32,
	pub pStdSPSs: *const StdVideoH265SequenceParameterSet,
	pub stdPPSCount: u32,
	pub pStdPPSs: *const StdVideoH265PictureParameterSet
}
pub struct VkVideoDecodeH265SessionParametersCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub maxStdVPSCount: u32,
	pub maxStdSPSCount: u32,
	pub maxStdPPSCount: u32,
	pub pParametersAddInfo: *const VkVideoDecodeH265SessionParametersAddInfoEXT
}
pub struct VkVideoDecodeH265PictureInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pStdPictureInfo: *mut StdVideoDecodeH265PictureInfo,
	pub sliceCount: u32,
	pub pSliceOffsets: *const u32
}
pub struct VkVideoDecodeH265DpbSlotInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pStdReferenceInfo: *const StdVideoDecodeH265ReferenceInfo
}
pub struct VkVideoSessionCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub queueFamilyIndex: u32,
	pub flags: VkVideoSessionCreateFlagsKHR,
	pub pVideoProfile: *const VkVideoProfileInfoKHR,
	pub pictureFormat: VkFormat,
	pub maxCodedExtent: VkExtent2D,
	pub referencePictureFormat: VkFormat,
	pub maxDpbSlots: u32,
	pub maxActiveReferencePictures: u32,
	pub pStdHeaderVersion: *const VkExtensionProperties
}
pub struct VkVideoSessionParametersCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkVideoSessionParametersCreateFlagsKHR,
	pub videoSessionParametersTemplate: VkVideoSessionParametersKHR,
	pub videoSession: VkVideoSessionKHR
}
pub struct VkVideoSessionParametersUpdateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub updateSequenceCount: u32
}
pub struct VkVideoBeginCodingInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkVideoBeginCodingFlagsKHR,
	pub videoSession: VkVideoSessionKHR,
	pub videoSessionParameters: VkVideoSessionParametersKHR,
	pub referenceSlotCount: u32,
	pub pReferenceSlots: *const VkVideoReferenceSlotInfoKHR
}
pub struct VkVideoEndCodingInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkVideoEndCodingFlagsKHR
}
pub struct VkVideoCodingControlInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkVideoCodingControlFlagsKHR
}
pub struct VkVideoEncodeUsageInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub videoUsageHints: VkVideoEncodeUsageFlagsKHR,
	pub videoContentHints: VkVideoEncodeContentFlagsKHR,
	pub tuningMode: VkVideoEncodeTuningModeKHR
}
pub struct VkVideoEncodeInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkVideoEncodeFlagsKHR,
	pub qualityLevel: u32,
	pub dstBitstreamBuffer: VkBuffer,
	pub dstBitstreamBufferOffset: VkDeviceSize,
	pub dstBitstreamBufferMaxRange: VkDeviceSize,
	pub srcPictureResource: VkVideoPictureResourceInfoKHR,
	pub pSetupReferenceSlot: *const VkVideoReferenceSlotInfoKHR,
	pub referenceSlotCount: u32,
	pub pReferenceSlots: *const VkVideoReferenceSlotInfoKHR,
	pub precedingExternallyEncodedBytes: u32
}
pub struct VkVideoEncodeRateControlInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkVideoEncodeRateControlFlagsKHR,
	pub rateControlMode: VkVideoEncodeRateControlModeFlagBitsKHR,
	pub layerCount: u8,
	pub pLayerConfigs: *const VkVideoEncodeRateControlLayerInfoKHR
}
pub struct VkVideoEncodeRateControlLayerInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub averageBitrate: u32,
	pub maxBitrate: u32,
	pub frameRateNumerator: u32,
	pub frameRateDenominator: u32,
	pub virtualBufferSizeInMs: u32,
	pub initialVirtualBufferSizeInMs: u32
}
pub struct VkVideoEncodeCapabilitiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub flags: VkVideoEncodeCapabilityFlagsKHR,
	pub rateControlModes: VkVideoEncodeRateControlModeFlagsKHR,
	pub rateControlLayerCount: u8,
	pub qualityLevelCount: u8,
	pub inputImageDataFillAlignment: VkExtent2D
}
pub struct VkVideoEncodeH264CapabilitiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub flags: VkVideoEncodeH264CapabilityFlagsEXT,
	pub inputModeFlags: VkVideoEncodeH264InputModeFlagsEXT,
	pub outputModeFlags: VkVideoEncodeH264OutputModeFlagsEXT,
	pub maxPPictureL0ReferenceCount: u8,
	pub maxBPictureL0ReferenceCount: u8,
	pub maxL1ReferenceCount: u8,
	pub motionVectorsOverPicBoundariesFlag: VkBool32,
	pub maxBytesPerPicDenom: u32,
	pub maxBitsPerMbDenom: u32,
	pub log2MaxMvLengthHorizontal: u32,
	pub log2MaxMvLengthVertical: u32
}
pub struct VkVideoEncodeH264SessionParametersAddInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub stdSPSCount: u32,
	pub pStdSPSs: *const StdVideoH264SequenceParameterSet,
	pub stdPPSCount: u32,
	pub pStdPPSs: *const StdVideoH264PictureParameterSet
}
pub struct VkVideoEncodeH264SessionParametersCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub maxStdSPSCount: u32,
	pub maxStdPPSCount: u32,
	pub pParametersAddInfo: *const VkVideoEncodeH264SessionParametersAddInfoEXT
}
pub struct VkVideoEncodeH264DpbSlotInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub slotIndex: i8,
	pub pStdReferenceInfo: *const StdVideoEncodeH264ReferenceInfo
}
pub struct VkVideoEncodeH264VclFrameInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pReferenceFinalLists: *const VkVideoEncodeH264ReferenceListsInfoEXT,
	pub naluSliceEntryCount: u32,
	pub pNaluSliceEntries: *const VkVideoEncodeH264NaluSliceInfoEXT,
	pub pCurrentPictureInfo: *const StdVideoEncodeH264PictureInfo
}
pub struct VkVideoEncodeH264ReferenceListsInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub referenceList0EntryCount: u8,
	pub pReferenceList0Entries: *const VkVideoEncodeH264DpbSlotInfoEXT,
	pub referenceList1EntryCount: u8,
	pub pReferenceList1Entries: *const VkVideoEncodeH264DpbSlotInfoEXT,
	pub pMemMgmtCtrlOperations: *const StdVideoEncodeH264RefMemMgmtCtrlOperations
}
pub struct VkVideoEncodeH264EmitPictureParametersInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub spsId: u8,
	pub emitSpsEnable: VkBool32,
	pub ppsIdEntryCount: u32,
	pub ppsIdEntries: *const u8
}
pub struct VkVideoEncodeH264ProfileInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub stdProfileIdc: StdVideoH264ProfileIdc
}
pub struct VkVideoEncodeH264NaluSliceInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub mbCount: u32,
	pub pReferenceFinalLists: *const VkVideoEncodeH264ReferenceListsInfoEXT,
	pub pSliceHeaderStd: *const StdVideoEncodeH264SliceHeader
}
pub struct VkVideoEncodeH264RateControlInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub gopFrameCount: u32,
	pub idrPeriod: u32,
	pub consecutiveBFrameCount: u32,
	pub rateControlStructure: VkVideoEncodeH264RateControlStructureEXT,
	pub temporalLayerCount: u8
}
pub struct VkVideoEncodeH264QpEXT
{
	pub qpI: i32,
	pub qpP: i32,
	pub qpB: i32
}
pub struct VkVideoEncodeH264FrameSizeEXT
{
	pub frameISize: u32,
	pub framePSize: u32,
	pub frameBSize: u32
}
pub struct VkVideoEncodeH264RateControlLayerInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub temporalLayerId: u8,
	pub useInitialRcQp: VkBool32,
	pub initialRcQp: VkVideoEncodeH264QpEXT,
	pub useMinQp: VkBool32,
	pub minQp: VkVideoEncodeH264QpEXT,
	pub useMaxQp: VkBool32,
	pub maxQp: VkVideoEncodeH264QpEXT,
	pub useMaxFrameSize: VkBool32,
	pub maxFrameSize: VkVideoEncodeH264FrameSizeEXT
}
pub struct VkVideoEncodeH265CapabilitiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub flags: VkVideoEncodeH265CapabilityFlagsEXT,
	pub inputModeFlags: VkVideoEncodeH265InputModeFlagsEXT,
	pub outputModeFlags: VkVideoEncodeH265OutputModeFlagsEXT,
	pub ctbSizes: VkVideoEncodeH265CtbSizeFlagsEXT,
	pub transformBlockSizes: VkVideoEncodeH265TransformBlockSizeFlagsEXT,
	pub maxPPictureL0ReferenceCount: u8,
	pub maxBPictureL0ReferenceCount: u8,
	pub maxL1ReferenceCount: u8,
	pub maxSubLayersCount: u8,
	pub minLog2MinLumaCodingBlockSizeMinus3: u8,
	pub maxLog2MinLumaCodingBlockSizeMinus3: u8,
	pub minLog2MinLumaTransformBlockSizeMinus2: u8,
	pub maxLog2MinLumaTransformBlockSizeMinus2: u8,
	pub minMaxTransformHierarchyDepthInter: u8,
	pub maxMaxTransformHierarchyDepthInter: u8,
	pub minMaxTransformHierarchyDepthIntra: u8,
	pub maxMaxTransformHierarchyDepthIntra: u8,
	pub maxDiffCuQpDeltaDepth: u8,
	pub minMaxNumMergeCand: u8,
	pub maxMaxNumMergeCand: u8
}
pub struct VkVideoEncodeH265SessionParametersAddInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub stdVPSCount: u32,
	pub pStdVPSs: *const StdVideoH265VideoParameterSet,
	pub stdSPSCount: u32,
	pub pStdSPSs: *const StdVideoH265SequenceParameterSet,
	pub stdPPSCount: u32,
	pub pStdPPSs: *const StdVideoH265PictureParameterSet
}
pub struct VkVideoEncodeH265SessionParametersCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub maxStdVPSCount: u32,
	pub maxStdSPSCount: u32,
	pub maxStdPPSCount: u32,
	pub pParametersAddInfo: *const VkVideoEncodeH265SessionParametersAddInfoEXT
}
pub struct VkVideoEncodeH265VclFrameInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pReferenceFinalLists: *const VkVideoEncodeH265ReferenceListsInfoEXT,
	pub naluSliceSegmentEntryCount: u32,
	pub pNaluSliceSegmentEntries: *const VkVideoEncodeH265NaluSliceSegmentInfoEXT,
	pub pCurrentPictureInfo: *const StdVideoEncodeH265PictureInfo
}
pub struct VkVideoEncodeH265EmitPictureParametersInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub vpsId: u8,
	pub spsId: u8,
	pub emitVpsEnable: VkBool32,
	pub emitSpsEnable: VkBool32,
	pub ppsIdEntryCount: u32,
	pub ppsIdEntries: *const u8
}
pub struct VkVideoEncodeH265NaluSliceSegmentInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub ctbCount: u32,
	pub pReferenceFinalLists: *const VkVideoEncodeH265ReferenceListsInfoEXT,
	pub pSliceSegmentHeaderStd: *const StdVideoEncodeH265SliceSegmentHeader
}
pub struct VkVideoEncodeH265RateControlInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub gopFrameCount: u32,
	pub idrPeriod: u32,
	pub consecutiveBFrameCount: u32,
	pub rateControlStructure: VkVideoEncodeH265RateControlStructureEXT,
	pub subLayerCount: u8
}
pub struct VkVideoEncodeH265QpEXT
{
	pub qpI: i32,
	pub qpP: i32,
	pub qpB: i32
}
pub struct VkVideoEncodeH265FrameSizeEXT
{
	pub frameISize: u32,
	pub framePSize: u32,
	pub frameBSize: u32
}
pub struct VkVideoEncodeH265RateControlLayerInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub temporalId: u8,
	pub useInitialRcQp: VkBool32,
	pub initialRcQp: VkVideoEncodeH265QpEXT,
	pub useMinQp: VkBool32,
	pub minQp: VkVideoEncodeH265QpEXT,
	pub useMaxQp: VkBool32,
	pub maxQp: VkVideoEncodeH265QpEXT,
	pub useMaxFrameSize: VkBool32,
	pub maxFrameSize: VkVideoEncodeH265FrameSizeEXT
}
pub struct VkVideoEncodeH265ProfileInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub stdProfileIdc: StdVideoH265ProfileIdc
}
pub struct VkVideoEncodeH265DpbSlotInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub slotIndex: i8,
	pub pStdReferenceInfo: *const StdVideoEncodeH265ReferenceInfo
}
pub struct VkVideoEncodeH265ReferenceListsInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub referenceList0EntryCount: u8,
	pub pReferenceList0Entries: *const VkVideoEncodeH265DpbSlotInfoEXT,
	pub referenceList1EntryCount: u8,
	pub pReferenceList1Entries: *const VkVideoEncodeH265DpbSlotInfoEXT,
	pub pReferenceModifications: *const StdVideoEncodeH265ReferenceModifications
}
pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub inheritedViewportScissor2D: VkBool32
}
pub struct VkCommandBufferInheritanceViewportScissorInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub viewportScissor2D: VkBool32,
	pub viewportDepthCount: u32,
	pub pViewportDepths: *const VkViewport
}
pub struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub ycbcr2plane444Formats: VkBool32
}
pub struct VkPhysicalDeviceProvokingVertexFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub provokingVertexLast: VkBool32,
	pub transformFeedbackPreservesProvokingVertex: VkBool32
}
pub struct VkPhysicalDeviceProvokingVertexPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub provokingVertexModePerPipeline: VkBool32,
	pub transformFeedbackPreservesTriangleFanProvokingVertex: VkBool32
}
pub struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub provokingVertexMode: VkProvokingVertexModeEXT
}
pub struct VkCuModuleCreateInfoNVX
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub dataSize: usize,
	pub pData: *const c_void
}
pub struct VkCuFunctionCreateInfoNVX
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub module: VkCuModuleNVX,
	pub pName: *const i8
}
pub struct VkCuLaunchInfoNVX
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub function: VkCuFunctionNVX,
	pub gridDimX: u32,
	pub gridDimY: u32,
	pub gridDimZ: u32,
	pub blockDimX: u32,
	pub blockDimY: u32,
	pub blockDimZ: u32,
	pub sharedMemBytes: u32,
	pub paramCount: usize,
	pub pParams: *const *const c_void,
	pub extraCount: usize,
	pub pExtras: *const *const c_void
}
pub struct VkPhysicalDeviceShaderIntegerDotProductFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderIntegerDotProduct: VkBool32
}
type VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR = VkPhysicalDeviceShaderIntegerDotProductFeatures;
pub struct VkPhysicalDeviceShaderIntegerDotProductProperties
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct8BitSignedAccelerated: VkBool32,
	pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct16BitSignedAccelerated: VkBool32,
	pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct32BitSignedAccelerated: VkBool32,
	pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct64BitSignedAccelerated: VkBool32,
	pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32
}
type VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR = VkPhysicalDeviceShaderIntegerDotProductProperties;
pub struct VkPhysicalDeviceDrmPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub hasPrimary: VkBool32,
	pub hasRender: VkBool32,
	pub primaryMajor: int64_t,
	pub primaryMinor: int64_t,
	pub renderMajor: int64_t,
	pub renderMinor: int64_t
}
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub fragmentShaderBarycentric: VkBool32
}
pub struct VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub triStripVertexOrderIndependentOfProvokingVertex: VkBool32
}
pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub rayTracingMotionBlur: VkBool32,
	pub rayTracingMotionBlurPipelineTraceRaysIndirect: VkBool32
}
pub struct VkAccelerationStructureGeometryMotionTrianglesDataNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub vertexData: VkDeviceOrHostAddressConstKHR
}
pub struct VkAccelerationStructureMotionInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub maxInstances: u32,
	pub flags: VkAccelerationStructureMotionInfoFlagsNV
}
pub struct VkSRTDataNV
{
	pub sx: f32,
	pub a: f32,
	pub b: f32,
	pub pvx: f32,
	pub sy: f32,
	pub c: f32,
	pub pvy: f32,
	pub sz: f32,
	pub pvz: f32,
	pub qx: f32,
	pub qy: f32,
	pub qz: f32,
	pub qw: f32,
	pub tx: f32,
	pub ty: f32,
	pub tz: f32
}
pub struct VkAccelerationStructureMotionInstanceNV
{
	pub kind: VkAccelerationStructureMotionInstanceTypeNV,
	pub flags: VkAccelerationStructureMotionInstanceFlagsNV,
	pub data: VkAccelerationStructureMotionInstanceDataNV
}
pub struct VkMemoryGetRemoteAddressInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub memory: VkDeviceMemory,
	pub handleType: VkExternalMemoryHandleTypeFlagBits
}
pub struct VkImportMemoryBufferCollectionFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub collection: VkBufferCollectionFUCHSIA,
	pub index: u32
}
pub struct VkBufferCollectionImageCreateInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub collection: VkBufferCollectionFUCHSIA,
	pub index: u32
}
pub struct VkBufferCollectionBufferCreateInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub collection: VkBufferCollectionFUCHSIA,
	pub index: u32
}
pub struct VkBufferCollectionCreateInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub collectionToken: zx_handle_t
}
pub struct VkBufferCollectionPropertiesFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub memoryTypeBits: u32,
	pub bufferCount: u32,
	pub createInfoIndex: u32,
	pub sysmemPixelFormat: uint64_t,
	pub formatFeatures: VkFormatFeatureFlags,
	pub sysmemColorSpaceIndex: VkSysmemColorSpaceFUCHSIA,
	pub samplerYcbcrConversionComponents: VkComponentMapping,
	pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
	pub suggestedYcbcrRange: VkSamplerYcbcrRange,
	pub suggestedXChromaOffset: VkChromaLocation,
	pub suggestedYChromaOffset: VkChromaLocation
}
pub struct VkBufferConstraintsInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub createInfo: VkBufferCreateInfo,
	pub requiredFormatFeatures: VkFormatFeatureFlags,
	pub bufferCollectionConstraints: VkBufferCollectionConstraintsInfoFUCHSIA
}
pub struct VkSysmemColorSpaceFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub colorSpace: u32
}
pub struct VkImageFormatConstraintsInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub imageCreateInfo: VkImageCreateInfo,
	pub requiredFormatFeatures: VkFormatFeatureFlags,
	pub flags: VkImageFormatConstraintsFlagsFUCHSIA,
	pub sysmemPixelFormat: uint64_t,
	pub colorSpaceCount: u32,
	pub pColorSpaces: *const VkSysmemColorSpaceFUCHSIA
}
pub struct VkImageConstraintsInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub formatConstraintsCount: u32,
	pub pFormatConstraints: *const VkImageFormatConstraintsInfoFUCHSIA,
	pub bufferCollectionConstraints: VkBufferCollectionConstraintsInfoFUCHSIA,
	pub flags: VkImageConstraintsInfoFlagsFUCHSIA
}
pub struct VkBufferCollectionConstraintsInfoFUCHSIA
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub minBufferCount: u32,
	pub maxBufferCount: u32,
	pub minBufferCountForCamping: u32,
	pub minBufferCountForDedicatedSlack: u32,
	pub minBufferCountForSharedSlack: u32
}
pub struct VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub formatRgba10x6WithoutYCbCrSampler: VkBool32
}
pub struct VkFormatProperties3
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub linearTilingFeatures: VkFormatFeatureFlags2,
	pub optimalTilingFeatures: VkFormatFeatureFlags2,
	pub bufferFeatures: VkFormatFeatureFlags2
}
type VkFormatProperties3KHR = VkFormatProperties3;
pub struct VkDrmFormatModifierPropertiesList2EXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub drmFormatModifierCount: u32,
	pub pDrmFormatModifierProperties: *mut VkDrmFormatModifierProperties2EXT
}
pub struct VkDrmFormatModifierProperties2EXT
{
	pub drmFormatModifier: uint64_t,
	pub drmFormatModifierPlaneCount: u32,
	pub drmFormatModifierTilingFeatures: VkFormatFeatureFlags2
}
pub struct VkAndroidHardwareBufferFormatProperties2ANDROID
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub format: VkFormat,
	pub externalFormat: uint64_t,
	pub formatFeatures: VkFormatFeatureFlags2,
	pub samplerYcbcrConversionComponents: VkComponentMapping,
	pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
	pub suggestedYcbcrRange: VkSamplerYcbcrRange,
	pub suggestedXChromaOffset: VkChromaLocation,
	pub suggestedYChromaOffset: VkChromaLocation
}
pub struct VkPipelineRenderingCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub viewMask: u32,
	pub colorAttachmentCount: u32,
	pub pColorAttachmentFormats: *const VkFormat,
	pub depthAttachmentFormat: VkFormat,
	pub stencilAttachmentFormat: VkFormat
}
type VkPipelineRenderingCreateInfoKHR = VkPipelineRenderingCreateInfo;
pub struct VkRenderingInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkRenderingFlags,
	pub renderArea: VkRect2D,
	pub layerCount: u32,
	pub viewMask: u32,
	pub colorAttachmentCount: u32,
	pub pColorAttachments: *const VkRenderingAttachmentInfo,
	pub pDepthAttachment: *const VkRenderingAttachmentInfo,
	pub pStencilAttachment: *const VkRenderingAttachmentInfo
}
type VkRenderingInfoKHR = VkRenderingInfo;
pub struct VkRenderingAttachmentInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub imageView: VkImageView,
	pub imageLayout: VkImageLayout,
	pub resolveMode: VkResolveModeFlagBits,
	pub resolveImageView: VkImageView,
	pub resolveImageLayout: VkImageLayout,
	pub loadOp: VkAttachmentLoadOp,
	pub storeOp: VkAttachmentStoreOp,
	pub clearValue: VkClearValue
}
type VkRenderingAttachmentInfoKHR = VkRenderingAttachmentInfo;
pub struct VkRenderingFragmentShadingRateAttachmentInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub imageView: VkImageView,
	pub imageLayout: VkImageLayout,
	pub shadingRateAttachmentTexelSize: VkExtent2D
}
pub struct VkRenderingFragmentDensityMapAttachmentInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub imageView: VkImageView,
	pub imageLayout: VkImageLayout
}
pub struct VkPhysicalDeviceDynamicRenderingFeatures
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub dynamicRendering: VkBool32
}
type VkPhysicalDeviceDynamicRenderingFeaturesKHR = VkPhysicalDeviceDynamicRenderingFeatures;
pub struct VkCommandBufferInheritanceRenderingInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkRenderingFlags,
	pub viewMask: u32,
	pub colorAttachmentCount: u32,
	pub pColorAttachmentFormats: *const VkFormat,
	pub depthAttachmentFormat: VkFormat,
	pub stencilAttachmentFormat: VkFormat,
	pub rasterizationSamples: VkSampleCountFlagBits
}
type VkCommandBufferInheritanceRenderingInfoKHR = VkCommandBufferInheritanceRenderingInfo;
pub struct VkAttachmentSampleCountInfoAMD
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub colorAttachmentCount: u32,
	pub pColorAttachmentSamples: *const VkSampleCountFlagBits,
	pub depthStencilAttachmentSamples: VkSampleCountFlagBits
}
type VkAttachmentSampleCountInfoNV = VkAttachmentSampleCountInfoAMD;
pub struct VkMultiviewPerViewAttributesInfoNVX
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub perViewAttributes: VkBool32,
	pub perViewAttributesPositionXOnly: VkBool32
}
pub struct VkPhysicalDeviceImageViewMinLodFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub minLod: VkBool32
}
pub struct VkImageViewMinLodCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub minLod: f32
}
pub struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub rasterizationOrderColorAttachmentAccess: VkBool32,
	pub rasterizationOrderDepthAttachmentAccess: VkBool32,
	pub rasterizationOrderStencilAttachmentAccess: VkBool32
}
type VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM = VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT;
pub struct VkPhysicalDeviceLinearColorAttachmentFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub linearColorAttachment: VkBool32
}
pub struct VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub graphicsPipelineLibrary: VkBool32
}
pub struct VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub graphicsPipelineLibraryFastLinking: VkBool32,
	pub graphicsPipelineLibraryIndependentInterpolationDecoration: VkBool32
}
pub struct VkGraphicsPipelineLibraryCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub flags: VkGraphicsPipelineLibraryFlagsEXT
}
pub struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub descriptorSetHostMapping: VkBool32
}
pub struct VkDescriptorSetBindingReferenceVALVE
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub descriptorSetLayout: VkDescriptorSetLayout,
	pub binding: u32
}
pub struct VkDescriptorSetLayoutHostMappingInfoVALVE
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub descriptorOffset: usize,
	pub descriptorSize: u32
}
pub struct VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderModuleIdentifier: VkBool32
}
pub struct VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderModuleIdentifierAlgorithmUUID: [u8; VK_UUID_SIZE]
}
pub struct VkPipelineShaderStageModuleIdentifierCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub identifierSize: u32,
	pub pIdentifier: *const u8
}
pub struct VkShaderModuleIdentifierEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub identifierSize: u32,
	pub identifier: [u8; VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT]
}
pub struct VkImageCompressionControlEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkImageCompressionFlagsEXT,
	pub compressionControlPlaneCount: u32,
	pub pFixedRateFlags: *mut VkImageCompressionFixedRateFlagsEXT
}
pub struct VkPhysicalDeviceImageCompressionControlFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub imageCompressionControl: VkBool32
}
pub struct VkImageCompressionPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub imageCompressionFlags: VkImageCompressionFlagsEXT,
	pub imageCompressionFixedRateFlags: VkImageCompressionFixedRateFlagsEXT
}
pub struct VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub imageCompressionControlSwapchain: VkBool32
}
pub struct VkImageSubresource2EXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub imageSubresource: VkImageSubresource
}
pub struct VkSubresourceLayout2EXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub subresourceLayout: VkSubresourceLayout
}
pub struct VkRenderPassCreationControlEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub disallowMerging: VkBool32
}
pub struct VkRenderPassCreationFeedbackInfoEXT
{
	pub postMergeSubpassCount: u32
}
pub struct VkRenderPassCreationFeedbackCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pRenderPassFeedback: *mut VkRenderPassCreationFeedbackInfoEXT
}
pub struct VkRenderPassSubpassFeedbackInfoEXT
{
	pub subpassMergeStatus: VkSubpassMergeStatusEXT,
	pub description: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub postMergeIndex: u32
}
pub struct VkRenderPassSubpassFeedbackCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pSubpassFeedback: *mut VkRenderPassSubpassFeedbackInfoEXT
}
pub struct VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub subpassMergeFeedback: VkBool32
}
pub struct VkMicromapBuildInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub kind: VkMicromapTypeEXT,
	pub flags: VkBuildMicromapFlagsEXT,
	pub mode: VkBuildMicromapModeEXT,
	pub dstMicromap: VkMicromapEXT,
	pub usageCountsCount: u32,
	pub pUsageCounts: *const VkMicromapUsageEXT,
	pub ppUsageCounts: *const *const VkMicromapUsageEXT,
	pub data: VkDeviceOrHostAddressConstKHR,
	pub scratchData: VkDeviceOrHostAddressKHR,
	pub triangleArray: VkDeviceOrHostAddressConstKHR,
	pub triangleArrayStride: VkDeviceSize
}
pub struct VkMicromapCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub createFlags: VkMicromapCreateFlagsEXT,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub kind: VkMicromapTypeEXT,
	pub deviceAddress: VkDeviceAddress
}
pub struct VkMicromapVersionInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub pVersionData: *const u8
}
pub struct VkCopyMicromapInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub src: VkMicromapEXT,
	pub dst: VkMicromapEXT,
	pub mode: VkCopyMicromapModeEXT
}
pub struct VkCopyMicromapToMemoryInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub src: VkMicromapEXT,
	pub dst: VkDeviceOrHostAddressKHR,
	pub mode: VkCopyMicromapModeEXT
}
pub struct VkCopyMemoryToMicromapInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub src: VkDeviceOrHostAddressConstKHR,
	pub dst: VkMicromapEXT,
	pub mode: VkCopyMicromapModeEXT
}
pub struct VkMicromapBuildSizesInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub micromapSize: VkDeviceSize,
	pub buildScratchSize: VkDeviceSize,
	pub discardable: VkBool32
}
pub struct VkMicromapUsageEXT
{
	pub count: u32,
	pub subdivisionLevel: u32,
	pub format: u32
}
pub struct VkMicromapTriangleEXT
{
	pub dataOffset: u32,
	pub subdivisionLevel: u16,
	pub format: u16
}
pub struct VkPhysicalDeviceOpacityMicromapFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub micromap: VkBool32,
	pub micromapCaptureReplay: VkBool32,
	pub micromapHostCommands: VkBool32
}
pub struct VkPhysicalDeviceOpacityMicromapPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxOpacity2StateSubdivisionLevel: u32,
	pub maxOpacity4StateSubdivisionLevel: u32
}
pub struct VkAccelerationStructureTrianglesOpacityMicromapEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub indexType: VkIndexType,
	pub indexBuffer: VkDeviceOrHostAddressConstKHR,
	pub indexStride: VkDeviceSize,
	pub baseTriangle: u32,
	pub usageCountsCount: u32,
	pub pUsageCounts: *const VkMicromapUsageEXT,
	pub ppUsageCounts: *const *const VkMicromapUsageEXT,
	pub micromap: VkMicromapEXT
}
pub struct VkPipelinePropertiesIdentifierEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub pipelineIdentifier: [u8; VK_UUID_SIZE]
}
pub struct VkPhysicalDevicePipelinePropertiesFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub pipelinePropertiesIdentifier: VkBool32
}
pub struct VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderEarlyAndLateFragmentTests: VkBool32
}
pub struct VkExportMetalObjectCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub exportObjectType: VkExportMetalObjectTypeFlagBitsEXT
}
pub struct VkExportMetalObjectsInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void
}
pub struct VkExportMetalDeviceInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub mtlDevice: MTLDevice_id
}
pub struct VkExportMetalCommandQueueInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub queue: VkQueue,
	pub mtlCommandQueue: MTLCommandQueue_id
}
pub struct VkExportMetalBufferInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub memory: VkDeviceMemory,
	pub mtlBuffer: MTLBuffer_id
}
pub struct VkImportMetalBufferInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub mtlBuffer: MTLBuffer_id
}
pub struct VkExportMetalTextureInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub image: VkImage,
	pub imageView: VkImageView,
	pub bufferView: VkBufferView,
	pub plane: VkImageAspectFlagBits,
	pub mtlTexture: MTLTexture_id
}
pub struct VkImportMetalTextureInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub plane: VkImageAspectFlagBits,
	pub mtlTexture: MTLTexture_id
}
pub struct VkExportMetalIOSurfaceInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub image: VkImage,
	pub ioSurface: IOSurfaceRef
}
pub struct VkImportMetalIOSurfaceInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub ioSurface: IOSurfaceRef
}
pub struct VkExportMetalSharedEventInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub semaphore: VkSemaphore,
	pub event: VkEvent,
	pub mtlSharedEvent: MTLSharedEvent_id
}
pub struct VkImportMetalSharedEventInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub mtlSharedEvent: MTLSharedEvent_id
}
pub struct VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub nonSeamlessCubeMap: VkBool32
}
pub struct VkPhysicalDevicePipelineRobustnessFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub pipelineRobustness: VkBool32
}
pub struct VkPipelineRobustnessCreateInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub storageBuffers: VkPipelineRobustnessBufferBehaviorEXT,
	pub uniformBuffers: VkPipelineRobustnessBufferBehaviorEXT,
	pub vertexInputs: VkPipelineRobustnessBufferBehaviorEXT,
	pub images: VkPipelineRobustnessImageBehaviorEXT
}
pub struct VkPhysicalDevicePipelineRobustnessPropertiesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub defaultRobustnessStorageBuffers: VkPipelineRobustnessBufferBehaviorEXT,
	pub defaultRobustnessUniformBuffers: VkPipelineRobustnessBufferBehaviorEXT,
	pub defaultRobustnessVertexInputs: VkPipelineRobustnessBufferBehaviorEXT,
	pub defaultRobustnessImages: VkPipelineRobustnessImageBehaviorEXT
}
pub struct VkImageViewSampleWeightCreateInfoQCOM
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub filterCenter: VkOffset2D,
	pub filterSize: VkExtent2D,
	pub numPhases: u32
}
pub struct VkPhysicalDeviceImageProcessingFeaturesQCOM
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub textureSampleWeighted: VkBool32,
	pub textureBoxFilter: VkBool32,
	pub textureBlockMatch: VkBool32
}
pub struct VkPhysicalDeviceImageProcessingPropertiesQCOM
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub maxWeightFilterPhases: u32,
	pub maxWeightFilterDimension: VkExtent2D,
	pub maxBlockMatchRegion: VkExtent2D,
	pub maxBoxFilterBlockSize: VkExtent2D
}
pub struct VkPhysicalDeviceTilePropertiesFeaturesQCOM
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub tileProperties: VkBool32
}
pub struct VkTilePropertiesQCOM
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub tileSize: VkExtent3D,
	pub apronSize: VkExtent2D,
	pub origin: VkOffset2D
}
pub struct VkPhysicalDeviceAmigoProfilingFeaturesSEC
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub amigoProfiling: VkBool32
}
pub struct VkAmigoProfilingSubmitInfoSEC
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub firstDrawTimestamp: uint64_t,
	pub swapBufferTimestamp: uint64_t
}
pub struct VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub attachmentFeedbackLoopLayout: VkBool32
}
pub struct VkPhysicalDeviceDepthClampZeroOneFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub depthClampZeroOne: VkBool32
}
pub struct VkPhysicalDeviceAddressBindingReportFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub reportAddressBinding: VkBool32
}
pub struct VkDeviceAddressBindingCallbackDataEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub flags: VkDeviceAddressBindingFlagsEXT,
	pub baseAddress: VkDeviceAddress,
	pub size: VkDeviceSize,
	pub bindingType: VkDeviceAddressBindingTypeEXT
}
pub struct VkPhysicalDeviceOpticalFlowFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub opticalFlow: VkBool32
}
pub struct VkPhysicalDeviceOpticalFlowPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub supportedOutputGridSizes: VkOpticalFlowGridSizeFlagsNV,
	pub supportedHintGridSizes: VkOpticalFlowGridSizeFlagsNV,
	pub hintSupported: VkBool32,
	pub costSupported: VkBool32,
	pub bidirectionalFlowSupported: VkBool32,
	pub globalFlowSupported: VkBool32,
	pub minWidth: u32,
	pub minHeight: u32,
	pub maxWidth: u32,
	pub maxHeight: u32,
	pub maxNumRegionsOfInterest: u32
}
pub struct VkOpticalFlowImageFormatInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub usage: VkOpticalFlowUsageFlagsNV
}
pub struct VkOpticalFlowImageFormatPropertiesNV
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub format: VkFormat
}
pub struct VkOpticalFlowSessionCreateInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub width: u32,
	pub height: u32,
	pub imageFormat: VkFormat,
	pub flowVectorFormat: VkFormat,
	pub costFormat: VkFormat,
	pub outputGridSize: VkOpticalFlowGridSizeFlagsNV,
	pub hintGridSize: VkOpticalFlowGridSizeFlagsNV,
	pub performanceLevel: VkOpticalFlowPerformanceLevelNV,
	pub flags: VkOpticalFlowSessionCreateFlagsNV
}
pub struct VkOpticalFlowSessionCreatePrivateDataInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub id: u32,
	pub size: u32,
	pub pPrivateData: *const c_void
}
pub struct VkOpticalFlowExecuteInfoNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub flags: VkOpticalFlowExecuteFlagsNV,
	pub regionCount: u32,
	pub pRegions: *const VkRect2D
}
pub struct VkPhysicalDeviceFaultFeaturesEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub deviceFault: VkBool32,
	pub deviceFaultVendorBinary: VkBool32
}
pub struct VkDeviceFaultAddressInfoEXT
{
	pub addressType: VkDeviceFaultAddressTypeEXT,
	pub reportedAddress: VkDeviceAddress,
	pub addressPrecision: VkDeviceSize
}
pub struct VkDeviceFaultVendorInfoEXT
{
	pub description: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub vendorFaultCode: uint64_t,
	pub vendorFaultData: uint64_t
}
pub struct VkDeviceFaultCountsEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub addressInfoCount: u32,
	pub vendorInfoCount: u32,
	pub vendorBinarySize: VkDeviceSize
}
pub struct VkDeviceFaultInfoEXT
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub description: [i8; VK_MAX_DESCRIPTION_SIZE],
	pub pAddressInfos: *mut VkDeviceFaultAddressInfoEXT,
	pub pVendorInfos: *mut VkDeviceFaultVendorInfoEXT,
	pub pVendorBinaryData: *mut c_void
}
pub struct VkDeviceFaultVendorBinaryHeaderVersionOneEXT
{
	pub headerSize: u32,
	pub headerVersion: VkDeviceFaultVendorBinaryHeaderVersionEXT,
	pub vendorID: u32,
	pub deviceID: u32,
	pub driverVersion: u32,
	pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
	pub applicationNameOffset: u32,
	pub applicationVersion: u32,
	pub engineNameOffset: u32
}
pub struct VkDecompressMemoryRegionNV
{
	pub srcAddress: VkDeviceAddress,
	pub dstAddress: VkDeviceAddress,
	pub compressedSize: VkDeviceSize,
	pub decompressedSize: VkDeviceSize,
	pub decompressionMethod: VkMemoryDecompressionMethodFlagsNV
}
pub struct VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderCoreMask: uint64_t,
	pub shaderCoreCount: u32,
	pub shaderWarpsPerCore: u32
}
pub struct VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub shaderCoreBuiltins: VkBool32
}
pub struct VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV
{
	pub sType: VkStructureType,
	pub pNext: *mut c_void,
	pub rayTracingInvocationReorder: VkBool32
}
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
	pCreateInfo: *mut VkInstanceCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pInstance: *mut VkInstance
) -> VkResult;

pub type PFN_vkDestroyInstance = unsafe extern "system" fn(
	instance: VkInstance,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(
	instance: VkInstance,
	pPhysicalDeviceCount: *mut u32,
	pPhysicalDevices: *mut VkPhysicalDevice
) -> VkResult;

pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(
	device: VkDevice,
	pName: *mut char
) -> VkResult;

pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(
	instance: VkInstance,
	pName: *mut char
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pProperties: *mut VkPhysicalDeviceProperties
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pQueueFamilyPropertyCount: *mut u32,
	pQueueFamilyProperties: *mut VkQueueFamilyProperties
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pFeatures: *mut VkPhysicalDeviceFeatures
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	pFormatProperties: *mut VkFormatProperties
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	imageType: VkImageType,
	tiling: VkImageTiling,
	usage: VkImageUsageFlags,
	flags: VkImageCreateFlags,
	pImageFormatProperties: *mut VkImageFormatProperties
) -> VkResult;

pub type PFN_vkCreateDevice = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pCreateInfo: *mut VkDeviceCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pDevice: *mut VkDevice
) -> VkResult;

pub type PFN_vkDestroyDevice = unsafe extern "system" fn(
	device: VkDevice,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkEnumerateInstanceVersion = unsafe extern "system" fn(
	pApiVersion: *mut u32
) -> VkResult;

pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(
	pPropertyCount: *mut u32,
	pProperties: *mut VkLayerProperties
) -> VkResult;

pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(
	pLayerName: *mut char,
	pPropertyCount: *mut u32,
	pProperties: *mut VkExtensionProperties
) -> VkResult;

pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *mut u32,
	pProperties: *mut VkLayerProperties
) -> VkResult;

pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pLayerName: *mut char,
	pPropertyCount: *mut u32,
	pProperties: *mut VkExtensionProperties
) -> VkResult;

pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(
	device: VkDevice,
	queueFamilyIndex: u32,
	queueIndex: u32,
	pQueue: *mut VkQueue
) -> VkResult;

pub type PFN_vkQueueSubmit = unsafe extern "system" fn(
	queue: VkQueue,
	submitCount: u32,
	pSubmits: *mut VkSubmitInfo,
	fence: VkFence
) -> VkResult;

pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(
	queue: VkQueue
) -> VkResult;

pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(
	device: VkDevice
) -> VkResult;

pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
	device: VkDevice,
	pAllocateInfo: *mut VkMemoryAllocateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pMemory: *mut VkDeviceMemory
) -> VkResult;

pub type PFN_vkFreeMemory = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkMapMemory = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory,
	offset: VkDeviceSize,
	size: VkDeviceSize,
	flags: VkMemoryMapFlags,
	ppData: *mut *mut c_void
) -> VkResult;

pub type PFN_vkUnmapMemory = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory
) -> VkResult;

pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
	device: VkDevice,
	memoryRangeCount: u32,
	pMemoryRanges: *mut VkMappedMemoryRange
) -> VkResult;

pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
	device: VkDevice,
	memoryRangeCount: u32,
	pMemoryRanges: *mut VkMappedMemoryRange
) -> VkResult;

pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory,
	pCommittedMemoryInBytes: *mut VkDeviceSize
) -> VkResult;

pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	buffer: VkBuffer,
	pMemoryRequirements: *mut VkMemoryRequirements
) -> VkResult;

pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(
	device: VkDevice,
	buffer: VkBuffer,
	memory: VkDeviceMemory,
	memoryOffset: VkDeviceSize
) -> VkResult;

pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pMemoryRequirements: *mut VkMemoryRequirements
) -> VkResult;

pub type PFN_vkBindImageMemory = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	memory: VkDeviceMemory,
	memoryOffset: VkDeviceSize
) -> VkResult;

pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pSparseMemoryRequirementCount: *mut u32,
	pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	imageType: VkImageType,
	samples: VkSampleCountFlagBits,
	usage: VkImageUsageFlags,
	tiling: VkImageTiling,
	pPropertyCount: *mut u32,
	pProperties: *mut VkSparseImageFormatProperties
) -> VkResult;

pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(
	queue: VkQueue,
	bindInfoCount: u32,
	pBindInfo: *mut VkBindSparseInfo,
	fence: VkFence
) -> VkResult;

pub type PFN_vkCreateFence = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkFenceCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pFence: *mut VkFence
) -> VkResult;

pub type PFN_vkDestroyFence = unsafe extern "system" fn(
	device: VkDevice,
	fence: VkFence,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkResetFences = unsafe extern "system" fn(
	device: VkDevice,
	fenceCount: u32,
	pFences: *mut VkFence
) -> VkResult;

pub type PFN_vkGetFenceStatus = unsafe extern "system" fn(
	device: VkDevice,
	fence: VkFence
) -> VkResult;

pub type PFN_vkWaitForFences = unsafe extern "system" fn(
	device: VkDevice,
	fenceCount: u32,
	pFences: *mut VkFence,
	waitAll: VkBool32,
	timeout: uint64_t
) -> VkResult;

pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkSemaphoreCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pSemaphore: *mut VkSemaphore
) -> VkResult;

pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(
	device: VkDevice,
	semaphore: VkSemaphore,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreateEvent = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkEventCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pEvent: *mut VkEvent
) -> VkResult;

pub type PFN_vkDestroyEvent = unsafe extern "system" fn(
	device: VkDevice,
	event: VkEvent,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetEventStatus = unsafe extern "system" fn(
	device: VkDevice,
	event: VkEvent
) -> VkResult;

pub type PFN_vkSetEvent = unsafe extern "system" fn(
	device: VkDevice,
	event: VkEvent
) -> VkResult;

pub type PFN_vkResetEvent = unsafe extern "system" fn(
	device: VkDevice,
	event: VkEvent
) -> VkResult;

pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkQueryPoolCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pQueryPool: *mut VkQueryPool
) -> VkResult;

pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(
	device: VkDevice,
	queryPool: VkQueryPool,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(
	device: VkDevice,
	queryPool: VkQueryPool,
	firstQuery: u32,
	queryCount: u32,
	dataSize: size_t,
	pData: *mut c_void,
	stride: VkDeviceSize,
	flags: VkQueryResultFlags
) -> VkResult;

pub type PFN_vkResetQueryPool = unsafe extern "system" fn(
	device: VkDevice,
	queryPool: VkQueryPool,
	firstQuery: u32,
	queryCount: u32
) -> VkResult;

pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkBufferCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pBuffer: *mut VkBuffer
) -> VkResult;

pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
	device: VkDevice,
	buffer: VkBuffer,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreateBufferView = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkBufferViewCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pView: *mut VkBufferView
) -> VkResult;

pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(
	device: VkDevice,
	bufferView: VkBufferView,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreateImage = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkImageCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pImage: *mut VkImage
) -> VkResult;

pub type PFN_vkDestroyImage = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pSubresource: *mut VkImageSubresource,
	pLayout: *mut VkSubresourceLayout
) -> VkResult;

pub type PFN_vkCreateImageView = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkImageViewCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pView: *mut VkImageView
) -> VkResult;

pub type PFN_vkDestroyImageView = unsafe extern "system" fn(
	device: VkDevice,
	imageView: VkImageView,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkShaderModuleCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pShaderModule: *mut VkShaderModule
) -> VkResult;

pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(
	device: VkDevice,
	shaderModule: VkShaderModule,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkPipelineCacheCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pPipelineCache: *mut VkPipelineCache
) -> VkResult;

pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	pDataSize: *mut size_t,
	pData: *mut c_void
) -> VkResult;

pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(
	device: VkDevice,
	dstCache: VkPipelineCache,
	srcCacheCount: u32,
	pSrcCaches: *mut VkPipelineCache
) -> VkResult;

pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *mut VkGraphicsPipelineCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pPipelines: *mut VkPipeline
) -> VkResult;

pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *mut VkComputePipelineCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pPipelines: *mut VkPipeline
) -> VkResult;

pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = unsafe extern "system" fn(
	device: VkDevice,
	renderpass: VkRenderPass,
	pMaxWorkgroupSize: *mut VkExtent2D
) -> VkResult;

pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkPipelineLayoutCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pPipelineLayout: *mut VkPipelineLayout
) -> VkResult;

pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(
	device: VkDevice,
	pipelineLayout: VkPipelineLayout,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreateSampler = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkSamplerCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pSampler: *mut VkSampler
) -> VkResult;

pub type PFN_vkDestroySampler = unsafe extern "system" fn(
	device: VkDevice,
	sampler: VkSampler,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkDescriptorSetLayoutCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pSetLayout: *mut VkDescriptorSetLayout
) -> VkResult;

pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(
	device: VkDevice,
	descriptorSetLayout: VkDescriptorSetLayout,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkDescriptorPoolCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pDescriptorPool: *mut VkDescriptorPool
) -> VkResult;

pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(
	device: VkDevice,
	descriptorPool: VkDescriptorPool,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(
	device: VkDevice,
	descriptorPool: VkDescriptorPool,
	flags: VkDescriptorPoolResetFlags
) -> VkResult;

pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(
	device: VkDevice,
	pAllocateInfo: *mut VkDescriptorSetAllocateInfo,
	pDescriptorSets: *mut VkDescriptorSet
) -> VkResult;

pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(
	device: VkDevice,
	descriptorPool: VkDescriptorPool,
	descriptorSetCount: u32,
	pDescriptorSets: *mut VkDescriptorSet
) -> VkResult;

pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(
	device: VkDevice,
	descriptorWriteCount: u32,
	pDescriptorWrites: *mut VkWriteDescriptorSet,
	descriptorCopyCount: u32,
	pDescriptorCopies: *mut VkCopyDescriptorSet
) -> VkResult;

pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkFramebufferCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pFramebuffer: *mut VkFramebuffer
) -> VkResult;

pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(
	device: VkDevice,
	framebuffer: VkFramebuffer,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkRenderPassCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pRenderPass: *mut VkRenderPass
) -> VkResult;

pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(
	device: VkDevice,
	renderPass: VkRenderPass,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(
	device: VkDevice,
	renderPass: VkRenderPass,
	pGranularity: *mut VkExtent2D
) -> VkResult;

pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkCommandPoolCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pCommandPool: *mut VkCommandPool
) -> VkResult;

pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(
	device: VkDevice,
	commandPool: VkCommandPool,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkResetCommandPool = unsafe extern "system" fn(
	device: VkDevice,
	commandPool: VkCommandPool,
	flags: VkCommandPoolResetFlags
) -> VkResult;

pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
	device: VkDevice,
	pAllocateInfo: *mut VkCommandBufferAllocateInfo,
	pCommandBuffers: *mut VkCommandBuffer
) -> VkResult;

pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(
	device: VkDevice,
	commandPool: VkCommandPool,
	commandBufferCount: u32,
	pCommandBuffers: *mut VkCommandBuffer
) -> VkResult;

pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pBeginInfo: *mut VkCommandBufferBeginInfo
) -> VkResult;

pub type PFN_vkEndCommandBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	flags: VkCommandBufferResetFlags
) -> VkResult;

pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineBindPoint: VkPipelineBindPoint,
	pipeline: VkPipeline
) -> VkResult;

pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstViewport: u32,
	viewportCount: u32,
	pViewports: *mut VkViewport
) -> VkResult;

pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstScissor: u32,
	scissorCount: u32,
	pScissors: *mut VkRect2D
) -> VkResult;

pub type PFN_vkCmdSetLineWidth = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	lineWidth: float
) -> VkResult;

pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthBiasConstantFactor: float,
	depthBiasClamp: float,
	depthBiasSlopeFactor: float
) -> VkResult;

pub type PFN_vkCmdSetBlendConstants = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	blendConstants: [float; 4]
) -> VkResult;

pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	minDepthBounds: float,
	maxDepthBounds: float
) -> VkResult;

pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	faceMask: VkStencilFaceFlags,
	compareMask: u32
) -> VkResult;

pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	faceMask: VkStencilFaceFlags,
	writeMask: u32
) -> VkResult;

pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	faceMask: VkStencilFaceFlags,
	reference: u32
) -> VkResult;

pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineBindPoint: VkPipelineBindPoint,
	layout: VkPipelineLayout,
	firstSet: u32,
	descriptorSetCount: u32,
	pDescriptorSets: *mut VkDescriptorSet,
	dynamicOffsetCount: u32,
	pDynamicOffsets: *mut u32
) -> VkResult;

pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	indexType: VkIndexType
) -> VkResult;

pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstBinding: u32,
	bindingCount: u32,
	pBuffers: *mut VkBuffer,
	pOffsets: *mut VkDeviceSize
) -> VkResult;

pub type PFN_vkCmdDraw = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	vertexCount: u32,
	instanceCount: u32,
	firstVertex: u32,
	firstInstance: u32
) -> VkResult;

pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	indexCount: u32,
	instanceCount: u32,
	firstIndex: u32,
	vertexOffset: int32_t,
	firstInstance: u32
) -> VkResult;

pub type PFN_vkCmdDrawMultiEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	drawCount: u32,
	pVertexInfo: *mut VkMultiDrawInfoEXT,
	instanceCount: u32,
	firstInstance: u32,
	stride: u32
) -> VkResult;

pub type PFN_vkCmdDrawMultiIndexedEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	drawCount: u32,
	pIndexInfo: *mut VkMultiDrawIndexedInfoEXT,
	instanceCount: u32,
	firstInstance: u32,
	stride: u32,
	pVertexOffset: *mut int32_t
) -> VkResult;

pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	drawCount: u32,
	stride: u32
) -> VkResult;

pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	drawCount: u32,
	stride: u32
) -> VkResult;

pub type PFN_vkCmdDispatch = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	groupCountX: u32,
	groupCountY: u32,
	groupCountZ: u32
) -> VkResult;

pub type PFN_vkCmdDispatchIndirect = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize
) -> VkResult;

pub type PFN_vkCmdSubpassShadingHUAWEI = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcBuffer: VkBuffer,
	dstBuffer: VkBuffer,
	regionCount: u32,
	pRegions: *mut VkBufferCopy
) -> VkResult;

pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *mut VkImageCopy
) -> VkResult;

pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *mut VkImageBlit,
	filter: VkFilter
) -> VkResult;

pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcBuffer: VkBuffer,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *mut VkBufferImageCopy
) -> VkResult;

pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstBuffer: VkBuffer,
	regionCount: u32,
	pRegions: *mut VkBufferImageCopy
) -> VkResult;

pub type PFN_vkCmdCopyMemoryIndirectNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	copyBufferAddress: VkDeviceAddress,
	copyCount: u32,
	stride: u32
) -> VkResult;

pub type PFN_vkCmdCopyMemoryToImageIndirectNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	copyBufferAddress: VkDeviceAddress,
	copyCount: u32,
	stride: u32,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	pImageSubresources: *mut VkImageSubresourceLayers
) -> VkResult;

pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	dstBuffer: VkBuffer,
	dstOffset: VkDeviceSize,
	dataSize: VkDeviceSize,
	pData: *mut c_void
) -> VkResult;

pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	dstBuffer: VkBuffer,
	dstOffset: VkDeviceSize,
	size: VkDeviceSize,
	data: u32
) -> VkResult;

pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	image: VkImage,
	imageLayout: VkImageLayout,
	pColor: *mut VkClearColorValue,
	rangeCount: u32,
	pRanges: *mut VkImageSubresourceRange
) -> VkResult;

pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	image: VkImage,
	imageLayout: VkImageLayout,
	pDepthStencil: *mut VkClearDepthStencilValue,
	rangeCount: u32,
	pRanges: *mut VkImageSubresourceRange
) -> VkResult;

pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	attachmentCount: u32,
	pAttachments: *mut VkClearAttachment,
	rectCount: u32,
	pRects: *mut VkClearRect
) -> VkResult;

pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *mut VkImageResolve
) -> VkResult;

pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	event: VkEvent,
	stageMask: VkPipelineStageFlags
) -> VkResult;

pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	event: VkEvent,
	stageMask: VkPipelineStageFlags
) -> VkResult;

pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	eventCount: u32,
	pEvents: *mut VkEvent,
	srcStageMask: VkPipelineStageFlags,
	dstStageMask: VkPipelineStageFlags,
	memoryBarrierCount: u32,
	pMemoryBarriers: *mut VkMemoryBarrier,
	bufferMemoryBarrierCount: u32,
	pBufferMemoryBarriers: *mut VkBufferMemoryBarrier,
	imageMemoryBarrierCount: u32,
	pImageMemoryBarriers: *mut VkImageMemoryBarrier
) -> VkResult;

pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcStageMask: VkPipelineStageFlags,
	dstStageMask: VkPipelineStageFlags,
	dependencyFlags: VkDependencyFlags,
	memoryBarrierCount: u32,
	pMemoryBarriers: *mut VkMemoryBarrier,
	bufferMemoryBarrierCount: u32,
	pBufferMemoryBarriers: *mut VkBufferMemoryBarrier,
	imageMemoryBarrierCount: u32,
	pImageMemoryBarriers: *mut VkImageMemoryBarrier
) -> VkResult;

pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	query: u32,
	flags: VkQueryControlFlags
) -> VkResult;

pub type PFN_vkCmdEndQuery = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	query: u32
) -> VkResult;

pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pConditionalRenderingBegin: *mut VkConditionalRenderingBeginInfoEXT
) -> VkResult;

pub type PFN_vkCmdEndConditionalRenderingEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	firstQuery: u32,
	queryCount: u32
) -> VkResult;

pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineStage: VkPipelineStageFlagBits,
	queryPool: VkQueryPool,
	query: u32
) -> VkResult;

pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	firstQuery: u32,
	queryCount: u32,
	dstBuffer: VkBuffer,
	dstOffset: VkDeviceSize,
	stride: VkDeviceSize,
	flags: VkQueryResultFlags
) -> VkResult;

pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	layout: VkPipelineLayout,
	stageFlags: VkShaderStageFlags,
	offset: u32,
	size: u32,
	pValues: *mut c_void
) -> VkResult;

pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pRenderPassBegin: *mut VkRenderPassBeginInfo,
	contents: VkSubpassContents
) -> VkResult;

pub type PFN_vkCmdNextSubpass = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	contents: VkSubpassContents
) -> VkResult;

pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	commandBufferCount: u32,
	pCommandBuffers: *mut VkCommandBuffer
) -> VkResult;

pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkAndroidSurfaceCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *mut u32,
	pProperties: *mut VkDisplayPropertiesKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *mut u32,
	pProperties: *mut VkDisplayPlanePropertiesKHR
) -> VkResult;

pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	planeIndex: u32,
	pDisplayCount: *mut u32,
	pDisplays: *mut VkDisplayKHR
) -> VkResult;

pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR,
	pPropertyCount: *mut u32,
	pProperties: *mut VkDisplayModePropertiesKHR
) -> VkResult;

pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR,
	pCreateInfo: *mut VkDisplayModeCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pMode: *mut VkDisplayModeKHR
) -> VkResult;

pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	mode: VkDisplayModeKHR,
	planeIndex: u32,
	pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR
) -> VkResult;

pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkDisplaySurfaceCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchainCount: u32,
	pCreateInfos: *mut VkSwapchainCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pSwapchains: *mut VkSwapchainKHR
) -> VkResult;

pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	surface: VkSurfaceKHR,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	surface: VkSurfaceKHR,
	pSupported: *mut VkBool32
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	surface: VkSurfaceKHR,
	pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	surface: VkSurfaceKHR,
	pSurfaceFormatCount: *mut u32,
	pSurfaceFormats: *mut VkSurfaceFormatKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	surface: VkSurfaceKHR,
	pPresentModeCount: *mut u32,
	pPresentModes: *mut VkPresentModeKHR
) -> VkResult;

pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkSwapchainCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pSwapchain: *mut VkSwapchainKHR
) -> VkResult;

pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	pSwapchainImageCount: *mut u32,
	pSwapchainImages: *mut VkImage
) -> VkResult;

pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	timeout: uint64_t,
	semaphore: VkSemaphore,
	fence: VkFence,
	pImageIndex: *mut u32
) -> VkResult;

pub type PFN_vkQueuePresentKHR = unsafe extern "system" fn(
	queue: VkQueue,
	pPresentInfo: *mut VkPresentInfoKHR
) -> VkResult;

pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkViSurfaceCreateInfoNN,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkWaylandSurfaceCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	display: *mut wl_display
) -> VkResult;

pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkWin32SurfaceCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32
) -> VkResult;

pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkXlibSurfaceCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	dpy: *mut Display,
	visualID: VisualID
) -> VkResult;

pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkXcbSurfaceCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	connection: *mut xcb_connection_t,
	visual_id: xcb_visualid_t
) -> VkResult;

pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkDirectFBSurfaceCreateInfoEXT,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	dfb: *mut IDirectFB
) -> VkResult;

pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkImagePipeSurfaceCreateInfoFUCHSIA,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkStreamDescriptorSurfaceCreateInfoGGP,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkScreenSurfaceCreateInfoQNX,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	window: *mut _screen_window
) -> VkResult;

pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkDebugReportCallbackCreateInfoEXT,
	pAllocator: *mut VkAllocationCallbacks,
	pCallback: *mut VkDebugReportCallbackEXT
) -> VkResult;

pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(
	instance: VkInstance,
	callback: VkDebugReportCallbackEXT,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(
	instance: VkInstance,
	flags: VkDebugReportFlagsEXT,
	objectType: VkDebugReportObjectTypeEXT,
	object: uint64_t,
	location: size_t,
	messageCode: int32_t,
	pLayerPrefix: *mut char,
	pMessage: *mut char
) -> VkResult;

pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
	device: VkDevice,
	pNameInfo: *mut VkDebugMarkerObjectNameInfoEXT
) -> VkResult;

pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
	device: VkDevice,
	pTagInfo: *mut VkDebugMarkerObjectTagInfoEXT
) -> VkResult;

pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pMarkerInfo: *mut VkDebugMarkerMarkerInfoEXT
) -> VkResult;

pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pMarkerInfo: *mut VkDebugMarkerMarkerInfoEXT
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	imageType: VkImageType,
	tiling: VkImageTiling,
	usage: VkImageUsageFlags,
	flags: VkImageCreateFlags,
	externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
	pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV
) -> VkResult;

pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory,
	handleType: VkExternalMemoryHandleTypeFlagsNV,
	pHandle: *mut HANDLE
) -> VkResult;

pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	isPreprocessed: VkBool32,
	pGeneratedCommandsInfo: *mut VkGeneratedCommandsInfoNV
) -> VkResult;

pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pGeneratedCommandsInfo: *mut VkGeneratedCommandsInfoNV
) -> VkResult;

pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineBindPoint: VkPipelineBindPoint,
	pipeline: VkPipeline,
	groupIndex: u32
) -> VkResult;

pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkGeneratedCommandsMemoryRequirementsInfoNV,
	pMemoryRequirements: *mut VkMemoryRequirements2
) -> VkResult;

pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkIndirectCommandsLayoutCreateInfoNV,
	pAllocator: *mut VkAllocationCallbacks,
	pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNV
) -> VkResult;

pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
	device: VkDevice,
	indirectCommandsLayout: VkIndirectCommandsLayoutNV,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pFeatures: *mut VkPhysicalDeviceFeatures2
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pProperties: *mut VkPhysicalDeviceProperties2
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	pFormatProperties: *mut VkFormatProperties2
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pImageFormatInfo: *mut VkPhysicalDeviceImageFormatInfo2,
	pImageFormatProperties: *mut VkImageFormatProperties2
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pQueueFamilyPropertyCount: *mut u32,
	pQueueFamilyProperties: *mut VkQueueFamilyProperties2
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pFormatInfo: *mut VkPhysicalDeviceSparseImageFormatInfo2,
	pPropertyCount: *mut u32,
	pProperties: *mut VkSparseImageFormatProperties2
) -> VkResult;

pub type PFN_vkCmdPushDescriptorSetKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineBindPoint: VkPipelineBindPoint,
	layout: VkPipelineLayout,
	set: u32,
	descriptorWriteCount: u32,
	pDescriptorWrites: *mut VkWriteDescriptorSet
) -> VkResult;

pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
	device: VkDevice,
	commandPool: VkCommandPool,
	flags: VkCommandPoolTrimFlags
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pExternalBufferInfo: *mut VkPhysicalDeviceExternalBufferInfo,
	pExternalBufferProperties: *mut VkExternalBufferProperties
) -> VkResult;

pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetWin32HandleInfo: *mut VkMemoryGetWin32HandleInfoKHR,
	pHandle: *mut HANDLE
) -> VkResult;

pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(
	device: VkDevice,
	handleType: VkExternalMemoryHandleTypeFlagBits,
	handle: HANDLE,
	pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR
) -> VkResult;

pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetFdInfo: *mut VkMemoryGetFdInfoKHR,
	pFd: *mut int
) -> VkResult;

pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
	device: VkDevice,
	handleType: VkExternalMemoryHandleTypeFlagBits,
	fd: int,
	pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR
) -> VkResult;

pub type PFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	pGetZirconHandleInfo: *mut VkMemoryGetZirconHandleInfoFUCHSIA,
	pZirconHandle: *mut zx_handle_t
) -> VkResult;

pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	handleType: VkExternalMemoryHandleTypeFlagBits,
	zirconHandle: zx_handle_t,
	pMemoryZirconHandleProperties: *mut VkMemoryZirconHandlePropertiesFUCHSIA
) -> VkResult;

pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
	device: VkDevice,
	pMemoryGetRemoteAddressInfo: *mut VkMemoryGetRemoteAddressInfoNV,
	pAddress: *mut VkRemoteAddressNV
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pExternalSemaphoreInfo: *mut VkPhysicalDeviceExternalSemaphoreInfo,
	pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties
) -> VkResult;

pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetWin32HandleInfo: *mut VkSemaphoreGetWin32HandleInfoKHR,
	pHandle: *mut HANDLE
) -> VkResult;

pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(
	device: VkDevice,
	pImportSemaphoreWin32HandleInfo: *mut VkImportSemaphoreWin32HandleInfoKHR
) -> VkResult;

pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetFdInfo: *mut VkSemaphoreGetFdInfoKHR,
	pFd: *mut int
) -> VkResult;

pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn(
	device: VkDevice,
	pImportSemaphoreFdInfo: *mut VkImportSemaphoreFdInfoKHR
) -> VkResult;

pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	pGetZirconHandleInfo: *mut VkSemaphoreGetZirconHandleInfoFUCHSIA,
	pZirconHandle: *mut zx_handle_t
) -> VkResult;

pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	pImportSemaphoreZirconHandleInfo: *mut VkImportSemaphoreZirconHandleInfoFUCHSIA
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pExternalFenceInfo: *mut VkPhysicalDeviceExternalFenceInfo,
	pExternalFenceProperties: *mut VkExternalFenceProperties
) -> VkResult;

pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetWin32HandleInfo: *mut VkFenceGetWin32HandleInfoKHR,
	pHandle: *mut HANDLE
) -> VkResult;

pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
	device: VkDevice,
	pImportFenceWin32HandleInfo: *mut VkImportFenceWin32HandleInfoKHR
) -> VkResult;

pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetFdInfo: *mut VkFenceGetFdInfoKHR,
	pFd: *mut int
) -> VkResult;

pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(
	device: VkDevice,
	pImportFenceFdInfo: *mut VkImportFenceFdInfoKHR
) -> VkResult;

pub type PFN_vkReleaseDisplayEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR
) -> VkResult;

pub type PFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	dpy: *mut Display,
	display: VkDisplayKHR
) -> VkResult;

pub type PFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	dpy: *mut Display,
	rrOutput: RROutput,
	pDisplay: *mut VkDisplayKHR
) -> VkResult;

pub type PFN_vkAcquireWinrtDisplayNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR
) -> VkResult;

pub type PFN_vkGetWinrtDisplayNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	deviceRelativeId: u32,
	pDisplay: *mut VkDisplayKHR
) -> VkResult;

pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(
	device: VkDevice,
	display: VkDisplayKHR,
	pDisplayPowerInfo: *mut VkDisplayPowerInfoEXT
) -> VkResult;

pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
	device: VkDevice,
	pDeviceEventInfo: *mut VkDeviceEventInfoEXT,
	pAllocator: *mut VkAllocationCallbacks,
	pFence: *mut VkFence
) -> VkResult;

pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
	device: VkDevice,
	display: VkDisplayKHR,
	pDisplayEventInfo: *mut VkDisplayEventInfoEXT,
	pAllocator: *mut VkAllocationCallbacks,
	pFence: *mut VkFence
) -> VkResult;

pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	counter: VkSurfaceCounterFlagBitsEXT,
	pCounterValue: *mut uint64_t
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	surface: VkSurfaceKHR,
	pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT
) -> VkResult;

pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
	instance: VkInstance,
	pPhysicalDeviceGroupCount: *mut u32,
	pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties
) -> VkResult;

pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
	device: VkDevice,
	heapIndex: u32,
	localDeviceIndex: u32,
	remoteDeviceIndex: u32,
	pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags
) -> VkResult;

pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
	device: VkDevice,
	bindInfoCount: u32,
	pBindInfos: *mut VkBindBufferMemoryInfo
) -> VkResult;

pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
	device: VkDevice,
	bindInfoCount: u32,
	pBindInfos: *mut VkBindImageMemoryInfo
) -> VkResult;

pub type PFN_vkCmdSetDeviceMask = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	deviceMask: u32
) -> VkResult;

pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(
	device: VkDevice,
	pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR
) -> VkResult;

pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(
	device: VkDevice,
	surface: VkSurfaceKHR,
	pModes: *mut VkDeviceGroupPresentModeFlagsKHR
) -> VkResult;

pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(
	device: VkDevice,
	pAcquireInfo: *mut VkAcquireNextImageInfoKHR,
	pImageIndex: *mut u32
) -> VkResult;

pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	baseGroupX: u32,
	baseGroupY: u32,
	baseGroupZ: u32,
	groupCountX: u32,
	groupCountY: u32,
	groupCountZ: u32
) -> VkResult;

pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	surface: VkSurfaceKHR,
	pRectCount: *mut u32,
	pRects: *mut VkRect2D
) -> VkResult;

pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkDescriptorUpdateTemplateCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate
) -> VkResult;

pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
	device: VkDevice,
	descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
	device: VkDevice,
	descriptorSet: VkDescriptorSet,
	descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
	pData: *mut c_void
) -> VkResult;

pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
	layout: VkPipelineLayout,
	set: u32,
	pData: *mut c_void
) -> VkResult;

pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
	device: VkDevice,
	swapchainCount: u32,
	pSwapchains: *mut VkSwapchainKHR,
	pMetadata: *mut VkHdrMetadataEXT
) -> VkResult;

pub type PFN_vkGetSwapchainStatusKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR
) -> VkResult;

pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE
) -> VkResult;

pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	pPresentationTimingCount: *mut u32,
	pPresentationTimings: *mut VkPastPresentationTimingGOOGLE
) -> VkResult;

pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkIOSSurfaceCreateInfoMVK,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkMacOSSurfaceCreateInfoMVK,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkMetalSurfaceCreateInfoEXT,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstViewport: u32,
	viewportCount: u32,
	pViewportWScalings: *mut VkViewportWScalingNV
) -> VkResult;

pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstDiscardRectangle: u32,
	discardRectangleCount: u32,
	pDiscardRectangles: *mut VkRect2D
) -> VkResult;

pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pSampleLocationsInfo: *mut VkSampleLocationsInfoEXT
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	samples: VkSampleCountFlagBits,
	pMultisampleProperties: *mut VkMultisamplePropertiesEXT
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pSurfaceInfo: *mut VkPhysicalDeviceSurfaceInfo2KHR,
	pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pSurfaceInfo: *mut VkPhysicalDeviceSurfaceInfo2KHR,
	pSurfaceFormatCount: *mut u32,
	pSurfaceFormats: *mut VkSurfaceFormat2KHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *mut u32,
	pProperties: *mut VkDisplayProperties2KHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *mut u32,
	pProperties: *mut VkDisplayPlaneProperties2KHR
) -> VkResult;

pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR,
	pPropertyCount: *mut u32,
	pProperties: *mut VkDisplayModeProperties2KHR
) -> VkResult;

pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pDisplayPlaneInfo: *mut VkDisplayPlaneInfo2KHR,
	pCapabilities: *mut VkDisplayPlaneCapabilities2KHR
) -> VkResult;

pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkBufferMemoryRequirementsInfo2,
	pMemoryRequirements: *mut VkMemoryRequirements2
) -> VkResult;

pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkImageMemoryRequirementsInfo2,
	pMemoryRequirements: *mut VkMemoryRequirements2
) -> VkResult;

pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkImageSparseMemoryRequirementsInfo2,
	pSparseMemoryRequirementCount: *mut u32,
	pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2
) -> VkResult;

pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkDeviceBufferMemoryRequirements,
	pMemoryRequirements: *mut VkMemoryRequirements2
) -> VkResult;

pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkDeviceImageMemoryRequirements,
	pMemoryRequirements: *mut VkMemoryRequirements2
) -> VkResult;

pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkDeviceImageMemoryRequirements,
	pSparseMemoryRequirementCount: *mut u32,
	pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2
) -> VkResult;

pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkSamplerYcbcrConversionCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pYcbcrConversion: *mut VkSamplerYcbcrConversion
) -> VkResult;

pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
	device: VkDevice,
	ycbcrConversion: VkSamplerYcbcrConversion,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
	device: VkDevice,
	pQueueInfo: *mut VkDeviceQueueInfo2,
	pQueue: *mut VkQueue
) -> VkResult;

pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkValidationCacheCreateInfoEXT,
	pAllocator: *mut VkAllocationCallbacks,
	pValidationCache: *mut VkValidationCacheEXT
) -> VkResult;

pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "system" fn(
	device: VkDevice,
	validationCache: VkValidationCacheEXT,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "system" fn(
	device: VkDevice,
	validationCache: VkValidationCacheEXT,
	pDataSize: *mut size_t,
	pData: *mut c_void
) -> VkResult;

pub type PFN_vkMergeValidationCachesEXT = unsafe extern "system" fn(
	device: VkDevice,
	dstCache: VkValidationCacheEXT,
	srcCacheCount: u32,
	pSrcCaches: *mut VkValidationCacheEXT
) -> VkResult;

pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkDescriptorSetLayoutCreateInfo,
	pSupport: *mut VkDescriptorSetLayoutSupport
) -> VkResult;

pub type PFN_vkGetSwapchainGrallocUsageANDROID = unsafe extern "system" fn(
	device: VkDevice,
	format: VkFormat,
	imageUsage: VkImageUsageFlags,
	grallocUsage: *mut int
) -> VkResult;

pub type PFN_vkGetSwapchainGrallocUsage2ANDROID = unsafe extern "system" fn(
	device: VkDevice,
	format: VkFormat,
	imageUsage: VkImageUsageFlags,
	swapchainImageUsage: VkSwapchainImageUsageFlagsANDROID,
	grallocConsumerUsage: *mut uint64_t,
	grallocProducerUsage: *mut uint64_t
) -> VkResult;

pub type PFN_vkAcquireImageANDROID = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	nativeFenceFd: int,
	semaphore: VkSemaphore,
	fence: VkFence
) -> VkResult;

pub type PFN_vkQueueSignalReleaseImageANDROID = unsafe extern "system" fn(
	queue: VkQueue,
	waitSemaphoreCount: u32,
	pWaitSemaphores: *mut VkSemaphore,
	image: VkImage,
	pNativeFenceFd: *mut int
) -> VkResult;

pub type PFN_vkGetShaderInfoAMD = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	shaderStage: VkShaderStageFlagBits,
	infoType: VkShaderInfoTypeAMD,
	pInfoSize: *mut size_t,
	pInfo: *mut c_void
) -> VkResult;

pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
	device: VkDevice,
	swapChain: VkSwapchainKHR,
	localDimmingEnable: VkBool32
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pTimeDomainCount: *mut u32,
	pTimeDomains: *mut VkTimeDomainEXT
) -> VkResult;

pub type PFN_vkGetCalibratedTimestampsEXT = unsafe extern "system" fn(
	device: VkDevice,
	timestampCount: u32,
	pTimestampInfos: *mut VkCalibratedTimestampInfoEXT,
	pTimestamps: *mut uint64_t,
	pMaxDeviation: *mut uint64_t
) -> VkResult;

pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
	device: VkDevice,
	pNameInfo: *mut VkDebugUtilsObjectNameInfoEXT
) -> VkResult;

pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
	device: VkDevice,
	pTagInfo: *mut VkDebugUtilsObjectTagInfoEXT
) -> VkResult;

pub type PFN_vkQueueBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
	queue: VkQueue,
	pLabelInfo: *mut VkDebugUtilsLabelEXT
) -> VkResult;

pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "system" fn(
	queue: VkQueue
) -> VkResult;

pub type PFN_vkQueueInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
	queue: VkQueue,
	pLabelInfo: *mut VkDebugUtilsLabelEXT
) -> VkResult;

pub type PFN_vkCmdBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pLabelInfo: *mut VkDebugUtilsLabelEXT
) -> VkResult;

pub type PFN_vkCmdEndDebugUtilsLabelEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

pub type PFN_vkCmdInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pLabelInfo: *mut VkDebugUtilsLabelEXT
) -> VkResult;

pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkDebugUtilsMessengerCreateInfoEXT,
	pAllocator: *mut VkAllocationCallbacks,
	pMessenger: *mut VkDebugUtilsMessengerEXT
) -> VkResult;

pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "system" fn(
	instance: VkInstance,
	messenger: VkDebugUtilsMessengerEXT,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
	instance: VkInstance,
	messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
	messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
	pCallbackData: *mut VkDebugUtilsMessengerCallbackDataEXT
) -> VkResult;

pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
	device: VkDevice,
	handleType: VkExternalMemoryHandleTypeFlagBits,
	pHostPointer: *mut c_void,
	pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT
) -> VkResult;

pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineStage: VkPipelineStageFlagBits,
	dstBuffer: VkBuffer,
	dstOffset: VkDeviceSize,
	marker: u32
) -> VkResult;

pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkRenderPassCreateInfo2,
	pAllocator: *mut VkAllocationCallbacks,
	pRenderPass: *mut VkRenderPass
) -> VkResult;

pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pRenderPassBegin: *mut VkRenderPassBeginInfo,
	pSubpassBeginInfo: *mut VkSubpassBeginInfo
) -> VkResult;

pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pSubpassBeginInfo: *mut VkSubpassBeginInfo,
	pSubpassEndInfo: *mut VkSubpassEndInfo
) -> VkResult;

pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pSubpassEndInfo: *mut VkSubpassEndInfo
) -> VkResult;

pub type PFN_vkGetSemaphoreCounterValue = unsafe extern "system" fn(
	device: VkDevice,
	semaphore: VkSemaphore,
	pValue: *mut uint64_t
) -> VkResult;

pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(
	device: VkDevice,
	pWaitInfo: *mut VkSemaphoreWaitInfo,
	timeout: uint64_t
) -> VkResult;

pub type PFN_vkSignalSemaphore = unsafe extern "system" fn(
	device: VkDevice,
	pSignalInfo: *mut VkSemaphoreSignalInfo
) -> VkResult;

pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn(
	device: VkDevice,
	buffer: *const AHardwareBuffer,
	pProperties: *mut VkAndroidHardwareBufferPropertiesANDROID
) -> VkResult;

pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkMemoryGetAndroidHardwareBufferInfoANDROID,
	pBuffer: *mut *mut AHardwareBuffer
) -> VkResult;

pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	countBuffer: VkBuffer,
	countBufferOffset: VkDeviceSize,
	maxDrawCount: u32,
	stride: u32
) -> VkResult;

pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	countBuffer: VkBuffer,
	countBufferOffset: VkDeviceSize,
	maxDrawCount: u32,
	stride: u32
) -> VkResult;

pub type PFN_vkCmdSetCheckpointNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCheckpointMarker: *mut c_void
) -> VkResult;

pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
	queue: VkQueue,
	pCheckpointDataCount: *mut u32,
	pCheckpointData: *mut VkCheckpointDataNV
) -> VkResult;

pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstBinding: u32,
	bindingCount: u32,
	pBuffers: *mut VkBuffer,
	pOffsets: *mut VkDeviceSize,
	pSizes: *mut VkDeviceSize
) -> VkResult;

pub type PFN_vkCmdBeginTransformFeedbackEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstCounterBuffer: u32,
	counterBufferCount: u32,
	pCounterBuffers: *mut VkBuffer,
	pCounterBufferOffsets: *mut VkDeviceSize
) -> VkResult;

pub type PFN_vkCmdEndTransformFeedbackEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstCounterBuffer: u32,
	counterBufferCount: u32,
	pCounterBuffers: *mut VkBuffer,
	pCounterBufferOffsets: *mut VkDeviceSize
) -> VkResult;

pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	query: u32,
	flags: VkQueryControlFlags,
	index: u32
) -> VkResult;

pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	query: u32,
	index: u32
) -> VkResult;

pub type PFN_vkCmdDrawIndirectByteCountEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	instanceCount: u32,
	firstInstance: u32,
	counterBuffer: VkBuffer,
	counterBufferOffset: VkDeviceSize,
	counterOffset: u32,
	vertexStride: u32
) -> VkResult;

pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstExclusiveScissor: u32,
	exclusiveScissorCount: u32,
	pExclusiveScissors: *mut VkRect2D
) -> VkResult;

pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	imageView: VkImageView,
	imageLayout: VkImageLayout
) -> VkResult;

pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstViewport: u32,
	viewportCount: u32,
	pShadingRatePalettes: *mut VkShadingRatePaletteNV
) -> VkResult;

pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	sampleOrderType: VkCoarseSampleOrderTypeNV,
	customSampleOrderCount: u32,
	pCustomSampleOrders: *mut VkCoarseSampleOrderCustomNV
) -> VkResult;

pub type PFN_vkCmdDrawMeshTasksNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	taskCount: u32,
	firstTask: u32
) -> VkResult;

pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	drawCount: u32,
	stride: u32
) -> VkResult;

pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	countBuffer: VkBuffer,
	countBufferOffset: VkDeviceSize,
	maxDrawCount: u32,
	stride: u32
) -> VkResult;

pub type PFN_vkCmdDrawMeshTasksEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	groupCountX: u32,
	groupCountY: u32,
	groupCountZ: u32
) -> VkResult;

pub type PFN_vkCmdDrawMeshTasksIndirectEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	drawCount: u32,
	stride: u32
) -> VkResult;

pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	countBuffer: VkBuffer,
	countBufferOffset: VkDeviceSize,
	maxDrawCount: u32,
	stride: u32
) -> VkResult;

pub type PFN_vkCompileDeferredNV = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	shader: u32
) -> VkResult;

pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkAccelerationStructureCreateInfoNV,
	pAllocator: *mut VkAllocationCallbacks,
	pAccelerationStructure: *mut VkAccelerationStructureNV
) -> VkResult;

pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	imageView: VkImageView,
	imageLayout: VkImageLayout
) -> VkResult;

pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
	device: VkDevice,
	accelerationStructure: VkAccelerationStructureKHR,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "system" fn(
	device: VkDevice,
	accelerationStructure: VkAccelerationStructureNV,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkAccelerationStructureMemoryRequirementsInfoNV,
	pMemoryRequirements: *mut VkMemoryRequirements2KHR
) -> VkResult;

pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(
	device: VkDevice,
	bindInfoCount: u32,
	pBindInfos: *mut VkBindAccelerationStructureMemoryInfoNV
) -> VkResult;

pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	dst: VkAccelerationStructureNV,
	src: VkAccelerationStructureNV,
	mode: VkCopyAccelerationStructureModeKHR
) -> VkResult;

pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *mut VkCopyAccelerationStructureInfoKHR
) -> VkResult;

pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *mut VkCopyAccelerationStructureInfoKHR
) -> VkResult;

pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *mut VkCopyAccelerationStructureToMemoryInfoKHR
) -> VkResult;

pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *mut VkCopyAccelerationStructureToMemoryInfoKHR
) -> VkResult;

pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *mut VkCopyMemoryToAccelerationStructureInfoKHR
) -> VkResult;

pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *mut VkCopyMemoryToAccelerationStructureInfoKHR
) -> VkResult;

pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	accelerationStructureCount: u32,
	pAccelerationStructures: *mut VkAccelerationStructureKHR,
	queryType: VkQueryType,
	queryPool: VkQueryPool,
	firstQuery: u32
) -> VkResult;

pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	accelerationStructureCount: u32,
	pAccelerationStructures: *mut VkAccelerationStructureNV,
	queryType: VkQueryType,
	queryPool: VkQueryPool,
	firstQuery: u32
) -> VkResult;

pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *mut VkAccelerationStructureInfoNV,
	instanceData: VkBuffer,
	instanceOffset: VkDeviceSize,
	update: VkBool32,
	dst: VkAccelerationStructureNV,
	src: VkAccelerationStructureNV,
	scratch: VkBuffer,
	scratchOffset: VkDeviceSize
) -> VkResult;

pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
	device: VkDevice,
	accelerationStructureCount: u32,
	pAccelerationStructures: *mut VkAccelerationStructureKHR,
	queryType: VkQueryType,
	dataSize: size_t,
	pData: *mut c_void,
	stride: size_t
) -> VkResult;

pub type PFN_vkCmdTraceRaysKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pRaygenShaderBindingTable: *mut VkStridedDeviceAddressRegionKHR,
	pMissShaderBindingTable: *mut VkStridedDeviceAddressRegionKHR,
	pHitShaderBindingTable: *mut VkStridedDeviceAddressRegionKHR,
	pCallableShaderBindingTable: *mut VkStridedDeviceAddressRegionKHR,
	width: u32,
	height: u32,
	depth: u32
) -> VkResult;

pub type PFN_vkCmdTraceRaysNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	raygenShaderBindingTableBuffer: VkBuffer,
	raygenShaderBindingOffset: VkDeviceSize,
	missShaderBindingTableBuffer: VkBuffer,
	missShaderBindingOffset: VkDeviceSize,
	missShaderBindingStride: VkDeviceSize,
	hitShaderBindingTableBuffer: VkBuffer,
	hitShaderBindingOffset: VkDeviceSize,
	hitShaderBindingStride: VkDeviceSize,
	callableShaderBindingTableBuffer: VkBuffer,
	callableShaderBindingOffset: VkDeviceSize,
	callableShaderBindingStride: VkDeviceSize,
	width: u32,
	height: u32,
	depth: u32
) -> VkResult;

pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	firstGroup: u32,
	groupCount: u32,
	dataSize: size_t,
	pData: *mut c_void
) -> VkResult;

pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	firstGroup: u32,
	groupCount: u32,
	dataSize: size_t,
	pData: *mut c_void
) -> VkResult;

pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "system" fn(
	device: VkDevice,
	accelerationStructure: VkAccelerationStructureNV,
	dataSize: size_t,
	pData: *mut c_void
) -> VkResult;

pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *mut VkRayTracingPipelineCreateInfoNV,
	pAllocator: *mut VkAllocationCallbacks,
	pPipelines: *mut VkPipeline
) -> VkResult;

pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *mut VkRayTracingPipelineCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pPipelines: *mut VkPipeline
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *mut u32,
	pProperties: *mut VkCooperativeMatrixPropertiesNV
) -> VkResult;

pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pRaygenShaderBindingTable: *mut VkStridedDeviceAddressRegionKHR,
	pMissShaderBindingTable: *mut VkStridedDeviceAddressRegionKHR,
	pHitShaderBindingTable: *mut VkStridedDeviceAddressRegionKHR,
	pCallableShaderBindingTable: *mut VkStridedDeviceAddressRegionKHR,
	indirectDeviceAddress: VkDeviceAddress
) -> VkResult;

pub type PFN_vkCmdTraceRaysIndirect2KHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	indirectDeviceAddress: VkDeviceAddress
) -> VkResult;

pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(
	device: VkDevice,
	pVersionInfo: *mut VkAccelerationStructureVersionInfoKHR,
	pCompatibility: *mut VkAccelerationStructureCompatibilityKHR
) -> VkResult;

pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	group: u32,
	groupShader: VkShaderGroupShaderKHR
) -> VkResult;

pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineStackSize: u32
) -> VkResult;

pub type PFN_vkGetImageViewHandleNVX = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkImageViewHandleInfoNVX
) -> VkResult;

pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
	device: VkDevice,
	imageView: VkImageView,
	pProperties: *mut VkImageViewAddressPropertiesNVX
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pSurfaceInfo: *mut VkPhysicalDeviceSurfaceInfo2KHR,
	pPresentModeCount: *mut u32,
	pPresentModes: *mut VkPresentModeKHR
) -> VkResult;

pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
	device: VkDevice,
	pSurfaceInfo: *mut VkPhysicalDeviceSurfaceInfo2KHR,
	pModes: *mut VkDeviceGroupPresentModeFlagsKHR
) -> VkResult;

pub type PFN_vkAcquireFullScreenExclusiveModeEXT = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR
) -> VkResult;

pub type PFN_vkReleaseFullScreenExclusiveModeEXT = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR
) -> VkResult;

pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	pCounterCount: *mut u32,
	pCounters: *mut VkPerformanceCounterKHR,
	pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPerformanceQueryCreateInfo: *mut VkQueryPoolPerformanceCreateInfoKHR,
	pNumPasses: *mut u32
) -> VkResult;

pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkAcquireProfilingLockInfoKHR
) -> VkResult;

pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "system" fn(
	device: VkDevice
) -> VkResult;

pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pProperties: *mut VkImageDrmFormatModifierPropertiesEXT
) -> VkResult;

pub type PFN_vkGetBufferOpaqueCaptureAddress = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkBufferDeviceAddressInfo
) -> VkResult;

pub type PFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkBufferDeviceAddressInfo
) -> VkResult;

pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *mut VkHeadlessSurfaceCreateInfoEXT,
	pAllocator: *mut VkAllocationCallbacks,
	pSurface: *mut VkSurfaceKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pCombinationCount: *mut u32,
	pCombinations: *mut VkFramebufferMixedSamplesCombinationNV
) -> VkResult;

pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
	device: VkDevice,
	pInitializeInfo: *mut VkInitializePerformanceApiInfoINTEL
) -> VkResult;

pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(
	device: VkDevice
) -> VkResult;

pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pMarkerInfo: *mut VkPerformanceMarkerInfoINTEL
) -> VkResult;

pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pMarkerInfo: *mut VkPerformanceStreamMarkerInfoINTEL
) -> VkResult;

pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pOverrideInfo: *mut VkPerformanceOverrideInfoINTEL
) -> VkResult;

pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
	device: VkDevice,
	pAcquireInfo: *mut VkPerformanceConfigurationAcquireInfoINTEL,
	pConfiguration: *mut VkPerformanceConfigurationINTEL
) -> VkResult;

pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(
	device: VkDevice,
	configuration: VkPerformanceConfigurationINTEL
) -> VkResult;

pub type PFN_vkQueueSetPerformanceConfigurationINTEL = unsafe extern "system" fn(
	queue: VkQueue,
	configuration: VkPerformanceConfigurationINTEL
) -> VkResult;

pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
	device: VkDevice,
	parameter: VkPerformanceParameterTypeINTEL,
	pValue: *mut VkPerformanceValueINTEL
) -> VkResult;

pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkDeviceMemoryOpaqueCaptureAddressInfo
) -> VkResult;

pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
	device: VkDevice,
	pPipelineInfo: *mut VkPipelineInfoKHR,
	pExecutableCount: *mut u32,
	pProperties: *mut VkPipelineExecutablePropertiesKHR
) -> VkResult;

pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
	device: VkDevice,
	pExecutableInfo: *mut VkPipelineExecutableInfoKHR,
	pStatisticCount: *mut u32,
	pStatistics: *mut VkPipelineExecutableStatisticKHR
) -> VkResult;

pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = unsafe extern "system" fn(
	device: VkDevice,
	pExecutableInfo: *mut VkPipelineExecutableInfoKHR,
	pInternalRepresentationCount: *mut u32,
	pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR
) -> VkResult;

pub type PFN_vkCmdSetLineStippleEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	lineStippleFactor: u32,
	lineStipplePattern: uint16_t
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pToolCount: *mut u32,
	pToolProperties: *mut VkPhysicalDeviceToolProperties
) -> VkResult;

pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkAccelerationStructureCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pAccelerationStructure: *mut VkAccelerationStructureKHR
) -> VkResult;

pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	infoCount: u32,
	pInfos: *mut VkAccelerationStructureBuildGeometryInfoKHR,
	ppBuildRangeInfos: *mut *mut VkAccelerationStructureBuildRangeInfoKHR
) -> VkResult;

pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	infoCount: u32,
	pInfos: *mut VkAccelerationStructureBuildGeometryInfoKHR,
	pIndirectDeviceAddresses: *mut VkDeviceAddress,
	pIndirectStrides: *mut u32,
	ppMaxPrimitiveCounts: *mut *mut u32
) -> VkResult;

pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	infoCount: u32,
	pInfos: *mut VkAccelerationStructureBuildGeometryInfoKHR,
	ppBuildRangeInfos: *mut *mut VkAccelerationStructureBuildRangeInfoKHR
) -> VkResult;

pub type PFN_vkGetAccelerationStructureDeviceAddressKHR = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *mut VkAccelerationStructureDeviceAddressInfoKHR
) -> VkResult;

pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
	device: VkDevice,
	pAllocator: *mut VkAllocationCallbacks,
	pDeferredOperation: *mut VkDeferredOperationKHR
) -> VkResult;

pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
	device: VkDevice,
	operation: VkDeferredOperationKHR,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR = unsafe extern "system" fn(
	device: VkDevice,
	operation: VkDeferredOperationKHR
) -> VkResult;

pub type PFN_vkGetDeferredOperationResultKHR = unsafe extern "system" fn(
	device: VkDevice,
	operation: VkDeferredOperationKHR
) -> VkResult;

pub type PFN_vkDeferredOperationJoinKHR = unsafe extern "system" fn(
	device: VkDevice,
	operation: VkDeferredOperationKHR
) -> VkResult;

pub type PFN_vkCmdSetCullMode = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	cullMode: VkCullModeFlags
) -> VkResult;

pub type PFN_vkCmdSetFrontFace = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	frontFace: VkFrontFace
) -> VkResult;

pub type PFN_vkCmdSetPrimitiveTopology = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	primitiveTopology: VkPrimitiveTopology
) -> VkResult;

pub type PFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	viewportCount: u32,
	pViewports: *mut VkViewport
) -> VkResult;

pub type PFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	scissorCount: u32,
	pScissors: *mut VkRect2D
) -> VkResult;

pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstBinding: u32,
	bindingCount: u32,
	pBuffers: *mut VkBuffer,
	pOffsets: *mut VkDeviceSize,
	pSizes: *mut VkDeviceSize,
	pStrides: *mut VkDeviceSize
) -> VkResult;

pub type PFN_vkCmdSetDepthTestEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthTestEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetDepthWriteEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthWriteEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetDepthCompareOp = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthCompareOp: VkCompareOp
) -> VkResult;

pub type PFN_vkCmdSetDepthBoundsTestEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthBoundsTestEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetStencilTestEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	stencilTestEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetStencilOp = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	faceMask: VkStencilFaceFlags,
	failOp: VkStencilOp,
	passOp: VkStencilOp,
	depthFailOp: VkStencilOp,
	compareOp: VkCompareOp
) -> VkResult;

pub type PFN_vkCmdSetPatchControlPointsEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	patchControlPoints: u32
) -> VkResult;

pub type PFN_vkCmdSetRasterizerDiscardEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	rasterizerDiscardEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetDepthBiasEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthBiasEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetLogicOpEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	logicOp: VkLogicOp
) -> VkResult;

pub type PFN_vkCmdSetPrimitiveRestartEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	primitiveRestartEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetTessellationDomainOriginEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	domainOrigin: VkTessellationDomainOrigin
) -> VkResult;

pub type PFN_vkCmdSetDepthClampEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthClampEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetPolygonModeEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	polygonMode: VkPolygonMode
) -> VkResult;

pub type PFN_vkCmdSetRasterizationSamplesEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	rasterizationSamples: VkSampleCountFlagBits
) -> VkResult;

pub type PFN_vkCmdSetSampleMaskEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	samples: VkSampleCountFlagBits,
	pSampleMask: *mut VkSampleMask
) -> VkResult;

pub type PFN_vkCmdSetAlphaToCoverageEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	alphaToCoverageEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetAlphaToOneEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	alphaToOneEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetLogicOpEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	logicOpEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetColorBlendEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstAttachment: u32,
	attachmentCount: u32,
	pColorBlendEnables: *mut VkBool32
) -> VkResult;

pub type PFN_vkCmdSetColorBlendEquationEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstAttachment: u32,
	attachmentCount: u32,
	pColorBlendEquations: *mut VkColorBlendEquationEXT
) -> VkResult;

pub type PFN_vkCmdSetColorWriteMaskEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstAttachment: u32,
	attachmentCount: u32,
	pColorWriteMasks: *mut VkColorComponentFlags
) -> VkResult;

pub type PFN_vkCmdSetRasterizationStreamEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	rasterizationStream: u32
) -> VkResult;

pub type PFN_vkCmdSetConservativeRasterizationModeEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	conservativeRasterizationMode: VkConservativeRasterizationModeEXT
) -> VkResult;

pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	extraPrimitiveOverestimationSize: float
) -> VkResult;

pub type PFN_vkCmdSetDepthClipEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthClipEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetSampleLocationsEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	sampleLocationsEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetColorBlendAdvancedEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstAttachment: u32,
	attachmentCount: u32,
	pColorBlendAdvanced: *mut VkColorBlendAdvancedEXT
) -> VkResult;

pub type PFN_vkCmdSetProvokingVertexModeEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	provokingVertexMode: VkProvokingVertexModeEXT
) -> VkResult;

pub type PFN_vkCmdSetLineRasterizationModeEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	lineRasterizationMode: VkLineRasterizationModeEXT
) -> VkResult;

pub type PFN_vkCmdSetLineStippleEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	stippledLineEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	negativeOneToOne: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetViewportWScalingEnableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	viewportWScalingEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetViewportSwizzleNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstViewport: u32,
	viewportCount: u32,
	pViewportSwizzles: *mut VkViewportSwizzleNV
) -> VkResult;

pub type PFN_vkCmdSetCoverageToColorEnableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageToColorEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetCoverageToColorLocationNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageToColorLocation: u32
) -> VkResult;

pub type PFN_vkCmdSetCoverageModulationModeNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageModulationMode: VkCoverageModulationModeNV
) -> VkResult;

pub type PFN_vkCmdSetCoverageModulationTableEnableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageModulationTableEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetCoverageModulationTableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageModulationTableCount: u32,
	pCoverageModulationTable: *mut float
) -> VkResult;

pub type PFN_vkCmdSetShadingRateImageEnableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	shadingRateImageEnable: VkBool32
) -> VkResult;

pub type PFN_vkCmdSetCoverageReductionModeNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageReductionMode: VkCoverageReductionModeNV
) -> VkResult;

pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	representativeFragmentTestEnable: VkBool32
) -> VkResult;

pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkPrivateDataSlotCreateInfo,
	pAllocator: *mut VkAllocationCallbacks,
	pPrivateDataSlot: *mut VkPrivateDataSlot
) -> VkResult;

pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(
	device: VkDevice,
	privateDataSlot: VkPrivateDataSlot,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkSetPrivateData = unsafe extern "system" fn(
	device: VkDevice,
	objectType: VkObjectType,
	objectHandle: uint64_t,
	privateDataSlot: VkPrivateDataSlot,
	data: uint64_t
) -> VkResult;

pub type PFN_vkGetPrivateData = unsafe extern "system" fn(
	device: VkDevice,
	objectType: VkObjectType,
	objectHandle: uint64_t,
	privateDataSlot: VkPrivateDataSlot,
	pData: *mut uint64_t
) -> VkResult;

pub type PFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCopyBufferInfo: *mut VkCopyBufferInfo2
) -> VkResult;

pub type PFN_vkCmdCopyImage2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCopyImageInfo: *mut VkCopyImageInfo2
) -> VkResult;

pub type PFN_vkCmdBlitImage2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pBlitImageInfo: *mut VkBlitImageInfo2
) -> VkResult;

pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCopyBufferToImageInfo: *mut VkCopyBufferToImageInfo2
) -> VkResult;

pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCopyImageToBufferInfo: *mut VkCopyImageToBufferInfo2
) -> VkResult;

pub type PFN_vkCmdResolveImage2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pResolveImageInfo: *mut VkResolveImageInfo2
) -> VkResult;

pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pFragmentSize: *mut VkExtent2D,
	combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2]
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pFragmentShadingRateCount: *mut u32,
	pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR
) -> VkResult;

pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	shadingRate: VkFragmentShadingRateNV,
	combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2]
) -> VkResult;

pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(
	device: VkDevice,
	buildType: VkAccelerationStructureBuildTypeKHR,
	pBuildInfo: *mut VkAccelerationStructureBuildGeometryInfoKHR,
	pMaxPrimitiveCounts: *mut u32,
	pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR
) -> VkResult;

pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	vertexBindingDescriptionCount: u32,
	pVertexBindingDescriptions: *mut VkVertexInputBindingDescription2EXT,
	vertexAttributeDescriptionCount: u32,
	pVertexAttributeDescriptions: *mut VkVertexInputAttributeDescription2EXT
) -> VkResult;

pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	attachmentCount: u32,
	pColorWriteEnables: *mut VkBool32
) -> VkResult;

pub type PFN_vkCmdSetEvent2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	event: VkEvent,
	pDependencyInfo: *mut VkDependencyInfo
) -> VkResult;

pub type PFN_vkCmdResetEvent2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	event: VkEvent,
	stageMask: VkPipelineStageFlags2
) -> VkResult;

pub type PFN_vkCmdWaitEvents2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	eventCount: u32,
	pEvents: *mut VkEvent,
	pDependencyInfos: *mut VkDependencyInfo
) -> VkResult;

pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pDependencyInfo: *mut VkDependencyInfo
) -> VkResult;

pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(
	queue: VkQueue,
	submitCount: u32,
	pSubmits: *mut VkSubmitInfo2,
	fence: VkFence
) -> VkResult;

pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	stage: VkPipelineStageFlags2,
	queryPool: VkQueryPool,
	query: u32
) -> VkResult;

pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	stage: VkPipelineStageFlags2,
	dstBuffer: VkBuffer,
	dstOffset: VkDeviceSize,
	marker: u32
) -> VkResult;

pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
	queue: VkQueue,
	pCheckpointDataCount: *mut u32,
	pCheckpointData: *mut VkCheckpointData2NV
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pVideoProfile: *mut VkVideoProfileInfoKHR,
	pCapabilities: *mut VkVideoCapabilitiesKHR
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pVideoFormatInfo: *mut VkPhysicalDeviceVideoFormatInfoKHR,
	pVideoFormatPropertyCount: *mut u32,
	pVideoFormatProperties: *mut VkVideoFormatPropertiesKHR
) -> VkResult;

pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkVideoSessionCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pVideoSession: *mut VkVideoSessionKHR
) -> VkResult;

pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(
	device: VkDevice,
	videoSession: VkVideoSessionKHR,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkVideoSessionParametersCreateInfoKHR,
	pAllocator: *mut VkAllocationCallbacks,
	pVideoSessionParameters: *mut VkVideoSessionParametersKHR
) -> VkResult;

pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
	device: VkDevice,
	videoSessionParameters: VkVideoSessionParametersKHR,
	pUpdateInfo: *mut VkVideoSessionParametersUpdateInfoKHR
) -> VkResult;

pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
	device: VkDevice,
	videoSessionParameters: VkVideoSessionParametersKHR,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
	device: VkDevice,
	videoSession: VkVideoSessionKHR,
	pMemoryRequirementsCount: *mut u32,
	pMemoryRequirements: *mut VkVideoSessionMemoryRequirementsKHR
) -> VkResult;

pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
	device: VkDevice,
	videoSession: VkVideoSessionKHR,
	bindSessionMemoryInfoCount: u32,
	pBindSessionMemoryInfos: *mut VkBindVideoSessionMemoryInfoKHR
) -> VkResult;

pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pDecodeInfo: *mut VkVideoDecodeInfoKHR
) -> VkResult;

pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pBeginInfo: *mut VkVideoBeginCodingInfoKHR
) -> VkResult;

pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCodingControlInfo: *mut VkVideoCodingControlInfoKHR
) -> VkResult;

pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pEndCodingInfo: *mut VkVideoEndCodingInfoKHR
) -> VkResult;

pub type PFN_vkCmdEncodeVideoKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pEncodeInfo: *mut VkVideoEncodeInfoKHR
) -> VkResult;

pub type PFN_vkCmdDecompressMemoryNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	decompressRegionCount: u32,
	pDecompressMemoryRegions: *mut VkDecompressMemoryRegionNV
) -> VkResult;

pub type PFN_vkCmdDecompressMemoryIndirectCountNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	indirectCommandsAddress: VkDeviceAddress,
	indirectCommandsCountAddress: VkDeviceAddress,
	stride: u32
) -> VkResult;

pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkCuModuleCreateInfoNVX,
	pAllocator: *mut VkAllocationCallbacks,
	pModule: *mut VkCuModuleNVX
) -> VkResult;

pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkCuFunctionCreateInfoNVX,
	pAllocator: *mut VkAllocationCallbacks,
	pFunction: *mut VkCuFunctionNVX
) -> VkResult;

pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(
	device: VkDevice,
	module: VkCuModuleNVX,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(
	device: VkDevice,
	function: VkCuFunctionNVX,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCmdCuLaunchKernelNVX = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pLaunchInfo: *mut VkCuLaunchInfoNVX
) -> VkResult;

pub type PFN_vkSetDeviceMemoryPriorityEXT = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory,
	priority: float
) -> VkResult;

pub type PFN_vkAcquireDrmDisplayEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	drmFd: int32_t,
	display: VkDisplayKHR
) -> VkResult;

pub type PFN_vkGetDrmDisplayEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	drmFd: int32_t,
	connectorId: u32,
	display: *mut VkDisplayKHR
) -> VkResult;

pub type PFN_vkWaitForPresentKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	presentId: uint64_t,
	timeout: uint64_t
) -> VkResult;

pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkBufferCollectionCreateInfoFUCHSIA,
	pAllocator: *mut VkAllocationCallbacks,
	pCollection: *mut VkBufferCollectionFUCHSIA
) -> VkResult;

pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	collection: VkBufferCollectionFUCHSIA,
	pBufferConstraintsInfo: *mut VkBufferConstraintsInfoFUCHSIA
) -> VkResult;

pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	collection: VkBufferCollectionFUCHSIA,
	pImageConstraintsInfo: *mut VkImageConstraintsInfoFUCHSIA
) -> VkResult;

pub type PFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	collection: VkBufferCollectionFUCHSIA,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	collection: VkBufferCollectionFUCHSIA,
	pProperties: *mut VkBufferCollectionPropertiesFUCHSIA
) -> VkResult;

pub type PFN_vkCmdBeginRendering = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pRenderingInfo: *mut VkRenderingInfo
) -> VkResult;

pub type PFN_vkCmdEndRendering = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "system" fn(
	device: VkDevice,
	pBindingReference: *mut VkDescriptorSetBindingReferenceVALVE,
	pHostMapping: *mut VkDescriptorSetLayoutHostMappingInfoVALVE
) -> VkResult;

pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "system" fn(
	device: VkDevice,
	descriptorSet: VkDescriptorSet,
	ppData: *mut *mut c_void
) -> VkResult;

pub type PFN_vkCreateMicromapEXT = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkMicromapCreateInfoEXT,
	pAllocator: *mut VkAllocationCallbacks,
	pMicromap: *mut VkMicromapEXT
) -> VkResult;

pub type PFN_vkCmdBuildMicromapsEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	infoCount: u32,
	pInfos: *mut VkMicromapBuildInfoEXT
) -> VkResult;

pub type PFN_vkBuildMicromapsEXT = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	infoCount: u32,
	pInfos: *mut VkMicromapBuildInfoEXT
) -> VkResult;

pub type PFN_vkDestroyMicromapEXT = unsafe extern "system" fn(
	device: VkDevice,
	micromap: VkMicromapEXT,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkCmdCopyMicromapEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *mut VkCopyMicromapInfoEXT
) -> VkResult;

pub type PFN_vkCopyMicromapEXT = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *mut VkCopyMicromapInfoEXT
) -> VkResult;

pub type PFN_vkCmdCopyMicromapToMemoryEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *mut VkCopyMicromapToMemoryInfoEXT
) -> VkResult;

pub type PFN_vkCopyMicromapToMemoryEXT = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *mut VkCopyMicromapToMemoryInfoEXT
) -> VkResult;

pub type PFN_vkCmdCopyMemoryToMicromapEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *mut VkCopyMemoryToMicromapInfoEXT
) -> VkResult;

pub type PFN_vkCopyMemoryToMicromapEXT = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *mut VkCopyMemoryToMicromapInfoEXT
) -> VkResult;

pub type PFN_vkCmdWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	micromapCount: u32,
	pMicromaps: *mut VkMicromapEXT,
	queryType: VkQueryType,
	queryPool: VkQueryPool,
	firstQuery: u32
) -> VkResult;

pub type PFN_vkWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
	device: VkDevice,
	micromapCount: u32,
	pMicromaps: *mut VkMicromapEXT,
	queryType: VkQueryType,
	dataSize: size_t,
	pData: *mut c_void,
	stride: size_t
) -> VkResult;

pub type PFN_vkGetDeviceMicromapCompatibilityEXT = unsafe extern "system" fn(
	device: VkDevice,
	pVersionInfo: *mut VkMicromapVersionInfoEXT,
	pCompatibility: *mut VkAccelerationStructureCompatibilityKHR
) -> VkResult;

pub type PFN_vkGetMicromapBuildSizesEXT = unsafe extern "system" fn(
	device: VkDevice,
	buildType: VkAccelerationStructureBuildTypeKHR,
	pBuildInfo: *mut VkMicromapBuildInfoEXT,
	pSizeInfo: *mut VkMicromapBuildSizesInfoEXT
) -> VkResult;

pub type PFN_vkGetShaderModuleIdentifierEXT = unsafe extern "system" fn(
	device: VkDevice,
	shaderModule: VkShaderModule,
	pIdentifier: *mut VkShaderModuleIdentifierEXT
) -> VkResult;

pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkShaderModuleCreateInfo,
	pIdentifier: *mut VkShaderModuleIdentifierEXT
) -> VkResult;

pub type PFN_vkGetImageSubresourceLayout2EXT = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pSubresource: *mut VkImageSubresource2EXT,
	pLayout: *mut VkSubresourceLayout2EXT
) -> VkResult;

pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
	device: VkDevice,
	pPipelineInfo: *mut VkPipelineInfoEXT,
	pPipelineProperties: *mut VkBaseOutStructure
) -> VkResult;

pub type PFN_vkExportMetalObjectsEXT = unsafe extern "system" fn(
	device: VkDevice,
	pMetalObjectsInfo: *mut VkExportMetalObjectsInfoEXT
) -> VkResult;

pub type PFN_vkGetFramebufferTilePropertiesQCOM = unsafe extern "system" fn(
	device: VkDevice,
	framebuffer: VkFramebuffer,
	pPropertiesCount: *mut u32,
	pProperties: *mut VkTilePropertiesQCOM
) -> VkResult;

pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "system" fn(
	device: VkDevice,
	pRenderingInfo: *mut VkRenderingInfo,
	pProperties: *mut VkTilePropertiesQCOM
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pOpticalFlowImageFormatInfo: *mut VkOpticalFlowImageFormatInfoNV,
	pFormatCount: *mut u32,
	pImageFormatProperties: *mut VkOpticalFlowImageFormatPropertiesNV
) -> VkResult;

pub type PFN_vkCreateOpticalFlowSessionNV = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *mut VkOpticalFlowSessionCreateInfoNV,
	pAllocator: *mut VkAllocationCallbacks,
	pSession: *mut VkOpticalFlowSessionNV
) -> VkResult;

pub type PFN_vkDestroyOpticalFlowSessionNV = unsafe extern "system" fn(
	device: VkDevice,
	session: VkOpticalFlowSessionNV,
	pAllocator: *mut VkAllocationCallbacks
) -> VkResult;

pub type PFN_vkBindOpticalFlowSessionImageNV = unsafe extern "system" fn(
	device: VkDevice,
	session: VkOpticalFlowSessionNV,
	bindingPoint: VkOpticalFlowSessionBindingPointNV,
	view: VkImageView,
	layout: VkImageLayout
) -> VkResult;

pub type PFN_vkCmdOpticalFlowExecuteNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	session: VkOpticalFlowSessionNV,
	pExecuteInfo: *mut VkOpticalFlowExecuteInfoNV
) -> VkResult;

