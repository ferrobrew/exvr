#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub [f32; 3]);
static_assertions::const_assert!(std::mem::size_of::<Vec3>() == 12);

impl From<Vec3> for glam::Vec3 {
    fn from(val: Vec3) -> Self {
        glam::Vec3::from_slice(&val.0)
    }
}

impl From<glam::Vec3> for Vec3 {
    fn from(val: glam::Vec3) -> Self {
        Vec3(val.to_array())
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec3A(pub [f32; 4]);
static_assertions::const_assert!(std::mem::size_of::<Vec3A>() == 16);

impl From<Vec3A> for glam::Vec3A {
    fn from(val: Vec3A) -> Self {
        glam::Vec3A::from_slice(&val.0)
    }
}

impl From<glam::Vec3A> for Vec3A {
    fn from(val: glam::Vec3A) -> Self {
        let [x, y, z] = val.to_array();
        Vec3A([x, y, z, 0.0])
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec4(pub [f32; 4]);
static_assertions::const_assert!(std::mem::size_of::<Vec4>() == 16);

impl From<Vec4> for glam::Vec4 {
    fn from(val: Vec4) -> Self {
        glam::Vec4::from_slice(&val.0)
    }
}

impl From<glam::Vec4> for Vec4 {
    fn from(val: glam::Vec4) -> Self {
        Vec4(val.to_array())
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat3x4(pub [f32; 12]);
static_assertions::const_assert!(std::mem::size_of::<Mat3x4>() == 48);

impl From<Mat3x4> for glam::Affine3A {
    fn from(val: Mat3x4) -> Self {
        glam::Affine3A::from_cols_array(&val.0)
    }
}

impl From<Mat4> for glam::Affine3A {
    fn from(val: Mat4) -> Self {
        glam::Affine3A::from_mat4(val.into())
    }
}

impl From<glam::Affine3A> for Mat3x4 {
    fn from(val: glam::Affine3A) -> Self {
        Mat3x4(val.to_cols_array())
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat4(pub [f32; 16]);
static_assertions::const_assert!(std::mem::size_of::<Mat4>() == 64);

impl From<Mat4> for glam::Mat4 {
    fn from(val: Mat4) -> Self {
        glam::Mat4::from_cols_array(&val.0)
    }
}

impl From<glam::Mat4> for Mat4 {
    fn from(val: glam::Mat4) -> Self {
        Mat4(val.to_cols_array())
    }
}
