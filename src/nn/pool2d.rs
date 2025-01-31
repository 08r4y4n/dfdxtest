#[cfg(feature = "nightly")]
use crate::{
    shapes::Const,
    tensor_ops::{Pool2DKind, TryPool2D},
};

#[allow(unused)]
use super::{BuildModule, Module, NonMutableModule, ZeroSizedModule};

/// Average pool with 2d kernel that operates on images (3d) and batches of images (4d).
/// Each patch reduces to the average of the values in the patch.
///
/// Generics:
/// - `KERNEL_SIZE`: The size of the kernel applied to both width and height of the images.
/// - `STRIDE`: How far to move the kernel each step. Defaults to `1`
/// - `PADDING`: How much zero padding to add around the images. Defaults to `0`.
/// - `DILATION` How dilated the kernel should be. Defaults to `1`.
#[derive(Debug, Default, Clone)]
pub struct AvgPool2D<
    const KERNEL_SIZE: usize,
    const STRIDE: usize = 1,
    const PADDING: usize = 0,
    const DILATION: usize = 1,
>;

/// Max pool with 2d kernel that operates on images (3d) and batches of images (4d).
/// Each patch reduces to the maximum value in that patch.
///
/// Generics:
/// - `KERNEL_SIZE`: The size of the kernel applied to both width and height of the images.
/// - `STRIDE`: How far to move the kernel each step. Defaults to `1`
/// - `PADDING`: How much zero padding to add around the images. Defaults to `0`.
/// - `DILATION` How dilated the kernel should be. Defaults to `1`.
#[derive(Debug, Default, Clone)]
pub struct MaxPool2D<
    const KERNEL_SIZE: usize,
    const STRIDE: usize = 1,
    const PADDING: usize = 0,
    const DILATION: usize = 1,
>;

/// Minimum pool with 2d kernel that operates on images (3d) and batches of images (4d).
/// Each patch reduces to the minimum of the values in the patch.
///
/// Generics:
/// - `KERNEL_SIZE`: The size of the kernel applied to both width and height of the images.
/// - `STRIDE`: How far to move the kernel each step. Defaults to `1`
/// - `PADDING`: How much zero padding to add around the images. Defaults to `0`.
/// - `DILATION` How dilated the kernel should be. Defaults to `1`.
#[derive(Debug, Default, Clone)]
pub struct MinPool2D<
    const KERNEL_SIZE: usize,
    const STRIDE: usize = 1,
    const PADDING: usize = 0,
    const DILATION: usize = 1,
>;

macro_rules! impl_pools {
    ($PoolTy:tt, $Op:expr) => {
        impl<const K: usize, const S: usize, const P: usize, const L: usize> ZeroSizedModule
            for $PoolTy<K, S, P, L>
        {
        }
        impl<const K: usize, const S: usize, const P: usize, const L: usize> NonMutableModule
            for $PoolTy<K, S, P, L>
        {
        }

        #[cfg(feature = "nightly")]
        impl<
                const K: usize,
                const S: usize,
                const P: usize,
                const L: usize,
                Img: TryPool2D<Const<K>, Const<S>, Const<P>, Const<L>>,
            > Module<Img> for $PoolTy<K, S, P, L>
        {
            type Output = Img::Pooled;
            type Error = Img::Error;

            fn try_forward(&self, x: Img) -> Result<Self::Output, Self::Error> {
                x.try_pool2d($Op, Const, Const, Const, Const)
            }
        }
    };
}

impl_pools!(AvgPool2D, Pool2DKind::Avg);
impl_pools!(MaxPool2D, Pool2DKind::Max);
impl_pools!(MinPool2D, Pool2DKind::Min);

#[cfg(feature = "nightly")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{shapes::*, tensor::*, tests::*};

    #[test]
    fn test_max_forward_3d_sizes() {
        let dev: TestDevice = Default::default();
        let x: Tensor<Rank3<3, 10, 10>, TestDtype, _> = dev.zeros();
        let _: Tensor<Rank3<3, 8, 8>, _, _> = MaxPool2D::<3>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 9, 9>, _, _> = MaxPool2D::<2>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 7, 7>, _, _> = MaxPool2D::<4>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 4, 4>, _, _> = MaxPool2D::<3, 2>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 3, 3>, _, _> = MaxPool2D::<3, 3>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 10, 10>, _, _> = MaxPool2D::<3, 1, 1>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 12, 12>, _, _> = MaxPool2D::<3, 1, 2>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 6, 6>, _, _> = MaxPool2D::<3, 2, 2>::default().forward(x.clone());
    }

    #[test]
    fn test_max_forward_4d_sizes() {
        let dev: TestDevice = Default::default();
        let x: Tensor<Rank4<5, 3, 10, 10>, TestDtype, _> = dev.zeros();
        let _: Tensor<Rank4<5, 3, 7, 7>, _, _> = MaxPool2D::<4>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 8, 8>, _, _> = MaxPool2D::<3>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 9, 9>, _, _> = MaxPool2D::<2>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 4, 4>, _, _> = MaxPool2D::<3, 2>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 3, 3>, _, _> = MaxPool2D::<3, 3>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 10, 10>, _, _> =
            MaxPool2D::<3, 1, 1>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 12, 12>, _, _> =
            MaxPool2D::<3, 1, 2>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 6, 6>, _, _> = MaxPool2D::<3, 2, 2>::default().forward(x.clone());
    }

    #[test]
    fn test_max_tuple_pool_sizes() {
        type A = MaxPool2D<3>;
        type B = MaxPool2D<1, 1, 1>;
        let dev: TestDevice = Default::default();
        let x: Tensor<Rank3<1, 10, 10>, TestDtype, _> = dev.zeros();

        let _: Tensor<Rank3<1, 6, 6>, _, _> = <(A, A)>::default().forward(x.clone());
        let _: Tensor<Rank3<1, 8, 8>, _, _> = <(A, A, B)>::default().forward(x.clone());
    }

    #[test]
    fn test_min_forward_3d_sizes() {
        let dev: TestDevice = Default::default();
        let x: Tensor<Rank3<3, 10, 10>, TestDtype, _> = dev.zeros();
        let _: Tensor<Rank3<3, 8, 8>, _, _> = MinPool2D::<3>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 9, 9>, _, _> = MinPool2D::<2>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 7, 7>, _, _> = MinPool2D::<4>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 4, 4>, _, _> = MinPool2D::<3, 2>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 3, 3>, _, _> = MinPool2D::<3, 3>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 10, 10>, _, _> = MinPool2D::<3, 1, 1>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 12, 12>, _, _> = MinPool2D::<3, 1, 2>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 6, 6>, _, _> = MinPool2D::<3, 2, 2>::default().forward(x.clone());
    }

    #[test]
    fn test_min_forward_4d_sizes() {
        let dev: TestDevice = Default::default();
        let x: Tensor<Rank4<5, 3, 10, 10>, TestDtype, _> = dev.zeros();
        let _: Tensor<Rank4<5, 3, 7, 7>, _, _> = MinPool2D::<4>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 8, 8>, _, _> = MinPool2D::<3>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 9, 9>, _, _> = MinPool2D::<2>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 4, 4>, _, _> = MinPool2D::<3, 2>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 3, 3>, _, _> = MinPool2D::<3, 3>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 10, 10>, _, _> =
            MinPool2D::<3, 1, 1>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 12, 12>, _, _> =
            MinPool2D::<3, 1, 2>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 6, 6>, _, _> = MinPool2D::<3, 2, 2>::default().forward(x.clone());
    }

    #[test]
    fn test_min_tuple_pool_sizes() {
        type A = MinPool2D<3>;
        type B = MinPool2D<1, 1, 1>;
        let dev: TestDevice = Default::default();
        let x: Tensor<Rank3<1, 10, 10>, TestDtype, _> = dev.zeros();

        let _: Tensor<Rank3<1, 6, 6>, _, _> = <(A, A)>::default().forward(x.clone());
        let _: Tensor<Rank3<1, 8, 8>, _, _> = <(A, A, B)>::default().forward(x.clone());
    }

    #[test]
    fn test_avgforward_3d_sizes() {
        let dev: TestDevice = Default::default();
        let x: Tensor<Rank3<3, 10, 10>, TestDtype, _> = dev.zeros();
        let _: Tensor<Rank3<3, 8, 8>, _, _> = AvgPool2D::<3>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 9, 9>, _, _> = AvgPool2D::<2>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 7, 7>, _, _> = AvgPool2D::<4>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 4, 4>, _, _> = AvgPool2D::<3, 2>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 3, 3>, _, _> = AvgPool2D::<3, 3>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 10, 10>, _, _> = AvgPool2D::<3, 1, 1>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 12, 12>, _, _> = AvgPool2D::<3, 1, 2>::default().forward(x.clone());
        let _: Tensor<Rank3<3, 6, 6>, _, _> = AvgPool2D::<3, 2, 2>::default().forward(x.clone());
    }

    #[test]
    fn test_avgforward_4d_sizes() {
        let dev: TestDevice = Default::default();
        let x: Tensor<Rank4<5, 3, 10, 10>, TestDtype, _> = dev.zeros();
        let _: Tensor<Rank4<5, 3, 7, 7>, _, _> = AvgPool2D::<4>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 8, 8>, _, _> = AvgPool2D::<3>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 9, 9>, _, _> = AvgPool2D::<2>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 4, 4>, _, _> = AvgPool2D::<3, 2>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 3, 3>, _, _> = AvgPool2D::<3, 3>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 10, 10>, _, _> =
            AvgPool2D::<3, 1, 1>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 12, 12>, _, _> =
            AvgPool2D::<3, 1, 2>::default().forward(x.clone());
        let _: Tensor<Rank4<5, 3, 6, 6>, _, _> = AvgPool2D::<3, 2, 2>::default().forward(x.clone());
    }

    #[test]
    fn test_avgtuple_pool_sizes() {
        type A = AvgPool2D<3>;
        type B = AvgPool2D<1, 1, 1>;
        let dev: TestDevice = Default::default();
        let x: Tensor<Rank3<1, 10, 10>, TestDtype, _> = dev.zeros();

        let _: Tensor<Rank3<1, 6, 6>, _, _> = <(A, A)>::default().forward(x.clone());
        let _: Tensor<Rank3<1, 8, 8>, _, _> = <(A, A, B)>::default().forward(x.clone());
    }
}
