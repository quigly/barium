/** vkCreateInstance
	const *
	VkInstanceCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkInstance
	pInstance

**/
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
	pCreateInfo: *const VkInstanceCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pInstance: *const VkInstance
) -> VkResult;

/** vkDestroyInstance
	
	VkInstance
	instance

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyInstance = unsafe extern "system" fn(
	instance: VkInstance,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkEnumeratePhysicalDevices
	
	VkInstance
	instance

	*
	uint32_t
	pPhysicalDeviceCount

	*
	VkPhysicalDevice
	pPhysicalDevices

**/
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(
	instance: VkInstance,
	pPhysicalDeviceCount: *const u32,
	pPhysicalDevices: *const VkPhysicalDevice
) -> VkResult;

/** vkGetDeviceProcAddr
	
	VkDevice
	device

	const *
	char
	pName

**/
pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(
	device: VkDevice,
	pName: *const char
) -> VkResult;

/** vkGetInstanceProcAddr
	
	VkInstance
	instance

	const *
	char
	pName

**/
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(
	instance: VkInstance,
	pName: *const char
) -> VkResult;

/** vkGetPhysicalDeviceProperties
	
	VkPhysicalDevice
	physicalDevice

	*
	VkPhysicalDeviceProperties
	pProperties

**/
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pProperties: *const VkPhysicalDeviceProperties
) -> VkResult;

/** vkGetPhysicalDeviceQueueFamilyProperties
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pQueueFamilyPropertyCount

	*
	VkQueueFamilyProperties
	pQueueFamilyProperties

**/
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pQueueFamilyPropertyCount: *const u32,
	pQueueFamilyProperties: *const VkQueueFamilyProperties
) -> VkResult;

/** vkGetPhysicalDeviceMemoryProperties
	
	VkPhysicalDevice
	physicalDevice

	*
	VkPhysicalDeviceMemoryProperties
	pMemoryProperties

**/
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pMemoryProperties: *const VkPhysicalDeviceMemoryProperties
) -> VkResult;

/** vkGetPhysicalDeviceFeatures
	
	VkPhysicalDevice
	physicalDevice

	*
	VkPhysicalDeviceFeatures
	pFeatures

**/
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pFeatures: *const VkPhysicalDeviceFeatures
) -> VkResult;

/** vkGetPhysicalDeviceFormatProperties
	
	VkPhysicalDevice
	physicalDevice

	
	VkFormat
	format

	*
	VkFormatProperties
	pFormatProperties

**/
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	pFormatProperties: *const VkFormatProperties
) -> VkResult;

/** vkGetPhysicalDeviceImageFormatProperties
	
	VkPhysicalDevice
	physicalDevice

	
	VkFormat
	format

	
	VkImageType
	type

	
	VkImageTiling
	tiling

	
	VkImageUsageFlags
	usage

	
	VkImageCreateFlags
	flags

	*
	VkImageFormatProperties
	pImageFormatProperties

**/
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	type: VkImageType,
	tiling: VkImageTiling,
	usage: VkImageUsageFlags,
	flags: VkImageCreateFlags,
	pImageFormatProperties: *const VkImageFormatProperties
) -> VkResult;

/** vkCreateDevice
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkDeviceCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkDevice
	pDevice

**/
pub type PFN_vkCreateDevice = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pCreateInfo: *const VkDeviceCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pDevice: *const VkDevice
) -> VkResult;

/** vkDestroyDevice
	
	VkDevice
	device

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyDevice = unsafe extern "system" fn(
	device: VkDevice,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkEnumerateInstanceVersion
	*
	uint32_t
	pApiVersion

**/
pub type PFN_vkEnumerateInstanceVersion = unsafe extern "system" fn(
	pApiVersion: *const u32
) -> VkResult;

/** vkEnumerateInstanceLayerProperties
	*
	uint32_t
	pPropertyCount

	*
	VkLayerProperties
	pProperties

**/
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(
	pPropertyCount: *const u32,
	pProperties: *const VkLayerProperties
) -> VkResult;

/** vkEnumerateInstanceExtensionProperties
	const *
	char
	pLayerName

	*
	uint32_t
	pPropertyCount

	*
	VkExtensionProperties
	pProperties

**/
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(
	pLayerName: *const char,
	pPropertyCount: *const u32,
	pProperties: *const VkExtensionProperties
) -> VkResult;

/** vkEnumerateDeviceLayerProperties
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pPropertyCount

	*
	VkLayerProperties
	pProperties

**/
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *const u32,
	pProperties: *const VkLayerProperties
) -> VkResult;

/** vkEnumerateDeviceExtensionProperties
	
	VkPhysicalDevice
	physicalDevice

	const *
	char
	pLayerName

	*
	uint32_t
	pPropertyCount

	*
	VkExtensionProperties
	pProperties

**/
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pLayerName: *const char,
	pPropertyCount: *const u32,
	pProperties: *const VkExtensionProperties
) -> VkResult;

/** vkGetDeviceQueue
	
	VkDevice
	device

	
	uint32_t
	queueFamilyIndex

	
	uint32_t
	queueIndex

	*
	VkQueue
	pQueue

**/
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(
	device: VkDevice,
	queueFamilyIndex: u32,
	queueIndex: u32,
	pQueue: *const VkQueue
) -> VkResult;

/** vkQueueSubmit
	
	VkQueue
	queue

	
	uint32_t
	submitCount

	const *
	VkSubmitInfo
	pSubmits

	
	VkFence
	fence

**/
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(
	queue: VkQueue,
	submitCount: u32,
	pSubmits: *const VkSubmitInfo,
	fence: VkFence
) -> VkResult;

/** vkQueueWaitIdle
	
	VkQueue
	queue

**/
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(
	queue: VkQueue
) -> VkResult;

/** vkDeviceWaitIdle
	
	VkDevice
	device

**/
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(
	device: VkDevice
) -> VkResult;

/** vkAllocateMemory
	
	VkDevice
	device

	const *
	VkMemoryAllocateInfo
	pAllocateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkDeviceMemory
	pMemory

**/
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
	device: VkDevice,
	pAllocateInfo: *const VkMemoryAllocateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pMemory: *const VkDeviceMemory
) -> VkResult;

/** vkFreeMemory
	
	VkDevice
	device

	
	VkDeviceMemory
	memory

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkFreeMemory = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkMapMemory
	
	VkDevice
	device

	
	VkDeviceMemory
	memory

	
	VkDeviceSize
	offset

	
	VkDeviceSize
	size

	
	VkMemoryMapFlags
	flags

	**
	void
	ppData

**/
pub type PFN_vkMapMemory = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory,
	offset: VkDeviceSize,
	size: VkDeviceSize,
	flags: VkMemoryMapFlags,
	ppData: ** void
) -> VkResult;

/** vkUnmapMemory
	
	VkDevice
	device

	
	VkDeviceMemory
	memory

**/
pub type PFN_vkUnmapMemory = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory
) -> VkResult;

/** vkFlushMappedMemoryRanges
	
	VkDevice
	device

	
	uint32_t
	memoryRangeCount

	const *
	VkMappedMemoryRange
	pMemoryRanges

**/
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
	device: VkDevice,
	memoryRangeCount: u32,
	pMemoryRanges: *const VkMappedMemoryRange
) -> VkResult;

/** vkInvalidateMappedMemoryRanges
	
	VkDevice
	device

	
	uint32_t
	memoryRangeCount

	const *
	VkMappedMemoryRange
	pMemoryRanges

**/
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
	device: VkDevice,
	memoryRangeCount: u32,
	pMemoryRanges: *const VkMappedMemoryRange
) -> VkResult;

/** vkGetDeviceMemoryCommitment
	
	VkDevice
	device

	
	VkDeviceMemory
	memory

	*
	VkDeviceSize
	pCommittedMemoryInBytes

**/
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory,
	pCommittedMemoryInBytes: *const VkDeviceSize
) -> VkResult;

/** vkGetBufferMemoryRequirements
	
	VkDevice
	device

	
	VkBuffer
	buffer

	*
	VkMemoryRequirements
	pMemoryRequirements

**/
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	buffer: VkBuffer,
	pMemoryRequirements: *const VkMemoryRequirements
) -> VkResult;

/** vkBindBufferMemory
	
	VkDevice
	device

	
	VkBuffer
	buffer

	
	VkDeviceMemory
	memory

	
	VkDeviceSize
	memoryOffset

**/
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(
	device: VkDevice,
	buffer: VkBuffer,
	memory: VkDeviceMemory,
	memoryOffset: VkDeviceSize
) -> VkResult;

/** vkGetImageMemoryRequirements
	
	VkDevice
	device

	
	VkImage
	image

	*
	VkMemoryRequirements
	pMemoryRequirements

**/
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pMemoryRequirements: *const VkMemoryRequirements
) -> VkResult;

/** vkBindImageMemory
	
	VkDevice
	device

	
	VkImage
	image

	
	VkDeviceMemory
	memory

	
	VkDeviceSize
	memoryOffset

**/
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	memory: VkDeviceMemory,
	memoryOffset: VkDeviceSize
) -> VkResult;

/** vkGetImageSparseMemoryRequirements
	
	VkDevice
	device

	
	VkImage
	image

	*
	uint32_t
	pSparseMemoryRequirementCount

	*
	VkSparseImageMemoryRequirements
	pSparseMemoryRequirements

**/
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pSparseMemoryRequirementCount: *const u32,
	pSparseMemoryRequirements: *const VkSparseImageMemoryRequirements
) -> VkResult;

/** vkGetPhysicalDeviceSparseImageFormatProperties
	
	VkPhysicalDevice
	physicalDevice

	
	VkFormat
	format

	
	VkImageType
	type

	
	VkSampleCountFlagBits
	samples

	
	VkImageUsageFlags
	usage

	
	VkImageTiling
	tiling

	*
	uint32_t
	pPropertyCount

	*
	VkSparseImageFormatProperties
	pProperties

**/
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	type: VkImageType,
	samples: VkSampleCountFlagBits,
	usage: VkImageUsageFlags,
	tiling: VkImageTiling,
	pPropertyCount: *const u32,
	pProperties: *const VkSparseImageFormatProperties
) -> VkResult;

/** vkQueueBindSparse
	
	VkQueue
	queue

	
	uint32_t
	bindInfoCount

	const *
	VkBindSparseInfo
	pBindInfo

	
	VkFence
	fence

**/
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(
	queue: VkQueue,
	bindInfoCount: u32,
	pBindInfo: *const VkBindSparseInfo,
	fence: VkFence
) -> VkResult;

/** vkCreateFence
	
	VkDevice
	device

	const *
	VkFenceCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkFence
	pFence

**/
pub type PFN_vkCreateFence = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkFenceCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pFence: *const VkFence
) -> VkResult;

/** vkDestroyFence
	
	VkDevice
	device

	
	VkFence
	fence

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyFence = unsafe extern "system" fn(
	device: VkDevice,
	fence: VkFence,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkResetFences
	
	VkDevice
	device

	
	uint32_t
	fenceCount

	const *
	VkFence
	pFences

**/
pub type PFN_vkResetFences = unsafe extern "system" fn(
	device: VkDevice,
	fenceCount: u32,
	pFences: *const VkFence
) -> VkResult;

/** vkGetFenceStatus
	
	VkDevice
	device

	
	VkFence
	fence

**/
pub type PFN_vkGetFenceStatus = unsafe extern "system" fn(
	device: VkDevice,
	fence: VkFence
) -> VkResult;

/** vkWaitForFences
	
	VkDevice
	device

	
	uint32_t
	fenceCount

	const *
	VkFence
	pFences

	
	VkBool32
	waitAll

	
	uint64_t
	timeout

**/
pub type PFN_vkWaitForFences = unsafe extern "system" fn(
	device: VkDevice,
	fenceCount: u32,
	pFences: *const VkFence,
	waitAll: VkBool32,
	timeout: uint64_t
) -> VkResult;

/** vkCreateSemaphore
	
	VkDevice
	device

	const *
	VkSemaphoreCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSemaphore
	pSemaphore

**/
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkSemaphoreCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pSemaphore: *const VkSemaphore
) -> VkResult;

/** vkDestroySemaphore
	
	VkDevice
	device

	
	VkSemaphore
	semaphore

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(
	device: VkDevice,
	semaphore: VkSemaphore,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreateEvent
	
	VkDevice
	device

	const *
	VkEventCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkEvent
	pEvent

**/
pub type PFN_vkCreateEvent = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkEventCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pEvent: *const VkEvent
) -> VkResult;

/** vkDestroyEvent
	
	VkDevice
	device

	
	VkEvent
	event

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(
	device: VkDevice,
	event: VkEvent,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetEventStatus
	
	VkDevice
	device

	
	VkEvent
	event

**/
pub type PFN_vkGetEventStatus = unsafe extern "system" fn(
	device: VkDevice,
	event: VkEvent
) -> VkResult;

/** vkSetEvent
	
	VkDevice
	device

	
	VkEvent
	event

**/
pub type PFN_vkSetEvent = unsafe extern "system" fn(
	device: VkDevice,
	event: VkEvent
) -> VkResult;

/** vkResetEvent
	
	VkDevice
	device

	
	VkEvent
	event

**/
pub type PFN_vkResetEvent = unsafe extern "system" fn(
	device: VkDevice,
	event: VkEvent
) -> VkResult;

/** vkCreateQueryPool
	
	VkDevice
	device

	const *
	VkQueryPoolCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkQueryPool
	pQueryPool

**/
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkQueryPoolCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pQueryPool: *const VkQueryPool
) -> VkResult;

/** vkDestroyQueryPool
	
	VkDevice
	device

	
	VkQueryPool
	queryPool

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(
	device: VkDevice,
	queryPool: VkQueryPool,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetQueryPoolResults
	
	VkDevice
	device

	
	VkQueryPool
	queryPool

	
	uint32_t
	firstQuery

	
	uint32_t
	queryCount

	
	size_t
	dataSize

	*
	void
	pData

	
	VkDeviceSize
	stride

	
	VkQueryResultFlags
	flags

**/
pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(
	device: VkDevice,
	queryPool: VkQueryPool,
	firstQuery: u32,
	queryCount: u32,
	dataSize: size_t,
	pData: *const void,
	stride: VkDeviceSize,
	flags: VkQueryResultFlags
) -> VkResult;

/** vkResetQueryPool
	
	VkDevice
	device

	
	VkQueryPool
	queryPool

	
	uint32_t
	firstQuery

	
	uint32_t
	queryCount

**/
pub type PFN_vkResetQueryPool = unsafe extern "system" fn(
	device: VkDevice,
	queryPool: VkQueryPool,
	firstQuery: u32,
	queryCount: u32
) -> VkResult;

/** vkCreateBuffer
	
	VkDevice
	device

	const *
	VkBufferCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkBuffer
	pBuffer

**/
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkBufferCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pBuffer: *const VkBuffer
) -> VkResult;

/** vkDestroyBuffer
	
	VkDevice
	device

	
	VkBuffer
	buffer

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
	device: VkDevice,
	buffer: VkBuffer,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreateBufferView
	
	VkDevice
	device

	const *
	VkBufferViewCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkBufferView
	pView

**/
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkBufferViewCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pView: *const VkBufferView
) -> VkResult;

/** vkDestroyBufferView
	
	VkDevice
	device

	
	VkBufferView
	bufferView

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(
	device: VkDevice,
	bufferView: VkBufferView,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreateImage
	
	VkDevice
	device

	const *
	VkImageCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkImage
	pImage

**/
pub type PFN_vkCreateImage = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkImageCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pImage: *const VkImage
) -> VkResult;

/** vkDestroyImage
	
	VkDevice
	device

	
	VkImage
	image

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyImage = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetImageSubresourceLayout
	
	VkDevice
	device

	
	VkImage
	image

	const *
	VkImageSubresource
	pSubresource

	*
	VkSubresourceLayout
	pLayout

**/
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pSubresource: *const VkImageSubresource,
	pLayout: *const VkSubresourceLayout
) -> VkResult;

/** vkCreateImageView
	
	VkDevice
	device

	const *
	VkImageViewCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkImageView
	pView

**/
pub type PFN_vkCreateImageView = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkImageViewCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pView: *const VkImageView
) -> VkResult;

/** vkDestroyImageView
	
	VkDevice
	device

	
	VkImageView
	imageView

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(
	device: VkDevice,
	imageView: VkImageView,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreateShaderModule
	
	VkDevice
	device

	const *
	VkShaderModuleCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkShaderModule
	pShaderModule

**/
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkShaderModuleCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pShaderModule: *const VkShaderModule
) -> VkResult;

/** vkDestroyShaderModule
	
	VkDevice
	device

	
	VkShaderModule
	shaderModule

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(
	device: VkDevice,
	shaderModule: VkShaderModule,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreatePipelineCache
	
	VkDevice
	device

	const *
	VkPipelineCacheCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkPipelineCache
	pPipelineCache

**/
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkPipelineCacheCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pPipelineCache: *const VkPipelineCache
) -> VkResult;

/** vkDestroyPipelineCache
	
	VkDevice
	device

	
	VkPipelineCache
	pipelineCache

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetPipelineCacheData
	
	VkDevice
	device

	
	VkPipelineCache
	pipelineCache

	*
	size_t
	pDataSize

	*
	void
	pData

**/
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	pDataSize: *const size_t,
	pData: *const void
) -> VkResult;

/** vkMergePipelineCaches
	
	VkDevice
	device

	
	VkPipelineCache
	dstCache

	
	uint32_t
	srcCacheCount

	const *
	VkPipelineCache
	pSrcCaches

**/
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(
	device: VkDevice,
	dstCache: VkPipelineCache,
	srcCacheCount: u32,
	pSrcCaches: *const VkPipelineCache
) -> VkResult;

/** vkCreateGraphicsPipelines
	
	VkDevice
	device

	
	VkPipelineCache
	pipelineCache

	
	uint32_t
	createInfoCount

	const *
	VkGraphicsPipelineCreateInfo
	pCreateInfos

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkPipeline
	pPipelines

**/
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *const VkGraphicsPipelineCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pPipelines: *const VkPipeline
) -> VkResult;

/** vkCreateComputePipelines
	
	VkDevice
	device

	
	VkPipelineCache
	pipelineCache

	
	uint32_t
	createInfoCount

	const *
	VkComputePipelineCreateInfo
	pCreateInfos

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkPipeline
	pPipelines

**/
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *const VkComputePipelineCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pPipelines: *const VkPipeline
) -> VkResult;

/** vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI
	
	VkDevice
	device

	
	VkRenderPass
	renderpass

	*
	VkExtent2D
	pMaxWorkgroupSize

**/
pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = unsafe extern "system" fn(
	device: VkDevice,
	renderpass: VkRenderPass,
	pMaxWorkgroupSize: *const VkExtent2D
) -> VkResult;

/** vkDestroyPipeline
	
	VkDevice
	device

	
	VkPipeline
	pipeline

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreatePipelineLayout
	
	VkDevice
	device

	const *
	VkPipelineLayoutCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkPipelineLayout
	pPipelineLayout

**/
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkPipelineLayoutCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pPipelineLayout: *const VkPipelineLayout
) -> VkResult;

/** vkDestroyPipelineLayout
	
	VkDevice
	device

	
	VkPipelineLayout
	pipelineLayout

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(
	device: VkDevice,
	pipelineLayout: VkPipelineLayout,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreateSampler
	
	VkDevice
	device

	const *
	VkSamplerCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSampler
	pSampler

**/
pub type PFN_vkCreateSampler = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkSamplerCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pSampler: *const VkSampler
) -> VkResult;

/** vkDestroySampler
	
	VkDevice
	device

	
	VkSampler
	sampler

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroySampler = unsafe extern "system" fn(
	device: VkDevice,
	sampler: VkSampler,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreateDescriptorSetLayout
	
	VkDevice
	device

	const *
	VkDescriptorSetLayoutCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkDescriptorSetLayout
	pSetLayout

**/
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pSetLayout: *const VkDescriptorSetLayout
) -> VkResult;

/** vkDestroyDescriptorSetLayout
	
	VkDevice
	device

	
	VkDescriptorSetLayout
	descriptorSetLayout

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(
	device: VkDevice,
	descriptorSetLayout: VkDescriptorSetLayout,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreateDescriptorPool
	
	VkDevice
	device

	const *
	VkDescriptorPoolCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkDescriptorPool
	pDescriptorPool

**/
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkDescriptorPoolCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pDescriptorPool: *const VkDescriptorPool
) -> VkResult;

/** vkDestroyDescriptorPool
	
	VkDevice
	device

	
	VkDescriptorPool
	descriptorPool

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(
	device: VkDevice,
	descriptorPool: VkDescriptorPool,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkResetDescriptorPool
	
	VkDevice
	device

	
	VkDescriptorPool
	descriptorPool

	
	VkDescriptorPoolResetFlags
	flags

**/
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(
	device: VkDevice,
	descriptorPool: VkDescriptorPool,
	flags: VkDescriptorPoolResetFlags
) -> VkResult;

/** vkAllocateDescriptorSets
	
	VkDevice
	device

	const *
	VkDescriptorSetAllocateInfo
	pAllocateInfo

	*
	VkDescriptorSet
	pDescriptorSets

**/
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(
	device: VkDevice,
	pAllocateInfo: *const VkDescriptorSetAllocateInfo,
	pDescriptorSets: *const VkDescriptorSet
) -> VkResult;

/** vkFreeDescriptorSets
	
	VkDevice
	device

	
	VkDescriptorPool
	descriptorPool

	
	uint32_t
	descriptorSetCount

	const *
	VkDescriptorSet
	pDescriptorSets

**/
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(
	device: VkDevice,
	descriptorPool: VkDescriptorPool,
	descriptorSetCount: u32,
	pDescriptorSets: *const VkDescriptorSet
) -> VkResult;

/** vkUpdateDescriptorSets
	
	VkDevice
	device

	
	uint32_t
	descriptorWriteCount

	const *
	VkWriteDescriptorSet
	pDescriptorWrites

	
	uint32_t
	descriptorCopyCount

	const *
	VkCopyDescriptorSet
	pDescriptorCopies

**/
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(
	device: VkDevice,
	descriptorWriteCount: u32,
	pDescriptorWrites: *const VkWriteDescriptorSet,
	descriptorCopyCount: u32,
	pDescriptorCopies: *const VkCopyDescriptorSet
) -> VkResult;

/** vkCreateFramebuffer
	
	VkDevice
	device

	const *
	VkFramebufferCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkFramebuffer
	pFramebuffer

**/
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkFramebufferCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pFramebuffer: *const VkFramebuffer
) -> VkResult;

/** vkDestroyFramebuffer
	
	VkDevice
	device

	
	VkFramebuffer
	framebuffer

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(
	device: VkDevice,
	framebuffer: VkFramebuffer,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreateRenderPass
	
	VkDevice
	device

	const *
	VkRenderPassCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkRenderPass
	pRenderPass

**/
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkRenderPassCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pRenderPass: *const VkRenderPass
) -> VkResult;

/** vkDestroyRenderPass
	
	VkDevice
	device

	
	VkRenderPass
	renderPass

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(
	device: VkDevice,
	renderPass: VkRenderPass,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetRenderAreaGranularity
	
	VkDevice
	device

	
	VkRenderPass
	renderPass

	*
	VkExtent2D
	pGranularity

**/
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(
	device: VkDevice,
	renderPass: VkRenderPass,
	pGranularity: *const VkExtent2D
) -> VkResult;

/** vkCreateCommandPool
	
	VkDevice
	device

	const *
	VkCommandPoolCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkCommandPool
	pCommandPool

**/
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkCommandPoolCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pCommandPool: *const VkCommandPool
) -> VkResult;

/** vkDestroyCommandPool
	
	VkDevice
	device

	
	VkCommandPool
	commandPool

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(
	device: VkDevice,
	commandPool: VkCommandPool,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkResetCommandPool
	
	VkDevice
	device

	
	VkCommandPool
	commandPool

	
	VkCommandPoolResetFlags
	flags

**/
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(
	device: VkDevice,
	commandPool: VkCommandPool,
	flags: VkCommandPoolResetFlags
) -> VkResult;

/** vkAllocateCommandBuffers
	
	VkDevice
	device

	const *
	VkCommandBufferAllocateInfo
	pAllocateInfo

	*
	VkCommandBuffer
	pCommandBuffers

**/
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
	device: VkDevice,
	pAllocateInfo: *const VkCommandBufferAllocateInfo,
	pCommandBuffers: *const VkCommandBuffer
) -> VkResult;

/** vkFreeCommandBuffers
	
	VkDevice
	device

	
	VkCommandPool
	commandPool

	
	uint32_t
	commandBufferCount

	const *
	VkCommandBuffer
	pCommandBuffers

**/
pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(
	device: VkDevice,
	commandPool: VkCommandPool,
	commandBufferCount: u32,
	pCommandBuffers: *const VkCommandBuffer
) -> VkResult;

/** vkBeginCommandBuffer
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCommandBufferBeginInfo
	pBeginInfo

**/
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pBeginInfo: *const VkCommandBufferBeginInfo
) -> VkResult;

/** vkEndCommandBuffer
	
	VkCommandBuffer
	commandBuffer

**/
pub type PFN_vkEndCommandBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

/** vkResetCommandBuffer
	
	VkCommandBuffer
	commandBuffer

	
	VkCommandBufferResetFlags
	flags

**/
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	flags: VkCommandBufferResetFlags
) -> VkResult;

/** vkCmdBindPipeline
	
	VkCommandBuffer
	commandBuffer

	
	VkPipelineBindPoint
	pipelineBindPoint

	
	VkPipeline
	pipeline

**/
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineBindPoint: VkPipelineBindPoint,
	pipeline: VkPipeline
) -> VkResult;

/** vkCmdSetViewport
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstViewport

	
	uint32_t
	viewportCount

	const *
	VkViewport
	pViewports

**/
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstViewport: u32,
	viewportCount: u32,
	pViewports: *const VkViewport
) -> VkResult;

/** vkCmdSetScissor
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstScissor

	
	uint32_t
	scissorCount

	const *
	VkRect2D
	pScissors

**/
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstScissor: u32,
	scissorCount: u32,
	pScissors: *const VkRect2D
) -> VkResult;

/** vkCmdSetLineWidth
	
	VkCommandBuffer
	commandBuffer

	
	float
	lineWidth

**/
pub type PFN_vkCmdSetLineWidth = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	lineWidth: float
) -> VkResult;

/** vkCmdSetDepthBias
	
	VkCommandBuffer
	commandBuffer

	
	float
	depthBiasConstantFactor

	
	float
	depthBiasClamp

	
	float
	depthBiasSlopeFactor

**/
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthBiasConstantFactor: float,
	depthBiasClamp: float,
	depthBiasSlopeFactor: float
) -> VkResult;

/** vkCmdSetBlendConstants
	
	VkCommandBuffer
	commandBuffer

	const  [4]
	float
	blendConstants

**/
pub type PFN_vkCmdSetBlendConstants = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	blendConstants: const  [4] float
) -> VkResult;

/** vkCmdSetDepthBounds
	
	VkCommandBuffer
	commandBuffer

	
	float
	minDepthBounds

	
	float
	maxDepthBounds

**/
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	minDepthBounds: float,
	maxDepthBounds: float
) -> VkResult;

/** vkCmdSetStencilCompareMask
	
	VkCommandBuffer
	commandBuffer

	
	VkStencilFaceFlags
	faceMask

	
	uint32_t
	compareMask

**/
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	faceMask: VkStencilFaceFlags,
	compareMask: u32
) -> VkResult;

/** vkCmdSetStencilWriteMask
	
	VkCommandBuffer
	commandBuffer

	
	VkStencilFaceFlags
	faceMask

	
	uint32_t
	writeMask

**/
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	faceMask: VkStencilFaceFlags,
	writeMask: u32
) -> VkResult;

/** vkCmdSetStencilReference
	
	VkCommandBuffer
	commandBuffer

	
	VkStencilFaceFlags
	faceMask

	
	uint32_t
	reference

**/
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	faceMask: VkStencilFaceFlags,
	reference: u32
) -> VkResult;

/** vkCmdBindDescriptorSets
	
	VkCommandBuffer
	commandBuffer

	
	VkPipelineBindPoint
	pipelineBindPoint

	
	VkPipelineLayout
	layout

	
	uint32_t
	firstSet

	
	uint32_t
	descriptorSetCount

	const *
	VkDescriptorSet
	pDescriptorSets

	
	uint32_t
	dynamicOffsetCount

	const *
	uint32_t
	pDynamicOffsets

**/
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineBindPoint: VkPipelineBindPoint,
	layout: VkPipelineLayout,
	firstSet: u32,
	descriptorSetCount: u32,
	pDescriptorSets: *const VkDescriptorSet,
	dynamicOffsetCount: u32,
	pDynamicOffsets: *const u32
) -> VkResult;

/** vkCmdBindIndexBuffer
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	buffer

	
	VkDeviceSize
	offset

	
	VkIndexType
	indexType

**/
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	indexType: VkIndexType
) -> VkResult;

/** vkCmdBindVertexBuffers
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstBinding

	
	uint32_t
	bindingCount

	const *
	VkBuffer
	pBuffers

	const *
	VkDeviceSize
	pOffsets

**/
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstBinding: u32,
	bindingCount: u32,
	pBuffers: *const VkBuffer,
	pOffsets: *const VkDeviceSize
) -> VkResult;

/** vkCmdDraw
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	vertexCount

	
	uint32_t
	instanceCount

	
	uint32_t
	firstVertex

	
	uint32_t
	firstInstance

**/
pub type PFN_vkCmdDraw = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	vertexCount: u32,
	instanceCount: u32,
	firstVertex: u32,
	firstInstance: u32
) -> VkResult;

/** vkCmdDrawIndexed
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	indexCount

	
	uint32_t
	instanceCount

	
	uint32_t
	firstIndex

	
	int32_t
	vertexOffset

	
	uint32_t
	firstInstance

**/
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	indexCount: u32,
	instanceCount: u32,
	firstIndex: u32,
	vertexOffset: int32_t,
	firstInstance: u32
) -> VkResult;

/** vkCmdDrawMultiEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	drawCount

	const *
	VkMultiDrawInfoEXT
	pVertexInfo

	
	uint32_t
	instanceCount

	
	uint32_t
	firstInstance

	
	uint32_t
	stride

**/
pub type PFN_vkCmdDrawMultiEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	drawCount: u32,
	pVertexInfo: *const VkMultiDrawInfoEXT,
	instanceCount: u32,
	firstInstance: u32,
	stride: u32
) -> VkResult;

/** vkCmdDrawMultiIndexedEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	drawCount

	const *
	VkMultiDrawIndexedInfoEXT
	pIndexInfo

	
	uint32_t
	instanceCount

	
	uint32_t
	firstInstance

	
	uint32_t
	stride

	const *
	int32_t
	pVertexOffset

**/
pub type PFN_vkCmdDrawMultiIndexedEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	drawCount: u32,
	pIndexInfo: *const VkMultiDrawIndexedInfoEXT,
	instanceCount: u32,
	firstInstance: u32,
	stride: u32,
	pVertexOffset: *const int32_t
) -> VkResult;

/** vkCmdDrawIndirect
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	buffer

	
	VkDeviceSize
	offset

	
	uint32_t
	drawCount

	
	uint32_t
	stride

**/
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	drawCount: u32,
	stride: u32
) -> VkResult;

/** vkCmdDrawIndexedIndirect
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	buffer

	
	VkDeviceSize
	offset

	
	uint32_t
	drawCount

	
	uint32_t
	stride

**/
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	drawCount: u32,
	stride: u32
) -> VkResult;

/** vkCmdDispatch
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	groupCountX

	
	uint32_t
	groupCountY

	
	uint32_t
	groupCountZ

**/
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	groupCountX: u32,
	groupCountY: u32,
	groupCountZ: u32
) -> VkResult;

/** vkCmdDispatchIndirect
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	buffer

	
	VkDeviceSize
	offset

**/
pub type PFN_vkCmdDispatchIndirect = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize
) -> VkResult;

/** vkCmdSubpassShadingHUAWEI
	
	VkCommandBuffer
	commandBuffer

**/
pub type PFN_vkCmdSubpassShadingHUAWEI = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

/** vkCmdCopyBuffer
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	srcBuffer

	
	VkBuffer
	dstBuffer

	
	uint32_t
	regionCount

	const *
	VkBufferCopy
	pRegions

**/
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcBuffer: VkBuffer,
	dstBuffer: VkBuffer,
	regionCount: u32,
	pRegions: *const VkBufferCopy
) -> VkResult;

/** vkCmdCopyImage
	
	VkCommandBuffer
	commandBuffer

	
	VkImage
	srcImage

	
	VkImageLayout
	srcImageLayout

	
	VkImage
	dstImage

	
	VkImageLayout
	dstImageLayout

	
	uint32_t
	regionCount

	const *
	VkImageCopy
	pRegions

**/
pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *const VkImageCopy
) -> VkResult;

/** vkCmdBlitImage
	
	VkCommandBuffer
	commandBuffer

	
	VkImage
	srcImage

	
	VkImageLayout
	srcImageLayout

	
	VkImage
	dstImage

	
	VkImageLayout
	dstImageLayout

	
	uint32_t
	regionCount

	const *
	VkImageBlit
	pRegions

	
	VkFilter
	filter

**/
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *const VkImageBlit,
	filter: VkFilter
) -> VkResult;

/** vkCmdCopyBufferToImage
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	srcBuffer

	
	VkImage
	dstImage

	
	VkImageLayout
	dstImageLayout

	
	uint32_t
	regionCount

	const *
	VkBufferImageCopy
	pRegions

**/
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcBuffer: VkBuffer,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *const VkBufferImageCopy
) -> VkResult;

/** vkCmdCopyImageToBuffer
	
	VkCommandBuffer
	commandBuffer

	
	VkImage
	srcImage

	
	VkImageLayout
	srcImageLayout

	
	VkBuffer
	dstBuffer

	
	uint32_t
	regionCount

	const *
	VkBufferImageCopy
	pRegions

**/
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstBuffer: VkBuffer,
	regionCount: u32,
	pRegions: *const VkBufferImageCopy
) -> VkResult;

/** vkCmdCopyMemoryIndirectNV
	
	VkCommandBuffer
	commandBuffer

	
	VkDeviceAddress
	copyBufferAddress

	
	uint32_t
	copyCount

	
	uint32_t
	stride

**/
pub type PFN_vkCmdCopyMemoryIndirectNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	copyBufferAddress: VkDeviceAddress,
	copyCount: u32,
	stride: u32
) -> VkResult;

/** vkCmdCopyMemoryToImageIndirectNV
	
	VkCommandBuffer
	commandBuffer

	
	VkDeviceAddress
	copyBufferAddress

	
	uint32_t
	copyCount

	
	uint32_t
	stride

	
	VkImage
	dstImage

	
	VkImageLayout
	dstImageLayout

	const *
	VkImageSubresourceLayers
	pImageSubresources

**/
pub type PFN_vkCmdCopyMemoryToImageIndirectNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	copyBufferAddress: VkDeviceAddress,
	copyCount: u32,
	stride: u32,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	pImageSubresources: *const VkImageSubresourceLayers
) -> VkResult;

/** vkCmdUpdateBuffer
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	dstBuffer

	
	VkDeviceSize
	dstOffset

	
	VkDeviceSize
	dataSize

	const *
	void
	pData

**/
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	dstBuffer: VkBuffer,
	dstOffset: VkDeviceSize,
	dataSize: VkDeviceSize,
	pData: *const void
) -> VkResult;

/** vkCmdFillBuffer
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	dstBuffer

	
	VkDeviceSize
	dstOffset

	
	VkDeviceSize
	size

	
	uint32_t
	data

**/
pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	dstBuffer: VkBuffer,
	dstOffset: VkDeviceSize,
	size: VkDeviceSize,
	data: u32
) -> VkResult;

/** vkCmdClearColorImage
	
	VkCommandBuffer
	commandBuffer

	
	VkImage
	image

	
	VkImageLayout
	imageLayout

	const *
	VkClearColorValue
	pColor

	
	uint32_t
	rangeCount

	const *
	VkImageSubresourceRange
	pRanges

**/
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	image: VkImage,
	imageLayout: VkImageLayout,
	pColor: *const VkClearColorValue,
	rangeCount: u32,
	pRanges: *const VkImageSubresourceRange
) -> VkResult;

/** vkCmdClearDepthStencilImage
	
	VkCommandBuffer
	commandBuffer

	
	VkImage
	image

	
	VkImageLayout
	imageLayout

	const *
	VkClearDepthStencilValue
	pDepthStencil

	
	uint32_t
	rangeCount

	const *
	VkImageSubresourceRange
	pRanges

**/
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	image: VkImage,
	imageLayout: VkImageLayout,
	pDepthStencil: *const VkClearDepthStencilValue,
	rangeCount: u32,
	pRanges: *const VkImageSubresourceRange
) -> VkResult;

/** vkCmdClearAttachments
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	attachmentCount

	const *
	VkClearAttachment
	pAttachments

	
	uint32_t
	rectCount

	const *
	VkClearRect
	pRects

**/
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	attachmentCount: u32,
	pAttachments: *const VkClearAttachment,
	rectCount: u32,
	pRects: *const VkClearRect
) -> VkResult;

/** vkCmdResolveImage
	
	VkCommandBuffer
	commandBuffer

	
	VkImage
	srcImage

	
	VkImageLayout
	srcImageLayout

	
	VkImage
	dstImage

	
	VkImageLayout
	dstImageLayout

	
	uint32_t
	regionCount

	const *
	VkImageResolve
	pRegions

**/
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *const VkImageResolve
) -> VkResult;

/** vkCmdSetEvent
	
	VkCommandBuffer
	commandBuffer

	
	VkEvent
	event

	
	VkPipelineStageFlags
	stageMask

**/
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	event: VkEvent,
	stageMask: VkPipelineStageFlags
) -> VkResult;

/** vkCmdResetEvent
	
	VkCommandBuffer
	commandBuffer

	
	VkEvent
	event

	
	VkPipelineStageFlags
	stageMask

**/
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	event: VkEvent,
	stageMask: VkPipelineStageFlags
) -> VkResult;

/** vkCmdWaitEvents
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	eventCount

	const *
	VkEvent
	pEvents

	
	VkPipelineStageFlags
	srcStageMask

	
	VkPipelineStageFlags
	dstStageMask

	
	uint32_t
	memoryBarrierCount

	const *
	VkMemoryBarrier
	pMemoryBarriers

	
	uint32_t
	bufferMemoryBarrierCount

	const *
	VkBufferMemoryBarrier
	pBufferMemoryBarriers

	
	uint32_t
	imageMemoryBarrierCount

	const *
	VkImageMemoryBarrier
	pImageMemoryBarriers

**/
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	eventCount: u32,
	pEvents: *const VkEvent,
	srcStageMask: VkPipelineStageFlags,
	dstStageMask: VkPipelineStageFlags,
	memoryBarrierCount: u32,
	pMemoryBarriers: *const VkMemoryBarrier,
	bufferMemoryBarrierCount: u32,
	pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
	imageMemoryBarrierCount: u32,
	pImageMemoryBarriers: *const VkImageMemoryBarrier
) -> VkResult;

/** vkCmdPipelineBarrier
	
	VkCommandBuffer
	commandBuffer

	
	VkPipelineStageFlags
	srcStageMask

	
	VkPipelineStageFlags
	dstStageMask

	
	VkDependencyFlags
	dependencyFlags

	
	uint32_t
	memoryBarrierCount

	const *
	VkMemoryBarrier
	pMemoryBarriers

	
	uint32_t
	bufferMemoryBarrierCount

	const *
	VkBufferMemoryBarrier
	pBufferMemoryBarriers

	
	uint32_t
	imageMemoryBarrierCount

	const *
	VkImageMemoryBarrier
	pImageMemoryBarriers

**/
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	srcStageMask: VkPipelineStageFlags,
	dstStageMask: VkPipelineStageFlags,
	dependencyFlags: VkDependencyFlags,
	memoryBarrierCount: u32,
	pMemoryBarriers: *const VkMemoryBarrier,
	bufferMemoryBarrierCount: u32,
	pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
	imageMemoryBarrierCount: u32,
	pImageMemoryBarriers: *const VkImageMemoryBarrier
) -> VkResult;

/** vkCmdBeginQuery
	
	VkCommandBuffer
	commandBuffer

	
	VkQueryPool
	queryPool

	
	uint32_t
	query

	
	VkQueryControlFlags
	flags

**/
pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	query: u32,
	flags: VkQueryControlFlags
) -> VkResult;

/** vkCmdEndQuery
	
	VkCommandBuffer
	commandBuffer

	
	VkQueryPool
	queryPool

	
	uint32_t
	query

**/
pub type PFN_vkCmdEndQuery = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	query: u32
) -> VkResult;

/** vkCmdBeginConditionalRenderingEXT
	
	VkCommandBuffer
	commandBuffer

	const *
	VkConditionalRenderingBeginInfoEXT
	pConditionalRenderingBegin

**/
pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfoEXT
) -> VkResult;

/** vkCmdEndConditionalRenderingEXT
	
	VkCommandBuffer
	commandBuffer

**/
pub type PFN_vkCmdEndConditionalRenderingEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

/** vkCmdResetQueryPool
	
	VkCommandBuffer
	commandBuffer

	
	VkQueryPool
	queryPool

	
	uint32_t
	firstQuery

	
	uint32_t
	queryCount

**/
pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	firstQuery: u32,
	queryCount: u32
) -> VkResult;

/** vkCmdWriteTimestamp
	
	VkCommandBuffer
	commandBuffer

	
	VkPipelineStageFlagBits
	pipelineStage

	
	VkQueryPool
	queryPool

	
	uint32_t
	query

**/
pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineStage: VkPipelineStageFlagBits,
	queryPool: VkQueryPool,
	query: u32
) -> VkResult;

/** vkCmdCopyQueryPoolResults
	
	VkCommandBuffer
	commandBuffer

	
	VkQueryPool
	queryPool

	
	uint32_t
	firstQuery

	
	uint32_t
	queryCount

	
	VkBuffer
	dstBuffer

	
	VkDeviceSize
	dstOffset

	
	VkDeviceSize
	stride

	
	VkQueryResultFlags
	flags

**/
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

/** vkCmdPushConstants
	
	VkCommandBuffer
	commandBuffer

	
	VkPipelineLayout
	layout

	
	VkShaderStageFlags
	stageFlags

	
	uint32_t
	offset

	
	uint32_t
	size

	const *
	void
	pValues

**/
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	layout: VkPipelineLayout,
	stageFlags: VkShaderStageFlags,
	offset: u32,
	size: u32,
	pValues: *const void
) -> VkResult;

/** vkCmdBeginRenderPass
	
	VkCommandBuffer
	commandBuffer

	const *
	VkRenderPassBeginInfo
	pRenderPassBegin

	
	VkSubpassContents
	contents

**/
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pRenderPassBegin: *const VkRenderPassBeginInfo,
	contents: VkSubpassContents
) -> VkResult;

/** vkCmdNextSubpass
	
	VkCommandBuffer
	commandBuffer

	
	VkSubpassContents
	contents

**/
pub type PFN_vkCmdNextSubpass = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	contents: VkSubpassContents
) -> VkResult;

/** vkCmdEndRenderPass
	
	VkCommandBuffer
	commandBuffer

**/
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

/** vkCmdExecuteCommands
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	commandBufferCount

	const *
	VkCommandBuffer
	pCommandBuffers

**/
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	commandBufferCount: u32,
	pCommandBuffers: *const VkCommandBuffer
) -> VkResult;

/** vkCreateAndroidSurfaceKHR
	
	VkInstance
	instance

	const *
	VkAndroidSurfaceCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkGetPhysicalDeviceDisplayPropertiesKHR
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pPropertyCount

	*
	VkDisplayPropertiesKHR
	pProperties

**/
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *const u32,
	pProperties: *const VkDisplayPropertiesKHR
) -> VkResult;

/** vkGetPhysicalDeviceDisplayPlanePropertiesKHR
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pPropertyCount

	*
	VkDisplayPlanePropertiesKHR
	pProperties

**/
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *const u32,
	pProperties: *const VkDisplayPlanePropertiesKHR
) -> VkResult;

/** vkGetDisplayPlaneSupportedDisplaysKHR
	
	VkPhysicalDevice
	physicalDevice

	
	uint32_t
	planeIndex

	*
	uint32_t
	pDisplayCount

	*
	VkDisplayKHR
	pDisplays

**/
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	planeIndex: u32,
	pDisplayCount: *const u32,
	pDisplays: *const VkDisplayKHR
) -> VkResult;

/** vkGetDisplayModePropertiesKHR
	
	VkPhysicalDevice
	physicalDevice

	
	VkDisplayKHR
	display

	*
	uint32_t
	pPropertyCount

	*
	VkDisplayModePropertiesKHR
	pProperties

**/
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR,
	pPropertyCount: *const u32,
	pProperties: *const VkDisplayModePropertiesKHR
) -> VkResult;

/** vkCreateDisplayModeKHR
	
	VkPhysicalDevice
	physicalDevice

	
	VkDisplayKHR
	display

	const *
	VkDisplayModeCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkDisplayModeKHR
	pMode

**/
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR,
	pCreateInfo: *const VkDisplayModeCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pMode: *const VkDisplayModeKHR
) -> VkResult;

/** vkGetDisplayPlaneCapabilitiesKHR
	
	VkPhysicalDevice
	physicalDevice

	
	VkDisplayModeKHR
	mode

	
	uint32_t
	planeIndex

	*
	VkDisplayPlaneCapabilitiesKHR
	pCapabilities

**/
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	mode: VkDisplayModeKHR,
	planeIndex: u32,
	pCapabilities: *const VkDisplayPlaneCapabilitiesKHR
) -> VkResult;

/** vkCreateDisplayPlaneSurfaceKHR
	
	VkInstance
	instance

	const *
	VkDisplaySurfaceCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkCreateSharedSwapchainsKHR
	
	VkDevice
	device

	
	uint32_t
	swapchainCount

	const *
	VkSwapchainCreateInfoKHR
	pCreateInfos

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSwapchainKHR
	pSwapchains

**/
pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchainCount: u32,
	pCreateInfos: *const VkSwapchainCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pSwapchains: *const VkSwapchainKHR
) -> VkResult;

/** vkDestroySurfaceKHR
	
	VkInstance
	instance

	
	VkSurfaceKHR
	surface

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	surface: VkSurfaceKHR,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetPhysicalDeviceSurfaceSupportKHR
	
	VkPhysicalDevice
	physicalDevice

	
	uint32_t
	queueFamilyIndex

	
	VkSurfaceKHR
	surface

	*
	VkBool32
	pSupported

**/
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	surface: VkSurfaceKHR,
	pSupported: *const VkBool32
) -> VkResult;

/** vkGetPhysicalDeviceSurfaceCapabilitiesKHR
	
	VkPhysicalDevice
	physicalDevice

	
	VkSurfaceKHR
	surface

	*
	VkSurfaceCapabilitiesKHR
	pSurfaceCapabilities

**/
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	surface: VkSurfaceKHR,
	pSurfaceCapabilities: *const VkSurfaceCapabilitiesKHR
) -> VkResult;

/** vkGetPhysicalDeviceSurfaceFormatsKHR
	
	VkPhysicalDevice
	physicalDevice

	
	VkSurfaceKHR
	surface

	*
	uint32_t
	pSurfaceFormatCount

	*
	VkSurfaceFormatKHR
	pSurfaceFormats

**/
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	surface: VkSurfaceKHR,
	pSurfaceFormatCount: *const u32,
	pSurfaceFormats: *const VkSurfaceFormatKHR
) -> VkResult;

/** vkGetPhysicalDeviceSurfacePresentModesKHR
	
	VkPhysicalDevice
	physicalDevice

	
	VkSurfaceKHR
	surface

	*
	uint32_t
	pPresentModeCount

	*
	VkPresentModeKHR
	pPresentModes

**/
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	surface: VkSurfaceKHR,
	pPresentModeCount: *const u32,
	pPresentModes: *const VkPresentModeKHR
) -> VkResult;

/** vkCreateSwapchainKHR
	
	VkDevice
	device

	const *
	VkSwapchainCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSwapchainKHR
	pSwapchain

**/
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkSwapchainCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pSwapchain: *const VkSwapchainKHR
) -> VkResult;

/** vkDestroySwapchainKHR
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapchain

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetSwapchainImagesKHR
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapchain

	*
	uint32_t
	pSwapchainImageCount

	*
	VkImage
	pSwapchainImages

**/
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	pSwapchainImageCount: *const u32,
	pSwapchainImages: *const VkImage
) -> VkResult;

/** vkAcquireNextImageKHR
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapchain

	
	uint64_t
	timeout

	
	VkSemaphore
	semaphore

	
	VkFence
	fence

	*
	uint32_t
	pImageIndex

**/
pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	timeout: uint64_t,
	semaphore: VkSemaphore,
	fence: VkFence,
	pImageIndex: *const u32
) -> VkResult;

/** vkQueuePresentKHR
	
	VkQueue
	queue

	const *
	VkPresentInfoKHR
	pPresentInfo

**/
pub type PFN_vkQueuePresentKHR = unsafe extern "system" fn(
	queue: VkQueue,
	pPresentInfo: *const VkPresentInfoKHR
) -> VkResult;

/** vkCreateViSurfaceNN
	
	VkInstance
	instance

	const *
	VkViSurfaceCreateInfoNN
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkViSurfaceCreateInfoNN,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkCreateWaylandSurfaceKHR
	
	VkInstance
	instance

	const *
	VkWaylandSurfaceCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkGetPhysicalDeviceWaylandPresentationSupportKHR
	
	VkPhysicalDevice
	physicalDevice

	
	uint32_t
	queueFamilyIndex

	struct *
	wl_display
	display

**/
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	display: struct * wl_display
) -> VkResult;

/** vkCreateWin32SurfaceKHR
	
	VkInstance
	instance

	const *
	VkWin32SurfaceCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkGetPhysicalDeviceWin32PresentationSupportKHR
	
	VkPhysicalDevice
	physicalDevice

	
	uint32_t
	queueFamilyIndex

**/
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32
) -> VkResult;

/** vkCreateXlibSurfaceKHR
	
	VkInstance
	instance

	const *
	VkXlibSurfaceCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkGetPhysicalDeviceXlibPresentationSupportKHR
	
	VkPhysicalDevice
	physicalDevice

	
	uint32_t
	queueFamilyIndex

	*
	Display
	dpy

	
	VisualID
	visualID

**/
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	dpy: *const Display,
	visualID: VisualID
) -> VkResult;

/** vkCreateXcbSurfaceKHR
	
	VkInstance
	instance

	const *
	VkXcbSurfaceCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkGetPhysicalDeviceXcbPresentationSupportKHR
	
	VkPhysicalDevice
	physicalDevice

	
	uint32_t
	queueFamilyIndex

	*
	xcb_connection_t
	connection

	
	xcb_visualid_t
	visual_id

**/
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	connection: *const xcb_connection_t,
	visual_id: xcb_visualid_t
) -> VkResult;

/** vkCreateDirectFBSurfaceEXT
	
	VkInstance
	instance

	const *
	VkDirectFBSurfaceCreateInfoEXT
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkDirectFBSurfaceCreateInfoEXT,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkGetPhysicalDeviceDirectFBPresentationSupportEXT
	
	VkPhysicalDevice
	physicalDevice

	
	uint32_t
	queueFamilyIndex

	*
	IDirectFB
	dfb

**/
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	dfb: *const IDirectFB
) -> VkResult;

/** vkCreateImagePipeSurfaceFUCHSIA
	
	VkInstance
	instance

	const *
	VkImagePipeSurfaceCreateInfoFUCHSIA
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkImagePipeSurfaceCreateInfoFUCHSIA,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkCreateStreamDescriptorSurfaceGGP
	
	VkInstance
	instance

	const *
	VkStreamDescriptorSurfaceCreateInfoGGP
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkStreamDescriptorSurfaceCreateInfoGGP,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkCreateScreenSurfaceQNX
	
	VkInstance
	instance

	const *
	VkScreenSurfaceCreateInfoQNX
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkScreenSurfaceCreateInfoQNX,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkGetPhysicalDeviceScreenPresentationSupportQNX
	
	VkPhysicalDevice
	physicalDevice

	
	uint32_t
	queueFamilyIndex

	struct *
	_screen_window
	window

**/
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	window: struct * _screen_window
) -> VkResult;

/** vkCreateDebugReportCallbackEXT
	
	VkInstance
	instance

	const *
	VkDebugReportCallbackCreateInfoEXT
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkDebugReportCallbackEXT
	pCallback

**/
pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
	pAllocator: *const VkAllocationCallbacks,
	pCallback: *const VkDebugReportCallbackEXT
) -> VkResult;

/** vkDestroyDebugReportCallbackEXT
	
	VkInstance
	instance

	
	VkDebugReportCallbackEXT
	callback

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(
	instance: VkInstance,
	callback: VkDebugReportCallbackEXT,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkDebugReportMessageEXT
	
	VkInstance
	instance

	
	VkDebugReportFlagsEXT
	flags

	
	VkDebugReportObjectTypeEXT
	objectType

	
	uint64_t
	object

	
	size_t
	location

	
	int32_t
	messageCode

	const *
	char
	pLayerPrefix

	const *
	char
	pMessage

**/
pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(
	instance: VkInstance,
	flags: VkDebugReportFlagsEXT,
	objectType: VkDebugReportObjectTypeEXT,
	object: uint64_t,
	location: size_t,
	messageCode: int32_t,
	pLayerPrefix: *const char,
	pMessage: *const char
) -> VkResult;

/** vkDebugMarkerSetObjectNameEXT
	
	VkDevice
	device

	const *
	VkDebugMarkerObjectNameInfoEXT
	pNameInfo

**/
pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
	device: VkDevice,
	pNameInfo: *const VkDebugMarkerObjectNameInfoEXT
) -> VkResult;

/** vkDebugMarkerSetObjectTagEXT
	
	VkDevice
	device

	const *
	VkDebugMarkerObjectTagInfoEXT
	pTagInfo

**/
pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
	device: VkDevice,
	pTagInfo: *const VkDebugMarkerObjectTagInfoEXT
) -> VkResult;

/** vkCmdDebugMarkerBeginEXT
	
	VkCommandBuffer
	commandBuffer

	const *
	VkDebugMarkerMarkerInfoEXT
	pMarkerInfo

**/
pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT
) -> VkResult;

/** vkCmdDebugMarkerEndEXT
	
	VkCommandBuffer
	commandBuffer

**/
pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

/** vkCmdDebugMarkerInsertEXT
	
	VkCommandBuffer
	commandBuffer

	const *
	VkDebugMarkerMarkerInfoEXT
	pMarkerInfo

**/
pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT
) -> VkResult;

/** vkGetPhysicalDeviceExternalImageFormatPropertiesNV
	
	VkPhysicalDevice
	physicalDevice

	
	VkFormat
	format

	
	VkImageType
	type

	
	VkImageTiling
	tiling

	
	VkImageUsageFlags
	usage

	
	VkImageCreateFlags
	flags

	
	VkExternalMemoryHandleTypeFlagsNV
	externalHandleType

	*
	VkExternalImageFormatPropertiesNV
	pExternalImageFormatProperties

**/
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	type: VkImageType,
	tiling: VkImageTiling,
	usage: VkImageUsageFlags,
	flags: VkImageCreateFlags,
	externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
	pExternalImageFormatProperties: *const VkExternalImageFormatPropertiesNV
) -> VkResult;

/** vkGetMemoryWin32HandleNV
	
	VkDevice
	device

	
	VkDeviceMemory
	memory

	
	VkExternalMemoryHandleTypeFlagsNV
	handleType

	*
	HANDLE
	pHandle

**/
pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory,
	handleType: VkExternalMemoryHandleTypeFlagsNV,
	pHandle: *const HANDLE
) -> VkResult;

/** vkCmdExecuteGeneratedCommandsNV
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	isPreprocessed

	const *
	VkGeneratedCommandsInfoNV
	pGeneratedCommandsInfo

**/
pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	isPreprocessed: VkBool32,
	pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV
) -> VkResult;

/** vkCmdPreprocessGeneratedCommandsNV
	
	VkCommandBuffer
	commandBuffer

	const *
	VkGeneratedCommandsInfoNV
	pGeneratedCommandsInfo

**/
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV
) -> VkResult;

/** vkCmdBindPipelineShaderGroupNV
	
	VkCommandBuffer
	commandBuffer

	
	VkPipelineBindPoint
	pipelineBindPoint

	
	VkPipeline
	pipeline

	
	uint32_t
	groupIndex

**/
pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineBindPoint: VkPipelineBindPoint,
	pipeline: VkPipeline,
	groupIndex: u32
) -> VkResult;

/** vkGetGeneratedCommandsMemoryRequirementsNV
	
	VkDevice
	device

	const *
	VkGeneratedCommandsMemoryRequirementsInfoNV
	pInfo

	*
	VkMemoryRequirements2
	pMemoryRequirements

**/
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoNV,
	pMemoryRequirements: *const VkMemoryRequirements2
) -> VkResult;

/** vkCreateIndirectCommandsLayoutNV
	
	VkDevice
	device

	const *
	VkIndirectCommandsLayoutCreateInfoNV
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkIndirectCommandsLayoutNV
	pIndirectCommandsLayout

**/
pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNV,
	pAllocator: *const VkAllocationCallbacks,
	pIndirectCommandsLayout: *const VkIndirectCommandsLayoutNV
) -> VkResult;

/** vkDestroyIndirectCommandsLayoutNV
	
	VkDevice
	device

	
	VkIndirectCommandsLayoutNV
	indirectCommandsLayout

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
	device: VkDevice,
	indirectCommandsLayout: VkIndirectCommandsLayoutNV,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetPhysicalDeviceFeatures2
	
	VkPhysicalDevice
	physicalDevice

	*
	VkPhysicalDeviceFeatures2
	pFeatures

**/
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pFeatures: *const VkPhysicalDeviceFeatures2
) -> VkResult;

/** vkGetPhysicalDeviceProperties2
	
	VkPhysicalDevice
	physicalDevice

	*
	VkPhysicalDeviceProperties2
	pProperties

**/
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pProperties: *const VkPhysicalDeviceProperties2
) -> VkResult;

/** vkGetPhysicalDeviceFormatProperties2
	
	VkPhysicalDevice
	physicalDevice

	
	VkFormat
	format

	*
	VkFormatProperties2
	pFormatProperties

**/
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	pFormatProperties: *const VkFormatProperties2
) -> VkResult;

/** vkGetPhysicalDeviceImageFormatProperties2
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkPhysicalDeviceImageFormatInfo2
	pImageFormatInfo

	*
	VkImageFormatProperties2
	pImageFormatProperties

**/
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
	pImageFormatProperties: *const VkImageFormatProperties2
) -> VkResult;

/** vkGetPhysicalDeviceQueueFamilyProperties2
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pQueueFamilyPropertyCount

	*
	VkQueueFamilyProperties2
	pQueueFamilyProperties

**/
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pQueueFamilyPropertyCount: *const u32,
	pQueueFamilyProperties: *const VkQueueFamilyProperties2
) -> VkResult;

/** vkGetPhysicalDeviceMemoryProperties2
	
	VkPhysicalDevice
	physicalDevice

	*
	VkPhysicalDeviceMemoryProperties2
	pMemoryProperties

**/
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pMemoryProperties: *const VkPhysicalDeviceMemoryProperties2
) -> VkResult;

/** vkGetPhysicalDeviceSparseImageFormatProperties2
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkPhysicalDeviceSparseImageFormatInfo2
	pFormatInfo

	*
	uint32_t
	pPropertyCount

	*
	VkSparseImageFormatProperties2
	pProperties

**/
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
	pPropertyCount: *const u32,
	pProperties: *const VkSparseImageFormatProperties2
) -> VkResult;

/** vkCmdPushDescriptorSetKHR
	
	VkCommandBuffer
	commandBuffer

	
	VkPipelineBindPoint
	pipelineBindPoint

	
	VkPipelineLayout
	layout

	
	uint32_t
	set

	
	uint32_t
	descriptorWriteCount

	const *
	VkWriteDescriptorSet
	pDescriptorWrites

**/
pub type PFN_vkCmdPushDescriptorSetKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineBindPoint: VkPipelineBindPoint,
	layout: VkPipelineLayout,
	set: u32,
	descriptorWriteCount: u32,
	pDescriptorWrites: *const VkWriteDescriptorSet
) -> VkResult;

/** vkTrimCommandPool
	
	VkDevice
	device

	
	VkCommandPool
	commandPool

	
	VkCommandPoolTrimFlags
	flags

**/
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
	device: VkDevice,
	commandPool: VkCommandPool,
	flags: VkCommandPoolTrimFlags
) -> VkResult;

/** vkGetPhysicalDeviceExternalBufferProperties
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkPhysicalDeviceExternalBufferInfo
	pExternalBufferInfo

	*
	VkExternalBufferProperties
	pExternalBufferProperties

**/
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
	pExternalBufferProperties: *const VkExternalBufferProperties
) -> VkResult;

/** vkGetMemoryWin32HandleKHR
	
	VkDevice
	device

	const *
	VkMemoryGetWin32HandleInfoKHR
	pGetWin32HandleInfo

	*
	HANDLE
	pHandle

**/
pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
	pHandle: *const HANDLE
) -> VkResult;

/** vkGetMemoryWin32HandlePropertiesKHR
	
	VkDevice
	device

	
	VkExternalMemoryHandleTypeFlagBits
	handleType

	
	HANDLE
	handle

	*
	VkMemoryWin32HandlePropertiesKHR
	pMemoryWin32HandleProperties

**/
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(
	device: VkDevice,
	handleType: VkExternalMemoryHandleTypeFlagBits,
	handle: HANDLE,
	pMemoryWin32HandleProperties: *const VkMemoryWin32HandlePropertiesKHR
) -> VkResult;

/** vkGetMemoryFdKHR
	
	VkDevice
	device

	const *
	VkMemoryGetFdInfoKHR
	pGetFdInfo

	*
	int
	pFd

**/
pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetFdInfo: *const VkMemoryGetFdInfoKHR,
	pFd: *const int
) -> VkResult;

/** vkGetMemoryFdPropertiesKHR
	
	VkDevice
	device

	
	VkExternalMemoryHandleTypeFlagBits
	handleType

	
	int
	fd

	*
	VkMemoryFdPropertiesKHR
	pMemoryFdProperties

**/
pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
	device: VkDevice,
	handleType: VkExternalMemoryHandleTypeFlagBits,
	fd: int,
	pMemoryFdProperties: *const VkMemoryFdPropertiesKHR
) -> VkResult;

/** vkGetMemoryZirconHandleFUCHSIA
	
	VkDevice
	device

	const *
	VkMemoryGetZirconHandleInfoFUCHSIA
	pGetZirconHandleInfo

	*
	zx_handle_t
	pZirconHandle

**/
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	pGetZirconHandleInfo: *const VkMemoryGetZirconHandleInfoFUCHSIA,
	pZirconHandle: *const zx_handle_t
) -> VkResult;

/** vkGetMemoryZirconHandlePropertiesFUCHSIA
	
	VkDevice
	device

	
	VkExternalMemoryHandleTypeFlagBits
	handleType

	
	zx_handle_t
	zirconHandle

	*
	VkMemoryZirconHandlePropertiesFUCHSIA
	pMemoryZirconHandleProperties

**/
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	handleType: VkExternalMemoryHandleTypeFlagBits,
	zirconHandle: zx_handle_t,
	pMemoryZirconHandleProperties: *const VkMemoryZirconHandlePropertiesFUCHSIA
) -> VkResult;

/** vkGetMemoryRemoteAddressNV
	
	VkDevice
	device

	const *
	VkMemoryGetRemoteAddressInfoNV
	pMemoryGetRemoteAddressInfo

	*
	VkRemoteAddressNV
	pAddress

**/
pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
	device: VkDevice,
	pMemoryGetRemoteAddressInfo: *const VkMemoryGetRemoteAddressInfoNV,
	pAddress: *const VkRemoteAddressNV
) -> VkResult;

/** vkGetPhysicalDeviceExternalSemaphoreProperties
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkPhysicalDeviceExternalSemaphoreInfo
	pExternalSemaphoreInfo

	*
	VkExternalSemaphoreProperties
	pExternalSemaphoreProperties

**/
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
	pExternalSemaphoreProperties: *const VkExternalSemaphoreProperties
) -> VkResult;

/** vkGetSemaphoreWin32HandleKHR
	
	VkDevice
	device

	const *
	VkSemaphoreGetWin32HandleInfoKHR
	pGetWin32HandleInfo

	*
	HANDLE
	pHandle

**/
pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR,
	pHandle: *const HANDLE
) -> VkResult;

/** vkImportSemaphoreWin32HandleKHR
	
	VkDevice
	device

	const *
	VkImportSemaphoreWin32HandleInfoKHR
	pImportSemaphoreWin32HandleInfo

**/
pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(
	device: VkDevice,
	pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR
) -> VkResult;

/** vkGetSemaphoreFdKHR
	
	VkDevice
	device

	const *
	VkSemaphoreGetFdInfoKHR
	pGetFdInfo

	*
	int
	pFd

**/
pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetFdInfo: *const VkSemaphoreGetFdInfoKHR,
	pFd: *const int
) -> VkResult;

/** vkImportSemaphoreFdKHR
	
	VkDevice
	device

	const *
	VkImportSemaphoreFdInfoKHR
	pImportSemaphoreFdInfo

**/
pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn(
	device: VkDevice,
	pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR
) -> VkResult;

/** vkGetSemaphoreZirconHandleFUCHSIA
	
	VkDevice
	device

	const *
	VkSemaphoreGetZirconHandleInfoFUCHSIA
	pGetZirconHandleInfo

	*
	zx_handle_t
	pZirconHandle

**/
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	pGetZirconHandleInfo: *const VkSemaphoreGetZirconHandleInfoFUCHSIA,
	pZirconHandle: *const zx_handle_t
) -> VkResult;

/** vkImportSemaphoreZirconHandleFUCHSIA
	
	VkDevice
	device

	const *
	VkImportSemaphoreZirconHandleInfoFUCHSIA
	pImportSemaphoreZirconHandleInfo

**/
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	pImportSemaphoreZirconHandleInfo: *const VkImportSemaphoreZirconHandleInfoFUCHSIA
) -> VkResult;

/** vkGetPhysicalDeviceExternalFenceProperties
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkPhysicalDeviceExternalFenceInfo
	pExternalFenceInfo

	*
	VkExternalFenceProperties
	pExternalFenceProperties

**/
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
	pExternalFenceProperties: *const VkExternalFenceProperties
) -> VkResult;

/** vkGetFenceWin32HandleKHR
	
	VkDevice
	device

	const *
	VkFenceGetWin32HandleInfoKHR
	pGetWin32HandleInfo

	*
	HANDLE
	pHandle

**/
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR,
	pHandle: *const HANDLE
) -> VkResult;

/** vkImportFenceWin32HandleKHR
	
	VkDevice
	device

	const *
	VkImportFenceWin32HandleInfoKHR
	pImportFenceWin32HandleInfo

**/
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
	device: VkDevice,
	pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR
) -> VkResult;

/** vkGetFenceFdKHR
	
	VkDevice
	device

	const *
	VkFenceGetFdInfoKHR
	pGetFdInfo

	*
	int
	pFd

**/
pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
	device: VkDevice,
	pGetFdInfo: *const VkFenceGetFdInfoKHR,
	pFd: *const int
) -> VkResult;

/** vkImportFenceFdKHR
	
	VkDevice
	device

	const *
	VkImportFenceFdInfoKHR
	pImportFenceFdInfo

**/
pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(
	device: VkDevice,
	pImportFenceFdInfo: *const VkImportFenceFdInfoKHR
) -> VkResult;

/** vkReleaseDisplayEXT
	
	VkPhysicalDevice
	physicalDevice

	
	VkDisplayKHR
	display

**/
pub type PFN_vkReleaseDisplayEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR
) -> VkResult;

/** vkAcquireXlibDisplayEXT
	
	VkPhysicalDevice
	physicalDevice

	*
	Display
	dpy

	
	VkDisplayKHR
	display

**/
pub type PFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	dpy: *const Display,
	display: VkDisplayKHR
) -> VkResult;

/** vkGetRandROutputDisplayEXT
	
	VkPhysicalDevice
	physicalDevice

	*
	Display
	dpy

	
	RROutput
	rrOutput

	*
	VkDisplayKHR
	pDisplay

**/
pub type PFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	dpy: *const Display,
	rrOutput: RROutput,
	pDisplay: *const VkDisplayKHR
) -> VkResult;

/** vkAcquireWinrtDisplayNV
	
	VkPhysicalDevice
	physicalDevice

	
	VkDisplayKHR
	display

**/
pub type PFN_vkAcquireWinrtDisplayNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR
) -> VkResult;

/** vkGetWinrtDisplayNV
	
	VkPhysicalDevice
	physicalDevice

	
	uint32_t
	deviceRelativeId

	*
	VkDisplayKHR
	pDisplay

**/
pub type PFN_vkGetWinrtDisplayNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	deviceRelativeId: u32,
	pDisplay: *const VkDisplayKHR
) -> VkResult;

/** vkDisplayPowerControlEXT
	
	VkDevice
	device

	
	VkDisplayKHR
	display

	const *
	VkDisplayPowerInfoEXT
	pDisplayPowerInfo

**/
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(
	device: VkDevice,
	display: VkDisplayKHR,
	pDisplayPowerInfo: *const VkDisplayPowerInfoEXT
) -> VkResult;

/** vkRegisterDeviceEventEXT
	
	VkDevice
	device

	const *
	VkDeviceEventInfoEXT
	pDeviceEventInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkFence
	pFence

**/
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
	device: VkDevice,
	pDeviceEventInfo: *const VkDeviceEventInfoEXT,
	pAllocator: *const VkAllocationCallbacks,
	pFence: *const VkFence
) -> VkResult;

/** vkRegisterDisplayEventEXT
	
	VkDevice
	device

	
	VkDisplayKHR
	display

	const *
	VkDisplayEventInfoEXT
	pDisplayEventInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkFence
	pFence

**/
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
	device: VkDevice,
	display: VkDisplayKHR,
	pDisplayEventInfo: *const VkDisplayEventInfoEXT,
	pAllocator: *const VkAllocationCallbacks,
	pFence: *const VkFence
) -> VkResult;

/** vkGetSwapchainCounterEXT
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapchain

	
	VkSurfaceCounterFlagBitsEXT
	counter

	*
	uint64_t
	pCounterValue

**/
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	counter: VkSurfaceCounterFlagBitsEXT,
	pCounterValue: *const uint64_t
) -> VkResult;

/** vkGetPhysicalDeviceSurfaceCapabilities2EXT
	
	VkPhysicalDevice
	physicalDevice

	
	VkSurfaceKHR
	surface

	*
	VkSurfaceCapabilities2EXT
	pSurfaceCapabilities

**/
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	surface: VkSurfaceKHR,
	pSurfaceCapabilities: *const VkSurfaceCapabilities2EXT
) -> VkResult;

/** vkEnumeratePhysicalDeviceGroups
	
	VkInstance
	instance

	*
	uint32_t
	pPhysicalDeviceGroupCount

	*
	VkPhysicalDeviceGroupProperties
	pPhysicalDeviceGroupProperties

**/
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
	instance: VkInstance,
	pPhysicalDeviceGroupCount: *const u32,
	pPhysicalDeviceGroupProperties: *const VkPhysicalDeviceGroupProperties
) -> VkResult;

/** vkGetDeviceGroupPeerMemoryFeatures
	
	VkDevice
	device

	
	uint32_t
	heapIndex

	
	uint32_t
	localDeviceIndex

	
	uint32_t
	remoteDeviceIndex

	*
	VkPeerMemoryFeatureFlags
	pPeerMemoryFeatures

**/
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
	device: VkDevice,
	heapIndex: u32,
	localDeviceIndex: u32,
	remoteDeviceIndex: u32,
	pPeerMemoryFeatures: *const VkPeerMemoryFeatureFlags
) -> VkResult;

/** vkBindBufferMemory2
	
	VkDevice
	device

	
	uint32_t
	bindInfoCount

	const *
	VkBindBufferMemoryInfo
	pBindInfos

**/
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
	device: VkDevice,
	bindInfoCount: u32,
	pBindInfos: *const VkBindBufferMemoryInfo
) -> VkResult;

/** vkBindImageMemory2
	
	VkDevice
	device

	
	uint32_t
	bindInfoCount

	const *
	VkBindImageMemoryInfo
	pBindInfos

**/
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
	device: VkDevice,
	bindInfoCount: u32,
	pBindInfos: *const VkBindImageMemoryInfo
) -> VkResult;

/** vkCmdSetDeviceMask
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	deviceMask

**/
pub type PFN_vkCmdSetDeviceMask = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	deviceMask: u32
) -> VkResult;

/** vkGetDeviceGroupPresentCapabilitiesKHR
	
	VkDevice
	device

	*
	VkDeviceGroupPresentCapabilitiesKHR
	pDeviceGroupPresentCapabilities

**/
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(
	device: VkDevice,
	pDeviceGroupPresentCapabilities: *const VkDeviceGroupPresentCapabilitiesKHR
) -> VkResult;

/** vkGetDeviceGroupSurfacePresentModesKHR
	
	VkDevice
	device

	
	VkSurfaceKHR
	surface

	*
	VkDeviceGroupPresentModeFlagsKHR
	pModes

**/
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(
	device: VkDevice,
	surface: VkSurfaceKHR,
	pModes: *const VkDeviceGroupPresentModeFlagsKHR
) -> VkResult;

/** vkAcquireNextImage2KHR
	
	VkDevice
	device

	const *
	VkAcquireNextImageInfoKHR
	pAcquireInfo

	*
	uint32_t
	pImageIndex

**/
pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(
	device: VkDevice,
	pAcquireInfo: *const VkAcquireNextImageInfoKHR,
	pImageIndex: *const u32
) -> VkResult;

/** vkCmdDispatchBase
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	baseGroupX

	
	uint32_t
	baseGroupY

	
	uint32_t
	baseGroupZ

	
	uint32_t
	groupCountX

	
	uint32_t
	groupCountY

	
	uint32_t
	groupCountZ

**/
pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	baseGroupX: u32,
	baseGroupY: u32,
	baseGroupZ: u32,
	groupCountX: u32,
	groupCountY: u32,
	groupCountZ: u32
) -> VkResult;

/** vkGetPhysicalDevicePresentRectanglesKHR
	
	VkPhysicalDevice
	physicalDevice

	
	VkSurfaceKHR
	surface

	*
	uint32_t
	pRectCount

	*
	VkRect2D
	pRects

**/
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	surface: VkSurfaceKHR,
	pRectCount: *const u32,
	pRects: *const VkRect2D
) -> VkResult;

/** vkCreateDescriptorUpdateTemplate
	
	VkDevice
	device

	const *
	VkDescriptorUpdateTemplateCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkDescriptorUpdateTemplate
	pDescriptorUpdateTemplate

**/
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pDescriptorUpdateTemplate: *const VkDescriptorUpdateTemplate
) -> VkResult;

/** vkDestroyDescriptorUpdateTemplate
	
	VkDevice
	device

	
	VkDescriptorUpdateTemplate
	descriptorUpdateTemplate

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
	device: VkDevice,
	descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkUpdateDescriptorSetWithTemplate
	
	VkDevice
	device

	
	VkDescriptorSet
	descriptorSet

	
	VkDescriptorUpdateTemplate
	descriptorUpdateTemplate

	const *
	void
	pData

**/
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
	device: VkDevice,
	descriptorSet: VkDescriptorSet,
	descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
	pData: *const void
) -> VkResult;

/** vkCmdPushDescriptorSetWithTemplateKHR
	
	VkCommandBuffer
	commandBuffer

	
	VkDescriptorUpdateTemplate
	descriptorUpdateTemplate

	
	VkPipelineLayout
	layout

	
	uint32_t
	set

	const *
	void
	pData

**/
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
	layout: VkPipelineLayout,
	set: u32,
	pData: *const void
) -> VkResult;

/** vkSetHdrMetadataEXT
	
	VkDevice
	device

	
	uint32_t
	swapchainCount

	const *
	VkSwapchainKHR
	pSwapchains

	const *
	VkHdrMetadataEXT
	pMetadata

**/
pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
	device: VkDevice,
	swapchainCount: u32,
	pSwapchains: *const VkSwapchainKHR,
	pMetadata: *const VkHdrMetadataEXT
) -> VkResult;

/** vkGetSwapchainStatusKHR
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapchain

**/
pub type PFN_vkGetSwapchainStatusKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR
) -> VkResult;

/** vkGetRefreshCycleDurationGOOGLE
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapchain

	*
	VkRefreshCycleDurationGOOGLE
	pDisplayTimingProperties

**/
pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	pDisplayTimingProperties: *const VkRefreshCycleDurationGOOGLE
) -> VkResult;

/** vkGetPastPresentationTimingGOOGLE
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapchain

	*
	uint32_t
	pPresentationTimingCount

	*
	VkPastPresentationTimingGOOGLE
	pPresentationTimings

**/
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	pPresentationTimingCount: *const u32,
	pPresentationTimings: *const VkPastPresentationTimingGOOGLE
) -> VkResult;

/** vkCreateIOSSurfaceMVK
	
	VkInstance
	instance

	const *
	VkIOSSurfaceCreateInfoMVK
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkIOSSurfaceCreateInfoMVK,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkCreateMacOSSurfaceMVK
	
	VkInstance
	instance

	const *
	VkMacOSSurfaceCreateInfoMVK
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkCreateMetalSurfaceEXT
	
	VkInstance
	instance

	const *
	VkMetalSurfaceCreateInfoEXT
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkMetalSurfaceCreateInfoEXT,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkCmdSetViewportWScalingNV
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstViewport

	
	uint32_t
	viewportCount

	const *
	VkViewportWScalingNV
	pViewportWScalings

**/
pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstViewport: u32,
	viewportCount: u32,
	pViewportWScalings: *const VkViewportWScalingNV
) -> VkResult;

/** vkCmdSetDiscardRectangleEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstDiscardRectangle

	
	uint32_t
	discardRectangleCount

	const *
	VkRect2D
	pDiscardRectangles

**/
pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstDiscardRectangle: u32,
	discardRectangleCount: u32,
	pDiscardRectangles: *const VkRect2D
) -> VkResult;

/** vkCmdSetSampleLocationsEXT
	
	VkCommandBuffer
	commandBuffer

	const *
	VkSampleLocationsInfoEXT
	pSampleLocationsInfo

**/
pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pSampleLocationsInfo: *const VkSampleLocationsInfoEXT
) -> VkResult;

/** vkGetPhysicalDeviceMultisamplePropertiesEXT
	
	VkPhysicalDevice
	physicalDevice

	
	VkSampleCountFlagBits
	samples

	*
	VkMultisamplePropertiesEXT
	pMultisampleProperties

**/
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	samples: VkSampleCountFlagBits,
	pMultisampleProperties: *const VkMultisamplePropertiesEXT
) -> VkResult;

/** vkGetPhysicalDeviceSurfaceCapabilities2KHR
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkPhysicalDeviceSurfaceInfo2KHR
	pSurfaceInfo

	*
	VkSurfaceCapabilities2KHR
	pSurfaceCapabilities

**/
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
	pSurfaceCapabilities: *const VkSurfaceCapabilities2KHR
) -> VkResult;

/** vkGetPhysicalDeviceSurfaceFormats2KHR
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkPhysicalDeviceSurfaceInfo2KHR
	pSurfaceInfo

	*
	uint32_t
	pSurfaceFormatCount

	*
	VkSurfaceFormat2KHR
	pSurfaceFormats

**/
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
	pSurfaceFormatCount: *const u32,
	pSurfaceFormats: *const VkSurfaceFormat2KHR
) -> VkResult;

/** vkGetPhysicalDeviceDisplayProperties2KHR
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pPropertyCount

	*
	VkDisplayProperties2KHR
	pProperties

**/
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *const u32,
	pProperties: *const VkDisplayProperties2KHR
) -> VkResult;

/** vkGetPhysicalDeviceDisplayPlaneProperties2KHR
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pPropertyCount

	*
	VkDisplayPlaneProperties2KHR
	pProperties

**/
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *const u32,
	pProperties: *const VkDisplayPlaneProperties2KHR
) -> VkResult;

/** vkGetDisplayModeProperties2KHR
	
	VkPhysicalDevice
	physicalDevice

	
	VkDisplayKHR
	display

	*
	uint32_t
	pPropertyCount

	*
	VkDisplayModeProperties2KHR
	pProperties

**/
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR,
	pPropertyCount: *const u32,
	pProperties: *const VkDisplayModeProperties2KHR
) -> VkResult;

/** vkGetDisplayPlaneCapabilities2KHR
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkDisplayPlaneInfo2KHR
	pDisplayPlaneInfo

	*
	VkDisplayPlaneCapabilities2KHR
	pCapabilities

**/
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pDisplayPlaneInfo: *const VkDisplayPlaneInfo2KHR,
	pCapabilities: *const VkDisplayPlaneCapabilities2KHR
) -> VkResult;

/** vkGetBufferMemoryRequirements2
	
	VkDevice
	device

	const *
	VkBufferMemoryRequirementsInfo2
	pInfo

	*
	VkMemoryRequirements2
	pMemoryRequirements

**/
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkBufferMemoryRequirementsInfo2,
	pMemoryRequirements: *const VkMemoryRequirements2
) -> VkResult;

/** vkGetImageMemoryRequirements2
	
	VkDevice
	device

	const *
	VkImageMemoryRequirementsInfo2
	pInfo

	*
	VkMemoryRequirements2
	pMemoryRequirements

**/
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkImageMemoryRequirementsInfo2,
	pMemoryRequirements: *const VkMemoryRequirements2
) -> VkResult;

/** vkGetImageSparseMemoryRequirements2
	
	VkDevice
	device

	const *
	VkImageSparseMemoryRequirementsInfo2
	pInfo

	*
	uint32_t
	pSparseMemoryRequirementCount

	*
	VkSparseImageMemoryRequirements2
	pSparseMemoryRequirements

**/
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkImageSparseMemoryRequirementsInfo2,
	pSparseMemoryRequirementCount: *const u32,
	pSparseMemoryRequirements: *const VkSparseImageMemoryRequirements2
) -> VkResult;

/** vkGetDeviceBufferMemoryRequirements
	
	VkDevice
	device

	const *
	VkDeviceBufferMemoryRequirements
	pInfo

	*
	VkMemoryRequirements2
	pMemoryRequirements

**/
pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkDeviceBufferMemoryRequirements,
	pMemoryRequirements: *const VkMemoryRequirements2
) -> VkResult;

/** vkGetDeviceImageMemoryRequirements
	
	VkDevice
	device

	const *
	VkDeviceImageMemoryRequirements
	pInfo

	*
	VkMemoryRequirements2
	pMemoryRequirements

**/
pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkDeviceImageMemoryRequirements,
	pMemoryRequirements: *const VkMemoryRequirements2
) -> VkResult;

/** vkGetDeviceImageSparseMemoryRequirements
	
	VkDevice
	device

	const *
	VkDeviceImageMemoryRequirements
	pInfo

	*
	uint32_t
	pSparseMemoryRequirementCount

	*
	VkSparseImageMemoryRequirements2
	pSparseMemoryRequirements

**/
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkDeviceImageMemoryRequirements,
	pSparseMemoryRequirementCount: *const u32,
	pSparseMemoryRequirements: *const VkSparseImageMemoryRequirements2
) -> VkResult;

/** vkCreateSamplerYcbcrConversion
	
	VkDevice
	device

	const *
	VkSamplerYcbcrConversionCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSamplerYcbcrConversion
	pYcbcrConversion

**/
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pYcbcrConversion: *const VkSamplerYcbcrConversion
) -> VkResult;

/** vkDestroySamplerYcbcrConversion
	
	VkDevice
	device

	
	VkSamplerYcbcrConversion
	ycbcrConversion

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
	device: VkDevice,
	ycbcrConversion: VkSamplerYcbcrConversion,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetDeviceQueue2
	
	VkDevice
	device

	const *
	VkDeviceQueueInfo2
	pQueueInfo

	*
	VkQueue
	pQueue

**/
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
	device: VkDevice,
	pQueueInfo: *const VkDeviceQueueInfo2,
	pQueue: *const VkQueue
) -> VkResult;

/** vkCreateValidationCacheEXT
	
	VkDevice
	device

	const *
	VkValidationCacheCreateInfoEXT
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkValidationCacheEXT
	pValidationCache

**/
pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkValidationCacheCreateInfoEXT,
	pAllocator: *const VkAllocationCallbacks,
	pValidationCache: *const VkValidationCacheEXT
) -> VkResult;

/** vkDestroyValidationCacheEXT
	
	VkDevice
	device

	
	VkValidationCacheEXT
	validationCache

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "system" fn(
	device: VkDevice,
	validationCache: VkValidationCacheEXT,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetValidationCacheDataEXT
	
	VkDevice
	device

	
	VkValidationCacheEXT
	validationCache

	*
	size_t
	pDataSize

	*
	void
	pData

**/
pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "system" fn(
	device: VkDevice,
	validationCache: VkValidationCacheEXT,
	pDataSize: *const size_t,
	pData: *const void
) -> VkResult;

/** vkMergeValidationCachesEXT
	
	VkDevice
	device

	
	VkValidationCacheEXT
	dstCache

	
	uint32_t
	srcCacheCount

	const *
	VkValidationCacheEXT
	pSrcCaches

**/
pub type PFN_vkMergeValidationCachesEXT = unsafe extern "system" fn(
	device: VkDevice,
	dstCache: VkValidationCacheEXT,
	srcCacheCount: u32,
	pSrcCaches: *const VkValidationCacheEXT
) -> VkResult;

/** vkGetDescriptorSetLayoutSupport
	
	VkDevice
	device

	const *
	VkDescriptorSetLayoutCreateInfo
	pCreateInfo

	*
	VkDescriptorSetLayoutSupport
	pSupport

**/
pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
	pSupport: *const VkDescriptorSetLayoutSupport
) -> VkResult;

/** vkGetSwapchainGrallocUsageANDROID
	
	VkDevice
	device

	
	VkFormat
	format

	
	VkImageUsageFlags
	imageUsage

	*
	int
	grallocUsage

**/
pub type PFN_vkGetSwapchainGrallocUsageANDROID = unsafe extern "system" fn(
	device: VkDevice,
	format: VkFormat,
	imageUsage: VkImageUsageFlags,
	grallocUsage: *const int
) -> VkResult;

/** vkGetSwapchainGrallocUsage2ANDROID
	
	VkDevice
	device

	
	VkFormat
	format

	
	VkImageUsageFlags
	imageUsage

	
	VkSwapchainImageUsageFlagsANDROID
	swapchainImageUsage

	*
	uint64_t
	grallocConsumerUsage

	*
	uint64_t
	grallocProducerUsage

**/
pub type PFN_vkGetSwapchainGrallocUsage2ANDROID = unsafe extern "system" fn(
	device: VkDevice,
	format: VkFormat,
	imageUsage: VkImageUsageFlags,
	swapchainImageUsage: VkSwapchainImageUsageFlagsANDROID,
	grallocConsumerUsage: *const uint64_t,
	grallocProducerUsage: *const uint64_t
) -> VkResult;

/** vkAcquireImageANDROID
	
	VkDevice
	device

	
	VkImage
	image

	
	int
	nativeFenceFd

	
	VkSemaphore
	semaphore

	
	VkFence
	fence

**/
pub type PFN_vkAcquireImageANDROID = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	nativeFenceFd: int,
	semaphore: VkSemaphore,
	fence: VkFence
) -> VkResult;

/** vkQueueSignalReleaseImageANDROID
	
	VkQueue
	queue

	
	uint32_t
	waitSemaphoreCount

	const *
	VkSemaphore
	pWaitSemaphores

	
	VkImage
	image

	*
	int
	pNativeFenceFd

**/
pub type PFN_vkQueueSignalReleaseImageANDROID = unsafe extern "system" fn(
	queue: VkQueue,
	waitSemaphoreCount: u32,
	pWaitSemaphores: *const VkSemaphore,
	image: VkImage,
	pNativeFenceFd: *const int
) -> VkResult;

/** vkGetShaderInfoAMD
	
	VkDevice
	device

	
	VkPipeline
	pipeline

	
	VkShaderStageFlagBits
	shaderStage

	
	VkShaderInfoTypeAMD
	infoType

	*
	size_t
	pInfoSize

	*
	void
	pInfo

**/
pub type PFN_vkGetShaderInfoAMD = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	shaderStage: VkShaderStageFlagBits,
	infoType: VkShaderInfoTypeAMD,
	pInfoSize: *const size_t,
	pInfo: *const void
) -> VkResult;

/** vkSetLocalDimmingAMD
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapChain

	
	VkBool32
	localDimmingEnable

**/
pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
	device: VkDevice,
	swapChain: VkSwapchainKHR,
	localDimmingEnable: VkBool32
) -> VkResult;

/** vkGetPhysicalDeviceCalibrateableTimeDomainsEXT
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pTimeDomainCount

	*
	VkTimeDomainEXT
	pTimeDomains

**/
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pTimeDomainCount: *const u32,
	pTimeDomains: *const VkTimeDomainEXT
) -> VkResult;

/** vkGetCalibratedTimestampsEXT
	
	VkDevice
	device

	
	uint32_t
	timestampCount

	const *
	VkCalibratedTimestampInfoEXT
	pTimestampInfos

	*
	uint64_t
	pTimestamps

	*
	uint64_t
	pMaxDeviation

**/
pub type PFN_vkGetCalibratedTimestampsEXT = unsafe extern "system" fn(
	device: VkDevice,
	timestampCount: u32,
	pTimestampInfos: *const VkCalibratedTimestampInfoEXT,
	pTimestamps: *const uint64_t,
	pMaxDeviation: *const uint64_t
) -> VkResult;

/** vkSetDebugUtilsObjectNameEXT
	
	VkDevice
	device

	const *
	VkDebugUtilsObjectNameInfoEXT
	pNameInfo

**/
pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
	device: VkDevice,
	pNameInfo: *const VkDebugUtilsObjectNameInfoEXT
) -> VkResult;

/** vkSetDebugUtilsObjectTagEXT
	
	VkDevice
	device

	const *
	VkDebugUtilsObjectTagInfoEXT
	pTagInfo

**/
pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
	device: VkDevice,
	pTagInfo: *const VkDebugUtilsObjectTagInfoEXT
) -> VkResult;

/** vkQueueBeginDebugUtilsLabelEXT
	
	VkQueue
	queue

	const *
	VkDebugUtilsLabelEXT
	pLabelInfo

**/
pub type PFN_vkQueueBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
	queue: VkQueue,
	pLabelInfo: *const VkDebugUtilsLabelEXT
) -> VkResult;

/** vkQueueEndDebugUtilsLabelEXT
	
	VkQueue
	queue

**/
pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "system" fn(
	queue: VkQueue
) -> VkResult;

/** vkQueueInsertDebugUtilsLabelEXT
	
	VkQueue
	queue

	const *
	VkDebugUtilsLabelEXT
	pLabelInfo

**/
pub type PFN_vkQueueInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
	queue: VkQueue,
	pLabelInfo: *const VkDebugUtilsLabelEXT
) -> VkResult;

/** vkCmdBeginDebugUtilsLabelEXT
	
	VkCommandBuffer
	commandBuffer

	const *
	VkDebugUtilsLabelEXT
	pLabelInfo

**/
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pLabelInfo: *const VkDebugUtilsLabelEXT
) -> VkResult;

/** vkCmdEndDebugUtilsLabelEXT
	
	VkCommandBuffer
	commandBuffer

**/
pub type PFN_vkCmdEndDebugUtilsLabelEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

/** vkCmdInsertDebugUtilsLabelEXT
	
	VkCommandBuffer
	commandBuffer

	const *
	VkDebugUtilsLabelEXT
	pLabelInfo

**/
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pLabelInfo: *const VkDebugUtilsLabelEXT
) -> VkResult;

/** vkCreateDebugUtilsMessengerEXT
	
	VkInstance
	instance

	const *
	VkDebugUtilsMessengerCreateInfoEXT
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkDebugUtilsMessengerEXT
	pMessenger

**/
pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
	pAllocator: *const VkAllocationCallbacks,
	pMessenger: *const VkDebugUtilsMessengerEXT
) -> VkResult;

/** vkDestroyDebugUtilsMessengerEXT
	
	VkInstance
	instance

	
	VkDebugUtilsMessengerEXT
	messenger

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "system" fn(
	instance: VkInstance,
	messenger: VkDebugUtilsMessengerEXT,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkSubmitDebugUtilsMessageEXT
	
	VkInstance
	instance

	
	VkDebugUtilsMessageSeverityFlagBitsEXT
	messageSeverity

	
	VkDebugUtilsMessageTypeFlagsEXT
	messageTypes

	const *
	VkDebugUtilsMessengerCallbackDataEXT
	pCallbackData

**/
pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
	instance: VkInstance,
	messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
	messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
	pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT
) -> VkResult;

/** vkGetMemoryHostPointerPropertiesEXT
	
	VkDevice
	device

	
	VkExternalMemoryHandleTypeFlagBits
	handleType

	const *
	void
	pHostPointer

	*
	VkMemoryHostPointerPropertiesEXT
	pMemoryHostPointerProperties

**/
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
	device: VkDevice,
	handleType: VkExternalMemoryHandleTypeFlagBits,
	pHostPointer: *const void,
	pMemoryHostPointerProperties: *const VkMemoryHostPointerPropertiesEXT
) -> VkResult;

/** vkCmdWriteBufferMarkerAMD
	
	VkCommandBuffer
	commandBuffer

	
	VkPipelineStageFlagBits
	pipelineStage

	
	VkBuffer
	dstBuffer

	
	VkDeviceSize
	dstOffset

	
	uint32_t
	marker

**/
pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineStage: VkPipelineStageFlagBits,
	dstBuffer: VkBuffer,
	dstOffset: VkDeviceSize,
	marker: u32
) -> VkResult;

/** vkCreateRenderPass2
	
	VkDevice
	device

	const *
	VkRenderPassCreateInfo2
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkRenderPass
	pRenderPass

**/
pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkRenderPassCreateInfo2,
	pAllocator: *const VkAllocationCallbacks,
	pRenderPass: *const VkRenderPass
) -> VkResult;

/** vkCmdBeginRenderPass2
	
	VkCommandBuffer
	commandBuffer

	const *
	VkRenderPassBeginInfo
	pRenderPassBegin

	const *
	VkSubpassBeginInfo
	pSubpassBeginInfo

**/
pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pRenderPassBegin: *const VkRenderPassBeginInfo,
	pSubpassBeginInfo: *const VkSubpassBeginInfo
) -> VkResult;

/** vkCmdNextSubpass2
	
	VkCommandBuffer
	commandBuffer

	const *
	VkSubpassBeginInfo
	pSubpassBeginInfo

	const *
	VkSubpassEndInfo
	pSubpassEndInfo

**/
pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pSubpassBeginInfo: *const VkSubpassBeginInfo,
	pSubpassEndInfo: *const VkSubpassEndInfo
) -> VkResult;

/** vkCmdEndRenderPass2
	
	VkCommandBuffer
	commandBuffer

	const *
	VkSubpassEndInfo
	pSubpassEndInfo

**/
pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pSubpassEndInfo: *const VkSubpassEndInfo
) -> VkResult;

/** vkGetSemaphoreCounterValue
	
	VkDevice
	device

	
	VkSemaphore
	semaphore

	*
	uint64_t
	pValue

**/
pub type PFN_vkGetSemaphoreCounterValue = unsafe extern "system" fn(
	device: VkDevice,
	semaphore: VkSemaphore,
	pValue: *const uint64_t
) -> VkResult;

/** vkWaitSemaphores
	
	VkDevice
	device

	const *
	VkSemaphoreWaitInfo
	pWaitInfo

	
	uint64_t
	timeout

**/
pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(
	device: VkDevice,
	pWaitInfo: *const VkSemaphoreWaitInfo,
	timeout: uint64_t
) -> VkResult;

/** vkSignalSemaphore
	
	VkDevice
	device

	const *
	VkSemaphoreSignalInfo
	pSignalInfo

**/
pub type PFN_vkSignalSemaphore = unsafe extern "system" fn(
	device: VkDevice,
	pSignalInfo: *const VkSemaphoreSignalInfo
) -> VkResult;

/** vkGetAndroidHardwareBufferPropertiesANDROID
	
	VkDevice
	device

	const struct *
	AHardwareBuffer
	buffer

	*
	VkAndroidHardwareBufferPropertiesANDROID
	pProperties

**/
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn(
	device: VkDevice,
	buffer: const struct * AHardwareBuffer,
	pProperties: *const VkAndroidHardwareBufferPropertiesANDROID
) -> VkResult;

/** vkGetMemoryAndroidHardwareBufferANDROID
	
	VkDevice
	device

	const *
	VkMemoryGetAndroidHardwareBufferInfoANDROID
	pInfo

	struct **
	AHardwareBuffer
	pBuffer

**/
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkMemoryGetAndroidHardwareBufferInfoANDROID,
	pBuffer: struct ** AHardwareBuffer
) -> VkResult;

/** vkCmdDrawIndirectCount
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	buffer

	
	VkDeviceSize
	offset

	
	VkBuffer
	countBuffer

	
	VkDeviceSize
	countBufferOffset

	
	uint32_t
	maxDrawCount

	
	uint32_t
	stride

**/
pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	countBuffer: VkBuffer,
	countBufferOffset: VkDeviceSize,
	maxDrawCount: u32,
	stride: u32
) -> VkResult;

/** vkCmdDrawIndexedIndirectCount
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	buffer

	
	VkDeviceSize
	offset

	
	VkBuffer
	countBuffer

	
	VkDeviceSize
	countBufferOffset

	
	uint32_t
	maxDrawCount

	
	uint32_t
	stride

**/
pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	countBuffer: VkBuffer,
	countBufferOffset: VkDeviceSize,
	maxDrawCount: u32,
	stride: u32
) -> VkResult;

/** vkCmdSetCheckpointNV
	
	VkCommandBuffer
	commandBuffer

	const *
	void
	pCheckpointMarker

**/
pub type PFN_vkCmdSetCheckpointNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCheckpointMarker: *const void
) -> VkResult;

/** vkGetQueueCheckpointDataNV
	
	VkQueue
	queue

	*
	uint32_t
	pCheckpointDataCount

	*
	VkCheckpointDataNV
	pCheckpointData

**/
pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
	queue: VkQueue,
	pCheckpointDataCount: *const u32,
	pCheckpointData: *const VkCheckpointDataNV
) -> VkResult;

/** vkCmdBindTransformFeedbackBuffersEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstBinding

	
	uint32_t
	bindingCount

	const *
	VkBuffer
	pBuffers

	const *
	VkDeviceSize
	pOffsets

	const *
	VkDeviceSize
	pSizes

**/
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstBinding: u32,
	bindingCount: u32,
	pBuffers: *const VkBuffer,
	pOffsets: *const VkDeviceSize,
	pSizes: *const VkDeviceSize
) -> VkResult;

/** vkCmdBeginTransformFeedbackEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstCounterBuffer

	
	uint32_t
	counterBufferCount

	const *
	VkBuffer
	pCounterBuffers

	const *
	VkDeviceSize
	pCounterBufferOffsets

**/
pub type PFN_vkCmdBeginTransformFeedbackEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstCounterBuffer: u32,
	counterBufferCount: u32,
	pCounterBuffers: *const VkBuffer,
	pCounterBufferOffsets: *const VkDeviceSize
) -> VkResult;

/** vkCmdEndTransformFeedbackEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstCounterBuffer

	
	uint32_t
	counterBufferCount

	const *
	VkBuffer
	pCounterBuffers

	const *
	VkDeviceSize
	pCounterBufferOffsets

**/
pub type PFN_vkCmdEndTransformFeedbackEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstCounterBuffer: u32,
	counterBufferCount: u32,
	pCounterBuffers: *const VkBuffer,
	pCounterBufferOffsets: *const VkDeviceSize
) -> VkResult;

/** vkCmdBeginQueryIndexedEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkQueryPool
	queryPool

	
	uint32_t
	query

	
	VkQueryControlFlags
	flags

	
	uint32_t
	index

**/
pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	query: u32,
	flags: VkQueryControlFlags,
	index: u32
) -> VkResult;

/** vkCmdEndQueryIndexedEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkQueryPool
	queryPool

	
	uint32_t
	query

	
	uint32_t
	index

**/
pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	query: u32,
	index: u32
) -> VkResult;

/** vkCmdDrawIndirectByteCountEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	instanceCount

	
	uint32_t
	firstInstance

	
	VkBuffer
	counterBuffer

	
	VkDeviceSize
	counterBufferOffset

	
	uint32_t
	counterOffset

	
	uint32_t
	vertexStride

**/
pub type PFN_vkCmdDrawIndirectByteCountEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	instanceCount: u32,
	firstInstance: u32,
	counterBuffer: VkBuffer,
	counterBufferOffset: VkDeviceSize,
	counterOffset: u32,
	vertexStride: u32
) -> VkResult;

/** vkCmdSetExclusiveScissorNV
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstExclusiveScissor

	
	uint32_t
	exclusiveScissorCount

	const *
	VkRect2D
	pExclusiveScissors

**/
pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstExclusiveScissor: u32,
	exclusiveScissorCount: u32,
	pExclusiveScissors: *const VkRect2D
) -> VkResult;

/** vkCmdBindShadingRateImageNV
	
	VkCommandBuffer
	commandBuffer

	
	VkImageView
	imageView

	
	VkImageLayout
	imageLayout

**/
pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	imageView: VkImageView,
	imageLayout: VkImageLayout
) -> VkResult;

/** vkCmdSetViewportShadingRatePaletteNV
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstViewport

	
	uint32_t
	viewportCount

	const *
	VkShadingRatePaletteNV
	pShadingRatePalettes

**/
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstViewport: u32,
	viewportCount: u32,
	pShadingRatePalettes: *const VkShadingRatePaletteNV
) -> VkResult;

/** vkCmdSetCoarseSampleOrderNV
	
	VkCommandBuffer
	commandBuffer

	
	VkCoarseSampleOrderTypeNV
	sampleOrderType

	
	uint32_t
	customSampleOrderCount

	const *
	VkCoarseSampleOrderCustomNV
	pCustomSampleOrders

**/
pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	sampleOrderType: VkCoarseSampleOrderTypeNV,
	customSampleOrderCount: u32,
	pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV
) -> VkResult;

/** vkCmdDrawMeshTasksNV
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	taskCount

	
	uint32_t
	firstTask

**/
pub type PFN_vkCmdDrawMeshTasksNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	taskCount: u32,
	firstTask: u32
) -> VkResult;

/** vkCmdDrawMeshTasksIndirectNV
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	buffer

	
	VkDeviceSize
	offset

	
	uint32_t
	drawCount

	
	uint32_t
	stride

**/
pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	drawCount: u32,
	stride: u32
) -> VkResult;

/** vkCmdDrawMeshTasksIndirectCountNV
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	buffer

	
	VkDeviceSize
	offset

	
	VkBuffer
	countBuffer

	
	VkDeviceSize
	countBufferOffset

	
	uint32_t
	maxDrawCount

	
	uint32_t
	stride

**/
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	countBuffer: VkBuffer,
	countBufferOffset: VkDeviceSize,
	maxDrawCount: u32,
	stride: u32
) -> VkResult;

/** vkCmdDrawMeshTasksEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	groupCountX

	
	uint32_t
	groupCountY

	
	uint32_t
	groupCountZ

**/
pub type PFN_vkCmdDrawMeshTasksEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	groupCountX: u32,
	groupCountY: u32,
	groupCountZ: u32
) -> VkResult;

/** vkCmdDrawMeshTasksIndirectEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	buffer

	
	VkDeviceSize
	offset

	
	uint32_t
	drawCount

	
	uint32_t
	stride

**/
pub type PFN_vkCmdDrawMeshTasksIndirectEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	drawCount: u32,
	stride: u32
) -> VkResult;

/** vkCmdDrawMeshTasksIndirectCountEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	buffer

	
	VkDeviceSize
	offset

	
	VkBuffer
	countBuffer

	
	VkDeviceSize
	countBufferOffset

	
	uint32_t
	maxDrawCount

	
	uint32_t
	stride

**/
pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	countBuffer: VkBuffer,
	countBufferOffset: VkDeviceSize,
	maxDrawCount: u32,
	stride: u32
) -> VkResult;

/** vkCompileDeferredNV
	
	VkDevice
	device

	
	VkPipeline
	pipeline

	
	uint32_t
	shader

**/
pub type PFN_vkCompileDeferredNV = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	shader: u32
) -> VkResult;

/** vkCreateAccelerationStructureNV
	
	VkDevice
	device

	const *
	VkAccelerationStructureCreateInfoNV
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkAccelerationStructureNV
	pAccelerationStructure

**/
pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkAccelerationStructureCreateInfoNV,
	pAllocator: *const VkAllocationCallbacks,
	pAccelerationStructure: *const VkAccelerationStructureNV
) -> VkResult;

/** vkCmdBindInvocationMaskHUAWEI
	
	VkCommandBuffer
	commandBuffer

	
	VkImageView
	imageView

	
	VkImageLayout
	imageLayout

**/
pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	imageView: VkImageView,
	imageLayout: VkImageLayout
) -> VkResult;

/** vkDestroyAccelerationStructureKHR
	
	VkDevice
	device

	
	VkAccelerationStructureKHR
	accelerationStructure

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
	device: VkDevice,
	accelerationStructure: VkAccelerationStructureKHR,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkDestroyAccelerationStructureNV
	
	VkDevice
	device

	
	VkAccelerationStructureNV
	accelerationStructure

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "system" fn(
	device: VkDevice,
	accelerationStructure: VkAccelerationStructureNV,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetAccelerationStructureMemoryRequirementsNV
	
	VkDevice
	device

	const *
	VkAccelerationStructureMemoryRequirementsInfoNV
	pInfo

	*
	VkMemoryRequirements2KHR
	pMemoryRequirements

**/
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV,
	pMemoryRequirements: *const VkMemoryRequirements2KHR
) -> VkResult;

/** vkBindAccelerationStructureMemoryNV
	
	VkDevice
	device

	
	uint32_t
	bindInfoCount

	const *
	VkBindAccelerationStructureMemoryInfoNV
	pBindInfos

**/
pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(
	device: VkDevice,
	bindInfoCount: u32,
	pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV
) -> VkResult;

/** vkCmdCopyAccelerationStructureNV
	
	VkCommandBuffer
	commandBuffer

	
	VkAccelerationStructureNV
	dst

	
	VkAccelerationStructureNV
	src

	
	VkCopyAccelerationStructureModeKHR
	mode

**/
pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	dst: VkAccelerationStructureNV,
	src: VkAccelerationStructureNV,
	mode: VkCopyAccelerationStructureModeKHR
) -> VkResult;

/** vkCmdCopyAccelerationStructureKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCopyAccelerationStructureInfoKHR
	pInfo

**/
pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *const VkCopyAccelerationStructureInfoKHR
) -> VkResult;

/** vkCopyAccelerationStructureKHR
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	deferredOperation

	const *
	VkCopyAccelerationStructureInfoKHR
	pInfo

**/
pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *const VkCopyAccelerationStructureInfoKHR
) -> VkResult;

/** vkCmdCopyAccelerationStructureToMemoryKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCopyAccelerationStructureToMemoryInfoKHR
	pInfo

**/
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR
) -> VkResult;

/** vkCopyAccelerationStructureToMemoryKHR
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	deferredOperation

	const *
	VkCopyAccelerationStructureToMemoryInfoKHR
	pInfo

**/
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR
) -> VkResult;

/** vkCmdCopyMemoryToAccelerationStructureKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCopyMemoryToAccelerationStructureInfoKHR
	pInfo

**/
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR
) -> VkResult;

/** vkCopyMemoryToAccelerationStructureKHR
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	deferredOperation

	const *
	VkCopyMemoryToAccelerationStructureInfoKHR
	pInfo

**/
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR
) -> VkResult;

/** vkCmdWriteAccelerationStructuresPropertiesKHR
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	accelerationStructureCount

	const *
	VkAccelerationStructureKHR
	pAccelerationStructures

	
	VkQueryType
	queryType

	
	VkQueryPool
	queryPool

	
	uint32_t
	firstQuery

**/
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	accelerationStructureCount: u32,
	pAccelerationStructures: *const VkAccelerationStructureKHR,
	queryType: VkQueryType,
	queryPool: VkQueryPool,
	firstQuery: u32
) -> VkResult;

/** vkCmdWriteAccelerationStructuresPropertiesNV
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	accelerationStructureCount

	const *
	VkAccelerationStructureNV
	pAccelerationStructures

	
	VkQueryType
	queryType

	
	VkQueryPool
	queryPool

	
	uint32_t
	firstQuery

**/
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	accelerationStructureCount: u32,
	pAccelerationStructures: *const VkAccelerationStructureNV,
	queryType: VkQueryType,
	queryPool: VkQueryPool,
	firstQuery: u32
) -> VkResult;

/** vkCmdBuildAccelerationStructureNV
	
	VkCommandBuffer
	commandBuffer

	const *
	VkAccelerationStructureInfoNV
	pInfo

	
	VkBuffer
	instanceData

	
	VkDeviceSize
	instanceOffset

	
	VkBool32
	update

	
	VkAccelerationStructureNV
	dst

	
	VkAccelerationStructureNV
	src

	
	VkBuffer
	scratch

	
	VkDeviceSize
	scratchOffset

**/
pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *const VkAccelerationStructureInfoNV,
	instanceData: VkBuffer,
	instanceOffset: VkDeviceSize,
	update: VkBool32,
	dst: VkAccelerationStructureNV,
	src: VkAccelerationStructureNV,
	scratch: VkBuffer,
	scratchOffset: VkDeviceSize
) -> VkResult;

/** vkWriteAccelerationStructuresPropertiesKHR
	
	VkDevice
	device

	
	uint32_t
	accelerationStructureCount

	const *
	VkAccelerationStructureKHR
	pAccelerationStructures

	
	VkQueryType
	queryType

	
	size_t
	dataSize

	*
	void
	pData

	
	size_t
	stride

**/
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
	device: VkDevice,
	accelerationStructureCount: u32,
	pAccelerationStructures: *const VkAccelerationStructureKHR,
	queryType: VkQueryType,
	dataSize: size_t,
	pData: *const void,
	stride: size_t
) -> VkResult;

/** vkCmdTraceRaysKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkStridedDeviceAddressRegionKHR
	pRaygenShaderBindingTable

	const *
	VkStridedDeviceAddressRegionKHR
	pMissShaderBindingTable

	const *
	VkStridedDeviceAddressRegionKHR
	pHitShaderBindingTable

	const *
	VkStridedDeviceAddressRegionKHR
	pCallableShaderBindingTable

	
	uint32_t
	width

	
	uint32_t
	height

	
	uint32_t
	depth

**/
pub type PFN_vkCmdTraceRaysKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	width: u32,
	height: u32,
	depth: u32
) -> VkResult;

/** vkCmdTraceRaysNV
	
	VkCommandBuffer
	commandBuffer

	
	VkBuffer
	raygenShaderBindingTableBuffer

	
	VkDeviceSize
	raygenShaderBindingOffset

	
	VkBuffer
	missShaderBindingTableBuffer

	
	VkDeviceSize
	missShaderBindingOffset

	
	VkDeviceSize
	missShaderBindingStride

	
	VkBuffer
	hitShaderBindingTableBuffer

	
	VkDeviceSize
	hitShaderBindingOffset

	
	VkDeviceSize
	hitShaderBindingStride

	
	VkBuffer
	callableShaderBindingTableBuffer

	
	VkDeviceSize
	callableShaderBindingOffset

	
	VkDeviceSize
	callableShaderBindingStride

	
	uint32_t
	width

	
	uint32_t
	height

	
	uint32_t
	depth

**/
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

/** vkGetRayTracingShaderGroupHandlesKHR
	
	VkDevice
	device

	
	VkPipeline
	pipeline

	
	uint32_t
	firstGroup

	
	uint32_t
	groupCount

	
	size_t
	dataSize

	*
	void
	pData

**/
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	firstGroup: u32,
	groupCount: u32,
	dataSize: size_t,
	pData: *const void
) -> VkResult;

/** vkGetRayTracingCaptureReplayShaderGroupHandlesKHR
	
	VkDevice
	device

	
	VkPipeline
	pipeline

	
	uint32_t
	firstGroup

	
	uint32_t
	groupCount

	
	size_t
	dataSize

	*
	void
	pData

**/
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	firstGroup: u32,
	groupCount: u32,
	dataSize: size_t,
	pData: *const void
) -> VkResult;

/** vkGetAccelerationStructureHandleNV
	
	VkDevice
	device

	
	VkAccelerationStructureNV
	accelerationStructure

	
	size_t
	dataSize

	*
	void
	pData

**/
pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "system" fn(
	device: VkDevice,
	accelerationStructure: VkAccelerationStructureNV,
	dataSize: size_t,
	pData: *const void
) -> VkResult;

/** vkCreateRayTracingPipelinesNV
	
	VkDevice
	device

	
	VkPipelineCache
	pipelineCache

	
	uint32_t
	createInfoCount

	const *
	VkRayTracingPipelineCreateInfoNV
	pCreateInfos

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkPipeline
	pPipelines

**/
pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
	pAllocator: *const VkAllocationCallbacks,
	pPipelines: *const VkPipeline
) -> VkResult;

/** vkCreateRayTracingPipelinesKHR
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	deferredOperation

	
	VkPipelineCache
	pipelineCache

	
	uint32_t
	createInfoCount

	const *
	VkRayTracingPipelineCreateInfoKHR
	pCreateInfos

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkPipeline
	pPipelines

**/
pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pPipelines: *const VkPipeline
) -> VkResult;

/** vkGetPhysicalDeviceCooperativeMatrixPropertiesNV
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pPropertyCount

	*
	VkCooperativeMatrixPropertiesNV
	pProperties

**/
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPropertyCount: *const u32,
	pProperties: *const VkCooperativeMatrixPropertiesNV
) -> VkResult;

/** vkCmdTraceRaysIndirectKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkStridedDeviceAddressRegionKHR
	pRaygenShaderBindingTable

	const *
	VkStridedDeviceAddressRegionKHR
	pMissShaderBindingTable

	const *
	VkStridedDeviceAddressRegionKHR
	pHitShaderBindingTable

	const *
	VkStridedDeviceAddressRegionKHR
	pCallableShaderBindingTable

	
	VkDeviceAddress
	indirectDeviceAddress

**/
pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	indirectDeviceAddress: VkDeviceAddress
) -> VkResult;

/** vkCmdTraceRaysIndirect2KHR
	
	VkCommandBuffer
	commandBuffer

	
	VkDeviceAddress
	indirectDeviceAddress

**/
pub type PFN_vkCmdTraceRaysIndirect2KHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	indirectDeviceAddress: VkDeviceAddress
) -> VkResult;

/** vkGetDeviceAccelerationStructureCompatibilityKHR
	
	VkDevice
	device

	const *
	VkAccelerationStructureVersionInfoKHR
	pVersionInfo

	*
	VkAccelerationStructureCompatibilityKHR
	pCompatibility

**/
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(
	device: VkDevice,
	pVersionInfo: *const VkAccelerationStructureVersionInfoKHR,
	pCompatibility: *const VkAccelerationStructureCompatibilityKHR
) -> VkResult;

/** vkGetRayTracingShaderGroupStackSizeKHR
	
	VkDevice
	device

	
	VkPipeline
	pipeline

	
	uint32_t
	group

	
	VkShaderGroupShaderKHR
	groupShader

**/
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "system" fn(
	device: VkDevice,
	pipeline: VkPipeline,
	group: u32,
	groupShader: VkShaderGroupShaderKHR
) -> VkResult;

/** vkCmdSetRayTracingPipelineStackSizeKHR
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	pipelineStackSize

**/
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pipelineStackSize: u32
) -> VkResult;

/** vkGetImageViewHandleNVX
	
	VkDevice
	device

	const *
	VkImageViewHandleInfoNVX
	pInfo

**/
pub type PFN_vkGetImageViewHandleNVX = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkImageViewHandleInfoNVX
) -> VkResult;

/** vkGetImageViewAddressNVX
	
	VkDevice
	device

	
	VkImageView
	imageView

	*
	VkImageViewAddressPropertiesNVX
	pProperties

**/
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
	device: VkDevice,
	imageView: VkImageView,
	pProperties: *const VkImageViewAddressPropertiesNVX
) -> VkResult;

/** vkGetPhysicalDeviceSurfacePresentModes2EXT
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkPhysicalDeviceSurfaceInfo2KHR
	pSurfaceInfo

	*
	uint32_t
	pPresentModeCount

	*
	VkPresentModeKHR
	pPresentModes

**/
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
	pPresentModeCount: *const u32,
	pPresentModes: *const VkPresentModeKHR
) -> VkResult;

/** vkGetDeviceGroupSurfacePresentModes2EXT
	
	VkDevice
	device

	const *
	VkPhysicalDeviceSurfaceInfo2KHR
	pSurfaceInfo

	*
	VkDeviceGroupPresentModeFlagsKHR
	pModes

**/
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
	device: VkDevice,
	pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
	pModes: *const VkDeviceGroupPresentModeFlagsKHR
) -> VkResult;

/** vkAcquireFullScreenExclusiveModeEXT
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapchain

**/
pub type PFN_vkAcquireFullScreenExclusiveModeEXT = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR
) -> VkResult;

/** vkReleaseFullScreenExclusiveModeEXT
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapchain

**/
pub type PFN_vkReleaseFullScreenExclusiveModeEXT = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR
) -> VkResult;

/** vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR
	
	VkPhysicalDevice
	physicalDevice

	
	uint32_t
	queueFamilyIndex

	*
	uint32_t
	pCounterCount

	*
	VkPerformanceCounterKHR
	pCounters

	*
	VkPerformanceCounterDescriptionKHR
	pCounterDescriptions

**/
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	pCounterCount: *const u32,
	pCounters: *const VkPerformanceCounterKHR,
	pCounterDescriptions: *const VkPerformanceCounterDescriptionKHR
) -> VkResult;

/** vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkQueryPoolPerformanceCreateInfoKHR
	pPerformanceQueryCreateInfo

	*
	uint32_t
	pNumPasses

**/
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pPerformanceQueryCreateInfo: *const VkQueryPoolPerformanceCreateInfoKHR,
	pNumPasses: *const u32
) -> VkResult;

/** vkAcquireProfilingLockKHR
	
	VkDevice
	device

	const *
	VkAcquireProfilingLockInfoKHR
	pInfo

**/
pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkAcquireProfilingLockInfoKHR
) -> VkResult;

/** vkReleaseProfilingLockKHR
	
	VkDevice
	device

**/
pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "system" fn(
	device: VkDevice
) -> VkResult;

/** vkGetImageDrmFormatModifierPropertiesEXT
	
	VkDevice
	device

	
	VkImage
	image

	*
	VkImageDrmFormatModifierPropertiesEXT
	pProperties

**/
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pProperties: *const VkImageDrmFormatModifierPropertiesEXT
) -> VkResult;

/** vkGetBufferOpaqueCaptureAddress
	
	VkDevice
	device

	const *
	VkBufferDeviceAddressInfo
	pInfo

**/
pub type PFN_vkGetBufferOpaqueCaptureAddress = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkBufferDeviceAddressInfo
) -> VkResult;

/** vkGetBufferDeviceAddress
	
	VkDevice
	device

	const *
	VkBufferDeviceAddressInfo
	pInfo

**/
pub type PFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkBufferDeviceAddressInfo
) -> VkResult;

/** vkCreateHeadlessSurfaceEXT
	
	VkInstance
	instance

	const *
	VkHeadlessSurfaceCreateInfoEXT
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkSurfaceKHR
	pSurface

**/
pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
	instance: VkInstance,
	pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT,
	pAllocator: *const VkAllocationCallbacks,
	pSurface: *const VkSurfaceKHR
) -> VkResult;

/** vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pCombinationCount

	*
	VkFramebufferMixedSamplesCombinationNV
	pCombinations

**/
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pCombinationCount: *const u32,
	pCombinations: *const VkFramebufferMixedSamplesCombinationNV
) -> VkResult;

/** vkInitializePerformanceApiINTEL
	
	VkDevice
	device

	const *
	VkInitializePerformanceApiInfoINTEL
	pInitializeInfo

**/
pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
	device: VkDevice,
	pInitializeInfo: *const VkInitializePerformanceApiInfoINTEL
) -> VkResult;

/** vkUninitializePerformanceApiINTEL
	
	VkDevice
	device

**/
pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(
	device: VkDevice
) -> VkResult;

/** vkCmdSetPerformanceMarkerINTEL
	
	VkCommandBuffer
	commandBuffer

	const *
	VkPerformanceMarkerInfoINTEL
	pMarkerInfo

**/
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pMarkerInfo: *const VkPerformanceMarkerInfoINTEL
) -> VkResult;

/** vkCmdSetPerformanceStreamMarkerINTEL
	
	VkCommandBuffer
	commandBuffer

	const *
	VkPerformanceStreamMarkerInfoINTEL
	pMarkerInfo

**/
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pMarkerInfo: *const VkPerformanceStreamMarkerInfoINTEL
) -> VkResult;

/** vkCmdSetPerformanceOverrideINTEL
	
	VkCommandBuffer
	commandBuffer

	const *
	VkPerformanceOverrideInfoINTEL
	pOverrideInfo

**/
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pOverrideInfo: *const VkPerformanceOverrideInfoINTEL
) -> VkResult;

/** vkAcquirePerformanceConfigurationINTEL
	
	VkDevice
	device

	const *
	VkPerformanceConfigurationAcquireInfoINTEL
	pAcquireInfo

	*
	VkPerformanceConfigurationINTEL
	pConfiguration

**/
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
	device: VkDevice,
	pAcquireInfo: *const VkPerformanceConfigurationAcquireInfoINTEL,
	pConfiguration: *const VkPerformanceConfigurationINTEL
) -> VkResult;

/** vkReleasePerformanceConfigurationINTEL
	
	VkDevice
	device

	
	VkPerformanceConfigurationINTEL
	configuration

**/
pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(
	device: VkDevice,
	configuration: VkPerformanceConfigurationINTEL
) -> VkResult;

/** vkQueueSetPerformanceConfigurationINTEL
	
	VkQueue
	queue

	
	VkPerformanceConfigurationINTEL
	configuration

**/
pub type PFN_vkQueueSetPerformanceConfigurationINTEL = unsafe extern "system" fn(
	queue: VkQueue,
	configuration: VkPerformanceConfigurationINTEL
) -> VkResult;

/** vkGetPerformanceParameterINTEL
	
	VkDevice
	device

	
	VkPerformanceParameterTypeINTEL
	parameter

	*
	VkPerformanceValueINTEL
	pValue

**/
pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
	device: VkDevice,
	parameter: VkPerformanceParameterTypeINTEL,
	pValue: *const VkPerformanceValueINTEL
) -> VkResult;

/** vkGetDeviceMemoryOpaqueCaptureAddress
	
	VkDevice
	device

	const *
	VkDeviceMemoryOpaqueCaptureAddressInfo
	pInfo

**/
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo
) -> VkResult;

/** vkGetPipelineExecutablePropertiesKHR
	
	VkDevice
	device

	const *
	VkPipelineInfoKHR
	pPipelineInfo

	*
	uint32_t
	pExecutableCount

	*
	VkPipelineExecutablePropertiesKHR
	pProperties

**/
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
	device: VkDevice,
	pPipelineInfo: *const VkPipelineInfoKHR,
	pExecutableCount: *const u32,
	pProperties: *const VkPipelineExecutablePropertiesKHR
) -> VkResult;

/** vkGetPipelineExecutableStatisticsKHR
	
	VkDevice
	device

	const *
	VkPipelineExecutableInfoKHR
	pExecutableInfo

	*
	uint32_t
	pStatisticCount

	*
	VkPipelineExecutableStatisticKHR
	pStatistics

**/
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
	device: VkDevice,
	pExecutableInfo: *const VkPipelineExecutableInfoKHR,
	pStatisticCount: *const u32,
	pStatistics: *const VkPipelineExecutableStatisticKHR
) -> VkResult;

/** vkGetPipelineExecutableInternalRepresentationsKHR
	
	VkDevice
	device

	const *
	VkPipelineExecutableInfoKHR
	pExecutableInfo

	*
	uint32_t
	pInternalRepresentationCount

	*
	VkPipelineExecutableInternalRepresentationKHR
	pInternalRepresentations

**/
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = unsafe extern "system" fn(
	device: VkDevice,
	pExecutableInfo: *const VkPipelineExecutableInfoKHR,
	pInternalRepresentationCount: *const u32,
	pInternalRepresentations: *const VkPipelineExecutableInternalRepresentationKHR
) -> VkResult;

/** vkCmdSetLineStippleEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	lineStippleFactor

	
	uint16_t
	lineStipplePattern

**/
pub type PFN_vkCmdSetLineStippleEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	lineStippleFactor: u32,
	lineStipplePattern: uint16_t
) -> VkResult;

/** vkGetPhysicalDeviceToolProperties
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pToolCount

	*
	VkPhysicalDeviceToolProperties
	pToolProperties

**/
pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pToolCount: *const u32,
	pToolProperties: *const VkPhysicalDeviceToolProperties
) -> VkResult;

/** vkCreateAccelerationStructureKHR
	
	VkDevice
	device

	const *
	VkAccelerationStructureCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkAccelerationStructureKHR
	pAccelerationStructure

**/
pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkAccelerationStructureCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pAccelerationStructure: *const VkAccelerationStructureKHR
) -> VkResult;

/** vkCmdBuildAccelerationStructuresKHR
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	infoCount

	const *
	VkAccelerationStructureBuildGeometryInfoKHR
	pInfos

	const * const*
	VkAccelerationStructureBuildRangeInfoKHR
	ppBuildRangeInfos

**/
pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	infoCount: u32,
	pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
	ppBuildRangeInfos: *const const* VkAccelerationStructureBuildRangeInfoKHR
) -> VkResult;

/** vkCmdBuildAccelerationStructuresIndirectKHR
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	infoCount

	const *
	VkAccelerationStructureBuildGeometryInfoKHR
	pInfos

	const *
	VkDeviceAddress
	pIndirectDeviceAddresses

	const *
	uint32_t
	pIndirectStrides

	const * const*
	uint32_t
	ppMaxPrimitiveCounts

**/
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	infoCount: u32,
	pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
	pIndirectDeviceAddresses: *const VkDeviceAddress,
	pIndirectStrides: *const u32,
	ppMaxPrimitiveCounts: *const const* u32
) -> VkResult;

/** vkBuildAccelerationStructuresKHR
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	deferredOperation

	
	uint32_t
	infoCount

	const *
	VkAccelerationStructureBuildGeometryInfoKHR
	pInfos

	const * const*
	VkAccelerationStructureBuildRangeInfoKHR
	ppBuildRangeInfos

**/
pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	infoCount: u32,
	pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
	ppBuildRangeInfos: *const const* VkAccelerationStructureBuildRangeInfoKHR
) -> VkResult;

/** vkGetAccelerationStructureDeviceAddressKHR
	
	VkDevice
	device

	const *
	VkAccelerationStructureDeviceAddressInfoKHR
	pInfo

**/
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR = unsafe extern "system" fn(
	device: VkDevice,
	pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR
) -> VkResult;

/** vkCreateDeferredOperationKHR
	
	VkDevice
	device

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkDeferredOperationKHR
	pDeferredOperation

**/
pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
	device: VkDevice,
	pAllocator: *const VkAllocationCallbacks,
	pDeferredOperation: *const VkDeferredOperationKHR
) -> VkResult;

/** vkDestroyDeferredOperationKHR
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	operation

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
	device: VkDevice,
	operation: VkDeferredOperationKHR,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetDeferredOperationMaxConcurrencyKHR
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	operation

**/
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR = unsafe extern "system" fn(
	device: VkDevice,
	operation: VkDeferredOperationKHR
) -> VkResult;

/** vkGetDeferredOperationResultKHR
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	operation

**/
pub type PFN_vkGetDeferredOperationResultKHR = unsafe extern "system" fn(
	device: VkDevice,
	operation: VkDeferredOperationKHR
) -> VkResult;

/** vkDeferredOperationJoinKHR
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	operation

**/
pub type PFN_vkDeferredOperationJoinKHR = unsafe extern "system" fn(
	device: VkDevice,
	operation: VkDeferredOperationKHR
) -> VkResult;

/** vkCmdSetCullMode
	
	VkCommandBuffer
	commandBuffer

	
	VkCullModeFlags
	cullMode

**/
pub type PFN_vkCmdSetCullMode = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	cullMode: VkCullModeFlags
) -> VkResult;

/** vkCmdSetFrontFace
	
	VkCommandBuffer
	commandBuffer

	
	VkFrontFace
	frontFace

**/
pub type PFN_vkCmdSetFrontFace = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	frontFace: VkFrontFace
) -> VkResult;

/** vkCmdSetPrimitiveTopology
	
	VkCommandBuffer
	commandBuffer

	
	VkPrimitiveTopology
	primitiveTopology

**/
pub type PFN_vkCmdSetPrimitiveTopology = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	primitiveTopology: VkPrimitiveTopology
) -> VkResult;

/** vkCmdSetViewportWithCount
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	viewportCount

	const *
	VkViewport
	pViewports

**/
pub type PFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	viewportCount: u32,
	pViewports: *const VkViewport
) -> VkResult;

/** vkCmdSetScissorWithCount
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	scissorCount

	const *
	VkRect2D
	pScissors

**/
pub type PFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	scissorCount: u32,
	pScissors: *const VkRect2D
) -> VkResult;

/** vkCmdBindVertexBuffers2
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstBinding

	
	uint32_t
	bindingCount

	const *
	VkBuffer
	pBuffers

	const *
	VkDeviceSize
	pOffsets

	const *
	VkDeviceSize
	pSizes

	const *
	VkDeviceSize
	pStrides

**/
pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstBinding: u32,
	bindingCount: u32,
	pBuffers: *const VkBuffer,
	pOffsets: *const VkDeviceSize,
	pSizes: *const VkDeviceSize,
	pStrides: *const VkDeviceSize
) -> VkResult;

/** vkCmdSetDepthTestEnable
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	depthTestEnable

**/
pub type PFN_vkCmdSetDepthTestEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthTestEnable: VkBool32
) -> VkResult;

/** vkCmdSetDepthWriteEnable
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	depthWriteEnable

**/
pub type PFN_vkCmdSetDepthWriteEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthWriteEnable: VkBool32
) -> VkResult;

/** vkCmdSetDepthCompareOp
	
	VkCommandBuffer
	commandBuffer

	
	VkCompareOp
	depthCompareOp

**/
pub type PFN_vkCmdSetDepthCompareOp = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthCompareOp: VkCompareOp
) -> VkResult;

/** vkCmdSetDepthBoundsTestEnable
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	depthBoundsTestEnable

**/
pub type PFN_vkCmdSetDepthBoundsTestEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthBoundsTestEnable: VkBool32
) -> VkResult;

/** vkCmdSetStencilTestEnable
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	stencilTestEnable

**/
pub type PFN_vkCmdSetStencilTestEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	stencilTestEnable: VkBool32
) -> VkResult;

/** vkCmdSetStencilOp
	
	VkCommandBuffer
	commandBuffer

	
	VkStencilFaceFlags
	faceMask

	
	VkStencilOp
	failOp

	
	VkStencilOp
	passOp

	
	VkStencilOp
	depthFailOp

	
	VkCompareOp
	compareOp

**/
pub type PFN_vkCmdSetStencilOp = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	faceMask: VkStencilFaceFlags,
	failOp: VkStencilOp,
	passOp: VkStencilOp,
	depthFailOp: VkStencilOp,
	compareOp: VkCompareOp
) -> VkResult;

/** vkCmdSetPatchControlPointsEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	patchControlPoints

**/
pub type PFN_vkCmdSetPatchControlPointsEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	patchControlPoints: u32
) -> VkResult;

/** vkCmdSetRasterizerDiscardEnable
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	rasterizerDiscardEnable

**/
pub type PFN_vkCmdSetRasterizerDiscardEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	rasterizerDiscardEnable: VkBool32
) -> VkResult;

/** vkCmdSetDepthBiasEnable
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	depthBiasEnable

**/
pub type PFN_vkCmdSetDepthBiasEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthBiasEnable: VkBool32
) -> VkResult;

/** vkCmdSetLogicOpEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkLogicOp
	logicOp

**/
pub type PFN_vkCmdSetLogicOpEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	logicOp: VkLogicOp
) -> VkResult;

/** vkCmdSetPrimitiveRestartEnable
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	primitiveRestartEnable

**/
pub type PFN_vkCmdSetPrimitiveRestartEnable = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	primitiveRestartEnable: VkBool32
) -> VkResult;

/** vkCmdSetTessellationDomainOriginEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkTessellationDomainOrigin
	domainOrigin

**/
pub type PFN_vkCmdSetTessellationDomainOriginEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	domainOrigin: VkTessellationDomainOrigin
) -> VkResult;

/** vkCmdSetDepthClampEnableEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	depthClampEnable

**/
pub type PFN_vkCmdSetDepthClampEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthClampEnable: VkBool32
) -> VkResult;

/** vkCmdSetPolygonModeEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkPolygonMode
	polygonMode

**/
pub type PFN_vkCmdSetPolygonModeEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	polygonMode: VkPolygonMode
) -> VkResult;

/** vkCmdSetRasterizationSamplesEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkSampleCountFlagBits
	rasterizationSamples

**/
pub type PFN_vkCmdSetRasterizationSamplesEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	rasterizationSamples: VkSampleCountFlagBits
) -> VkResult;

/** vkCmdSetSampleMaskEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkSampleCountFlagBits
	samples

	const *
	VkSampleMask
	pSampleMask

**/
pub type PFN_vkCmdSetSampleMaskEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	samples: VkSampleCountFlagBits,
	pSampleMask: *const VkSampleMask
) -> VkResult;

/** vkCmdSetAlphaToCoverageEnableEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	alphaToCoverageEnable

**/
pub type PFN_vkCmdSetAlphaToCoverageEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	alphaToCoverageEnable: VkBool32
) -> VkResult;

/** vkCmdSetAlphaToOneEnableEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	alphaToOneEnable

**/
pub type PFN_vkCmdSetAlphaToOneEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	alphaToOneEnable: VkBool32
) -> VkResult;

/** vkCmdSetLogicOpEnableEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	logicOpEnable

**/
pub type PFN_vkCmdSetLogicOpEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	logicOpEnable: VkBool32
) -> VkResult;

/** vkCmdSetColorBlendEnableEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstAttachment

	
	uint32_t
	attachmentCount

	const *
	VkBool32
	pColorBlendEnables

**/
pub type PFN_vkCmdSetColorBlendEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstAttachment: u32,
	attachmentCount: u32,
	pColorBlendEnables: *const VkBool32
) -> VkResult;

/** vkCmdSetColorBlendEquationEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstAttachment

	
	uint32_t
	attachmentCount

	const *
	VkColorBlendEquationEXT
	pColorBlendEquations

**/
pub type PFN_vkCmdSetColorBlendEquationEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstAttachment: u32,
	attachmentCount: u32,
	pColorBlendEquations: *const VkColorBlendEquationEXT
) -> VkResult;

/** vkCmdSetColorWriteMaskEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstAttachment

	
	uint32_t
	attachmentCount

	const *
	VkColorComponentFlags
	pColorWriteMasks

**/
pub type PFN_vkCmdSetColorWriteMaskEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstAttachment: u32,
	attachmentCount: u32,
	pColorWriteMasks: *const VkColorComponentFlags
) -> VkResult;

/** vkCmdSetRasterizationStreamEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	rasterizationStream

**/
pub type PFN_vkCmdSetRasterizationStreamEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	rasterizationStream: u32
) -> VkResult;

/** vkCmdSetConservativeRasterizationModeEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkConservativeRasterizationModeEXT
	conservativeRasterizationMode

**/
pub type PFN_vkCmdSetConservativeRasterizationModeEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	conservativeRasterizationMode: VkConservativeRasterizationModeEXT
) -> VkResult;

/** vkCmdSetExtraPrimitiveOverestimationSizeEXT
	
	VkCommandBuffer
	commandBuffer

	
	float
	extraPrimitiveOverestimationSize

**/
pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	extraPrimitiveOverestimationSize: float
) -> VkResult;

/** vkCmdSetDepthClipEnableEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	depthClipEnable

**/
pub type PFN_vkCmdSetDepthClipEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	depthClipEnable: VkBool32
) -> VkResult;

/** vkCmdSetSampleLocationsEnableEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	sampleLocationsEnable

**/
pub type PFN_vkCmdSetSampleLocationsEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	sampleLocationsEnable: VkBool32
) -> VkResult;

/** vkCmdSetColorBlendAdvancedEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstAttachment

	
	uint32_t
	attachmentCount

	const *
	VkColorBlendAdvancedEXT
	pColorBlendAdvanced

**/
pub type PFN_vkCmdSetColorBlendAdvancedEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstAttachment: u32,
	attachmentCount: u32,
	pColorBlendAdvanced: *const VkColorBlendAdvancedEXT
) -> VkResult;

/** vkCmdSetProvokingVertexModeEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkProvokingVertexModeEXT
	provokingVertexMode

**/
pub type PFN_vkCmdSetProvokingVertexModeEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	provokingVertexMode: VkProvokingVertexModeEXT
) -> VkResult;

/** vkCmdSetLineRasterizationModeEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkLineRasterizationModeEXT
	lineRasterizationMode

**/
pub type PFN_vkCmdSetLineRasterizationModeEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	lineRasterizationMode: VkLineRasterizationModeEXT
) -> VkResult;

/** vkCmdSetLineStippleEnableEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	stippledLineEnable

**/
pub type PFN_vkCmdSetLineStippleEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	stippledLineEnable: VkBool32
) -> VkResult;

/** vkCmdSetDepthClipNegativeOneToOneEXT
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	negativeOneToOne

**/
pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	negativeOneToOne: VkBool32
) -> VkResult;

/** vkCmdSetViewportWScalingEnableNV
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	viewportWScalingEnable

**/
pub type PFN_vkCmdSetViewportWScalingEnableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	viewportWScalingEnable: VkBool32
) -> VkResult;

/** vkCmdSetViewportSwizzleNV
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	firstViewport

	
	uint32_t
	viewportCount

	const *
	VkViewportSwizzleNV
	pViewportSwizzles

**/
pub type PFN_vkCmdSetViewportSwizzleNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	firstViewport: u32,
	viewportCount: u32,
	pViewportSwizzles: *const VkViewportSwizzleNV
) -> VkResult;

/** vkCmdSetCoverageToColorEnableNV
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	coverageToColorEnable

**/
pub type PFN_vkCmdSetCoverageToColorEnableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageToColorEnable: VkBool32
) -> VkResult;

/** vkCmdSetCoverageToColorLocationNV
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	coverageToColorLocation

**/
pub type PFN_vkCmdSetCoverageToColorLocationNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageToColorLocation: u32
) -> VkResult;

/** vkCmdSetCoverageModulationModeNV
	
	VkCommandBuffer
	commandBuffer

	
	VkCoverageModulationModeNV
	coverageModulationMode

**/
pub type PFN_vkCmdSetCoverageModulationModeNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageModulationMode: VkCoverageModulationModeNV
) -> VkResult;

/** vkCmdSetCoverageModulationTableEnableNV
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	coverageModulationTableEnable

**/
pub type PFN_vkCmdSetCoverageModulationTableEnableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageModulationTableEnable: VkBool32
) -> VkResult;

/** vkCmdSetCoverageModulationTableNV
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	coverageModulationTableCount

	const *
	float
	pCoverageModulationTable

**/
pub type PFN_vkCmdSetCoverageModulationTableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageModulationTableCount: u32,
	pCoverageModulationTable: *const float
) -> VkResult;

/** vkCmdSetShadingRateImageEnableNV
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	shadingRateImageEnable

**/
pub type PFN_vkCmdSetShadingRateImageEnableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	shadingRateImageEnable: VkBool32
) -> VkResult;

/** vkCmdSetCoverageReductionModeNV
	
	VkCommandBuffer
	commandBuffer

	
	VkCoverageReductionModeNV
	coverageReductionMode

**/
pub type PFN_vkCmdSetCoverageReductionModeNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	coverageReductionMode: VkCoverageReductionModeNV
) -> VkResult;

/** vkCmdSetRepresentativeFragmentTestEnableNV
	
	VkCommandBuffer
	commandBuffer

	
	VkBool32
	representativeFragmentTestEnable

**/
pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	representativeFragmentTestEnable: VkBool32
) -> VkResult;

/** vkCreatePrivateDataSlot
	
	VkDevice
	device

	const *
	VkPrivateDataSlotCreateInfo
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkPrivateDataSlot
	pPrivateDataSlot

**/
pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkPrivateDataSlotCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pPrivateDataSlot: *const VkPrivateDataSlot
) -> VkResult;

/** vkDestroyPrivateDataSlot
	
	VkDevice
	device

	
	VkPrivateDataSlot
	privateDataSlot

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(
	device: VkDevice,
	privateDataSlot: VkPrivateDataSlot,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkSetPrivateData
	
	VkDevice
	device

	
	VkObjectType
	objectType

	
	uint64_t
	objectHandle

	
	VkPrivateDataSlot
	privateDataSlot

	
	uint64_t
	data

**/
pub type PFN_vkSetPrivateData = unsafe extern "system" fn(
	device: VkDevice,
	objectType: VkObjectType,
	objectHandle: uint64_t,
	privateDataSlot: VkPrivateDataSlot,
	data: uint64_t
) -> VkResult;

/** vkGetPrivateData
	
	VkDevice
	device

	
	VkObjectType
	objectType

	
	uint64_t
	objectHandle

	
	VkPrivateDataSlot
	privateDataSlot

	*
	uint64_t
	pData

**/
pub type PFN_vkGetPrivateData = unsafe extern "system" fn(
	device: VkDevice,
	objectType: VkObjectType,
	objectHandle: uint64_t,
	privateDataSlot: VkPrivateDataSlot,
	pData: *const uint64_t
) -> VkResult;

/** vkCmdCopyBuffer2
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCopyBufferInfo2
	pCopyBufferInfo

**/
pub type PFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCopyBufferInfo: *const VkCopyBufferInfo2
) -> VkResult;

/** vkCmdCopyImage2
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCopyImageInfo2
	pCopyImageInfo

**/
pub type PFN_vkCmdCopyImage2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCopyImageInfo: *const VkCopyImageInfo2
) -> VkResult;

/** vkCmdBlitImage2
	
	VkCommandBuffer
	commandBuffer

	const *
	VkBlitImageInfo2
	pBlitImageInfo

**/
pub type PFN_vkCmdBlitImage2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pBlitImageInfo: *const VkBlitImageInfo2
) -> VkResult;

/** vkCmdCopyBufferToImage2
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCopyBufferToImageInfo2
	pCopyBufferToImageInfo

**/
pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2
) -> VkResult;

/** vkCmdCopyImageToBuffer2
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCopyImageToBufferInfo2
	pCopyImageToBufferInfo

**/
pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2
) -> VkResult;

/** vkCmdResolveImage2
	
	VkCommandBuffer
	commandBuffer

	const *
	VkResolveImageInfo2
	pResolveImageInfo

**/
pub type PFN_vkCmdResolveImage2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pResolveImageInfo: *const VkResolveImageInfo2
) -> VkResult;

/** vkCmdSetFragmentShadingRateKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkExtent2D
	pFragmentSize

	const     [2]
	VkFragmentShadingRateCombinerOpKHR
	combinerOps

**/
pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pFragmentSize: *const VkExtent2D,
	combinerOps: const     [2] VkFragmentShadingRateCombinerOpKHR
) -> VkResult;

/** vkGetPhysicalDeviceFragmentShadingRatesKHR
	
	VkPhysicalDevice
	physicalDevice

	*
	uint32_t
	pFragmentShadingRateCount

	*
	VkPhysicalDeviceFragmentShadingRateKHR
	pFragmentShadingRates

**/
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pFragmentShadingRateCount: *const u32,
	pFragmentShadingRates: *const VkPhysicalDeviceFragmentShadingRateKHR
) -> VkResult;

/** vkCmdSetFragmentShadingRateEnumNV
	
	VkCommandBuffer
	commandBuffer

	
	VkFragmentShadingRateNV
	shadingRate

	const     [2]
	VkFragmentShadingRateCombinerOpKHR
	combinerOps

**/
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	shadingRate: VkFragmentShadingRateNV,
	combinerOps: const     [2] VkFragmentShadingRateCombinerOpKHR
) -> VkResult;

/** vkGetAccelerationStructureBuildSizesKHR
	
	VkDevice
	device

	
	VkAccelerationStructureBuildTypeKHR
	buildType

	const *
	VkAccelerationStructureBuildGeometryInfoKHR
	pBuildInfo

	const *
	uint32_t
	pMaxPrimitiveCounts

	*
	VkAccelerationStructureBuildSizesInfoKHR
	pSizeInfo

**/
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(
	device: VkDevice,
	buildType: VkAccelerationStructureBuildTypeKHR,
	pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR,
	pMaxPrimitiveCounts: *const u32,
	pSizeInfo: *const VkAccelerationStructureBuildSizesInfoKHR
) -> VkResult;

/** vkCmdSetVertexInputEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	vertexBindingDescriptionCount

	const *
	VkVertexInputBindingDescription2EXT
	pVertexBindingDescriptions

	
	uint32_t
	vertexAttributeDescriptionCount

	const *
	VkVertexInputAttributeDescription2EXT
	pVertexAttributeDescriptions

**/
pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	vertexBindingDescriptionCount: u32,
	pVertexBindingDescriptions: *const VkVertexInputBindingDescription2EXT,
	vertexAttributeDescriptionCount: u32,
	pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription2EXT
) -> VkResult;

/** vkCmdSetColorWriteEnableEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	attachmentCount

	const *
	VkBool32
	pColorWriteEnables

**/
pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	attachmentCount: u32,
	pColorWriteEnables: *const VkBool32
) -> VkResult;

/** vkCmdSetEvent2
	
	VkCommandBuffer
	commandBuffer

	
	VkEvent
	event

	const *
	VkDependencyInfo
	pDependencyInfo

**/
pub type PFN_vkCmdSetEvent2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	event: VkEvent,
	pDependencyInfo: *const VkDependencyInfo
) -> VkResult;

/** vkCmdResetEvent2
	
	VkCommandBuffer
	commandBuffer

	
	VkEvent
	event

	
	VkPipelineStageFlags2
	stageMask

**/
pub type PFN_vkCmdResetEvent2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	event: VkEvent,
	stageMask: VkPipelineStageFlags2
) -> VkResult;

/** vkCmdWaitEvents2
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	eventCount

	const *
	VkEvent
	pEvents

	const *
	VkDependencyInfo
	pDependencyInfos

**/
pub type PFN_vkCmdWaitEvents2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	eventCount: u32,
	pEvents: *const VkEvent,
	pDependencyInfos: *const VkDependencyInfo
) -> VkResult;

/** vkCmdPipelineBarrier2
	
	VkCommandBuffer
	commandBuffer

	const *
	VkDependencyInfo
	pDependencyInfo

**/
pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pDependencyInfo: *const VkDependencyInfo
) -> VkResult;

/** vkQueueSubmit2
	
	VkQueue
	queue

	
	uint32_t
	submitCount

	const *
	VkSubmitInfo2
	pSubmits

	
	VkFence
	fence

**/
pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(
	queue: VkQueue,
	submitCount: u32,
	pSubmits: *const VkSubmitInfo2,
	fence: VkFence
) -> VkResult;

/** vkCmdWriteTimestamp2
	
	VkCommandBuffer
	commandBuffer

	
	VkPipelineStageFlags2
	stage

	
	VkQueryPool
	queryPool

	
	uint32_t
	query

**/
pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	stage: VkPipelineStageFlags2,
	queryPool: VkQueryPool,
	query: u32
) -> VkResult;

/** vkCmdWriteBufferMarker2AMD
	
	VkCommandBuffer
	commandBuffer

	
	VkPipelineStageFlags2
	stage

	
	VkBuffer
	dstBuffer

	
	VkDeviceSize
	dstOffset

	
	uint32_t
	marker

**/
pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	stage: VkPipelineStageFlags2,
	dstBuffer: VkBuffer,
	dstOffset: VkDeviceSize,
	marker: u32
) -> VkResult;

/** vkGetQueueCheckpointData2NV
	
	VkQueue
	queue

	*
	uint32_t
	pCheckpointDataCount

	*
	VkCheckpointData2NV
	pCheckpointData

**/
pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
	queue: VkQueue,
	pCheckpointDataCount: *const u32,
	pCheckpointData: *const VkCheckpointData2NV
) -> VkResult;

/** vkGetPhysicalDeviceVideoCapabilitiesKHR
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkVideoProfileInfoKHR
	pVideoProfile

	*
	VkVideoCapabilitiesKHR
	pCapabilities

**/
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pVideoProfile: *const VkVideoProfileInfoKHR,
	pCapabilities: *const VkVideoCapabilitiesKHR
) -> VkResult;

/** vkGetPhysicalDeviceVideoFormatPropertiesKHR
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkPhysicalDeviceVideoFormatInfoKHR
	pVideoFormatInfo

	*
	uint32_t
	pVideoFormatPropertyCount

	*
	VkVideoFormatPropertiesKHR
	pVideoFormatProperties

**/
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pVideoFormatInfo: *const VkPhysicalDeviceVideoFormatInfoKHR,
	pVideoFormatPropertyCount: *const u32,
	pVideoFormatProperties: *const VkVideoFormatPropertiesKHR
) -> VkResult;

/** vkCreateVideoSessionKHR
	
	VkDevice
	device

	const *
	VkVideoSessionCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkVideoSessionKHR
	pVideoSession

**/
pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkVideoSessionCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pVideoSession: *const VkVideoSessionKHR
) -> VkResult;

/** vkDestroyVideoSessionKHR
	
	VkDevice
	device

	
	VkVideoSessionKHR
	videoSession

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(
	device: VkDevice,
	videoSession: VkVideoSessionKHR,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCreateVideoSessionParametersKHR
	
	VkDevice
	device

	const *
	VkVideoSessionParametersCreateInfoKHR
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkVideoSessionParametersKHR
	pVideoSessionParameters

**/
pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkVideoSessionParametersCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pVideoSessionParameters: *const VkVideoSessionParametersKHR
) -> VkResult;

/** vkUpdateVideoSessionParametersKHR
	
	VkDevice
	device

	
	VkVideoSessionParametersKHR
	videoSessionParameters

	const *
	VkVideoSessionParametersUpdateInfoKHR
	pUpdateInfo

**/
pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
	device: VkDevice,
	videoSessionParameters: VkVideoSessionParametersKHR,
	pUpdateInfo: *const VkVideoSessionParametersUpdateInfoKHR
) -> VkResult;

/** vkDestroyVideoSessionParametersKHR
	
	VkDevice
	device

	
	VkVideoSessionParametersKHR
	videoSessionParameters

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
	device: VkDevice,
	videoSessionParameters: VkVideoSessionParametersKHR,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetVideoSessionMemoryRequirementsKHR
	
	VkDevice
	device

	
	VkVideoSessionKHR
	videoSession

	*
	uint32_t
	pMemoryRequirementsCount

	*
	VkVideoSessionMemoryRequirementsKHR
	pMemoryRequirements

**/
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
	device: VkDevice,
	videoSession: VkVideoSessionKHR,
	pMemoryRequirementsCount: *const u32,
	pMemoryRequirements: *const VkVideoSessionMemoryRequirementsKHR
) -> VkResult;

/** vkBindVideoSessionMemoryKHR
	
	VkDevice
	device

	
	VkVideoSessionKHR
	videoSession

	
	uint32_t
	bindSessionMemoryInfoCount

	const *
	VkBindVideoSessionMemoryInfoKHR
	pBindSessionMemoryInfos

**/
pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
	device: VkDevice,
	videoSession: VkVideoSessionKHR,
	bindSessionMemoryInfoCount: u32,
	pBindSessionMemoryInfos: *const VkBindVideoSessionMemoryInfoKHR
) -> VkResult;

/** vkCmdDecodeVideoKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkVideoDecodeInfoKHR
	pDecodeInfo

**/
pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pDecodeInfo: *const VkVideoDecodeInfoKHR
) -> VkResult;

/** vkCmdBeginVideoCodingKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkVideoBeginCodingInfoKHR
	pBeginInfo

**/
pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pBeginInfo: *const VkVideoBeginCodingInfoKHR
) -> VkResult;

/** vkCmdControlVideoCodingKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkVideoCodingControlInfoKHR
	pCodingControlInfo

**/
pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pCodingControlInfo: *const VkVideoCodingControlInfoKHR
) -> VkResult;

/** vkCmdEndVideoCodingKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkVideoEndCodingInfoKHR
	pEndCodingInfo

**/
pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pEndCodingInfo: *const VkVideoEndCodingInfoKHR
) -> VkResult;

/** vkCmdEncodeVideoKHR
	
	VkCommandBuffer
	commandBuffer

	const *
	VkVideoEncodeInfoKHR
	pEncodeInfo

**/
pub type PFN_vkCmdEncodeVideoKHR = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pEncodeInfo: *const VkVideoEncodeInfoKHR
) -> VkResult;

/** vkCmdDecompressMemoryNV
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	decompressRegionCount

	const *
	VkDecompressMemoryRegionNV
	pDecompressMemoryRegions

**/
pub type PFN_vkCmdDecompressMemoryNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	decompressRegionCount: u32,
	pDecompressMemoryRegions: *const VkDecompressMemoryRegionNV
) -> VkResult;

/** vkCmdDecompressMemoryIndirectCountNV
	
	VkCommandBuffer
	commandBuffer

	
	VkDeviceAddress
	indirectCommandsAddress

	
	VkDeviceAddress
	indirectCommandsCountAddress

	
	uint32_t
	stride

**/
pub type PFN_vkCmdDecompressMemoryIndirectCountNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	indirectCommandsAddress: VkDeviceAddress,
	indirectCommandsCountAddress: VkDeviceAddress,
	stride: u32
) -> VkResult;

/** vkCreateCuModuleNVX
	
	VkDevice
	device

	const *
	VkCuModuleCreateInfoNVX
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkCuModuleNVX
	pModule

**/
pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkCuModuleCreateInfoNVX,
	pAllocator: *const VkAllocationCallbacks,
	pModule: *const VkCuModuleNVX
) -> VkResult;

/** vkCreateCuFunctionNVX
	
	VkDevice
	device

	const *
	VkCuFunctionCreateInfoNVX
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkCuFunctionNVX
	pFunction

**/
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkCuFunctionCreateInfoNVX,
	pAllocator: *const VkAllocationCallbacks,
	pFunction: *const VkCuFunctionNVX
) -> VkResult;

/** vkDestroyCuModuleNVX
	
	VkDevice
	device

	
	VkCuModuleNVX
	module

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(
	device: VkDevice,
	module: VkCuModuleNVX,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkDestroyCuFunctionNVX
	
	VkDevice
	device

	
	VkCuFunctionNVX
	function

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(
	device: VkDevice,
	function: VkCuFunctionNVX,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCmdCuLaunchKernelNVX
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCuLaunchInfoNVX
	pLaunchInfo

**/
pub type PFN_vkCmdCuLaunchKernelNVX = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pLaunchInfo: *const VkCuLaunchInfoNVX
) -> VkResult;

/** vkSetDeviceMemoryPriorityEXT
	
	VkDevice
	device

	
	VkDeviceMemory
	memory

	
	float
	priority

**/
pub type PFN_vkSetDeviceMemoryPriorityEXT = unsafe extern "system" fn(
	device: VkDevice,
	memory: VkDeviceMemory,
	priority: float
) -> VkResult;

/** vkAcquireDrmDisplayEXT
	
	VkPhysicalDevice
	physicalDevice

	
	int32_t
	drmFd

	
	VkDisplayKHR
	display

**/
pub type PFN_vkAcquireDrmDisplayEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	drmFd: int32_t,
	display: VkDisplayKHR
) -> VkResult;

/** vkGetDrmDisplayEXT
	
	VkPhysicalDevice
	physicalDevice

	
	int32_t
	drmFd

	
	uint32_t
	connectorId

	*
	VkDisplayKHR
	display

**/
pub type PFN_vkGetDrmDisplayEXT = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	drmFd: int32_t,
	connectorId: u32,
	display: *const VkDisplayKHR
) -> VkResult;

/** vkWaitForPresentKHR
	
	VkDevice
	device

	
	VkSwapchainKHR
	swapchain

	
	uint64_t
	presentId

	
	uint64_t
	timeout

**/
pub type PFN_vkWaitForPresentKHR = unsafe extern "system" fn(
	device: VkDevice,
	swapchain: VkSwapchainKHR,
	presentId: uint64_t,
	timeout: uint64_t
) -> VkResult;

/** vkCreateBufferCollectionFUCHSIA
	
	VkDevice
	device

	const *
	VkBufferCollectionCreateInfoFUCHSIA
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkBufferCollectionFUCHSIA
	pCollection

**/
pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkBufferCollectionCreateInfoFUCHSIA,
	pAllocator: *const VkAllocationCallbacks,
	pCollection: *const VkBufferCollectionFUCHSIA
) -> VkResult;

/** vkSetBufferCollectionBufferConstraintsFUCHSIA
	
	VkDevice
	device

	
	VkBufferCollectionFUCHSIA
	collection

	const *
	VkBufferConstraintsInfoFUCHSIA
	pBufferConstraintsInfo

**/
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	collection: VkBufferCollectionFUCHSIA,
	pBufferConstraintsInfo: *const VkBufferConstraintsInfoFUCHSIA
) -> VkResult;

/** vkSetBufferCollectionImageConstraintsFUCHSIA
	
	VkDevice
	device

	
	VkBufferCollectionFUCHSIA
	collection

	const *
	VkImageConstraintsInfoFUCHSIA
	pImageConstraintsInfo

**/
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	collection: VkBufferCollectionFUCHSIA,
	pImageConstraintsInfo: *const VkImageConstraintsInfoFUCHSIA
) -> VkResult;

/** vkDestroyBufferCollectionFUCHSIA
	
	VkDevice
	device

	
	VkBufferCollectionFUCHSIA
	collection

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	collection: VkBufferCollectionFUCHSIA,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkGetBufferCollectionPropertiesFUCHSIA
	
	VkDevice
	device

	
	VkBufferCollectionFUCHSIA
	collection

	*
	VkBufferCollectionPropertiesFUCHSIA
	pProperties

**/
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
	device: VkDevice,
	collection: VkBufferCollectionFUCHSIA,
	pProperties: *const VkBufferCollectionPropertiesFUCHSIA
) -> VkResult;

/** vkCmdBeginRendering
	
	VkCommandBuffer
	commandBuffer

	const *
	VkRenderingInfo
	pRenderingInfo

**/
pub type PFN_vkCmdBeginRendering = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pRenderingInfo: *const VkRenderingInfo
) -> VkResult;

/** vkCmdEndRendering
	
	VkCommandBuffer
	commandBuffer

**/
pub type PFN_vkCmdEndRendering = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer
) -> VkResult;

/** vkGetDescriptorSetLayoutHostMappingInfoVALVE
	
	VkDevice
	device

	const *
	VkDescriptorSetBindingReferenceVALVE
	pBindingReference

	*
	VkDescriptorSetLayoutHostMappingInfoVALVE
	pHostMapping

**/
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "system" fn(
	device: VkDevice,
	pBindingReference: *const VkDescriptorSetBindingReferenceVALVE,
	pHostMapping: *const VkDescriptorSetLayoutHostMappingInfoVALVE
) -> VkResult;

/** vkGetDescriptorSetHostMappingVALVE
	
	VkDevice
	device

	
	VkDescriptorSet
	descriptorSet

	**
	void
	ppData

**/
pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "system" fn(
	device: VkDevice,
	descriptorSet: VkDescriptorSet,
	ppData: ** void
) -> VkResult;

/** vkCreateMicromapEXT
	
	VkDevice
	device

	const *
	VkMicromapCreateInfoEXT
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkMicromapEXT
	pMicromap

**/
pub type PFN_vkCreateMicromapEXT = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkMicromapCreateInfoEXT,
	pAllocator: *const VkAllocationCallbacks,
	pMicromap: *const VkMicromapEXT
) -> VkResult;

/** vkCmdBuildMicromapsEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	infoCount

	const *
	VkMicromapBuildInfoEXT
	pInfos

**/
pub type PFN_vkCmdBuildMicromapsEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	infoCount: u32,
	pInfos: *const VkMicromapBuildInfoEXT
) -> VkResult;

/** vkBuildMicromapsEXT
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	deferredOperation

	
	uint32_t
	infoCount

	const *
	VkMicromapBuildInfoEXT
	pInfos

**/
pub type PFN_vkBuildMicromapsEXT = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	infoCount: u32,
	pInfos: *const VkMicromapBuildInfoEXT
) -> VkResult;

/** vkDestroyMicromapEXT
	
	VkDevice
	device

	
	VkMicromapEXT
	micromap

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyMicromapEXT = unsafe extern "system" fn(
	device: VkDevice,
	micromap: VkMicromapEXT,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkCmdCopyMicromapEXT
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCopyMicromapInfoEXT
	pInfo

**/
pub type PFN_vkCmdCopyMicromapEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *const VkCopyMicromapInfoEXT
) -> VkResult;

/** vkCopyMicromapEXT
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	deferredOperation

	const *
	VkCopyMicromapInfoEXT
	pInfo

**/
pub type PFN_vkCopyMicromapEXT = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *const VkCopyMicromapInfoEXT
) -> VkResult;

/** vkCmdCopyMicromapToMemoryEXT
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCopyMicromapToMemoryInfoEXT
	pInfo

**/
pub type PFN_vkCmdCopyMicromapToMemoryEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *const VkCopyMicromapToMemoryInfoEXT
) -> VkResult;

/** vkCopyMicromapToMemoryEXT
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	deferredOperation

	const *
	VkCopyMicromapToMemoryInfoEXT
	pInfo

**/
pub type PFN_vkCopyMicromapToMemoryEXT = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *const VkCopyMicromapToMemoryInfoEXT
) -> VkResult;

/** vkCmdCopyMemoryToMicromapEXT
	
	VkCommandBuffer
	commandBuffer

	const *
	VkCopyMemoryToMicromapInfoEXT
	pInfo

**/
pub type PFN_vkCmdCopyMemoryToMicromapEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *const VkCopyMemoryToMicromapInfoEXT
) -> VkResult;

/** vkCopyMemoryToMicromapEXT
	
	VkDevice
	device

	
	VkDeferredOperationKHR
	deferredOperation

	const *
	VkCopyMemoryToMicromapInfoEXT
	pInfo

**/
pub type PFN_vkCopyMemoryToMicromapEXT = unsafe extern "system" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pInfo: *const VkCopyMemoryToMicromapInfoEXT
) -> VkResult;

/** vkCmdWriteMicromapsPropertiesEXT
	
	VkCommandBuffer
	commandBuffer

	
	uint32_t
	micromapCount

	const *
	VkMicromapEXT
	pMicromaps

	
	VkQueryType
	queryType

	
	VkQueryPool
	queryPool

	
	uint32_t
	firstQuery

**/
pub type PFN_vkCmdWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	micromapCount: u32,
	pMicromaps: *const VkMicromapEXT,
	queryType: VkQueryType,
	queryPool: VkQueryPool,
	firstQuery: u32
) -> VkResult;

/** vkWriteMicromapsPropertiesEXT
	
	VkDevice
	device

	
	uint32_t
	micromapCount

	const *
	VkMicromapEXT
	pMicromaps

	
	VkQueryType
	queryType

	
	size_t
	dataSize

	*
	void
	pData

	
	size_t
	stride

**/
pub type PFN_vkWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
	device: VkDevice,
	micromapCount: u32,
	pMicromaps: *const VkMicromapEXT,
	queryType: VkQueryType,
	dataSize: size_t,
	pData: *const void,
	stride: size_t
) -> VkResult;

/** vkGetDeviceMicromapCompatibilityEXT
	
	VkDevice
	device

	const *
	VkMicromapVersionInfoEXT
	pVersionInfo

	*
	VkAccelerationStructureCompatibilityKHR
	pCompatibility

**/
pub type PFN_vkGetDeviceMicromapCompatibilityEXT = unsafe extern "system" fn(
	device: VkDevice,
	pVersionInfo: *const VkMicromapVersionInfoEXT,
	pCompatibility: *const VkAccelerationStructureCompatibilityKHR
) -> VkResult;

/** vkGetMicromapBuildSizesEXT
	
	VkDevice
	device

	
	VkAccelerationStructureBuildTypeKHR
	buildType

	const *
	VkMicromapBuildInfoEXT
	pBuildInfo

	*
	VkMicromapBuildSizesInfoEXT
	pSizeInfo

**/
pub type PFN_vkGetMicromapBuildSizesEXT = unsafe extern "system" fn(
	device: VkDevice,
	buildType: VkAccelerationStructureBuildTypeKHR,
	pBuildInfo: *const VkMicromapBuildInfoEXT,
	pSizeInfo: *const VkMicromapBuildSizesInfoEXT
) -> VkResult;

/** vkGetShaderModuleIdentifierEXT
	
	VkDevice
	device

	
	VkShaderModule
	shaderModule

	*
	VkShaderModuleIdentifierEXT
	pIdentifier

**/
pub type PFN_vkGetShaderModuleIdentifierEXT = unsafe extern "system" fn(
	device: VkDevice,
	shaderModule: VkShaderModule,
	pIdentifier: *const VkShaderModuleIdentifierEXT
) -> VkResult;

/** vkGetShaderModuleCreateInfoIdentifierEXT
	
	VkDevice
	device

	const *
	VkShaderModuleCreateInfo
	pCreateInfo

	*
	VkShaderModuleIdentifierEXT
	pIdentifier

**/
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkShaderModuleCreateInfo,
	pIdentifier: *const VkShaderModuleIdentifierEXT
) -> VkResult;

/** vkGetImageSubresourceLayout2EXT
	
	VkDevice
	device

	
	VkImage
	image

	const *
	VkImageSubresource2EXT
	pSubresource

	*
	VkSubresourceLayout2EXT
	pLayout

**/
pub type PFN_vkGetImageSubresourceLayout2EXT = unsafe extern "system" fn(
	device: VkDevice,
	image: VkImage,
	pSubresource: *const VkImageSubresource2EXT,
	pLayout: *const VkSubresourceLayout2EXT
) -> VkResult;

/** vkGetPipelinePropertiesEXT
	
	VkDevice
	device

	const *
	VkPipelineInfoEXT
	pPipelineInfo

	*
	VkBaseOutStructure
	pPipelineProperties

**/
pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
	device: VkDevice,
	pPipelineInfo: *const VkPipelineInfoEXT,
	pPipelineProperties: *const VkBaseOutStructure
) -> VkResult;

/** vkExportMetalObjectsEXT
	
	VkDevice
	device

	*
	VkExportMetalObjectsInfoEXT
	pMetalObjectsInfo

**/
pub type PFN_vkExportMetalObjectsEXT = unsafe extern "system" fn(
	device: VkDevice,
	pMetalObjectsInfo: *const VkExportMetalObjectsInfoEXT
) -> VkResult;

/** vkGetFramebufferTilePropertiesQCOM
	
	VkDevice
	device

	
	VkFramebuffer
	framebuffer

	*
	uint32_t
	pPropertiesCount

	*
	VkTilePropertiesQCOM
	pProperties

**/
pub type PFN_vkGetFramebufferTilePropertiesQCOM = unsafe extern "system" fn(
	device: VkDevice,
	framebuffer: VkFramebuffer,
	pPropertiesCount: *const u32,
	pProperties: *const VkTilePropertiesQCOM
) -> VkResult;

/** vkGetDynamicRenderingTilePropertiesQCOM
	
	VkDevice
	device

	const *
	VkRenderingInfo
	pRenderingInfo

	*
	VkTilePropertiesQCOM
	pProperties

**/
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "system" fn(
	device: VkDevice,
	pRenderingInfo: *const VkRenderingInfo,
	pProperties: *const VkTilePropertiesQCOM
) -> VkResult;

/** vkGetPhysicalDeviceOpticalFlowImageFormatsNV
	
	VkPhysicalDevice
	physicalDevice

	const *
	VkOpticalFlowImageFormatInfoNV
	pOpticalFlowImageFormatInfo

	*
	uint32_t
	pFormatCount

	*
	VkOpticalFlowImageFormatPropertiesNV
	pImageFormatProperties

**/
pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = unsafe extern "system" fn(
	physicalDevice: VkPhysicalDevice,
	pOpticalFlowImageFormatInfo: *const VkOpticalFlowImageFormatInfoNV,
	pFormatCount: *const u32,
	pImageFormatProperties: *const VkOpticalFlowImageFormatPropertiesNV
) -> VkResult;

/** vkCreateOpticalFlowSessionNV
	
	VkDevice
	device

	const *
	VkOpticalFlowSessionCreateInfoNV
	pCreateInfo

	const *
	VkAllocationCallbacks
	pAllocator

	*
	VkOpticalFlowSessionNV
	pSession

**/
pub type PFN_vkCreateOpticalFlowSessionNV = unsafe extern "system" fn(
	device: VkDevice,
	pCreateInfo: *const VkOpticalFlowSessionCreateInfoNV,
	pAllocator: *const VkAllocationCallbacks,
	pSession: *const VkOpticalFlowSessionNV
) -> VkResult;

/** vkDestroyOpticalFlowSessionNV
	
	VkDevice
	device

	
	VkOpticalFlowSessionNV
	session

	const *
	VkAllocationCallbacks
	pAllocator

**/
pub type PFN_vkDestroyOpticalFlowSessionNV = unsafe extern "system" fn(
	device: VkDevice,
	session: VkOpticalFlowSessionNV,
	pAllocator: *const VkAllocationCallbacks
) -> VkResult;

/** vkBindOpticalFlowSessionImageNV
	
	VkDevice
	device

	
	VkOpticalFlowSessionNV
	session

	
	VkOpticalFlowSessionBindingPointNV
	bindingPoint

	
	VkImageView
	view

	
	VkImageLayout
	layout

**/
pub type PFN_vkBindOpticalFlowSessionImageNV = unsafe extern "system" fn(
	device: VkDevice,
	session: VkOpticalFlowSessionNV,
	bindingPoint: VkOpticalFlowSessionBindingPointNV,
	view: VkImageView,
	layout: VkImageLayout
) -> VkResult;

/** vkCmdOpticalFlowExecuteNV
	
	VkCommandBuffer
	commandBuffer

	
	VkOpticalFlowSessionNV
	session

	const *
	VkOpticalFlowExecuteInfoNV
	pExecuteInfo

**/
pub type PFN_vkCmdOpticalFlowExecuteNV = unsafe extern "system" fn(
	commandBuffer: VkCommandBuffer,
	session: VkOpticalFlowSessionNV,
	pExecuteInfo: *const VkOpticalFlowExecuteInfoNV
) -> VkResult;

