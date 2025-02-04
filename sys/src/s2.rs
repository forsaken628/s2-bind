/* automatically generated by rust-bindgen 0.70.0 */

#[doc = " A string of @c char"]
pub type std_string = [u64; 4usize];
pub type std_integral_constant_value_type<_Tp> = _Tp;
#[doc = " integral_constant"]
pub type std_integral_constant_type = u8;
#[doc = " The type used as a compile-time boolean with true value."]
pub type std_true_type = u8;
pub type std_conditional_type<_Iftrue> = _Iftrue;
#[doc = " Alias template make_integer_sequence"]
pub type std_make_integer_sequence = u8;
#[doc = " Alias template make_index_sequence"]
pub type std_make_index_sequence = std_make_integer_sequence;
pub type uint32 = libc::c_uint;
pub type uint64 = libc::c_ulonglong;
pub type util_math_internal_vector_BasicVector_D = u8;
pub type util_math_internal_vector_BasicVector_FloatType = u8;
pub type util_math_internal_vector_BasicVector_IdxSeqN = std_make_index_sequence;
pub const util_math_internal_vector_BasicVector_SIZE:
    util_math_internal_vector_BasicVector__bindgen_ty_1 = 0;
pub type util_math_internal_vector_BasicVector__bindgen_ty_1 = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub _address: u8,
}
pub type Vector2_Base = u8;
pub type Vector2_VType<T> = T;
pub type Vector2_BaseType<T> = Vector2_VType<T>;
pub type Vector2_FloatType = Vector2_Base;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub _address: u8,
}
pub type Vector3_Base = u8;
pub type Vector3_VType<T> = T;
pub type Vector3_BaseType<T> = Vector3_VType<T>;
pub type Vector3_FloatType = Vector3_Base;
pub type Vector2_d = [u64; 2usize];
pub type Vector3_d = [u64; 3usize];
pub type R2Point = Vector2_d;
#[repr(C)]
#[derive(Debug)]
pub struct Encoder {
    pub buf_: *mut libc::c_uchar,
    pub limit_: *mut libc::c_uchar,
    pub underlying_buffer_: *mut libc::c_uchar,
    pub orig_: *mut libc::c_uchar,
}
#[repr(C)]
#[derive(Debug)]
pub struct Encoder_Writer {
    pub enc: *mut Encoder,
    pub p: *mut libc::c_char,
}
const _: () = {
    ["Size of Encoder_Writer"][::std::mem::size_of::<Encoder_Writer>() - 16usize];
    ["Alignment of Encoder_Writer"][::std::mem::align_of::<Encoder_Writer>() - 8usize];
    ["Offset of field: Encoder_Writer::enc"][::std::mem::offset_of!(Encoder_Writer, enc) - 0usize];
    ["Offset of field: Encoder_Writer::p"][::std::mem::offset_of!(Encoder_Writer, p) - 8usize];
};
pub const Encoder_kVarintMax32: libc::c_int = 5;
pub const Encoder_kVarintMax64: libc::c_int = 10;
const _: () = {
    ["Size of Encoder"][::std::mem::size_of::<Encoder>() - 32usize];
    ["Alignment of Encoder"][::std::mem::align_of::<Encoder>() - 8usize];
    ["Offset of field: Encoder::buf_"][::std::mem::offset_of!(Encoder, buf_) - 0usize];
    ["Offset of field: Encoder::limit_"][::std::mem::offset_of!(Encoder, limit_) - 8usize];
    ["Offset of field: Encoder::underlying_buffer_"]
        [::std::mem::offset_of!(Encoder, underlying_buffer_) - 16usize];
    ["Offset of field: Encoder::orig_"][::std::mem::offset_of!(Encoder, orig_) - 24usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN7Encoder15varint32_lengthEj"]
    pub fn Encoder_varint32_length(v: uint32) -> libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}_ZN7Encoder15varint64_lengthEy"]
    pub fn Encoder_varint64_length(v: uint64) -> libc::c_int;
}
extern "C" {
    #[link_name = "\u{1}_ZN7Encoder6ResizeEm"]
    pub fn Encoder_Resize(this: *mut Encoder, N: usize);
}
extern "C" {
    #[link_name = "\u{1}_ZN7EncoderC1EOS_"]
    pub fn Encoder_Encoder(this: *mut Encoder, other: *mut Encoder);
}
extern "C" {
    #[link_name = "\u{1}_ZN7EncoderD1Ev"]
    pub fn Encoder_Encoder_destructor(this: *mut Encoder);
}
impl Encoder {
    #[inline]
    pub unsafe fn varint32_length(v: uint32) -> libc::c_int {
        Encoder_varint32_length(v)
    }
    #[inline]
    pub unsafe fn varint64_length(v: uint64) -> libc::c_int {
        Encoder_varint64_length(v)
    }
    #[inline]
    pub unsafe fn Resize(&mut self, N: usize) {
        Encoder_Resize(self, N)
    }
    #[inline]
    pub unsafe fn new(other: *mut Encoder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Encoder_Encoder(__bindgen_tmp.as_mut_ptr(), other);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Encoder_Encoder_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Decoder {
    pub orig_: *const libc::c_uchar,
    pub buf_: *const libc::c_uchar,
    pub limit_: *const libc::c_uchar,
}
const _: () = {
    ["Size of Decoder"][::std::mem::size_of::<Decoder>() - 24usize];
    ["Alignment of Decoder"][::std::mem::align_of::<Decoder>() - 8usize];
    ["Offset of field: Decoder::orig_"][::std::mem::offset_of!(Decoder, orig_) - 0usize];
    ["Offset of field: Decoder::buf_"][::std::mem::offset_of!(Decoder, buf_) - 8usize];
    ["Offset of field: Decoder::limit_"][::std::mem::offset_of!(Decoder, limit_) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S2Error {
    pub code_: S2Error_Code,
    pub text_: std_string,
}
pub const S2Error_Code_OK: S2Error_Code = 0;
pub const S2Error_Code_UNKNOWN: S2Error_Code = 1000;
pub const S2Error_Code_UNIMPLEMENTED: S2Error_Code = 1001;
pub const S2Error_Code_OUT_OF_RANGE: S2Error_Code = 1002;
pub const S2Error_Code_INVALID_ARGUMENT: S2Error_Code = 1003;
pub const S2Error_Code_FAILED_PRECONDITION: S2Error_Code = 1004;
pub const S2Error_Code_INTERNAL: S2Error_Code = 1005;
pub const S2Error_Code_DATA_LOSS: S2Error_Code = 1006;
pub const S2Error_Code_RESOURCE_EXHAUSTED: S2Error_Code = 1007;
pub const S2Error_Code_CANCELLED: S2Error_Code = 1008;
pub const S2Error_Code_USER_DEFINED_START: S2Error_Code = 1000000;
pub const S2Error_Code_USER_DEFINED_END: S2Error_Code = 9999999;
pub const S2Error_Code_NOT_UNIT_LENGTH: S2Error_Code = 1;
pub const S2Error_Code_DUPLICATE_VERTICES: S2Error_Code = 2;
pub const S2Error_Code_ANTIPODAL_VERTICES: S2Error_Code = 3;
pub const S2Error_Code_NOT_CONTINUOUS: S2Error_Code = 4;
pub const S2Error_Code_INVALID_VERTEX: S2Error_Code = 5;
pub const S2Error_Code_LOOP_NOT_ENOUGH_VERTICES: S2Error_Code = 100;
pub const S2Error_Code_LOOP_SELF_INTERSECTION: S2Error_Code = 101;
pub const S2Error_Code_POLYGON_LOOPS_SHARE_EDGE: S2Error_Code = 200;
pub const S2Error_Code_POLYGON_LOOPS_CROSS: S2Error_Code = 201;
pub const S2Error_Code_POLYGON_EMPTY_LOOP: S2Error_Code = 202;
pub const S2Error_Code_POLYGON_EXCESS_FULL_LOOP: S2Error_Code = 203;
pub const S2Error_Code_POLYGON_INCONSISTENT_LOOP_ORIENTATIONS: S2Error_Code = 204;
pub const S2Error_Code_POLYGON_INVALID_LOOP_DEPTH: S2Error_Code = 205;
pub const S2Error_Code_POLYGON_INVALID_LOOP_NESTING: S2Error_Code = 206;
pub const S2Error_Code_INVALID_DIMENSION: S2Error_Code = 207;
pub const S2Error_Code_SPLIT_INTERIOR: S2Error_Code = 208;
pub const S2Error_Code_OVERLAPPING_GEOMETRY: S2Error_Code = 209;
pub const S2Error_Code_BUILDER_SNAP_RADIUS_TOO_SMALL: S2Error_Code = 300;
pub const S2Error_Code_BUILDER_MISSING_EXPECTED_SIBLING_EDGES: S2Error_Code = 301;
pub const S2Error_Code_BUILDER_UNEXPECTED_DEGENERATE_EDGE: S2Error_Code = 302;
pub const S2Error_Code_BUILDER_EDGES_DO_NOT_FORM_LOOPS: S2Error_Code = 303;
pub const S2Error_Code_BUILDER_EDGES_DO_NOT_FORM_POLYLINE: S2Error_Code = 304;
pub const S2Error_Code_BUILDER_IS_FULL_PREDICATE_NOT_SPECIFIED: S2Error_Code = 305;
pub type S2Error_Code = libc::c_uint;
const _: () = {
    ["Size of S2Error"][::std::mem::size_of::<S2Error>() - 40usize];
    ["Alignment of S2Error"][::std::mem::align_of::<S2Error>() - 8usize];
    ["Offset of field: S2Error::code_"][::std::mem::offset_of!(S2Error, code_) - 0usize];
    ["Offset of field: S2Error::text_"][::std::mem::offset_of!(S2Error, text_) - 8usize];
};
#[repr(C)]
pub struct s2coding_S2Coder__bindgen_vtable(libc::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct s2coding_S2Coder {
    pub vtable_: *const s2coding_S2Coder__bindgen_vtable,
}
#[repr(C)]
#[derive(Debug)]
pub struct s2coding_S2BasicCoder {
    pub _base: s2coding_S2Coder,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S2Point {
    pub _base: Vector3_d,
}
pub type S2Point_ValType = libc::c_double;
pub type S2Point_Coder = s2coding_S2BasicCoder;
pub type S2Point_Base = Vector3_d;
const _: () = {
    ["Size of S2Point"][::std::mem::size_of::<S2Point>() - 24usize];
    ["Alignment of S2Point"][::std::mem::align_of::<S2Point>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S1Angle {
    pub radians_: libc::c_double,
}
#[doc = " integral_constant"]
pub type S1Angle_absl_btree_prefer_linear_node_search = std_true_type;
const _: () = {
    ["Size of S1Angle"][::std::mem::size_of::<S1Angle>() - 8usize];
    ["Alignment of S1Angle"][::std::mem::align_of::<S1Angle>() - 8usize];
    ["Offset of field: S1Angle::radians_"][::std::mem::offset_of!(S1Angle, radians_) - 0usize];
};
extern "C" {
    #[link_name = "\u{1}_ZNK7S1Angle10NormalizedEv"]
    pub fn S1Angle_Normalized(this: *const S1Angle) -> S1Angle;
}
extern "C" {
    #[link_name = "\u{1}_ZN7S1Angle9NormalizeEv"]
    pub fn S1Angle_Normalize(this: *mut S1Angle);
}
extern "C" {
    #[link_name = "\u{1}_ZN7S1AngleC1ERK7S2PointS2_"]
    pub fn S1Angle_S1Angle(this: *mut S1Angle, x: *const S2Point, y: *const S2Point);
}
extern "C" {
    #[link_name = "\u{1}_ZN7S1AngleC1ERK8S2LatLngS2_"]
    pub fn S1Angle_S1Angle1(this: *mut S1Angle, x: *const S2LatLng, y: *const S2LatLng);
}
impl S1Angle {
    #[inline]
    pub unsafe fn Normalized(&self) -> S1Angle {
        S1Angle_Normalized(self)
    }
    #[inline]
    pub unsafe fn Normalize(&mut self) {
        S1Angle_Normalize(self)
    }
    #[inline]
    pub unsafe fn new(x: *const S2Point, y: *const S2Point) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        S1Angle_S1Angle(__bindgen_tmp.as_mut_ptr(), x, y);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(x: *const S2LatLng, y: *const S2LatLng) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        S1Angle_S1Angle1(__bindgen_tmp.as_mut_ptr(), x, y);
        __bindgen_tmp.assume_init()
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct S2LatLng {
    pub coords_: R2Point,
}
pub type S2LatLng_Coder = s2coding_S2BasicCoder;
const _: () = {
    ["Size of S2LatLng"][::std::mem::size_of::<S2LatLng>() - 16usize];
    ["Alignment of S2LatLng"][::std::mem::align_of::<S2LatLng>() - 8usize];
    ["Offset of field: S2LatLng::coords_"][::std::mem::offset_of!(S2LatLng, coords_) - 0usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN8S2LatLng4InitEP7DecoderR7S2Error"]
    pub fn S2LatLng_Init(this: *mut S2LatLng, decoder: *mut Decoder, error: *mut S2Error) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNK8S2LatLng6EncodeEP7Encoder"]
    pub fn S2LatLng_Encode(this: *const S2LatLng, encoder: *mut Encoder);
}
extern "C" {
    #[link_name = "\u{1}_ZNK8S2LatLng10NormalizedEv"]
    pub fn S2LatLng_Normalized(this: *const S2LatLng) -> S2LatLng;
}
extern "C" {
    #[link_name = "\u{1}_ZNK8S2LatLng7ToPointEv"]
    pub fn S2LatLng_ToPoint(this: *const S2LatLng) -> S2Point;
}
extern "C" {
    #[link_name = "\u{1}_ZNK8S2LatLng11GetDistanceERKS_"]
    pub fn S2LatLng_GetDistance(this: *const S2LatLng, o: *const S2LatLng) -> S1Angle;
}
extern "C" {
    #[link_name = "\u{1}_ZNK8S2LatLng17ToStringInDegreesB5cxx11Ev"]
    pub fn S2LatLng_ToStringInDegrees(this: *const S2LatLng) -> std_string;
}
extern "C" {
    #[link_name = "\u{1}_ZN8S2LatLngC1ERK7S2Point"]
    pub fn S2LatLng_S2LatLng(this: *mut S2LatLng, p: *const S2Point);
}
impl S2LatLng {
    #[inline]
    pub unsafe fn Init(&mut self, decoder: *mut Decoder, error: *mut S2Error) -> bool {
        S2LatLng_Init(self, decoder, error)
    }
    #[inline]
    pub unsafe fn Encode(&self, encoder: *mut Encoder) {
        S2LatLng_Encode(self, encoder)
    }
    #[inline]
    pub unsafe fn Normalized(&self) -> S2LatLng {
        S2LatLng_Normalized(self)
    }
    #[inline]
    pub unsafe fn ToPoint(&self) -> S2Point {
        S2LatLng_ToPoint(self)
    }
    #[inline]
    pub unsafe fn GetDistance(&self, o: *const S2LatLng) -> S1Angle {
        S2LatLng_GetDistance(self, o)
    }
    #[inline]
    pub unsafe fn ToStringInDegrees(&self) -> std_string {
        S2LatLng_ToStringInDegrees(self)
    }
    #[inline]
    pub unsafe fn new(p: *const S2Point) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        S2LatLng_S2LatLng(__bindgen_tmp.as_mut_ptr(), p);
        __bindgen_tmp.assume_init()
    }
}
