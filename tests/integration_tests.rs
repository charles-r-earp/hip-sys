use hip_sys::hiprt::{
    hipError_t,
    hipInit
};
use hip_sys::hipblas::{
    hipblasStatus_t,
    hipblasHandle_t,
    hipblasCreate,
    hipblasDestroy
};

#[test]
fn test_hip_init() {
    let error = unsafe {
        hipInit(0)
    };
    assert_eq!(error, hipError_t::hipSuccess);
}

#[test] 
fn test_hipblas_create() {
    let mut handle: hipblasHandle_t = std::ptr::null_mut();
    let status = unsafe {
        hipblasCreate(&mut handle as *mut hipblasHandle_t)
    };
    assert_eq!(status, hipblasStatus_t::HIPBLAS_STATUS_SUCCESS);
    let status = unsafe {
        hipblasDestroy(handle)
    };
    assert_eq!(status, hipblasStatus_t::HIPBLAS_STATUS_SUCCESS);
}
