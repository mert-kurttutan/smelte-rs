use crate::TensorError;
use cudarc::driver::{CudaDevice, CudaSlice, DriverError};
use std::sync::Arc;

/// Tensor, can own, or borrow the underlying tensor
#[derive(Clone)]
pub struct Tensor {
    shape: Vec<usize>,
    device: Arc<CudaDevice>,
    data: CudaSlice<f32>,
}

impl Tensor {
    /// The shape of the tensor
    /// ```
    /// use smelte_rs::gpu::f32::Tensor;
    ///
    /// let tensor = Tensor::zeros(vec![2, 2], 0);
    /// assert_eq!(tensor.shape(), vec![2, 2]);
    /// ```
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    /// Creates a new nulled tensor with given shape
    /// ```
    /// use smelte_rs::gpu::f32::Tensor;
    ///
    /// let tensor = Tensor::zeros(vec![2, 2], 0).unwrap();
    /// ```
    pub fn zeros(shape: Vec<usize>, device_id: usize) -> Result<Self, DriverError> {
        let nelement: usize = shape.iter().product();
        let device = CudaDevice::new(device_id)?;
        let data: CudaSlice<f32> = device.alloc_zeros(nelement)?;
        Ok(Self {
            shape,
            data,
            device,
        })
    }

    // /// Creates a new borrowed tensor with given shape. Can fail if data doesn't match the shape
    // /// ```
    // /// use smelte-rs::cpu::f32::Tensor;
    // ///
    // /// let data = [1.0, 2.0, 3.0, 4.0];
    // /// let tensor = Tensor::borrowed(&data, vec![2, 2]).unwrap();
    // /// ```
    // pub fn borrowed(data: &'data [f32], shape: Vec<usize>) -> Result<Self, TensorError> {
    //     let cow: Cow<'data, [f32]> = data.into();
    //     Self::new(cow, shape)
    // }

    // /// Creates a new tensor with given shape. Can fail if data doesn't match the shape
    // /// ```
    // /// use smelte-rs::cpu::f32::Tensor;
    // ///
    // /// let data = vec![1.0, 2.0, 3.0, 4.0];
    // /// let tensor = Tensor::new(data, vec![2, 2]).unwrap();
    // /// ```
    // pub fn new<T>(data: T, shape: Vec<usize>) -> Result<Self, TensorError>
    // where
    //     T: Into<Cow<'data, [f32]>>,
    // {
    //     let data = data.into();
    //     if data.len() != shape.iter().product::<usize>() {
    //         return Err(TensorError::InvalidBuffer {
    //             buffer_size: data.len(),
    //             shape,
    //         });
    //     }
    //     Ok(Self { shape, data })
    // }
}
