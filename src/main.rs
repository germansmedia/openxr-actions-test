#![allow(non_snake_case)]

use std::{
    ptr::null_mut,
    mem::MaybeUninit,
    ffi::{
        c_char,
        c_void,
        CString,
    },
};

mod ffi;

fn vk_code_to_string(code: i32) -> &'static str {
    match code {
        ffi::VK_SUCCESS => "VK_SUCCESS",
        ffi::VK_NOT_READY => "VK_NOT_READY",
        ffi::VK_TIMEOUT => "VK_TIMEOUT",
        ffi::VK_EVENT_SET => "VK_EVENT_SET",
        ffi::VK_EVENT_RESET => "VK_EVENT_RESET",
        ffi::VK_INCOMPLETE => "VK_INCOMPLETE",
        ffi::VK_ERROR_OUT_OF_HOST_MEMORY => "VK_OUT_OF_HOST_MEMORY",
        ffi::VK_ERROR_OUT_OF_DEVICE_MEMORY => "VK_ERROR_OUT_OF_DEVICE_MEMORY",
        ffi::VK_ERROR_INITIALIZATION_FAILED => "VK_ERROR_INITIALIZATION_FAILED",
        ffi::VK_ERROR_DEVICE_LOST => "VK_ERROR_DEVICE_LOST",
        ffi::VK_ERROR_MEMORY_MAP_FAILED => "VK_ERROR_MEMORY_MAP_FAILED",
        ffi::VK_ERROR_LAYER_NOT_PRESENT => "VK_ERROR_LAYER_NOT_PRESENT",
        ffi::VK_ERROR_EXTENSION_NOT_PRESENT => "VK_ERROR_EXTENSION_NOT_PRESENT",
        ffi::VK_ERROR_FEATURE_NOT_PRESENT => "VK_ERROR_FEATURE_NOT_PRESENT",
        ffi::VK_ERROR_INCOMPATIBLE_DRIVER => "VK_ERROR_INCOMPATIBLE_DRIVER",
        ffi::VK_ERROR_TOO_MANY_OBJECTS => "VK_ERROR_TOO_MANY_OBJECTS",
        ffi::VK_ERROR_FORMAT_NOT_SUPPORTED => "VK_ERROR_FORMAT_NOT_SUPPORTED",
        ffi::VK_ERROR_FRAGMENTED_POOL => "VK_ERROR_FRAGMENTED_POOL",
        ffi::VK_ERROR_UNKNOWN => "VK_ERROR_UNKNOWN",
        ffi::VK_ERROR_OUT_OF_POOL_MEMORY => "VK_ERROR_OUT_OF_POOL_MEMORY",
        ffi::VK_ERROR_INVALID_EXTERNAL_HANDLE => "VK_ERROR_INVALID_EXTERNAL_HANDLE",
        ffi::VK_ERROR_FRAGMENTATION => "VK_ERROR_FRAGMENTATION",
        ffi::VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => "VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS",
        ffi::VK_ERROR_SURFACE_LOST_KHR => "VK_ERROR_SURFACE_LOST_KHR",
        ffi::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => "VK_ERROR_NATIVE_WINDOW_IN_USE_KHR",
        ffi::VK_SUBOPTIMAL_KHR => "VK_SUBOPTIMAL_KHR",
        ffi::VK_ERROR_OUT_OF_DATE_KHR => "VK_ERROR_OUT_OF_DATE_KHR",
        ffi::VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => "VK_ERROR_INCOMPATIBLE_DISPLAY_KHR",
        ffi::VK_ERROR_VALIDATION_FAILED_EXT => "VK_ERROR_VALIDATION_FAILED_EXT",
        ffi::VK_ERROR_INVALID_SHADER_NV => "VK_ERROR_INVALID_SHADER_NV",
        ffi::VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => "VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT",
        ffi::VK_ERROR_NOT_PERMITTED_EXT => "VK_ERROR_NOT_PERMITTED_EXT",
        ffi::VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => "VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT",
        _ => "(unknown)",
    }
}

fn xr_code_to_string(code: i32) -> &'static str {
    match code {
        ffi::XR_SUCCESS => "XR_SUCCESS",
        ffi::XR_TIMEOUT_EXPIRED => "XR_TIMEOUT_EXPIRED",
        ffi::XR_SESSION_LOSS_PENDING => "XR_SESSION_LOSS_PENDING",
        ffi::XR_EVENT_UNAVAILABLE => "XR_EVENT_UNAVAILABLE",
        ffi::XR_SPACE_BOUNDS_UNAVAILABLE => "XR_SPACE_BOUNDS_UNAVAILABLE",
        ffi::XR_SESSION_NOT_FOCUSED => "XR_SESSION_NOT_FOCUSED",
        ffi::XR_FRAME_DISCARDED => "XR_FRAME_DISCARDED",
        ffi::XR_ERROR_VALIDATION_FAILURE => "XR_ERROR_VALIDATION_FAILURE",
        ffi::XR_ERROR_RUNTIME_FAILURE => "XR_ERROR_RUNTIME_FAILURE",
        ffi::XR_ERROR_OUT_OF_MEMORY => "XR_ERROR_OUT_OF_MEMORY",
        ffi::XR_ERROR_API_VERSION_UNSUPPORTED => "XR_ERROR_API_VERSION_UNSUPPORTED",
        ffi::XR_ERROR_INITIALIZATION_FAILED => "XR_ERROR_INITIALIZATION_FAILED",
        ffi::XR_ERROR_FUNCTION_UNSUPPORTED => "XR_ERROR_FUNCTION_UNSUPPORTED",
        ffi::XR_ERROR_FEATURE_UNSUPPORTED => "XR_ERROR_FEATURE_UNSUPPORTED",
        ffi::XR_ERROR_EXTENSION_NOT_PRESENT => "XR_ERROR_EXTENSION_NOT_PRESENT",
        ffi::XR_ERROR_LIMIT_REACHED => "XR_ERROR_LIMIT_REACHED",
        ffi::XR_ERROR_SIZE_INSUFFICIENT => "XR_ERROR_SIZE_INSUFFICIENT",
        ffi::XR_ERROR_HANDLE_INVALID => "XR_ERROR_HANDLE_INVALID",
        ffi::XR_ERROR_INSTANCE_LOST => "XR_ERROR_INSTANCE_LOST",
        ffi::XR_ERROR_SESSION_RUNNING => "XR_ERROR_SESSION_RUNNING",
        ffi::XR_ERROR_SESSION_NOT_RUNNING => "XR_ERROR_SESSION_NOT_RUNNING",
        ffi::XR_ERROR_SESSION_LOST => "XR_ERROR_SESSION_LOST",
        ffi::XR_ERROR_SYSTEM_INVALID => "XR_ERROR_SYSTEM_INVALID",
        ffi::XR_ERROR_PATH_INVALID => "XR_ERROR_PATH_INVALID",
        ffi::XR_ERROR_PATH_COUNT_EXCEEDED => "XR_ERROR_PATH_COUNT_EXCEEDED",
        ffi::XR_ERROR_PATH_FORMAT_INVALID => "XR_ERROR_PATH_FORMAT_INVALID",
        ffi::XR_ERROR_PATH_UNSUPPORTED => "XR_ERROR_PATH_UNSUPPORTED",
        ffi::XR_ERROR_LAYER_INVALID => "XR_ERROR_LAYER_INVALID",
        ffi::XR_ERROR_LAYER_LIMIT_EXCEEDED => "XR_ERROR_LAYER_LIMIT_EXCEEDED",
        ffi::XR_ERROR_SWAPCHAIN_RECT_INVALID => "XR_ERROR_SWAPCHAIN_RECT_INVALID",
        ffi::XR_ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED => "XR_ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED",
        ffi::XR_ERROR_ACTION_TYPE_MISMATCH => "XR_ERROR_ACTION_TYPE_MISMATCH",
        ffi::XR_ERROR_SESSION_NOT_READY => "XR_ERROR_SESSION_NOT_READY",
        ffi::XR_ERROR_SESSION_NOT_STOPPING => "XR_ERROR_SESSION_NOT_STOPPING",
        ffi::XR_ERROR_TIME_INVALID => "XR_ERROR_TIME_INVALID",
        ffi::XR_ERROR_REFERENCE_SPACE_UNSUPPORTED => "XR_ERROR_REFERENCE_SPACE_UNSUPPORTED",
        ffi::XR_ERROR_FILE_ACCESS_ERROR => "XR_ERROR_FILE_ACCESS_ERROR",
        ffi::XR_ERROR_FILE_CONTENTS_INVALID => "XR_ERROR_FILE_CONTENTS_INVALID",
        ffi::XR_ERROR_FORM_FACTOR_UNSUPPORTED => "XR_ERROR_FORM_FACTOR_UNSUPPORTED",
        ffi::XR_ERROR_FORM_FACTOR_UNAVAILABLE => "XR_ERROR_FORM_FACTOR_UNAVAILABLE",
        ffi::XR_ERROR_API_LAYER_NOT_PRESENT => "XR_ERROR_API_LAYER_NOT_PRESENT",
        ffi::XR_ERROR_CALL_ORDER_INVALID => "XR_ERROR_CALL_ORDER_INVALID",
        ffi::XR_ERROR_GRAPHICS_DEVICE_INVALID => "XR_ERROR_GRAPHICS_DEVICE_INVALID",
        ffi::XR_ERROR_POSE_INVALID => "XR_ERROR_POSE_INVALID",
        ffi::XR_ERROR_INDEX_OUT_OF_RANGE => "XR_ERROR_INDEX_OUT_OF_RANGE",
        ffi::XR_ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED => "XR_ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED",
        ffi::XR_ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED => "XR_ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED",
        ffi::XR_ERROR_NAME_DUPLICATED => "XR_ERROR_NAME_DUPLICATED",
        ffi::XR_ERROR_NAME_INVALID => "XR_ERROR_NAME_INVALID",
        ffi::XR_ERROR_ACTIONSET_NOT_ATTACHED => "XR_ERROR_ACTIONSET_NOT_ATTACHED",
        ffi::XR_ERROR_ACTIONSETS_ALREADY_ATTACHED => "XR_ERROR_ACTIONSETS_ALREADY_ATTACHED",
        ffi::XR_ERROR_LOCALIZED_NAME_DUPLICATED => "XR_ERROR_LOCALIZED_NAME_DUPLICATED",
        ffi::XR_ERROR_LOCALIZED_NAME_INVALID => "XR_ERROR_LOCALIZED_NAME_INVALID",
        ffi::XR_ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING => "XR_ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING",
        ffi::XR_ERROR_RUNTIME_UNAVAILABLE => "XR_ERROR_RUNTIME_UNAVAILABLE",
        ffi::XR_ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR => "XR_ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR",
        ffi::XR_ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR => "XR_ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR",
        ffi::XR_ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT => "XR_ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT",
        ffi::XR_ERROR_SECONDARY_VIEW_CONFIGURATION_TYPE_NOT_ENABLED_MSFT => "XR_ERROR_SECONDARY_VIEW_CONFIGURATION_TYPE_NOT_ENABLED_MSFT",
        ffi::XR_ERROR_CONTROLLER_MODEL_KEY_INVALID_MSFT => "XR_ERROR_CONTROLLER_MODEL_KEY_INVALID_MSFT",
        ffi::XR_ERROR_REPROJECTION_MODE_UNSUPPORTED_MSFT => "XR_ERROR_REPROJECTION_MODE_UNSUPPORTED_MSFT",
        ffi::XR_ERROR_COMPUTE_NEW_SCENE_NOT_COMPLETED_MSFT => "XR_ERROR_COMPUTE_NEW_SCENE_NOT_COMPLETED_MSFT",
        ffi::XR_ERROR_SCENE_COMPONENT_ID_INVALID_MSFT => "XR_ERROR_SCENE_COMPONENT_ID_INVALID_MSFT",
        ffi::XR_ERROR_SCENE_COMPONENT_TYPE_MISMATCH_MSFT => "XR_ERROR_SCENE_COMPONENT_TYPE_MISMATCH_MSFT",
        ffi::XR_ERROR_SCENE_MESH_BUFFER_ID_INVALID_MSFT => "XR_ERROR_SCENE_MESH_BUFFER_ID_INVALID_MSFT",
        ffi::XR_ERROR_SCENE_COMPUTE_FEATURE_INCOMPATIBLE_MSFT => "XR_ERROR_SCENE_COMPUTE_FEATURE_INCOMPATIBLE_MSFT",
        ffi::XR_ERROR_SCENE_COMPUTE_CONSISTENCY_MISMATCH_MSFT => "XR_ERROR_SCENE_COMPUTE_CONSISTENCY_MISMATCH_MSFT",
        ffi::XR_ERROR_DISPLAY_REFRESH_RATE_UNSUPPORTED_FB => "XR_ERROR_DISPLAY_REFRESH_RATE_UNSUPPORTED_FB",
        ffi::XR_ERROR_COLOR_SPACE_UNSUPPORTED_FB => "XR_ERROR_COLOR_SPACE_UNSUPPORTED_FB",
        ffi::XR_ERROR_UNEXPECTED_STATE_PASSTHROUGH_FB => "XR_ERROR_UNEXPECTED_STATE_PASSTHROUGH_FB",
        ffi::XR_ERROR_FEATURE_ALREADY_CREATED_PASSTHROUGH_FB => "XR_ERROR_FEATURE_ALREADY_CREATED_PASSTHROUGH_FB",
        ffi::XR_ERROR_FEATURE_REQUIRED_PASSTHROUGH_FB => "XR_ERROR_FEATURE_REQUIRED_PASSTHROUGH_FB",
        ffi::XR_ERROR_NOT_PERMITTED_PASSTHROUGH_FB => "XR_ERROR_NOT_PERMITTED_PASSTHROUGH_FB",
        ffi::XR_ERROR_INSUFFICIENT_RESOURCES_PASSTHROUGH_FB => "XR_ERROR_INSUFFICIENT_RESOURCES_PASSTHROUGH_FB",
        ffi::XR_ERROR_UNKNOWN_PASSTHROUGH_FB => "XR_ERROR_UNKNOWN_PASSTHROUGH_FB",
        ffi::XR_ERROR_MARKER_NOT_TRACKED_VARJO => "XR_ERROR_MARKER_NOT_TRACKED_VARJO",
        ffi::XR_ERROR_MARKER_ID_INVALID_VARJO => "XR_ERROR_MARKER_ID_INVALID_VARJO",
        ffi::XR_ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT => "XR_ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT",
        ffi::XR_ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT => "XR_ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT",
        _ => "(unknown)",
    }
}

pub(crate) fn xr_load<F>(instance: Option<ffi::XrInstance>,name: &str) -> F {
    let mut xr_function = MaybeUninit::<Option<F>>::uninit();
    if unsafe { ffi::xrGetInstanceProcAddr(instance.unwrap_or(null_mut() as ffi::XrInstance),CString::new(name).unwrap().as_c_str().as_ptr(),xr_function.as_mut_ptr() as *mut ffi::PFN_xrVoidFunction) } != ffi::XR_SUCCESS {
        panic!("can't find {}",name);
    }
    if let Some(xr_function) = unsafe { xr_function.assume_init() } {
        return xr_function;
    }
    panic!("can't find {}",name);
}

fn build_fixed_length_cstring<const N: usize>(value: &str) -> [c_char; N] {
    let mut result: [c_char; N] = [0; N];
    let mut i = 0usize;
    value.chars().for_each(|c| {
        let utf8 = c as u32;
        result[i] = if utf8 < 256 { c as c_char } else { b'#' as c_char };
        i += 1;
    });
    result
}

fn main() {
    println!("creating instance");
    let xrCreateInstance: unsafe extern "C" fn(createInfo: *const ffi::XrInstanceCreateInfo,instance: *mut ffi::XrInstance,) -> ffi::XrResult = xr_load(None,"xrCreateInstance");
    let xr_extensions = [
        b"XR_EXT_debug_utils\0" as *const u8 as *const c_char,
        b"XR_KHR_vulkan_enable\0" as *const u8 as *const c_char,
        b"XR_KHR_vulkan_enable2\0" as *const u8 as *const c_char,
        b"XR_FB_display_refresh_rate\0" as *const u8 as *const c_char,
    ];
    let info = ffi::XrInstanceCreateInfo {
        type_: ffi::XR_TYPE_INSTANCE_CREATE_INFO,
        next: null_mut(),
        createFlags: 0,
        applicationInfo: ffi::XrApplicationInfo {
            applicationName: build_fixed_length_cstring::<128>("openxr_actions_test"),
            applicationVersion: 1u32 << 10,
            engineName: build_fixed_length_cstring::<128>("openxr_actions_test"),
            engineVersion: 1u32 << 10,
            apiVersion: 0x0001000000000000,
        },
        enabledApiLayerCount: 0,
        enabledApiLayerNames: null_mut(),
        enabledExtensionCount: xr_extensions.len() as u32,
        enabledExtensionNames: &xr_extensions as *const *const c_char,
    };
    let mut xr_instance = MaybeUninit::<ffi::XrInstance>::uninit();
    match unsafe { (xrCreateInstance)(&info,xr_instance.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to create XrInstance ({})",xr_code_to_string(code)),
    }
    let xr_instance = unsafe { xr_instance.assume_init() };

    println!("loading symbols");
    //let xrGetInstanceProperties: unsafe extern "C" fn(instance: ffi::XrInstance,instanceProperties: *mut ffi::XrInstanceProperties) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetInstanceProperties");
    let xrGetSystem: unsafe extern "C" fn(instance: ffi::XrInstance,getInfo: *const ffi::XrSystemGetInfo,systemId: *mut ffi::XrSystemId) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetSystem");
    //let xrGetSystemProperties: unsafe extern "C" fn(instance: ffi::XrInstance,systemId: ffi::XrSystemId,properties: *mut ffi::XrSystemProperties) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetSystemProperties");
    let xrGetVulkanGraphicsRequirementsKHR: unsafe extern "C" fn(instance: ffi::XrInstance,systemId: ffi::XrSystemId,graphicsRequirements: *mut ffi::XrGraphicsRequirementsVulkanKHR) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetVulkanGraphicsRequirementsKHR");
    let xrGetVulkanInstanceExtensionsKHR: unsafe extern "C" fn(instance: ffi::XrInstance,systemId: ffi::XrSystemId,bufferCapacityInput: u32,bufferCountOutput: *mut u32,buffer: *mut c_char) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetVulkanInstanceExtensionsKHR");
    let xrCreateVulkanInstanceKHR: unsafe extern "C" fn(instance: ffi::XrInstance,createInfo: *const ffi::XrVulkanInstanceCreateInfoKHR,vulkanInstance: *mut ffi::VkInstance,vulkanResult: *mut ffi::VkResult) -> ffi::XrResult = xr_load(Some(xr_instance),"xrCreateVulkanInstanceKHR");
    let xrGetVulkanGraphicsDeviceKHR: unsafe extern "C" fn(instance: ffi::XrInstance,systemId: ffi::XrSystemId,vkInstance: ffi::VkInstance,vkPhysicalDevice: *mut ffi::VkPhysicalDevice) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetVulkanGraphicsDeviceKHR");
    let xrGetVulkanDeviceExtensionsKHR: unsafe extern "C" fn(instance: ffi::XrInstance,systemId: ffi::XrSystemId,bufferCapacityInput: u32,bufferCountOutput: *mut u32,buffer: *mut c_char) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetVulkanDeviceExtensionsKHR");
    let xrCreateVulkanDeviceKHR: unsafe extern "C" fn(instance: ffi::XrInstance,createInfo: *const ffi::XrVulkanDeviceCreateInfoKHR,vulkanDevice: *mut ffi::VkDevice,vulkanResult: *mut ffi::VkResult) -> ffi::XrResult = xr_load(Some(xr_instance),"xrCreateVulkanDeviceKHR");
    let xrCreateSession: unsafe extern "C" fn(instance: ffi::XrInstance,createInfo: *const ffi::XrSessionCreateInfo,session: *mut ffi::XrSession) -> ffi::XrResult = xr_load(Some(xr_instance),"xrCreateSession");
    //let xrEnumerateViewConfigurations: unsafe extern "C" fn(instance: ffi::XrInstance,systemId: ffi::XrSystemId,viewConfigurationTypeCapacityInput: u32,viewConfigurationTypeCountOutput: *mut u32,viewConfigurationTypes: *mut ffi::XrViewConfigurationType) -> ffi::XrResult = xr_load(Some(xr_instance),"xrEnumerateViewConfigurations");
    //let xrGetViewConfigurationProperties: unsafe extern "C" fn(instance: ffi::XrInstance,systemId: ffi::XrSystemId,viewConfigurationType: ffi::XrViewConfigurationType,configurationProperties: *mut ffi::XrViewConfigurationProperties) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetViewConfigurationProperties");
    //let xrEnumerateViewConfigurationViews: unsafe extern "C" fn(instance: ffi::XrInstance,systemId: ffi::XrSystemId,viewConfigurationType: ffi::XrViewConfigurationType,viewCapacityInput: u32,viewCountOutput: *mut u32,views: *mut ffi::XrViewConfigurationView) -> ffi::XrResult = xr_load(Some(xr_instance),"xrEnumerateViewConfigurationViews");
    //let xrEnumerateSwapchainFormats: unsafe extern "C" fn(session: ffi::XrSession,formatCapacityInput: u32,formatCountOutput: *mut u32,formats: *mut i64) -> ffi::XrResult = xr_load(Some(xr_instance),"xrEnumerateSwapchainFormats");
    let xrPollEvent: unsafe extern "C" fn(instance: ffi::XrInstance,eventData: *mut ffi::XrEventDataBuffer) -> ffi::XrResult = xr_load(Some(xr_instance),"xrPollEvent");
    let xrBeginSession: unsafe extern "C" fn(session: ffi::XrSession, beginInfo: *const ffi::XrSessionBeginInfo) -> ffi::XrResult = xr_load(Some(xr_instance),"xrBeginSession");
    let xrEndSession: unsafe extern "C" fn(session: ffi::XrSession) -> ffi::XrResult = xr_load(Some(xr_instance),"xrEndSession");
    let xrWaitFrame: unsafe extern "C" fn(session: ffi::XrSession,frameWaitInfo: *const ffi::XrFrameWaitInfo,frameState: *mut ffi::XrFrameState) -> ffi::XrResult = xr_load(Some(xr_instance),"xrWaitFrame");
    let xrBeginFrame: unsafe extern "C" fn(session: ffi::XrSession, frameBeginInfo: *const ffi::XrFrameBeginInfo) -> ffi::XrResult = xr_load(Some(xr_instance),"xrBeginFrame");
    let xrEndFrame: unsafe extern "C" fn(session: ffi::XrSession, frameEndInfo: *const ffi::XrFrameEndInfo) -> ffi::XrResult = xr_load(Some(xr_instance),"xrEndFrame");
    //let xrLocateViews: unsafe extern "C" fn(session: ffi::XrSession,viewLocateInfo: *const ffi::XrViewLocateInfo,viewState: *mut ffi::XrViewState,viewCapacityInput: u32,viewCountOutput: *mut u32,views: *mut ffi::XrView) -> ffi::XrResult = xr_load(Some(xr_instance),"xrLocateViews");
    //let xrCreateReferenceSpace: unsafe extern "C" fn(session: ffi::XrSession,createInfo: *const ffi::XrReferenceSpaceCreateInfo,space: *mut ffi::XrSpace) -> ffi::XrResult = xr_load(Some(xr_instance),"xrCreateReferenceSpace");
    //let xrCreateActionSpace: unsafe extern "C" fn(session: ffi::XrSession,createInfo: *const ffi::XrActionSpaceCreateInfo,space: *mut ffi::XrSpace) -> ffi::XrResult = xr_load(Some(xr_instance),"xrCreateActionSpace");
    //let xrCreateSwapchain: unsafe extern "C" fn(session: ffi::XrSession,createInfo: *const ffi::XrSwapchainCreateInfo,swapchain: *mut ffi::XrSwapchain) -> ffi::XrResult = xr_load(Some(xr_instance),"xrCreateSwapchain");
    //let xrDestroySwapchain: unsafe extern "C" fn(swapchain: ffi::XrSwapchain) -> ffi::XrResult = xr_load(Some(xr_instance),"xrDestroySwapchain");
    //let xrEnumerateSwapchainImages: unsafe extern "C" fn(swapchain: ffi::XrSwapchain,imageCapacityInput: u32,imageCountOutput: *mut u32,images: *mut ffi::XrSwapchainImageBaseHeader) -> ffi::XrResult = xr_load(Some(xr_instance),"xrEnumerateSwapchainImages");
    //let xrAcquireSwapchainImage: unsafe extern "C" fn(swapchain: ffi::XrSwapchain,acquireInfo: *const ffi::XrSwapchainImageAcquireInfo,index: *mut u32) -> ffi::XrResult = xr_load(Some(xr_instance),"xrAcquireSwapchainImage");
    //let xrWaitSwapchainImage: unsafe extern "C" fn(swapchain: ffi::XrSwapchain,waitInfo: *const ffi::XrSwapchainImageWaitInfo) -> ffi::XrResult = xr_load(Some(xr_instance),"xrWaitSwapchainImage");
    //let xrReleaseSwapchainImage: unsafe extern "C" fn(swapchain: ffi::XrSwapchain,releaseInfo: *const ffi::XrSwapchainImageReleaseInfo) -> ffi::XrResult = xr_load(Some(xr_instance),"xrReleaseSwapchainImage");
    //let xrDestroyInstance: unsafe extern "C" fn(instance: ffi::XrInstance) -> ffi::XrResult = xr_load(Some(xr_instance),"xrDestroyInstance");
    //let xrLocateSpace: unsafe extern "C" fn(space: ffi::XrSpace,baseSpace: ffi::XrSpace,time: ffi::XrTime,location: *mut ffi::XrSpaceLocation) -> ffi::XrResult = xr_load(Some(xr_instance),"xrLocateSpace");
    let xrCreateActionSet: unsafe extern "C" fn(instance: ffi::XrInstance,createInfo: *const ffi::XrActionSetCreateInfo,actionSet: *mut ffi::XrActionSet) -> ffi::XrResult = xr_load(Some(xr_instance),"xrCreateActionSet");
    //let xrDestroyActionSet: unsafe extern "C" fn(actionSet: ffi::XrActionSet) -> ffi::XrResult = xr_load(Some(xr_instance),"xrDestroyActionSet");
    let xrStringToPath: unsafe extern "C" fn(instance: ffi::XrInstance,pathString: *const c_char,path: *mut ffi::XrPath) -> ffi::XrResult = xr_load(Some(xr_instance),"xrStringToPath");
    //let xrPathToString: unsafe extern "C" fn(instance: ffi::XrInstance,path: ffi::XrPath,bufferCapacityInput: u32,bufferCountOutput: *mut u32,buffer: *mut c_char) -> ffi::XrResult = xr_load(Some(xr_instance),"xrPathToString");
    let xrCreateAction: unsafe extern "C" fn(actionSet: ffi::XrActionSet,createInfo: *const ffi::XrActionCreateInfo,action: *mut ffi::XrAction) -> ffi::XrResult = xr_load(Some(xr_instance),"xrCreateAction");
    //let xrDestroyAction: unsafe extern "C" fn(action: ffi::XrAction) -> ffi::XrResult = xr_load(Some(xr_instance),"xrDestroyAction");
    let xrSuggestInteractionProfileBindings: unsafe extern "C" fn(instance: ffi::XrInstance,suggestedBindings: *const ffi::XrInteractionProfileSuggestedBinding) -> ffi::XrResult = xr_load(Some(xr_instance),"xrSuggestInteractionProfileBindings");
    let xrAttachSessionActionSets: unsafe extern "C" fn(session: ffi::XrSession,attachInfo: *const ffi::XrSessionActionSetsAttachInfo) -> ffi::XrResult = xr_load(Some(xr_instance),"xrAttachSessionActionSets");
    let xrSyncActions: unsafe extern "C" fn(session: ffi::XrSession, syncInfo: *const ffi::XrActionsSyncInfo) -> ffi::XrResult = xr_load(Some(xr_instance),"xrSyncActions");
    //let xrGetActionStatePose: unsafe extern "C" fn(session: ffi::XrSession,getInfo: *const ffi::XrActionStateGetInfo,state: *mut ffi::XrActionStatePose) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetActionStatePose");
    let xrGetActionStateBoolean: unsafe extern "C" fn(session: ffi::XrSession,getInfo: *const ffi::XrActionStateGetInfo,state: *mut ffi::XrActionStateBoolean) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetActionStateBoolean");
    //let xrGetActionStateFloat: unsafe extern "C" fn(session: ffi::XrSession,getInfo: *const ffi::XrActionStateGetInfo,state: *mut ffi::XrActionStateFloat) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetActionStateFloat");
    //let xrGetActionStateVector2f: unsafe extern "C" fn(session: ffi::XrSession,getInfo: *const ffi::XrActionStateGetInfo,state: *mut ffi::XrActionStateVector2f) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetActionStateVector2f");
    //let xrGetCurrentInteractionProfile: unsafe extern "C" fn(session: ffi::XrSession,topLevelUserPath: ffi::XrPath,interactionProfile: *mut ffi::XrInteractionProfileState) -> ffi::XrResult = xr_load(Some(xr_instance),"xrGetCurrentInteractionProfile");
    //let xrEnumerateBoundSourcesForAction: unsafe extern "C" fn(session: ffi::XrSession,enumerateInfo: *const ffi::XrBoundSourcesForActionEnumerateInfo,sourceCapacityInput: u32,sourceCountOutput: *mut u32,sources: *mut ffi::XrPath) -> ffi::XrResult = xr_load(Some(xr_instance),"xrEnumerateBoundSourcesForAction");

    println!("getting system ID");
    let info = ffi::XrSystemGetInfo {
        type_: ffi::XR_TYPE_SYSTEM_GET_INFO,
        next: null_mut(),
        formFactor: ffi::XR_FORM_FACTOR_HEAD_MOUNTED_DISPLAY,
    };
    let mut xr_system_id = MaybeUninit::<ffi::XrSystemId>::uninit();
    match unsafe { (xrGetSystem)(xr_instance,&info,xr_system_id.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to get system ID ({})",xr_code_to_string(code)),
    }
    let xr_system_id = unsafe { xr_system_id.assume_init() };

    println!("enumerating required Vulkan instance extensions:");
    let mut count = MaybeUninit::<u32>::uninit();
    match unsafe { (xrGetVulkanInstanceExtensionsKHR)(xr_instance,xr_system_id,0,count.as_mut_ptr(),null_mut()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to get required Vulkan instance extensions ({})",xr_code_to_string(code)),
    }
    let mut count = unsafe { count.assume_init() };
    let mut buffer: Vec<c_char> = vec![0; count as usize];
    match unsafe { (xrGetVulkanInstanceExtensionsKHR)(xr_instance,xr_system_id,count,&mut count,buffer.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to get required Vulkan instance extensions ({})",xr_code_to_string(code)),
    }
    let mut extensions = Vec::<String>::new();
    let mut extension = String::new();
    for c in buffer {
        match c {
            0 => {
                extensions.push(extension);
                break;
            },
            32 => {
                extensions.push(extension);
                extension = String::new();
            },
            c => {
                extension.push(char::from_u32(c as u32).unwrap());
            },
        }
    }
    extensions.iter().for_each(|extension| {
        println!("    {}",extension);
    });

    println!("creating Vulkan instance");
    let mut vk_extensions = Vec::<CString>::new();
    extensions.iter().for_each(|extension| {
        vk_extensions.push(CString::new(extension.as_str()).unwrap());
    });
    let mut vk_extension_ptrs = Vec::<*const c_char>::new();
    for vk_extension in vk_extensions.iter() {
        vk_extension_ptrs.push(vk_extension.as_ptr());
    }
    let vk_info = ffi::VkInstanceCreateInfo {
        sType: ffi::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: null_mut(),
        flags: 0,
        pApplicationInfo: &ffi::VkApplicationInfo {
            sType: ffi::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null_mut(),
            pApplicationName: b"openxr_actions_test\0" as *const u8 as *const c_char,
            applicationVersion: 1u32 << 11,
            pEngineName: b"openxr_actions_test\0" as *const u8 as *const c_char,
            engineVersion: 1u32 << 11,
            apiVersion: ((1 << 22) | (2 << 12)) as u32,
        },
        enabledExtensionCount: vk_extension_ptrs.len() as u32,
        ppEnabledExtensionNames: vk_extension_ptrs.as_ptr(),
        enabledLayerCount: 0,
        ppEnabledLayerNames: null_mut(),
    };
    let info = ffi::XrVulkanInstanceCreateInfoKHR {
        type_: ffi::XR_TYPE_VULKAN_INSTANCE_CREATE_INFO_KHR,
        next: null_mut(),
        systemId: xr_system_id,
        createFlags: 0,
        pfnGetInstanceProcAddr: Some(ffi::vkGetInstanceProcAddr),
        vulkanCreateInfo: &vk_info,
        vulkanAllocator: null_mut(),
    };
    let mut vk_instance = MaybeUninit::<ffi::VkInstance>::uninit();
    let mut vk_result = MaybeUninit::<ffi::VkResult>::uninit();
    match unsafe { (xrCreateVulkanInstanceKHR)(xr_instance,&info,vk_instance.as_mut_ptr(),vk_result.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to create Vulkan instance ({})",xr_code_to_string(code)),
    }
    match unsafe { vk_result.assume_init() } {
        ffi::VK_SUCCESS => { },
        code => panic!("unable to create Vulkan instance ({})",vk_code_to_string(code)),
    }
    let vk_instance = unsafe { vk_instance.assume_init() };

    println!("getting physical device");
    let mut vk_physical_device = MaybeUninit::<ffi::VkPhysicalDevice>::uninit();
    match unsafe { (xrGetVulkanGraphicsDeviceKHR)(xr_instance,xr_system_id,vk_instance,vk_physical_device.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to get Vulkan graphics device ({})",xr_code_to_string(code)),
    }    
    let vk_physical_device = unsafe { vk_physical_device.assume_init() };

    println!("obtaining physical device features");
    let mut vk_features = MaybeUninit::<ffi::VkPhysicalDeviceFeatures>::uninit();
    unsafe { ffi::vkGetPhysicalDeviceFeatures(vk_physical_device,vk_features.as_mut_ptr()) };
    let vk_features = unsafe { vk_features.assume_init() };

    println!("enumerating queue families");
    let mut count = MaybeUninit::<u32>::uninit();
    unsafe { ffi::vkGetPhysicalDeviceQueueFamilyProperties(vk_physical_device,count.as_mut_ptr(),null_mut()) };
    let mut count = unsafe { count.assume_init() };
    if count == 0 {
        panic!("no queue families found");
    }
    let mut vk_queue_families = vec![ffi::VkQueueFamilyProperties {
        queueFlags: 0,
        queueCount: 0,
        timestampValidBits: 0,
        minImageTransferGranularity: ffi::VkExtent3D {
            width: 0,
            height: 0,
            depth: 0,
        },
    }; count as usize];
    unsafe { ffi::vkGetPhysicalDeviceQueueFamilyProperties(vk_physical_device,&mut count,vk_queue_families.as_mut_ptr()) };

    println!("enumerating required Vulkan device extensions:");
    let mut count = MaybeUninit::<u32>::uninit();
    match unsafe { (xrGetVulkanDeviceExtensionsKHR)(xr_instance,xr_system_id,0,count.as_mut_ptr(),null_mut()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to get required Vulkan device extensions ({})",xr_code_to_string(code)),
    }
    let mut count = unsafe { count.assume_init() };
    let mut buffer: Vec<c_char> = vec![0; count as usize];
    match unsafe { (xrGetVulkanDeviceExtensionsKHR)(xr_instance,xr_system_id,count,&mut count as *mut u32,buffer.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to get required Vulkan device extensions ({})",xr_code_to_string(code)),
    }
    let mut extensions = Vec::<String>::new();
    let mut extension = String::new();
    for c in buffer {
        match c {
            0 => {
                extensions.push(extension);
                break;
            },
            32 => {
                extensions.push(extension);
                extension = String::new();
            },
            c => {
                extension.push(char::from_u32(c as u32).unwrap());
            },
        }
    }
    extensions.push("VK_KHR_swapchain".to_string());
    extensions.iter().for_each(|extension| {
        println!("    {}",extension);
    });

    println!("creating Vulkan device");
    let mut queue_create_infos = Vec::<ffi::VkDeviceQueueCreateInfo>::new();
    let priority = 1f32;
    queue_create_infos.push(ffi::VkDeviceQueueCreateInfo {
        sType: ffi::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
        pNext: null_mut(),
        flags: 0,
        queueFamilyIndex: 0,
        queueCount: 1,
        pQueuePriorities: &priority as *const f32,
    });
    let mut vk_extensions = Vec::<CString>::new();
    extensions.iter().for_each(|extension| {
        vk_extensions.push(CString::new(extension.as_str()).unwrap());
    });
    let mut vk_extension_ptrs = Vec::<*const c_char>::new();
    for vk_extension in vk_extensions.iter() {
        vk_extension_ptrs.push(vk_extension.as_ptr())
    }
    let vk_info = ffi::VkDeviceCreateInfo {
        sType: ffi::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        pNext: null_mut(),
        flags: 0,
        queueCreateInfoCount: queue_create_infos.len() as u32,
        pQueueCreateInfos: queue_create_infos.as_ptr(),
        enabledLayerCount: 0,
        ppEnabledLayerNames: null_mut(),
        enabledExtensionCount: vk_extension_ptrs.len() as u32,
        ppEnabledExtensionNames: vk_extension_ptrs.as_ptr(),
        pEnabledFeatures: &vk_features,
    };
    let info = ffi::XrVulkanDeviceCreateInfoKHR {
        type_: ffi::XR_TYPE_VULKAN_DEVICE_CREATE_INFO_KHR,
        next: null_mut(),
        systemId: xr_system_id,
        createFlags: 0,
        pfnGetInstanceProcAddr: Some(ffi::vkGetInstanceProcAddr),
        vulkanPhysicalDevice: vk_physical_device,
        vulkanCreateInfo: &vk_info,
        vulkanAllocator: null_mut(),
    };
    let mut vk_device = MaybeUninit::<ffi::VkDevice>::uninit();
    let mut vk_result = MaybeUninit::<ffi::VkResult>::uninit();
    match unsafe { (xrCreateVulkanDeviceKHR)(xr_instance,&info,vk_device.as_mut_ptr(),vk_result.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to create device ({})",xr_code_to_string(code)),
    }
    match unsafe { vk_result.assume_init() } {
        ffi::VK_SUCCESS => { },
        code => panic!("unable to create device ({})",vk_code_to_string(code)),
    }
    let vk_device = unsafe { vk_device.assume_init() };

    println!("obtaining graphics requirements");
    let mut xr_graphics_requirements = ffi::XrGraphicsRequirementsVulkanKHR {
        type_: ffi::XR_TYPE_GRAPHICS_REQUIREMENTS_VULKAN_KHR,
        next: null_mut(),
        minApiVersionSupported: 0,
        maxApiVersionSupported: 0,
    };
    match unsafe { (xrGetVulkanGraphicsRequirementsKHR)(xr_instance,xr_system_id,&mut xr_graphics_requirements) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to get Vulkan graphics requirements ({})",xr_code_to_string(code)),
    }

    println!("creating session");
    let graphics_binding = ffi::XrGraphicsBindingVulkanKHR {
        type_: ffi::XR_TYPE_GRAPHICS_BINDING_VULKAN_KHR,
        next: null_mut(),
        instance: vk_instance,
        physicalDevice: vk_physical_device,
        device: vk_device,
        queueFamilyIndex: 0,
        queueIndex: 0,
    };
    let info = ffi::XrSessionCreateInfo {
        type_: ffi::XR_TYPE_SESSION_CREATE_INFO,
        next: &graphics_binding as *const ffi::XrGraphicsBindingVulkanKHR as *const c_void,
        createFlags: 0,
        systemId: xr_system_id,
    };
    let mut xr_session = MaybeUninit::<ffi::XrSession>::uninit();
    match unsafe { (xrCreateSession)(xr_instance,&info,xr_session.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to create session ({})",xr_code_to_string(code)),
    }
    let xr_session = unsafe { xr_session.assume_init() };

    println!("creating action set");
    let info = ffi::XrActionSetCreateInfo {
        type_: ffi::XR_TYPE_ACTION_SET_CREATE_INFO,
        next: null_mut(),
        priority: 0,
        actionSetName: build_fixed_length_cstring::<64>("action_set"),
        localizedActionSetName: build_fixed_length_cstring::<128>("action_set"),
    };
    let mut xr_action_set = MaybeUninit::<ffi::XrActionSet>::uninit();
    match unsafe { (xrCreateActionSet)(xr_instance,&info,xr_action_set.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to create action set ({})",xr_code_to_string(code)),
    }
    let xr_action_set = unsafe { xr_action_set.assume_init() };

    println!("creating button action");
    let info = ffi::XrActionCreateInfo {
        type_: ffi::XR_TYPE_ACTION_CREATE_INFO,
        next: null_mut(),
        actionType: ffi::XR_ACTION_TYPE_BOOLEAN_INPUT,
        countSubactionPaths: 0,
        subactionPaths: null_mut(),
        actionName: build_fixed_length_cstring::<64>("button"),
        localizedActionName: build_fixed_length_cstring::<128>("button"),
    };
    let mut xr_action = MaybeUninit::<ffi::XrAction>::uninit();
    match unsafe { (xrCreateAction)(xr_action_set,&info,xr_action.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to create bool action ({})",xr_code_to_string(code)),
    }
    let xr_action = unsafe { xr_action.assume_init() };

    println!("creating paths");
    let mut xr_button_path = MaybeUninit::<ffi::XrPath>::uninit();
    match unsafe { (xrStringToPath)(xr_instance,b"/user/hand/right/input/a/click\0" as *const u8 as *const c_char,xr_button_path.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to convert string to path ({})",xr_code_to_string(code)),
    }
    let xr_button_path = unsafe { xr_button_path.assume_init() };
    let mut xr_profile_path = MaybeUninit::<ffi::XrPath>::uninit();
    match unsafe { (xrStringToPath)(xr_instance,b"/interaction_profiles/oculus/touch_controller\0" as *const u8 as *const c_char,xr_profile_path.as_mut_ptr()) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to convert string to path ({})",xr_code_to_string(code)),
    }
    let xr_profile_path = unsafe { xr_profile_path.assume_init() };

    println!("suggesting profile bindings");
    let bindings = [
        ffi::XrActionSuggestedBinding {
            action: xr_action,
            binding: xr_button_path,
        },
    ];
    let info = ffi::XrInteractionProfileSuggestedBinding {
        type_: ffi::XR_TYPE_INTERACTION_PROFILE_SUGGESTED_BINDING,
        next: null_mut(),
        interactionProfile: xr_profile_path,
        countSuggestedBindings: bindings.len() as u32,
        suggestedBindings: bindings.as_ptr(),
    };
    match unsafe { (xrSuggestInteractionProfileBindings)(xr_instance,&info) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to suggest bindings ({})",xr_code_to_string(code)),
    }

    println!("attaching action set");
    let info = ffi::XrSessionActionSetsAttachInfo {
        type_: ffi::XR_TYPE_SESSION_ACTION_SETS_ATTACH_INFO,
        next: null_mut(),
        countActionSets: 1,
        actionSets: &xr_action_set,
    };
    match unsafe { (xrAttachSessionActionSets)(xr_session,&info) } {
        ffi::XR_SUCCESS => { },
        code => panic!("unable to attach action set ({})",xr_code_to_string(code)),
    }

    println!("main loop");
    let mut running = false;
    let mut focused = false;
    loop {
        let mut xr_event = ffi::XrEventDataBuffer {
            type_: ffi::XR_TYPE_EVENT_DATA_BUFFER,
            next: null_mut(),
            varying: [0; 4000],
        };
        if unsafe { (xrPollEvent)(xr_instance,&mut xr_event) } == ffi::XR_SUCCESS {
            match xr_event.type_ {
                ffi::XR_TYPE_EVENT_DATA_EVENTS_LOST => {
                    let xr_event = unsafe { &*(&xr_event as *const ffi::XrEventDataBuffer as *const ffi::XrEventDataInstanceLossPending) };
                    println!("events lost at time {}",xr_event.lossTime);
                },
                ffi::XR_TYPE_EVENT_DATA_INSTANCE_LOSS_PENDING => {
                    println!("instance loss pending");
                },
                ffi::XR_TYPE_EVENT_DATA_INTERACTION_PROFILE_CHANGED => {
                    println!("interaction profile changed");
                },
                ffi::XR_TYPE_EVENT_DATA_PERF_SETTINGS_EXT => {
                    let xr_event = unsafe { &*(&xr_event as *const ffi::XrEventDataBuffer as *const ffi::XrEventDataPerfSettingsEXT) };
                    println!("changed for subdomain {}: level {} -> level {}",xr_event.subDomain,xr_event.fromLevel,xr_event.toLevel);
                },
                ffi::XR_TYPE_EVENT_DATA_DISPLAY_REFRESH_RATE_CHANGED_FB => {
                    let xr_event = unsafe { &*(&xr_event as *const ffi::XrEventDataBuffer as *const ffi::XrEventDataDisplayRefreshRateChangedFB) };
                    println!("changed from {} Hz to {} Hz",xr_event.fromDisplayRefreshRate,xr_event.toDisplayRefreshRate);
                },
                ffi::XR_TYPE_EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING => {
                    println!("data reference space change pending");
                },
                ffi::XR_TYPE_EVENT_DATA_SESSION_STATE_CHANGED => {
                    let xr_event = unsafe { &*(&xr_event as *const ffi::XrEventDataBuffer as *const ffi::XrEventDataSessionStateChanged) };
                    match xr_event.state {
                        ffi::XR_SESSION_STATE_IDLE => {
                            // here all resources are ready for use, but the app is not running the frame loop because it is paused
                            println!("state: IDLE");
                        },
                        ffi::XR_SESSION_STATE_READY => {
                            // received when the app is ready to go, call xrBeginSession to start the frame loop
                            println!("state: READY, beginning session");
                            let info = ffi::XrSessionBeginInfo {
                                type_: ffi::XR_TYPE_SESSION_BEGIN_INFO,
                                next: null_mut(),
                                primaryViewConfigurationType: ffi::XR_VIEW_CONFIGURATION_TYPE_PRIMARY_STEREO,
                            };
                            match unsafe { (xrBeginSession)(xr_session,&info) } {
                                ffi::XR_SUCCESS => { },
                                code => panic!("unable to begin session ({})",xr_code_to_string(code)),
                            }
                            println!("session started");
                            running = true;
                        },
                        ffi::XR_SESSION_STATE_SYNCHRONIZED => {
                            // here the frame loop is running, but nothing is rendered and no inputs are received
                            println!("state: SYNCHRONIZED");
                        },
                        ffi::XR_SESSION_STATE_VISIBLE => {
                            // here the app is rendering something to the eye buffers, but the system might be doing something else
                            println!("state: VISIBLE");
                        },
                        ffi::XR_SESSION_STATE_FOCUSED => {
                            // here the app is fully active, rendering and handling inputs
                            println!("state: FOCUSED");
                            focused = true;
                        },
                        ffi::XR_SESSION_STATE_STOPPING => {
                            // here the app should stop and return back to idle, call xrEndSession
                            println!("state: STOPPING");
                            match unsafe { (xrEndSession)(xr_session) } {
                                ffi::XR_SUCCESS => { },
                                code => panic!("unable to end session ({})",xr_code_to_string(code)),
                            }
                        },
                        ffi::XR_SESSION_STATE_LOSS_PENDING => {
                            // this is an exception, after this only call xrDestroySession and xrDestroyInstance, or attempt to resurrect by xrGetSystem and xrCreateSession
                            println!("state: LOSS_PENDING");
                        },
                        ffi::XR_SESSION_STATE_EXITING => {
                            // here the user ends the app, call xrDestroySession and xrDestroyInstance
                            println!("state: EXITING");
                            break;
                        }
                        _ => {
                            println!("state: unknown");
                        }
                    }
                }
                _ => {
                    println!("unknown event");
                } 
            }
        }

        if running {
            println!("waiting for frame");
            let info = ffi::XrFrameWaitInfo {
                type_: ffi::XR_TYPE_FRAME_WAIT_INFO,
                next: null_mut(),
            };
            let mut state = ffi::XrFrameState {
                type_: ffi::XR_TYPE_FRAME_STATE,
                next: null_mut(),
                predictedDisplayTime: 0,
                predictedDisplayPeriod: 0,
                shouldRender: ffi::XR_FALSE,
            };
            match unsafe { (xrWaitFrame)(xr_session,&info,&mut state) } {
                ffi::XR_SUCCESS => { },
                code => panic!("unable to wait for frame ({})",xr_code_to_string(code)),
            }

            println!("beginning frame");
            let info = ffi::XrFrameBeginInfo {
                type_: ffi::XR_TYPE_FRAME_BEGIN_INFO,
                next: null_mut(),
            };
            match unsafe { (xrBeginFrame)(xr_session,&info) } {
                ffi::XR_SUCCESS => { },
                code => panic!("unable to begin frame ({})",xr_code_to_string(code)),
            }

            println!("ending frame");
            let info = ffi::XrFrameEndInfo {
                type_: ffi::XR_TYPE_FRAME_END_INFO,
                next: null_mut(),
                displayTime: state.predictedDisplayTime as i64,
                environmentBlendMode: ffi::XR_ENVIRONMENT_BLEND_MODE_OPAQUE,
                layers: null_mut(),
                layerCount: 0,
            };
            match unsafe { (xrEndFrame)(xr_session,&info) } {
                ffi::XR_SUCCESS => { },
                code => panic!("unable to end frame ({})",xr_code_to_string(code)),
            }
        }

        if focused {
            let active_action_set = ffi::XrActiveActionSet {
                actionSet: xr_action_set,
                subactionPath: ffi::XR_NULL_PATH as u64,
            };
            let info = ffi::XrActionsSyncInfo {
                type_: ffi::XR_TYPE_ACTIONS_SYNC_INFO,
                next: null_mut(),
                countActiveActionSets: 1,
                activeActionSets: &active_action_set,
            };
            match unsafe { (xrSyncActions)(xr_session,&info) } {
                ffi::XR_SUCCESS => { },
                code => panic!("unable to sync actions ({})",xr_code_to_string(code)),
            }

            println!("button state:");
            let info = ffi::XrActionStateGetInfo {
                type_: ffi::XR_TYPE_ACTION_STATE_GET_INFO,
                next: null_mut(),
                action: xr_action,
                subactionPath: ffi::XR_NULL_PATH as u64,
            };
            let mut state = ffi::XrActionStateBoolean {
                type_: ffi::XR_TYPE_ACTION_STATE_BOOLEAN,
                next: null_mut(),
                currentState: ffi::XR_FALSE,
                changedSinceLastSync: ffi::XR_FALSE,
                lastChangeTime: 0,
                isActive: ffi::XR_FALSE,
            };
            match unsafe { (xrGetActionStateBoolean)(xr_session,&info,&mut state) } {
                ffi::XR_SUCCESS => { },
                code => panic!("unable to get boolean action state ({})",xr_code_to_string(code)),
            }
            println!("    currentState = {}",state.currentState);
            println!("    changedSinceLastSync = {}",state.changedSinceLastSync);
            println!("    lastChangeTime = {}",state.lastChangeTime);
            println!("    isActive = {}",state.isActive);
        }
    }
}