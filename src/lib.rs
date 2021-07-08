// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

#[cfg(feature = "std")]
extern crate core;
#[macro_use]
extern crate alloc;

use core::{ptr, mem};

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::string::ToString;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
extern crate gleam;

#[allow(non_camel_case_types, dead_code)]
pub mod ctypes {
    pub enum c_void {}
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
    pub type __int8 = i8;
    pub type __uint8 = u8;
    pub type __int16 = i16;
    pub type __uint16 = u16;
    pub type __int32 = i32;
    pub type __uint32 = u32;
    pub type __int64 = i64;
    pub type __uint64 = u64;
    pub type wchar_t = u16;
}

pub(crate) mod ffi {
    use super::GLenum;
    pub const HIGH_INT: GLenum = 0x8DF5;
    pub const MEDIUM_INT: GLenum = 0x8DF4;
    pub const LOW_INT: GLenum = 0x8DF3;
    pub const HIGH_FLOAT: GLenum = 0x8DF2;
    pub const MEDIUM_FLOAT: GLenum = 0x8DF1;
    pub const LOW_FLOAT: GLenum = 0x8DF0;
    pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
    pub const PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
    pub const UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
    pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
    pub const ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
    pub const MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;

    pub const UNSIGNED_BYTE: GLenum = 0x1401;
    pub const BYTE: GLenum = 0x1400;
    pub const UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
    pub const UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
    pub const UNSIGNED_SHORT: GLenum = 0x1403;
    pub const UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
    pub const UNSIGNED_SHORT_5_6_5_REV : GLenum = 0x8364;
    pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
    pub const UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
    pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
    pub const UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
    pub const SHORT: GLenum = 0x1402;
    pub const UNSIGNED_INT: GLenum = 0x1405;
    pub const UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
    pub const UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
    pub const UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
    pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
    pub const UNSIGNED_INT_24_8: GLenum = 0x84FA;
    pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
    pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
    pub const INT: GLenum = 0x1404;
    pub const HALF_FLOAT: GLenum = 0x140B;
    pub const FLOAT: GLenum = 0x1406;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;

    pub const PACK_ALIGNMENT: GLenum = 0x0D05;
}

pub use self::ctypes::*;

/// Typedef for an OpenGL handle
pub type GLuint = u32;
pub type GLint = i32;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLenum = u32;
pub type GLintptr = isize;
pub type GLboolean = u8;
pub type GLsizeiptr = isize;
pub type GLvoid = core::ffi::c_void;
pub type GLbitfield = u32;
pub type GLsizei = i32;
pub type GLclampf = f32;
pub type GLfloat = f32;
pub type GLchar = c_char; // = c_char
pub type GLclampd = f64;
pub type GLubyte = c_uchar;

#[cfg(feature = "std")]
pub type GLeglImageOES = gleam::gl::GLeglImageOES; // *const c_void;
#[cfg(not(feature = "std"))]
pub type GLeglImageOES = *const c_void;

#[cfg(feature = "std")]
pub type GLsync = gleam::gl::GLsync;
#[cfg(not(feature = "std"))]
pub type GLsync = *const c_void;

#[cfg(feature = "std")]
pub use gleam::gl::DebugMessage;

#[cfg(not(feature = "std"))]
#[repr(C)]
pub struct DebugMessage {
    pub message: String,
    pub source: GLenum,
    pub ty: GLenum,
    pub id: GLenum,
    pub severity: GLenum,
}

#[cfg(feature = "std")]
pub use gleam::gl::GlType;

#[cfg(not(feature = "std"))]
#[repr(C)]
pub enum GlType {
    Gl,
    GlEs,
}

pub struct GenericGlContext {
    pub glAccum: *mut c_void,
    pub glActiveTexture: *mut c_void,
    pub glAlphaFunc: *mut c_void,
    pub glAreTexturesResident: *mut c_void,
    pub glArrayElement: *mut c_void,
    pub glAttachShader: *mut c_void,
    pub glBegin: *mut c_void,
    pub glBeginConditionalRender: *mut c_void,
    pub glBeginQuery: *mut c_void,
    pub glBeginTransformFeedback: *mut c_void,
    pub glBindAttribLocation: *mut c_void,
    pub glBindBuffer: *mut c_void,
    pub glBindBufferBase: *mut c_void,
    pub glBindBufferRange: *mut c_void,
    pub glBindFragDataLocation: *mut c_void,
    pub glBindFragDataLocationIndexed: *mut c_void,
    pub glBindFramebuffer: *mut c_void,
    pub glBindRenderbuffer: *mut c_void,
    pub glBindSampler: *mut c_void,
    pub glBindTexture: *mut c_void,
    pub glBindVertexArray: *mut c_void,
    pub glBindVertexArrayAPPLE: *mut c_void,
    pub glBitmap: *mut c_void,
    pub glBlendBarrierKHR: *mut c_void,
    pub glBlendColor: *mut c_void,
    pub glBlendEquation: *mut c_void,
    pub glBlendEquationSeparate: *mut c_void,
    pub glBlendFunc: *mut c_void,
    pub glBlendFuncSeparate: *mut c_void,
    pub glBlitFramebuffer: *mut c_void,
    pub glBufferData: *mut c_void,
    pub glBufferStorage: *mut c_void,
    pub glBufferSubData: *mut c_void,
    pub glCallList: *mut c_void,
    pub glCallLists: *mut c_void,
    pub glCheckFramebufferStatus: *mut c_void,
    pub glClampColor: *mut c_void,
    pub glClear: *mut c_void,
    pub glClearAccum: *mut c_void,
    pub glClearBufferfi: *mut c_void,
    pub glClearBufferfv: *mut c_void,
    pub glClearBufferiv: *mut c_void,
    pub glClearBufferuiv: *mut c_void,
    pub glClearColor: *mut c_void,
    pub glClearDepth: *mut c_void,
    pub glClearIndex: *mut c_void,
    pub glClearStencil: *mut c_void,
    pub glClientActiveTexture: *mut c_void,
    pub glClientWaitSync: *mut c_void,
    pub glClipPlane: *mut c_void,
    pub glColor3b: *mut c_void,
    pub glColor3bv: *mut c_void,
    pub glColor3d: *mut c_void,
    pub glColor3dv: *mut c_void,
    pub glColor3f: *mut c_void,
    pub glColor3fv: *mut c_void,
    pub glColor3i: *mut c_void,
    pub glColor3iv: *mut c_void,
    pub glColor3s: *mut c_void,
    pub glColor3sv: *mut c_void,
    pub glColor3ub: *mut c_void,
    pub glColor3ubv: *mut c_void,
    pub glColor3ui: *mut c_void,
    pub glColor3uiv: *mut c_void,
    pub glColor3us: *mut c_void,
    pub glColor3usv: *mut c_void,
    pub glColor4b: *mut c_void,
    pub glColor4bv: *mut c_void,
    pub glColor4d: *mut c_void,
    pub glColor4dv: *mut c_void,
    pub glColor4f: *mut c_void,
    pub glColor4fv: *mut c_void,
    pub glColor4i: *mut c_void,
    pub glColor4iv: *mut c_void,
    pub glColor4s: *mut c_void,
    pub glColor4sv: *mut c_void,
    pub glColor4ub: *mut c_void,
    pub glColor4ubv: *mut c_void,
    pub glColor4ui: *mut c_void,
    pub glColor4uiv: *mut c_void,
    pub glColor4us: *mut c_void,
    pub glColor4usv: *mut c_void,
    pub glColorMask: *mut c_void,
    pub glColorMaski: *mut c_void,
    pub glColorMaterial: *mut c_void,
    pub glColorP3ui: *mut c_void,
    pub glColorP3uiv: *mut c_void,
    pub glColorP4ui: *mut c_void,
    pub glColorP4uiv: *mut c_void,
    pub glColorPointer: *mut c_void,
    pub glCompileShader: *mut c_void,
    pub glCompressedTexImage1D: *mut c_void,
    pub glCompressedTexImage2D: *mut c_void,
    pub glCompressedTexImage3D: *mut c_void,
    pub glCompressedTexSubImage1D: *mut c_void,
    pub glCompressedTexSubImage2D: *mut c_void,
    pub glCompressedTexSubImage3D: *mut c_void,
    pub glCopyBufferSubData: *mut c_void,
    pub glCopyImageSubData: *mut c_void,
    pub glCopyPixels: *mut c_void,
    pub glCopyTexImage1D: *mut c_void,
    pub glCopyTexImage2D: *mut c_void,
    pub glCopyTexSubImage1D: *mut c_void,
    pub glCopyTexSubImage2D: *mut c_void,
    pub glCopyTexSubImage3D: *mut c_void,
    pub glCreateProgram: *mut c_void,
    pub glCreateShader: *mut c_void,
    pub glCullFace: *mut c_void,
    pub glDebugMessageCallback: *mut c_void,
    pub glDebugMessageCallbackKHR: *mut c_void,
    pub glDebugMessageControl: *mut c_void,
    pub glDebugMessageControlKHR: *mut c_void,
    pub glDebugMessageInsert: *mut c_void,
    pub glDebugMessageInsertKHR: *mut c_void,
    pub glDeleteBuffers: *mut c_void,
    pub glDeleteFencesAPPLE: *mut c_void,
    pub glDeleteFramebuffers: *mut c_void,
    pub glDeleteLists: *mut c_void,
    pub glDeleteProgram: *mut c_void,
    pub glDeleteQueries: *mut c_void,
    pub glDeleteRenderbuffers: *mut c_void,
    pub glDeleteSamplers: *mut c_void,
    pub glDeleteShader: *mut c_void,
    pub glDeleteSync: *mut c_void,
    pub glDeleteTextures: *mut c_void,
    pub glDeleteVertexArrays: *mut c_void,
    pub glDeleteVertexArraysAPPLE: *mut c_void,
    pub glDepthFunc: *mut c_void,
    pub glDepthMask: *mut c_void,
    pub glDepthRange: *mut c_void,
    pub glDetachShader: *mut c_void,
    pub glDisable: *mut c_void,
    pub glDisableClientState: *mut c_void,
    pub glDisableVertexAttribArray: *mut c_void,
    pub glDisablei: *mut c_void,
    pub glDrawArrays: *mut c_void,
    pub glDrawArraysInstanced: *mut c_void,
    pub glDrawBuffer: *mut c_void,
    pub glDrawBuffers: *mut c_void,
    pub glDrawElements: *mut c_void,
    pub glDrawElementsBaseVertex: *mut c_void,
    pub glDrawElementsInstanced: *mut c_void,
    pub glDrawElementsInstancedBaseVertex: *mut c_void,
    pub glDrawPixels: *mut c_void,
    pub glDrawRangeElements: *mut c_void,
    pub glDrawRangeElementsBaseVertex: *mut c_void,
    pub glEdgeFlag: *mut c_void,
    pub glEdgeFlagPointer: *mut c_void,
    pub glEdgeFlagv: *mut c_void,
    pub glEnable: *mut c_void,
    pub glEnableClientState: *mut c_void,
    pub glEnableVertexAttribArray: *mut c_void,
    pub glEnablei: *mut c_void,
    pub glEnd: *mut c_void,
    pub glEndConditionalRender: *mut c_void,
    pub glEndList: *mut c_void,
    pub glEndQuery: *mut c_void,
    pub glEndTransformFeedback: *mut c_void,
    pub glEvalCoord1d: *mut c_void,
    pub glEvalCoord1dv: *mut c_void,
    pub glEvalCoord1f: *mut c_void,
    pub glEvalCoord1fv: *mut c_void,
    pub glEvalCoord2d: *mut c_void,
    pub glEvalCoord2dv: *mut c_void,
    pub glEvalCoord2f: *mut c_void,
    pub glEvalCoord2fv: *mut c_void,
    pub glEvalMesh1: *mut c_void,
    pub glEvalMesh2: *mut c_void,
    pub glEvalPoint1: *mut c_void,
    pub glEvalPoint2: *mut c_void,
    pub glFeedbackBuffer: *mut c_void,
    pub glFenceSync: *mut c_void,
    pub glFinish: *mut c_void,
    pub glFinishFenceAPPLE: *mut c_void,
    pub glFinishObjectAPPLE: *mut c_void,
    pub glFlush: *mut c_void,
    pub glFlushMappedBufferRange: *mut c_void,
    pub glFogCoordPointer: *mut c_void,
    pub glFogCoordd: *mut c_void,
    pub glFogCoorddv: *mut c_void,
    pub glFogCoordf: *mut c_void,
    pub glFogCoordfv: *mut c_void,
    pub glFogf: *mut c_void,
    pub glFogfv: *mut c_void,
    pub glFogi: *mut c_void,
    pub glFogiv: *mut c_void,
    pub glFramebufferRenderbuffer: *mut c_void,
    pub glFramebufferTexture: *mut c_void,
    pub glFramebufferTexture1D: *mut c_void,
    pub glFramebufferTexture2D: *mut c_void,
    pub glFramebufferTexture3D: *mut c_void,
    pub glFramebufferTextureLayer: *mut c_void,
    pub glFrontFace: *mut c_void,
    pub glFrustum: *mut c_void,
    pub glGenBuffers: *mut c_void,
    pub glGenFencesAPPLE: *mut c_void,
    pub glGenFramebuffers: *mut c_void,
    pub glGenLists: *mut c_void,
    pub glGenQueries: *mut c_void,
    pub glGenRenderbuffers: *mut c_void,
    pub glGenSamplers: *mut c_void,
    pub glGenTextures: *mut c_void,
    pub glGenVertexArrays: *mut c_void,
    pub glGenVertexArraysAPPLE: *mut c_void,
    pub glGenerateMipmap: *mut c_void,
    pub glGetActiveAttrib: *mut c_void,
    pub glGetActiveUniform: *mut c_void,
    pub glGetActiveUniformBlockName: *mut c_void,
    pub glGetActiveUniformBlockiv: *mut c_void,
    pub glGetActiveUniformName: *mut c_void,
    pub glGetActiveUniformsiv: *mut c_void,
    pub glGetAttachedShaders: *mut c_void,
    pub glGetAttribLocation: *mut c_void,
    pub glGetBooleani_v: *mut c_void,
    pub glGetBooleanv: *mut c_void,
    pub glGetBufferParameteri64v: *mut c_void,
    pub glGetBufferParameteriv: *mut c_void,
    pub glGetBufferPointerv: *mut c_void,
    pub glGetBufferSubData: *mut c_void,
    pub glGetClipPlane: *mut c_void,
    pub glGetCompressedTexImage: *mut c_void,
    pub glGetDebugMessageLog: *mut c_void,
    pub glGetDebugMessageLogKHR: *mut c_void,
    pub glGetDoublev: *mut c_void,
    pub glGetError: *mut c_void,
    pub glGetFloatv: *mut c_void,
    pub glGetFragDataIndex: *mut c_void,
    pub glGetFragDataLocation: *mut c_void,
    pub glGetFramebufferAttachmentParameteriv: *mut c_void,
    pub glGetInteger64i_v: *mut c_void,
    pub glGetInteger64v: *mut c_void,
    pub glGetIntegeri_v: *mut c_void,
    pub glGetIntegerv: *mut c_void,
    pub glGetLightfv: *mut c_void,
    pub glGetLightiv: *mut c_void,
    pub glGetMapdv: *mut c_void,
    pub glGetMapfv: *mut c_void,
    pub glGetMapiv: *mut c_void,
    pub glGetMaterialfv: *mut c_void,
    pub glGetMaterialiv: *mut c_void,
    pub glGetMultisamplefv: *mut c_void,
    pub glGetObjectLabel: *mut c_void,
    pub glGetObjectLabelKHR: *mut c_void,
    pub glGetObjectPtrLabel: *mut c_void,
    pub glGetObjectPtrLabelKHR: *mut c_void,
    pub glGetPixelMapfv: *mut c_void,
    pub glGetPixelMapuiv: *mut c_void,
    pub glGetPixelMapusv: *mut c_void,
    pub glGetPointerv: *mut c_void,
    pub glGetPointervKHR: *mut c_void,
    pub glGetPolygonStipple: *mut c_void,
    pub glGetProgramBinary: *mut c_void,
    pub glGetProgramInfoLog: *mut c_void,
    pub glGetProgramiv: *mut c_void,
    pub glGetQueryObjecti64v: *mut c_void,
    pub glGetQueryObjectiv: *mut c_void,
    pub glGetQueryObjectui64v: *mut c_void,
    pub glGetQueryObjectuiv: *mut c_void,
    pub glGetQueryiv: *mut c_void,
    pub glGetRenderbufferParameteriv: *mut c_void,
    pub glGetSamplerParameterIiv: *mut c_void,
    pub glGetSamplerParameterIuiv: *mut c_void,
    pub glGetSamplerParameterfv: *mut c_void,
    pub glGetSamplerParameteriv: *mut c_void,
    pub glGetShaderInfoLog: *mut c_void,
    pub glGetShaderSource: *mut c_void,
    pub glGetShaderiv: *mut c_void,
    pub glGetString: *mut c_void,
    pub glGetStringi: *mut c_void,
    pub glGetSynciv: *mut c_void,
    pub glGetTexEnvfv: *mut c_void,
    pub glGetTexEnviv: *mut c_void,
    pub glGetTexGendv: *mut c_void,
    pub glGetTexGenfv: *mut c_void,
    pub glGetTexGeniv: *mut c_void,
    pub glGetTexImage: *mut c_void,
    pub glGetTexLevelParameterfv: *mut c_void,
    pub glGetTexLevelParameteriv: *mut c_void,
    pub glGetTexParameterIiv: *mut c_void,
    pub glGetTexParameterIuiv: *mut c_void,
    pub glGetTexParameterPointervAPPLE: *mut c_void,
    pub glGetTexParameterfv: *mut c_void,
    pub glGetTexParameteriv: *mut c_void,
    pub glGetTransformFeedbackVarying: *mut c_void,
    pub glGetUniformBlockIndex: *mut c_void,
    pub glGetUniformIndices: *mut c_void,
    pub glGetUniformLocation: *mut c_void,
    pub glGetUniformfv: *mut c_void,
    pub glGetUniformiv: *mut c_void,
    pub glGetUniformuiv: *mut c_void,
    pub glGetVertexAttribIiv: *mut c_void,
    pub glGetVertexAttribIuiv: *mut c_void,
    pub glGetVertexAttribPointerv: *mut c_void,
    pub glGetVertexAttribdv: *mut c_void,
    pub glGetVertexAttribfv: *mut c_void,
    pub glGetVertexAttribiv: *mut c_void,
    pub glHint: *mut c_void,
    pub glIndexMask: *mut c_void,
    pub glIndexPointer: *mut c_void,
    pub glIndexd: *mut c_void,
    pub glIndexdv: *mut c_void,
    pub glIndexf: *mut c_void,
    pub glIndexfv: *mut c_void,
    pub glIndexi: *mut c_void,
    pub glIndexiv: *mut c_void,
    pub glIndexs: *mut c_void,
    pub glIndexsv: *mut c_void,
    pub glIndexub: *mut c_void,
    pub glIndexubv: *mut c_void,
    pub glInitNames: *mut c_void,
    pub glInsertEventMarkerEXT: *mut c_void,
    pub glInterleavedArrays: *mut c_void,
    pub glInvalidateBufferData: *mut c_void,
    pub glInvalidateBufferSubData: *mut c_void,
    pub glInvalidateFramebuffer: *mut c_void,
    pub glInvalidateSubFramebuffer: *mut c_void,
    pub glInvalidateTexImage: *mut c_void,
    pub glInvalidateTexSubImage: *mut c_void,
    pub glIsBuffer: *mut c_void,
    pub glIsEnabled: *mut c_void,
    pub glIsEnabledi: *mut c_void,
    pub glIsFenceAPPLE: *mut c_void,
    pub glIsFramebuffer: *mut c_void,
    pub glIsList: *mut c_void,
    pub glIsProgram: *mut c_void,
    pub glIsQuery: *mut c_void,
    pub glIsRenderbuffer: *mut c_void,
    pub glIsSampler: *mut c_void,
    pub glIsShader: *mut c_void,
    pub glIsSync: *mut c_void,
    pub glIsTexture: *mut c_void,
    pub glIsVertexArray: *mut c_void,
    pub glIsVertexArrayAPPLE: *mut c_void,
    pub glLightModelf: *mut c_void,
    pub glLightModelfv: *mut c_void,
    pub glLightModeli: *mut c_void,
    pub glLightModeliv: *mut c_void,
    pub glLightf: *mut c_void,
    pub glLightfv: *mut c_void,
    pub glLighti: *mut c_void,
    pub glLightiv: *mut c_void,
    pub glLineStipple: *mut c_void,
    pub glLineWidth: *mut c_void,
    pub glLinkProgram: *mut c_void,
    pub glListBase: *mut c_void,
    pub glLoadIdentity: *mut c_void,
    pub glLoadMatrixd: *mut c_void,
    pub glLoadMatrixf: *mut c_void,
    pub glLoadName: *mut c_void,
    pub glLoadTransposeMatrixd: *mut c_void,
    pub glLoadTransposeMatrixf: *mut c_void,
    pub glLogicOp: *mut c_void,
    pub glMap1d: *mut c_void,
    pub glMap1f: *mut c_void,
    pub glMap2d: *mut c_void,
    pub glMap2f: *mut c_void,
    pub glMapBuffer: *mut c_void,
    pub glMapBufferRange: *mut c_void,
    pub glMapGrid1d: *mut c_void,
    pub glMapGrid1f: *mut c_void,
    pub glMapGrid2d: *mut c_void,
    pub glMapGrid2f: *mut c_void,
    pub glMaterialf: *mut c_void,
    pub glMaterialfv: *mut c_void,
    pub glMateriali: *mut c_void,
    pub glMaterialiv: *mut c_void,
    pub glMatrixMode: *mut c_void,
    pub glMultMatrixd: *mut c_void,
    pub glMultMatrixf: *mut c_void,
    pub glMultTransposeMatrixd: *mut c_void,
    pub glMultTransposeMatrixf: *mut c_void,
    pub glMultiDrawArrays: *mut c_void,
    pub glMultiDrawElements: *mut c_void,
    pub glMultiDrawElementsBaseVertex: *mut c_void,
    pub glMultiTexCoord1d: *mut c_void,
    pub glMultiTexCoord1dv: *mut c_void,
    pub glMultiTexCoord1f: *mut c_void,
    pub glMultiTexCoord1fv: *mut c_void,
    pub glMultiTexCoord1i: *mut c_void,
    pub glMultiTexCoord1iv: *mut c_void,
    pub glMultiTexCoord1s: *mut c_void,
    pub glMultiTexCoord1sv: *mut c_void,
    pub glMultiTexCoord2d: *mut c_void,
    pub glMultiTexCoord2dv: *mut c_void,
    pub glMultiTexCoord2f: *mut c_void,
    pub glMultiTexCoord2fv: *mut c_void,
    pub glMultiTexCoord2i: *mut c_void,
    pub glMultiTexCoord2iv: *mut c_void,
    pub glMultiTexCoord2s: *mut c_void,
    pub glMultiTexCoord2sv: *mut c_void,
    pub glMultiTexCoord3d: *mut c_void,
    pub glMultiTexCoord3dv: *mut c_void,
    pub glMultiTexCoord3f: *mut c_void,
    pub glMultiTexCoord3fv: *mut c_void,
    pub glMultiTexCoord3i: *mut c_void,
    pub glMultiTexCoord3iv: *mut c_void,
    pub glMultiTexCoord3s: *mut c_void,
    pub glMultiTexCoord3sv: *mut c_void,
    pub glMultiTexCoord4d: *mut c_void,
    pub glMultiTexCoord4dv: *mut c_void,
    pub glMultiTexCoord4f: *mut c_void,
    pub glMultiTexCoord4fv: *mut c_void,
    pub glMultiTexCoord4i: *mut c_void,
    pub glMultiTexCoord4iv: *mut c_void,
    pub glMultiTexCoord4s: *mut c_void,
    pub glMultiTexCoord4sv: *mut c_void,
    pub glMultiTexCoordP1ui: *mut c_void,
    pub glMultiTexCoordP1uiv: *mut c_void,
    pub glMultiTexCoordP2ui: *mut c_void,
    pub glMultiTexCoordP2uiv: *mut c_void,
    pub glMultiTexCoordP3ui: *mut c_void,
    pub glMultiTexCoordP3uiv: *mut c_void,
    pub glMultiTexCoordP4ui: *mut c_void,
    pub glMultiTexCoordP4uiv: *mut c_void,
    pub glNewList: *mut c_void,
    pub glNormal3b: *mut c_void,
    pub glNormal3bv: *mut c_void,
    pub glNormal3d: *mut c_void,
    pub glNormal3dv: *mut c_void,
    pub glNormal3f: *mut c_void,
    pub glNormal3fv: *mut c_void,
    pub glNormal3i: *mut c_void,
    pub glNormal3iv: *mut c_void,
    pub glNormal3s: *mut c_void,
    pub glNormal3sv: *mut c_void,
    pub glNormalP3ui: *mut c_void,
    pub glNormalP3uiv: *mut c_void,
    pub glNormalPointer: *mut c_void,
    pub glObjectLabel: *mut c_void,
    pub glObjectLabelKHR: *mut c_void,
    pub glObjectPtrLabel: *mut c_void,
    pub glObjectPtrLabelKHR: *mut c_void,
    pub glOrtho: *mut c_void,
    pub glPassThrough: *mut c_void,
    pub glPixelMapfv: *mut c_void,
    pub glPixelMapuiv: *mut c_void,
    pub glPixelMapusv: *mut c_void,
    pub glPixelStoref: *mut c_void,
    pub glPixelStorei: *mut c_void,
    pub glPixelTransferf: *mut c_void,
    pub glPixelTransferi: *mut c_void,
    pub glPixelZoom: *mut c_void,
    pub glPointParameterf: *mut c_void,
    pub glPointParameterfv: *mut c_void,
    pub glPointParameteri: *mut c_void,
    pub glPointParameteriv: *mut c_void,
    pub glPointSize: *mut c_void,
    pub glPolygonMode: *mut c_void,
    pub glPolygonOffset: *mut c_void,
    pub glPolygonStipple: *mut c_void,
    pub glPopAttrib: *mut c_void,
    pub glPopClientAttrib: *mut c_void,
    pub glPopDebugGroup: *mut c_void,
    pub glPopDebugGroupKHR: *mut c_void,
    pub glPopGroupMarkerEXT: *mut c_void,
    pub glPopMatrix: *mut c_void,
    pub glPopName: *mut c_void,
    pub glPrimitiveRestartIndex: *mut c_void,
    pub glPrioritizeTextures: *mut c_void,
    pub glProgramBinary: *mut c_void,
    pub glProgramParameteri: *mut c_void,
    pub glProvokingVertex: *mut c_void,
    pub glPushAttrib: *mut c_void,
    pub glPushClientAttrib: *mut c_void,
    pub glPushDebugGroup: *mut c_void,
    pub glPushDebugGroupKHR: *mut c_void,
    pub glPushGroupMarkerEXT: *mut c_void,
    pub glPushMatrix: *mut c_void,
    pub glPushName: *mut c_void,
    pub glQueryCounter: *mut c_void,
    pub glRasterPos2d: *mut c_void,
    pub glRasterPos2dv: *mut c_void,
    pub glRasterPos2f: *mut c_void,
    pub glRasterPos2fv: *mut c_void,
    pub glRasterPos2i: *mut c_void,
    pub glRasterPos2iv: *mut c_void,
    pub glRasterPos2s: *mut c_void,
    pub glRasterPos2sv: *mut c_void,
    pub glRasterPos3d: *mut c_void,
    pub glRasterPos3dv: *mut c_void,
    pub glRasterPos3f: *mut c_void,
    pub glRasterPos3fv: *mut c_void,
    pub glRasterPos3i: *mut c_void,
    pub glRasterPos3iv: *mut c_void,
    pub glRasterPos3s: *mut c_void,
    pub glRasterPos3sv: *mut c_void,
    pub glRasterPos4d: *mut c_void,
    pub glRasterPos4dv: *mut c_void,
    pub glRasterPos4f: *mut c_void,
    pub glRasterPos4fv: *mut c_void,
    pub glRasterPos4i: *mut c_void,
    pub glRasterPos4iv: *mut c_void,
    pub glRasterPos4s: *mut c_void,
    pub glRasterPos4sv: *mut c_void,
    pub glReadBuffer: *mut c_void,
    pub glReadPixels: *mut c_void,
    pub glRectd: *mut c_void,
    pub glRectdv: *mut c_void,
    pub glRectf: *mut c_void,
    pub glRectfv: *mut c_void,
    pub glRecti: *mut c_void,
    pub glRectiv: *mut c_void,
    pub glRects: *mut c_void,
    pub glRectsv: *mut c_void,
    pub glRenderMode: *mut c_void,
    pub glRenderbufferStorage: *mut c_void,
    pub glRenderbufferStorageMultisample: *mut c_void,
    pub glRotated: *mut c_void,
    pub glRotatef: *mut c_void,
    pub glSampleCoverage: *mut c_void,
    pub glSampleMaski: *mut c_void,
    pub glSamplerParameterIiv: *mut c_void,
    pub glSamplerParameterIuiv: *mut c_void,
    pub glSamplerParameterf: *mut c_void,
    pub glSamplerParameterfv: *mut c_void,
    pub glSamplerParameteri: *mut c_void,
    pub glSamplerParameteriv: *mut c_void,
    pub glScaled: *mut c_void,
    pub glScalef: *mut c_void,
    pub glScissor: *mut c_void,
    pub glSecondaryColor3b: *mut c_void,
    pub glSecondaryColor3bv: *mut c_void,
    pub glSecondaryColor3d: *mut c_void,
    pub glSecondaryColor3dv: *mut c_void,
    pub glSecondaryColor3f: *mut c_void,
    pub glSecondaryColor3fv: *mut c_void,
    pub glSecondaryColor3i: *mut c_void,
    pub glSecondaryColor3iv: *mut c_void,
    pub glSecondaryColor3s: *mut c_void,
    pub glSecondaryColor3sv: *mut c_void,
    pub glSecondaryColor3ub: *mut c_void,
    pub glSecondaryColor3ubv: *mut c_void,
    pub glSecondaryColor3ui: *mut c_void,
    pub glSecondaryColor3uiv: *mut c_void,
    pub glSecondaryColor3us: *mut c_void,
    pub glSecondaryColor3usv: *mut c_void,
    pub glSecondaryColorP3ui: *mut c_void,
    pub glSecondaryColorP3uiv: *mut c_void,
    pub glSecondaryColorPointer: *mut c_void,
    pub glSelectBuffer: *mut c_void,
    pub glSetFenceAPPLE: *mut c_void,
    pub glShadeModel: *mut c_void,
    pub glShaderSource: *mut c_void,
    pub glShaderStorageBlockBinding: *mut c_void,
    pub glStencilFunc: *mut c_void,
    pub glStencilFuncSeparate: *mut c_void,
    pub glStencilMask: *mut c_void,
    pub glStencilMaskSeparate: *mut c_void,
    pub glStencilOp: *mut c_void,
    pub glStencilOpSeparate: *mut c_void,
    pub glTestFenceAPPLE: *mut c_void,
    pub glTestObjectAPPLE: *mut c_void,
    pub glTexBuffer: *mut c_void,
    pub glTexCoord1d: *mut c_void,
    pub glTexCoord1dv: *mut c_void,
    pub glTexCoord1f: *mut c_void,
    pub glTexCoord1fv: *mut c_void,
    pub glTexCoord1i: *mut c_void,
    pub glTexCoord1iv: *mut c_void,
    pub glTexCoord1s: *mut c_void,
    pub glTexCoord1sv: *mut c_void,
    pub glTexCoord2d: *mut c_void,
    pub glTexCoord2dv: *mut c_void,
    pub glTexCoord2f: *mut c_void,
    pub glTexCoord2fv: *mut c_void,
    pub glTexCoord2i: *mut c_void,
    pub glTexCoord2iv: *mut c_void,
    pub glTexCoord2s: *mut c_void,
    pub glTexCoord2sv: *mut c_void,
    pub glTexCoord3d: *mut c_void,
    pub glTexCoord3dv: *mut c_void,
    pub glTexCoord3f: *mut c_void,
    pub glTexCoord3fv: *mut c_void,
    pub glTexCoord3i: *mut c_void,
    pub glTexCoord3iv: *mut c_void,
    pub glTexCoord3s: *mut c_void,
    pub glTexCoord3sv: *mut c_void,
    pub glTexCoord4d: *mut c_void,
    pub glTexCoord4dv: *mut c_void,
    pub glTexCoord4f: *mut c_void,
    pub glTexCoord4fv: *mut c_void,
    pub glTexCoord4i: *mut c_void,
    pub glTexCoord4iv: *mut c_void,
    pub glTexCoord4s: *mut c_void,
    pub glTexCoord4sv: *mut c_void,
    pub glTexCoordP1ui: *mut c_void,
    pub glTexCoordP1uiv: *mut c_void,
    pub glTexCoordP2ui: *mut c_void,
    pub glTexCoordP2uiv: *mut c_void,
    pub glTexCoordP3ui: *mut c_void,
    pub glTexCoordP3uiv: *mut c_void,
    pub glTexCoordP4ui: *mut c_void,
    pub glTexCoordP4uiv: *mut c_void,
    pub glTexCoordPointer: *mut c_void,
    pub glTexEnvf: *mut c_void,
    pub glTexEnvfv: *mut c_void,
    pub glTexEnvi: *mut c_void,
    pub glTexEnviv: *mut c_void,
    pub glTexGend: *mut c_void,
    pub glTexGendv: *mut c_void,
    pub glTexGenf: *mut c_void,
    pub glTexGenfv: *mut c_void,
    pub glTexGeni: *mut c_void,
    pub glTexGeniv: *mut c_void,
    pub glTexImage1D: *mut c_void,
    pub glTexImage2D: *mut c_void,
    pub glTexImage2DMultisample: *mut c_void,
    pub glTexImage3D: *mut c_void,
    pub glTexImage3DMultisample: *mut c_void,
    pub glTexParameterIiv: *mut c_void,
    pub glTexParameterIuiv: *mut c_void,
    pub glTexParameterf: *mut c_void,
    pub glTexParameterfv: *mut c_void,
    pub glTexParameteri: *mut c_void,
    pub glTexParameteriv: *mut c_void,
    pub glTexStorage1D: *mut c_void,
    pub glTexStorage2D: *mut c_void,
    pub glTexStorage3D: *mut c_void,
    pub glTexSubImage1D: *mut c_void,
    pub glTexSubImage2D: *mut c_void,
    pub glTexSubImage3D: *mut c_void,
    pub glTextureRangeAPPLE: *mut c_void,
    pub glTransformFeedbackVaryings: *mut c_void,
    pub glTranslated: *mut c_void,
    pub glTranslatef: *mut c_void,
    pub glUniform1f: *mut c_void,
    pub glUniform1fv: *mut c_void,
    pub glUniform1i: *mut c_void,
    pub glUniform1iv: *mut c_void,
    pub glUniform1ui: *mut c_void,
    pub glUniform1uiv: *mut c_void,
    pub glUniform2f: *mut c_void,
    pub glUniform2fv: *mut c_void,
    pub glUniform2i: *mut c_void,
    pub glUniform2iv: *mut c_void,
    pub glUniform2ui: *mut c_void,
    pub glUniform2uiv: *mut c_void,
    pub glUniform3f: *mut c_void,
    pub glUniform3fv: *mut c_void,
    pub glUniform3i: *mut c_void,
    pub glUniform3iv: *mut c_void,
    pub glUniform3ui: *mut c_void,
    pub glUniform3uiv: *mut c_void,
    pub glUniform4f: *mut c_void,
    pub glUniform4fv: *mut c_void,
    pub glUniform4i: *mut c_void,
    pub glUniform4iv: *mut c_void,
    pub glUniform4ui: *mut c_void,
    pub glUniform4uiv: *mut c_void,
    pub glUniformBlockBinding: *mut c_void,
    pub glUniformMatrix2fv: *mut c_void,
    pub glUniformMatrix2x3fv: *mut c_void,
    pub glUniformMatrix2x4fv: *mut c_void,
    pub glUniformMatrix3fv: *mut c_void,
    pub glUniformMatrix3x2fv: *mut c_void,
    pub glUniformMatrix3x4fv: *mut c_void,
    pub glUniformMatrix4fv: *mut c_void,
    pub glUniformMatrix4x2fv: *mut c_void,
    pub glUniformMatrix4x3fv: *mut c_void,
    pub glUnmapBuffer: *mut c_void,
    pub glUseProgram: *mut c_void,
    pub glValidateProgram: *mut c_void,
    pub glVertex2d: *mut c_void,
    pub glVertex2dv: *mut c_void,
    pub glVertex2f: *mut c_void,
    pub glVertex2fv: *mut c_void,
    pub glVertex2i: *mut c_void,
    pub glVertex2iv: *mut c_void,
    pub glVertex2s: *mut c_void,
    pub glVertex2sv: *mut c_void,
    pub glVertex3d: *mut c_void,
    pub glVertex3dv: *mut c_void,
    pub glVertex3f: *mut c_void,
    pub glVertex3fv: *mut c_void,
    pub glVertex3i: *mut c_void,
    pub glVertex3iv: *mut c_void,
    pub glVertex3s: *mut c_void,
    pub glVertex3sv: *mut c_void,
    pub glVertex4d: *mut c_void,
    pub glVertex4dv: *mut c_void,
    pub glVertex4f: *mut c_void,
    pub glVertex4fv: *mut c_void,
    pub glVertex4i: *mut c_void,
    pub glVertex4iv: *mut c_void,
    pub glVertex4s: *mut c_void,
    pub glVertex4sv: *mut c_void,
    pub glVertexAttrib1d: *mut c_void,
    pub glVertexAttrib1dv: *mut c_void,
    pub glVertexAttrib1f: *mut c_void,
    pub glVertexAttrib1fv: *mut c_void,
    pub glVertexAttrib1s: *mut c_void,
    pub glVertexAttrib1sv: *mut c_void,
    pub glVertexAttrib2d: *mut c_void,
    pub glVertexAttrib2dv: *mut c_void,
    pub glVertexAttrib2f: *mut c_void,
    pub glVertexAttrib2fv: *mut c_void,
    pub glVertexAttrib2s: *mut c_void,
    pub glVertexAttrib2sv: *mut c_void,
    pub glVertexAttrib3d: *mut c_void,
    pub glVertexAttrib3dv: *mut c_void,
    pub glVertexAttrib3f: *mut c_void,
    pub glVertexAttrib3fv: *mut c_void,
    pub glVertexAttrib3s: *mut c_void,
    pub glVertexAttrib3sv: *mut c_void,
    pub glVertexAttrib4Nbv: *mut c_void,
    pub glVertexAttrib4Niv: *mut c_void,
    pub glVertexAttrib4Nsv: *mut c_void,
    pub glVertexAttrib4Nub: *mut c_void,
    pub glVertexAttrib4Nubv: *mut c_void,
    pub glVertexAttrib4Nuiv: *mut c_void,
    pub glVertexAttrib4Nusv: *mut c_void,
    pub glVertexAttrib4bv: *mut c_void,
    pub glVertexAttrib4d: *mut c_void,
    pub glVertexAttrib4dv: *mut c_void,
    pub glVertexAttrib4f: *mut c_void,
    pub glVertexAttrib4fv: *mut c_void,
    pub glVertexAttrib4iv: *mut c_void,
    pub glVertexAttrib4s: *mut c_void,
    pub glVertexAttrib4sv: *mut c_void,
    pub glVertexAttrib4ubv: *mut c_void,
    pub glVertexAttrib4uiv: *mut c_void,
    pub glVertexAttrib4usv: *mut c_void,
    pub glVertexAttribDivisor: *mut c_void,
    pub glVertexAttribI1i: *mut c_void,
    pub glVertexAttribI1iv: *mut c_void,
    pub glVertexAttribI1ui: *mut c_void,
    pub glVertexAttribI1uiv: *mut c_void,
    pub glVertexAttribI2i: *mut c_void,
    pub glVertexAttribI2iv: *mut c_void,
    pub glVertexAttribI2ui: *mut c_void,
    pub glVertexAttribI2uiv: *mut c_void,
    pub glVertexAttribI3i: *mut c_void,
    pub glVertexAttribI3iv: *mut c_void,
    pub glVertexAttribI3ui: *mut c_void,
    pub glVertexAttribI3uiv: *mut c_void,
    pub glVertexAttribI4bv: *mut c_void,
    pub glVertexAttribI4i: *mut c_void,
    pub glVertexAttribI4iv: *mut c_void,
    pub glVertexAttribI4sv: *mut c_void,
    pub glVertexAttribI4ubv: *mut c_void,
    pub glVertexAttribI4ui: *mut c_void,
    pub glVertexAttribI4uiv: *mut c_void,
    pub glVertexAttribI4usv: *mut c_void,
    pub glVertexAttribIPointer: *mut c_void,
    pub glVertexAttribP1ui: *mut c_void,
    pub glVertexAttribP1uiv: *mut c_void,
    pub glVertexAttribP2ui: *mut c_void,
    pub glVertexAttribP2uiv: *mut c_void,
    pub glVertexAttribP3ui: *mut c_void,
    pub glVertexAttribP3uiv: *mut c_void,
    pub glVertexAttribP4ui: *mut c_void,
    pub glVertexAttribP4uiv: *mut c_void,
    pub glVertexAttribPointer: *mut c_void,
    pub glVertexP2ui: *mut c_void,
    pub glVertexP2uiv: *mut c_void,
    pub glVertexP3ui: *mut c_void,
    pub glVertexP3uiv: *mut c_void,
    pub glVertexP4ui: *mut c_void,
    pub glVertexP4uiv: *mut c_void,
    pub glVertexPointer: *mut c_void,
    pub glViewport: *mut c_void,
    pub glWaitSync: *mut c_void,
    pub glWindowPos2d: *mut c_void,
    pub glWindowPos2dv: *mut c_void,
    pub glWindowPos2f: *mut c_void,
    pub glWindowPos2fv: *mut c_void,
    pub glWindowPos2i: *mut c_void,
    pub glWindowPos2iv: *mut c_void,
    pub glWindowPos2s: *mut c_void,
    pub glWindowPos2sv: *mut c_void,
    pub glWindowPos3d: *mut c_void,
    pub glWindowPos3dv: *mut c_void,
    pub glWindowPos3f: *mut c_void,
    pub glWindowPos3fv: *mut c_void,
    pub glWindowPos3i: *mut c_void,
    pub glWindowPos3iv: *mut c_void,
    pub glWindowPos3s: *mut c_void,
    pub glWindowPos3sv: *mut c_void,
}

fn encode_ascii(input: &str) -> Vec<i8> {
    input
    .chars()
    .filter(|c| c.is_ascii())
    .map(|c| c as i8)
    .chain(Some(0).into_iter())
    .collect::<Vec<_>>()
}

pub unsafe fn cstr_from_ptr<'a>(ptr: *const c_char) -> &'a str {

    #[inline]
    unsafe fn strlen(mut s: *const c_char) -> usize {
        let mut result = 0;
        while *s != 0 {
            s = s.offset(1);
            result += 1;
        }
        result
    }

    let len = strlen(ptr);
    let ptr = ptr as *const u8; // c_char is always one byte, safe cast
    core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len as usize + 1))
}

macro_rules! impl_gl_context {
    ($($opt:ident)?) => {
        $( $opt )? fn get_type(&self) -> GlType { GlType::Gl }

        $( $opt )? fn buffer_data_untyped(
            &self,
            target: GLenum,
            size: GLsizeiptr,
            data: *const GLvoid,
            usage: GLenum,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glBufferData"); }

            if self.glBufferData == ptr::null_mut() {
                _gl_impl_panic("glBufferData");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLsizeiptr, *const GLvoid, GLenum) = mem::transmute(self.glBufferData);
                (func)(target, size, data, usage)
            }
        }

        $( $opt )? fn buffer_sub_data_untyped(
            &self,
            target: GLenum,
            offset: isize,
            size: GLsizeiptr,
            data: *const GLvoid,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glBufferSubData"); }

            if self.glBufferSubData == ptr::null_mut() {
                _gl_impl_panic("glBufferSubData");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, isize, GLsizeiptr, *const GLvoid) = mem::transmute(self.glBufferSubData);
                (func)(target, offset, size, data)
            }
        }

        $( $opt )? fn map_buffer(&self, target: GLenum, access: GLbitfield) -> *mut GLvoid {

            #[cfg(feature = "debug")] { _gl_print_debug("glMapBuffer"); }

            if self.glMapBuffer == ptr::null_mut() {
                _gl_impl_panic("glMapBuffer");
                return ptr::null_mut();
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLbitfield) -> *mut GLvoid = mem::transmute(self.glMapBuffer);
                (func)(target, access)
            }
        }

        $( $opt )? fn map_buffer_range(
            &self,
            target: GLenum,
            offset: GLintptr,
            length: GLsizeiptr,
            access: GLbitfield,
        ) -> *mut GLvoid {

            #[cfg(feature = "debug")] { _gl_print_debug("glMapBufferRange"); }

            if self.glMapBufferRange == ptr::null_mut() {
                _gl_impl_panic("glMapBufferRange");
                return ptr::null_mut();
            }

            unsafe {
                let func: extern "system" fn( GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut GLvoid = mem::transmute(self.glMapBufferRange);
                (func)(target, offset, length, access)
            }
        }

        $( $opt )? fn unmap_buffer(&self, target: GLenum) -> GLboolean {

            #[cfg(feature = "debug")] { _gl_print_debug("glUnmapBuffer"); }

            if self.glUnmapBuffer == ptr::null_mut() {
                _gl_impl_panic("glUnmapBuffer");
                return 1;
            }
            unsafe {
                let func: extern "system" fn(GLenum) -> GLboolean = mem::transmute(self.glUnmapBuffer);
                (func)(target)
            }
        }

        $( $opt )? fn tex_buffer(&self, target: GLenum, internal_format: GLenum, buffer: GLuint) {

            #[cfg(feature = "debug")] { _gl_print_debug("glTexBuffer"); }

            if self.glTexBuffer == ptr::null_mut() {
                _gl_impl_panic("glTexBuffer");
                return;
            }
            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLuint) = mem::transmute(self.glTexBuffer);
                (func)(target, internal_format, buffer)
            }
        }

        $( $opt )? fn shader_source(&self, shader: GLuint, strings: &[&[u8]]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glShaderSource"); }

            let pointers: Vec<*const u8> = strings.iter().map(|string| (*string).as_ptr()).collect();
            let lengths: Vec<GLint> = strings.iter().map(|string| string.len() as GLint).collect();

            if self.glShaderSource == ptr::null_mut() {
                _gl_impl_panic("glShaderSource");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) = mem::transmute(self.glShaderSource);
                (func)(shader, pointers.len() as GLsizei, pointers.as_ptr() as *const *const GLchar, lengths.as_ptr())
            }

            mem::drop(lengths);
            mem::drop(pointers);
        }

        $( $opt )? fn read_buffer(&self, mode: GLenum) {

            #[cfg(feature = "debug")] { _gl_print_debug("glReadBuffer"); }

            if self.glReadBuffer == ptr::null_mut() {
                _gl_impl_panic("glReadBuffer");
                return;
            }
            unsafe {
                let func: extern "system" fn(GLenum) = mem::transmute(self.glReadBuffer);
                (func)(mode)
            }
        }

        $( $opt )? fn read_pixels_into_buffer(
            &self,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            pixel_type: GLenum,
            dst_buffer: &mut [u8],
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glReadPixels"); }

            if self.glReadPixels == ptr::null_mut() {
                _gl_impl_panic("glReadPixels");
                return;
            }


            #[cfg(feature = "debug")] { _gl_print_debug("glPixelStorei"); }

            if self.glPixelStorei == ptr::null_mut() {
                _gl_impl_panic("glPixelStorei");
                return;
            }

            unsafe {
                let glPixelStorei: extern "system" fn(GLenum, GLint) = mem::transmute(self.glPixelStorei);
                (glPixelStorei)(ffi::PACK_ALIGNMENT, 1);

                let func: extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut u8) = mem::transmute(self.glReadPixels);
                (func)(x, y, width, height, format, pixel_type, dst_buffer.as_mut_ptr())
            }
        }

        $( $opt )? fn read_pixels(
            &self,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            pixel_type: GLenum,
        ) -> Vec<u8> {

            use ffi::*;

            let bit_depth = match pixel_type {

                UNSIGNED_BYTE |
                BYTE |
                UNSIGNED_BYTE_3_3_2 |
                UNSIGNED_BYTE_2_3_3_REV => 1,

                UNSIGNED_SHORT |
                UNSIGNED_SHORT_5_6_5 |
                UNSIGNED_SHORT_5_6_5_REV |
                UNSIGNED_SHORT_4_4_4_4 |
                UNSIGNED_SHORT_4_4_4_4_REV |
                UNSIGNED_SHORT_5_5_5_1 |
                UNSIGNED_SHORT_1_5_5_5_REV |
                SHORT => 2,

                UNSIGNED_INT |
                UNSIGNED_INT_8_8_8_8 |
                UNSIGNED_INT_8_8_8_8_REV |
                UNSIGNED_INT_10_10_10_2 |
                UNSIGNED_INT_2_10_10_10_REV |
                UNSIGNED_INT_24_8 |
                UNSIGNED_INT_10F_11F_11F_REV |
                UNSIGNED_INT_5_9_9_9_REV |
                INT => 4,

                HALF_FLOAT => 2,

                FLOAT |
                FLOAT_32_UNSIGNED_INT_24_8_REV => 4,

                _ => 0,
            };

            let mut v = vec![0;width as usize * height as usize * bit_depth];
            self.read_pixels_into_buffer(x, y, width, height, format, pixel_type, &mut v[..]);
            v
        }

        $( $opt )? unsafe fn read_pixels_into_pbo(
            &self,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            pixel_type: GLenum,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glReadPixels"); }

            if self.glReadPixels == ptr::null_mut() {
                _gl_impl_panic("glReadPixels");
                return;
            }

            let func: extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut u8) = mem::transmute(self.glReadPixels);
            (func)(x, y, width, height, format, pixel_type, ptr::null_mut())
        }

        $( $opt )? fn sample_coverage(&self, value: GLclampf, invert: bool) {

            #[cfg(feature = "debug")] { _gl_print_debug("glSampleCoverage"); }

            if self.glSampleCoverage == ptr::null_mut() {
                _gl_impl_panic("glSampleCoverage");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLclampf, bool) = mem::transmute(self.glSampleCoverage);
                (func)(value, invert)
            }
        }

        $( $opt )? fn polygon_offset(&self, factor: GLfloat, units: GLfloat) {

            #[cfg(feature = "debug")] { _gl_print_debug("glPolygonOffset"); }

            if self.glPolygonOffset == ptr::null_mut() {
                _gl_impl_panic("glPolygonOffset");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLfloat, GLfloat) = mem::transmute(self.glPolygonOffset);
                (func)(factor, units)
            }
        }

        $( $opt )? fn pixel_store_i(&self, name: GLenum, param: GLint) {

            #[cfg(feature = "debug")] { _gl_print_debug("glPixelStorei"); }

            if self.glPixelStorei == ptr::null_mut() {
                _gl_impl_panic("glPixelStorei");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint) = mem::transmute(self.glPixelStorei);
                (func)(name, param)
            }
        }

        $( $opt )? fn gen_buffers(&self, n: GLsizei) -> Vec<GLuint> {

            #[cfg(feature = "debug")] { _gl_print_debug("glGenBuffers"); }

            if self.glGenBuffers == ptr::null_mut() {
                _gl_impl_panic("glGenBuffers");
                return Vec::new();
            }

            let mut v = vec![0;n.max(0) as usize];
            unsafe {
                let func: extern "system" fn(GLsizei, *mut GLuint) = mem::transmute(self.glGenBuffers);
                (func)(n, v.as_mut_ptr());
            }
            v
        }

        $( $opt )? fn gen_renderbuffers(&self, n: GLsizei) -> Vec<GLuint> {

            #[cfg(feature = "debug")] { _gl_print_debug("glGenRenderbuffers"); }

            if self.glGenRenderbuffers == ptr::null_mut() {
                _gl_impl_panic("glGenRenderbuffers");
                return Vec::new();
            }

            let mut v = vec![0;n.max(0) as usize];
            unsafe {
                let func: extern "system" fn(GLsizei, *mut GLuint) = mem::transmute(self.glGenRenderbuffers);
                (func)(n, v.as_mut_ptr());
            }
            v
        }

        $( $opt )? fn gen_framebuffers(&self, n: GLsizei) -> Vec<GLuint> {

            #[cfg(feature = "debug")] { _gl_print_debug("glGenFramebuffers"); }

            if self.glGenFramebuffers == ptr::null_mut() {
                _gl_impl_panic("glGenFramebuffers");
                return Vec::new();
            }

            let mut v = vec![0;n.max(0) as usize];
            unsafe {
                let func: extern "system" fn(GLsizei, *mut GLuint) = mem::transmute(self.glGenFramebuffers);
                (func)(n, v.as_mut_ptr());
            }
            v
        }

        $( $opt )? fn gen_textures(&self, n: GLsizei) -> Vec<GLuint> {

            #[cfg(feature = "debug")] { _gl_print_debug("glGenTextures"); }

            if self.glGenTextures == ptr::null_mut() {
                _gl_impl_panic("glGenTextures");
                return Vec::new();
            }

            let mut v = vec![0;n.max(0) as usize];
            unsafe {
                let func: extern "system" fn(GLsizei, *mut GLuint) = mem::transmute(self.glGenTextures);
                (func)(n, v.as_mut_ptr());
            }
            v
        }

        $( $opt )? fn gen_vertex_arrays(&self, n: GLsizei) -> Vec<GLuint> {

            #[cfg(feature = "debug")] { _gl_print_debug("glGenVertexArrays"); }

            if self.glGenVertexArrays == ptr::null_mut() {
                _gl_impl_panic("glGenVertexArrays");
                return Vec::new();
            }

            let mut v = vec![0;n.max(0) as usize];
            unsafe {
                let func: extern "system" fn(GLsizei, *mut GLuint) = mem::transmute(self.glGenVertexArrays);
                (func)(n, v.as_mut_ptr());
            }
            v
        }

        $( $opt )? fn gen_vertex_arrays_apple(&self, n: GLsizei) -> Vec<GLuint> {

            #[cfg(feature = "debug")] { _gl_print_debug("glGenVertexArraysAPPLE"); }

            if self.glGenVertexArraysAPPLE == ptr::null_mut() {
                _gl_impl_panic("glGenVertexArraysAPPLE");
                return Vec::new();
            }

            let mut v = vec![0;n.max(0) as usize];
            unsafe {
                let func: extern "system" fn(GLsizei, *mut GLuint) = mem::transmute(self.glGenVertexArraysAPPLE);
                (func)(n, v.as_mut_ptr());
            }
            v
        }

        $( $opt )? fn gen_queries(&self, n: GLsizei) -> Vec<GLuint> {

            #[cfg(feature = "debug")] { _gl_print_debug("glGenQueries"); }

            if self.glGenQueries == ptr::null_mut() {
                _gl_impl_panic("glGenQueries");
                return Vec::new();
            }

            let mut v = vec![0;n.max(0) as usize];
            unsafe {
                let func: extern "system" fn(GLsizei, *mut GLuint) = mem::transmute(self.glGenQueries);
                (func)(n, v.as_mut_ptr());
            }
            v
        }

        $( $opt )? fn begin_query(&self, target: GLenum, id: GLuint) {

            #[cfg(feature = "debug")] { _gl_print_debug("glBeginQuery"); }

            if self.glBeginQuery == ptr::null_mut() {
                _gl_impl_panic("glBeginQuery");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint) = mem::transmute(self.glBeginQuery);
                (func)(target, id)
            }
        }

        $( $opt )? fn end_query(&self, target: GLenum) {

            #[cfg(feature = "debug")] { _gl_print_debug("glEndQuery"); }

            if self.glEndQuery == ptr::null_mut() {
                _gl_impl_panic("glEndQuery");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum) = mem::transmute(self.glEndQuery);
                (func)(target)
            }
        }

        $( $opt )? fn query_counter(&self, id: GLuint, target: GLenum) {

            #[cfg(feature = "debug")] { _gl_print_debug("glQueryCounter"); }

            if self.glQueryCounter == ptr::null_mut() {
                _gl_impl_panic("glQueryCounter");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLenum) = mem::transmute(self.glQueryCounter);
                (func)(id, target)
            }
        }

        $( $opt )? fn get_query_object_iv(&self, id: GLuint, pname: GLenum) -> i32 {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetQueryObjectiv"); }

            if self.glGetQueryObjectiv == ptr::null_mut() {
                _gl_impl_panic("glGetQueryObjectiv");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLenum) -> i32 = mem::transmute(self.glGetQueryObjectiv);
                (func)(id, pname)
            }
        }

        $( $opt )? fn get_query_object_uiv(&self, id: GLuint, pname: GLenum) -> u32 {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetQueryObjectuiv"); }

            if self.glGetQueryObjectuiv == ptr::null_mut() {
                _gl_impl_panic("glGetQueryObjectuiv");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLenum) -> u32 = mem::transmute(self.glGetQueryObjectuiv);
                (func)(id, pname)
            }
        }

        $( $opt )? fn get_query_object_i64v(&self, id: GLuint, pname: GLenum) -> i64 {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetQueryObjecti64v"); }

            if self.glGetQueryObjecti64v == ptr::null_mut() {
                _gl_impl_panic("glGetQueryObjecti64v");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLenum) -> i64 = mem::transmute(self.glGetQueryObjecti64v);
                (func)(id, pname)
            }
        }

        $( $opt )? fn get_query_object_ui64v(&self, id: GLuint, pname: GLenum) -> u64 {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetQueryObjectui64v"); }

            if self.glGetQueryObjectui64v == ptr::null_mut() {
                _gl_impl_panic("glGetQueryObjectui64v");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLenum) -> u64 = mem::transmute(self.glGetQueryObjectui64v);
                (func)(id, pname)
            }
        }

        $( $opt )? fn delete_queries(&self, queries: &[GLuint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteQueries"); }

            if self.glDeleteQueries == ptr::null_mut() {
                _gl_impl_panic("glDeleteQueries");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLuint) = mem::transmute(self.glDeleteQueries);
                (func)(queries.len() as GLsizei, queries.as_ptr())
            }
        }

        $( $opt )? fn delete_vertex_arrays(&self, vertex_arrays: &[GLuint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteVertexArrays"); }

            if self.glDeleteVertexArrays == ptr::null_mut() {
                _gl_impl_panic("glDeleteVertexArrays");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLuint) = mem::transmute(self.glDeleteVertexArrays);
                (func)(vertex_arrays.len() as GLsizei, vertex_arrays.as_ptr())
            }
        }

        $( $opt )? fn delete_vertex_arrays_apple(&self, vertex_arrays: &[GLuint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteVertexArraysAPPLE"); }

            if self.glDeleteVertexArraysAPPLE == ptr::null_mut() {
                _gl_impl_panic("glDeleteVertexArraysAPPLE");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLuint) = mem::transmute(self.glDeleteVertexArraysAPPLE);
                (func)(vertex_arrays.len() as GLsizei, vertex_arrays.as_ptr())
            }
        }

        $( $opt )? fn delete_buffers(&self, buffers: &[GLuint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteBuffers"); }

            if self.glDeleteBuffers == ptr::null_mut() {
                _gl_impl_panic("glDeleteBuffers");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLuint) = mem::transmute(self.glDeleteBuffers);
                (func)(buffers.len() as GLsizei, buffers.as_ptr())
            }
        }

        $( $opt )? fn delete_renderbuffers(&self, renderbuffers: &[GLuint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteRenderbuffers"); }

            if self.glDeleteRenderbuffers == ptr::null_mut() {
                _gl_impl_panic("glDeleteRenderbuffers");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLuint) = mem::transmute(self.glDeleteRenderbuffers);
                (func)(renderbuffers.len() as GLsizei, renderbuffers.as_ptr())
            }
        }

        $( $opt )? fn delete_framebuffers(&self, framebuffers: &[GLuint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteFramebuffers"); }

            if self.glDeleteFramebuffers == ptr::null_mut() {
                _gl_impl_panic("glDeleteFramebuffers");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLuint) = mem::transmute(self.glDeleteFramebuffers);
                (func)(framebuffers.len() as GLsizei, framebuffers.as_ptr())
            }
        }

        $( $opt )? fn delete_textures(&self, textures: &[GLuint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteTextures"); }

            if self.glDeleteTextures == ptr::null_mut() {
                _gl_impl_panic("glDeleteTextures");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLuint) = mem::transmute(self.glDeleteTextures);
                (func)(textures.len() as GLsizei, textures.as_ptr())
            }
        }

        $( $opt )? fn framebuffer_renderbuffer(
            &self,
            target: GLenum,
            attachment: GLenum,
            renderbuffertarget: GLenum,
            renderbuffer: GLuint,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glFramebufferRenderbuffer"); }

            if self.glFramebufferRenderbuffer == ptr::null_mut() {
                _gl_impl_panic("glFramebufferRenderbuffer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLenum, GLuint) = mem::transmute(self.glFramebufferRenderbuffer);
                (func)( target, attachment, renderbuffertarget, renderbuffer)
            }
        }

        $( $opt )? fn renderbuffer_storage(
            &self,
            target: GLenum,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glRenderbufferStorage"); }

            if self.glRenderbufferStorage == ptr::null_mut() {
                _gl_impl_panic("glRenderbufferStorage");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLsizei, GLsizei) = mem::transmute(self.glRenderbufferStorage);
                (func)(target, internalformat, width, height)
            }
        }

        $( $opt )? fn depth_func(&self, func: GLenum) {

            #[cfg(feature = "debug")] { _gl_print_debug("glDepthFunc"); }

            if self.glDepthFunc == ptr::null_mut() {
                _gl_impl_panic("glDepthFunc");
                return;
            }

            unsafe {
                let glDepthFunc: extern "system" fn(GLenum) = mem::transmute(self.glDepthFunc);
                (glDepthFunc)(func)
            }
        }

        $( $opt )? fn active_texture(&self, texture: GLenum) {

            #[cfg(feature = "debug")] { _gl_print_debug("glActiveTexture"); }

            if self.glActiveTexture == ptr::null_mut() {
                _gl_impl_panic("glActiveTexture");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum) = mem::transmute(self.glActiveTexture);
                (func)(texture)
            }
        }

        $( $opt )? fn attach_shader(&self, program: GLuint, shader: GLuint) {

            #[cfg(feature = "debug")] { _gl_print_debug("glAttachShader"); }

            if self.glAttachShader == ptr::null_mut() {
                _gl_impl_panic("glAttachShader");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLuint) = mem::transmute(self.glAttachShader);
                (func)(program, shader)
            }
        }

        $( $opt )? fn bind_attrib_location(&self, program: GLuint, index: GLuint, name: &str) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBindAttribLocation"); }

            if self.glBindAttribLocation == ptr::null_mut() {
                _gl_impl_panic("glBindAttribLocation");
                return;
            }

            let cstr = encode_ascii(name);

            unsafe {
                let func: extern "system" fn(GLuint, GLuint, *const c_char) = mem::transmute(self.glBindAttribLocation);
                (func)(program, index, cstr.as_ptr())
            }
        }

        $( $opt )? unsafe fn get_uniform_iv(&self, program: GLuint, location: GLint, result: &mut [GLint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetUniformiv"); }

            if self.glGetUniformiv == ptr::null_mut() {
                _gl_impl_panic("glGetUniformiv");
                return;
            }

            let func: extern "system" fn(GLuint, GLint, *mut GLint) = mem::transmute(self.glGetUniformiv);
            (func)(program, location, result.as_mut_ptr())
        }

        $( $opt )? unsafe fn get_uniform_fv(&self, program: GLuint, location: GLint, result: &mut [GLfloat]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetUniformfv"); }

            if self.glGetUniformfv == ptr::null_mut() {
                _gl_impl_panic("glGetUniformfv");
                return;
            }

            let func: extern "system" fn(GLuint, GLint, *mut GLfloat) = mem::transmute(self.glGetUniformfv);
            (func)(program, location, result.as_mut_ptr())
        }

        $( $opt )? fn get_uniform_block_index(&self, program: GLuint, name: &str) -> GLuint {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetUniformBlockIndex"); }

            if self.glGetUniformBlockIndex == ptr::null_mut() {
                _gl_impl_panic("glGetUniformBlockIndex");
                return 0;
            }

            let cstr = encode_ascii(name);

            unsafe {
                let func: extern "system" fn(GLuint, *const c_char) -> GLuint = mem::transmute(self.glGetUniformBlockIndex);
                (func)(program, cstr.as_ptr())
            }
        }

        // ---------------------------------------------------------------------------------------------------------------------------
        // ---------------------------------------------------------------------------------------------------------------------------
        // ---------------------------------------------------------------------------------------------------------------------------
        // ---------------------------------------------------------------------------------------------------------------------------

        $( $opt )? fn get_uniform_indices(&self, program: GLuint, names: &[&str]) -> Vec<GLuint> {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetUniformIndices"); }

            if self.glGetUniformIndices == ptr::null_mut() {
                _gl_impl_panic("glGetUniformIndices");
                return Vec::new();
            }

            let c_strings: Vec<Vec<i8>> = names.iter().map(|n| encode_ascii(*n)).collect();
            let pointers: Vec<*const GLchar> = c_strings.iter().map(|string| string.as_ptr()).collect();
            let mut result = vec![0;c_strings.len()];
            unsafe {
                let func: extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLuint) -> GLuint = mem::transmute(self.glGetUniformIndices);
                (func)(
                    program,
                    pointers.len() as GLsizei,
                    pointers.as_ptr(),
                    result.as_mut_ptr(),
                );
            }
            result
        }

        $( $opt )? fn bind_buffer_base(&self, target: GLenum, index: GLuint, buffer: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBindBufferBase"); }

            if self.glBindBufferBase == ptr::null_mut() {
                _gl_impl_panic("glBindBufferBase");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint, GLuint) = mem::transmute(self.glBindBufferBase);
                (func)(target, index, buffer);
            }
        }

        $( $opt )? fn bind_buffer_range(
            &self,
            target: GLenum,
            index: GLuint,
            buffer: GLuint,
            offset: GLintptr,
            size: GLsizeiptr,
        ) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBindBufferRange"); }

            if self.glBindBufferRange == ptr::null_mut() {
                _gl_impl_panic("glBindBufferRange");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr) = mem::transmute(self.glBindBufferRange);
                (func)(target, index, buffer, offset, size);
            }
        }

        $( $opt )? fn uniform_block_binding(
            &self,
            program: GLuint,
            uniform_block_index: GLuint,
            uniform_block_binding: GLuint,
        ) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniformBlockBinding"); }

            if self.glUniformBlockBinding == ptr::null_mut() {
                _gl_impl_panic("glUniformBlockBinding");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLuint, GLuint) = mem::transmute(self.glUniformBlockBinding);
                (func)(program, uniform_block_index, uniform_block_binding);
            }
        }

        $( $opt )? fn bind_buffer(&self, target: GLenum, buffer: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBindBuffer"); }

            if self.glBindBuffer == ptr::null_mut() {
                _gl_impl_panic("glBindBuffer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint) = mem::transmute(self.glBindBuffer);
                (func)(target, buffer);
            }
        }

        $( $opt )? fn bind_vertex_array(&self, vao: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBindVertexArray"); }

            if self.glBindVertexArray == ptr::null_mut() {
                _gl_impl_panic("glBindVertexArray");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glBindVertexArray);
                (func)(vao);
            }
        }

        $( $opt )? fn bind_vertex_array_apple(&self, vao: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBindVertexArrayAPPLE"); }

            if self.glBindVertexArrayAPPLE == ptr::null_mut() {
                _gl_impl_panic("glBindVertexArrayAPPLE");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glBindVertexArrayAPPLE);
                (func)(vao)
            }
        }

        $( $opt )? fn bind_renderbuffer(&self, target: GLenum, renderbuffer: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBindRenderbuffer"); }

            if self.glBindRenderbuffer == ptr::null_mut() {
                _gl_impl_panic("glBindRenderbuffer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint) = mem::transmute(self.glBindRenderbuffer);
                (func)(target, renderbuffer);
            }
        }

        $( $opt )? fn bind_framebuffer(&self, target: GLenum, framebuffer: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBindFramebuffer"); }

            if self.glBindFramebuffer == ptr::null_mut() {
                _gl_impl_panic("glBindFramebuffer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint) = mem::transmute(self.glBindFramebuffer);
                (func)(target, framebuffer);
            }
        }

        $( $opt )? fn bind_texture(&self, target: GLenum, texture: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBindTexture"); }

            if self.glBindTexture == ptr::null_mut() {
                _gl_impl_panic("glBindTexture");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint) = mem::transmute(self.glBindTexture);
                (func)(target, texture);
            }
        }

        $( $opt )? fn draw_buffers(&self, bufs: &[GLenum]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDrawBuffers"); }

            if self.glDrawBuffers == ptr::null_mut() {
                _gl_impl_panic("glDrawBuffers");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLenum) = mem::transmute(self.glDrawBuffers);
                (func)(bufs.len() as GLsizei, bufs.as_ptr());
            }
        }

        // FIXME: Does not verify buffer size -- unsafe!
        $( $opt )? fn tex_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            internal_format: GLint,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
            format: GLenum,
            ty: GLenum,
            opt_data: Option<&[u8]>,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glTexImage2D"); }

            if self.glTexImage2D == ptr::null_mut() {
                _gl_impl_panic("glTexImage2D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const GLvoid) = mem::transmute(self.glTexImage2D);

                match opt_data {
                    Some(data) => {
                        (func)(
                            target,
                            level,
                            internal_format,
                            width,
                            height,
                            border,
                            format,
                            ty,
                            data.as_ptr() as *const GLvoid,
                        );
                    },
                    None => {
                        (func)(
                            target,
                            level,
                            internal_format,
                            width,
                            height,
                            border,
                            format,
                            ty,
                            ptr::null(),
                        );
                    },
                }
            }
        }

        $( $opt )? fn compressed_tex_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            internal_format: GLenum,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
            data: &[u8],
        ) {


            #[cfg(feature = "debug")] { _gl_print_debug("glCompressedTexImage2D"); }

            if self.glCompressedTexImage2D == ptr::null_mut() {
                _gl_impl_panic("glCompressedTexImage2D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const GLvoid) = mem::transmute(self.glCompressedTexImage2D);
                (func)(
                    target,
                    level,
                    internal_format,
                    width,
                    height,
                    border,
                    data.len() as GLsizei,
                    data.as_ptr() as *const GLvoid,
                );
            }
        }

        $( $opt )? fn compressed_tex_sub_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            data: &[u8],
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glCompressedTexSubImage2D"); }

            if self.glCompressedTexSubImage2D == ptr::null_mut() {
                _gl_impl_panic("glCompressedTexSubImage2D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const GLvoid) = mem::transmute(self.glCompressedTexSubImage2D);
                (func)(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    width,
                    height,
                    format,
                    data.len() as GLsizei,
                    data.as_ptr() as *const GLvoid,
                );
            }
        }

        // FIXME: Does not verify buffer size -- unsafe!
        $( $opt )? fn tex_image_3d(
            &self,
            target: GLenum,
            level: GLint,
            internal_format: GLint,
            width: GLsizei,
            height: GLsizei,
            depth: GLsizei,
            border: GLint,
            format: GLenum,
            ty: GLenum,
            opt_data: Option<&[u8]>,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glTexImage3D"); }

            if self.glTexImage3D == ptr::null_mut() {
                _gl_impl_panic("glTexImage3D");
                return;
            }

            unsafe {
                let pdata = match opt_data {
                    Some(data) => mem::transmute(data.as_ptr()),
                    None => ptr::null(),
                };
                let func: extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *const GLvoid) = mem::transmute(self.glTexImage3D);
                (func)(
                    target,
                    level,
                    internal_format,
                    width,
                    height,
                    depth,
                    border,
                    format,
                    ty,
                    pdata,
                );
            }
        }

        $( $opt )? fn copy_tex_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            internal_format: GLenum,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glCopyTexImage2D"); }

            if self.glCopyTexImage2D == ptr::null_mut() {
                _gl_impl_panic("glCopyTexImage2D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum,GLint,GLenum,GLint,GLint,GLsizei,GLsizei,GLint) = mem::transmute(self.glCopyTexImage2D);
                (func)(
                    target,
                    level,
                    internal_format,
                    x,
                    y,
                    width,
                    height,
                    border,
                );
            }
        }

        $( $opt )? fn copy_tex_sub_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glCopyTexSubImage2D"); }

            if self.glCopyTexSubImage2D == ptr::null_mut() {
                _gl_impl_panic("glCopyTexSubImage2D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) = mem::transmute(self.glCopyTexSubImage2D);
                (func)(target, level, xoffset, yoffset, x, y, width, height);
            }
        }

        $( $opt )? fn copy_tex_sub_image_3d(
            &self,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            zoffset: GLint,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glCopyTexSubImage3D"); }

            if self.glCopyTexSubImage3D == ptr::null_mut() {
                _gl_impl_panic("glCopyTexSubImage3D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) = mem::transmute(self.glCopyTexSubImage3D);
                (func)(
                    target, level, xoffset, yoffset, zoffset, x, y, width, height,
                );
            }
        }

        $( $opt )? fn tex_sub_image_2d(
            &self,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            ty: GLenum,
            data: &[u8],
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glTexSubImage2D"); }

            if self.glTexSubImage2D == ptr::null_mut() {
                _gl_impl_panic("glTexSubImage2D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void) = mem::transmute(self.glTexSubImage2D);
                (func)(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    width,
                    height,
                    format,
                    ty,
                    data.as_ptr() as *const c_void,
                );
            }
        }

        $( $opt )? fn tex_sub_image_2d_pbo(
            &self,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            ty: GLenum,
            offset: usize,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glTexSubImage2D"); }

            if self.glTexSubImage2D == ptr::null_mut() {
                _gl_impl_panic("glTexSubImage2D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void) = mem::transmute(self.glTexSubImage2D);
                (func)(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    width,
                    height,
                    format,
                    ty,
                    offset as *const c_void,
                );
            }
        }

        $( $opt )? fn tex_sub_image_3d(
            &self,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            zoffset: GLint,
            width: GLsizei,
            height: GLsizei,
            depth: GLsizei,
            format: GLenum,
            ty: GLenum,
            data: &[u8],
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glTexSubImage3D"); }

            if self.glTexSubImage3D == ptr::null_mut() {
                _gl_impl_panic("glTexSubImage3D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) = mem::transmute(self.glTexSubImage3D);
                (func)(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    zoffset,
                    width,
                    height,
                    depth,
                    format,
                    ty,
                    data.as_ptr() as *const c_void,
                );
            }
        }

        $( $opt )? fn tex_sub_image_3d_pbo(
            &self,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            zoffset: GLint,
            width: GLsizei,
            height: GLsizei,
            depth: GLsizei,
            format: GLenum,
            ty: GLenum,
            offset: usize,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glTexSubImage3D"); }

            if self.glTexSubImage3D == ptr::null_mut() {
                _gl_impl_panic("glTexSubImage3D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) = mem::transmute(self.glTexSubImage3D);
                (func)(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    zoffset,
                    width,
                    height,
                    depth,
                    format,
                    ty,
                    offset as *const c_void,
                );
            }
        }

        $( $opt )? fn tex_storage_2d(
            &self,
            target: GLenum,
            levels: GLint,
            internal_format: GLenum,
            width: GLsizei,
            height: GLsizei,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glTexStorage2D"); }

            if self.glTexStorage2D == ptr::null_mut() {
                _gl_impl_panic("glTexStorage2D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) = mem::transmute(self.glTexStorage2D);
                (func)(target, levels, internal_format, width, height);
            }
        }

        $( $opt )? fn tex_storage_3d(
            &self,
            target: GLenum,
            levels: GLint,
            internal_format: GLenum,
            width: GLsizei,
            height: GLsizei,
            depth: GLsizei,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glTexStorage3D"); }

            if self.glTexStorage3D == ptr::null_mut() {
                _gl_impl_panic("glTexStorage3D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) = mem::transmute(self.glTexStorage3D);
                (func)(target, levels, internal_format, width, height, depth);
            }
        }

        $( $opt )? fn get_tex_image_into_buffer(
            &self,
            target: GLenum,
            level: GLint,
            format: GLenum,
            ty: GLenum,
            output: &mut [u8],
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetTexImage"); }

            if self.glGetTexImage == ptr::null_mut() {
                _gl_impl_panic("glGetTexImage");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut GLvoid) = mem::transmute(self.glGetTexImage);
                (func)(target, level, format, ty, output.as_mut_ptr() as *mut _);
            }
        }

        $( $opt )? unsafe fn copy_image_sub_data(
            &self,
            src_name: GLuint,
            src_target: GLenum,
            src_level: GLint,
            src_x: GLint,
            src_y: GLint,
            src_z: GLint,
            dst_name: GLuint,
            dst_target: GLenum,
            dst_level: GLint,
            dst_x: GLint,
            dst_y: GLint,
            dst_z: GLint,
            src_width: GLsizei,
            src_height: GLsizei,
            src_depth: GLsizei,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glCopyImageSubData"); }

            if self.glCopyImageSubData == ptr::null_mut() {
                _gl_impl_panic("glCopyImageSubData");
                return;
            }

            let func: extern "system" fn(GLuint, GLenum, GLint, GLint, GLint, GLint, GLuint, GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) = mem::transmute(self.glCopyImageSubData);
            (func)(
                src_name, src_target, src_level, src_x, src_y, src_z, dst_name, dst_target, dst_level,
                dst_x, dst_y, dst_z, src_width, src_height, src_depth,
            );
        }

        $( $opt )? fn invalidate_framebuffer(&self, target: GLenum, attachments: &[GLenum]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glInvalidateFramebuffer"); }

            if self.glInvalidateFramebuffer == ptr::null_mut() {
                _gl_impl_panic("glInvalidateFramebuffer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLsizei, *const GLenum) = mem::transmute(self.glInvalidateFramebuffer);
                (func)(
                    target,
                    attachments.len() as GLsizei,
                    attachments.as_ptr(),
                );
            }
        }

        $( $opt )? fn invalidate_sub_framebuffer(
            &self,
            target: GLenum,
            attachments: &[GLenum],
            xoffset: GLint,
            yoffset: GLint,
            width: GLsizei,
            height: GLsizei,
        ) {


            #[cfg(feature = "debug")] { _gl_print_debug("glInvalidateSubFramebuffer"); }

            if self.glInvalidateSubFramebuffer == ptr::null_mut() {
                _gl_impl_panic("glInvalidateSubFramebuffer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei) = mem::transmute(self.glInvalidateSubFramebuffer);
                (func)(
                    target,
                    attachments.len() as GLsizei,
                    attachments.as_ptr(),
                    xoffset,
                    yoffset,
                    width,
                    height,
                );
            }
        }

        $( $opt )? unsafe fn get_integer_v(&self, name: GLenum, result: &mut [GLint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetIntegerv"); }

            if self.glGetIntegerv == ptr::null_mut() {
                _gl_impl_panic("glGetIntegerv");
                return;
            }

            let func: extern "system" fn(GLenum, *mut GLint) = mem::transmute(self.glGetIntegerv);
            (func)(name, result.as_mut_ptr());
        }

        $( $opt )? unsafe fn get_integer_64v(&self, name: GLenum, result: &mut [GLint64]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetInteger64v"); }

            if self.glGetInteger64v == ptr::null_mut() {
                _gl_impl_panic("glGetInteger64v");
                return;
            }
            let func: extern "system" fn(GLenum, *mut GLint64) = mem::transmute(self.glGetInteger64v);
            (func)(name, result.as_mut_ptr());
        }

        $( $opt )? unsafe fn get_integer_iv(&self, name: GLenum, index: GLuint, result: &mut [GLint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetIntegeri_v"); }

            if self.glGetIntegeri_v == ptr::null_mut() {
                _gl_impl_panic("glGetIntegeri_v");
                return;
            }
            let func: extern "system" fn(GLenum, GLuint, *mut GLint) = mem::transmute(self.glGetIntegeri_v);
            (func)(name, index, result.as_mut_ptr());
        }

        #[inline]
        $( $opt )? unsafe fn get_integer_64iv(&self, name: GLenum, index: GLuint, result: &mut [GLint64]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetInteger64i_v"); }

            if self.glGetInteger64i_v == ptr::null_mut() {
                _gl_impl_panic("glGetInteger64i_v");
                return;
            }
            let func: extern "system" fn(GLenum, GLuint, *mut GLint64) = mem::transmute(self.glGetInteger64i_v);
            (func)(name, index, result.as_mut_ptr());
        }

        #[inline]
        $( $opt )? unsafe fn get_boolean_v(&self, name: GLenum, result: &mut [GLboolean]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetBooleanv"); }

            if self.glGetBooleanv == ptr::null_mut() {
                _gl_impl_panic("glGetBooleanv");
                return;
            }
            let func: extern "system" fn(GLenum, *mut GLboolean) = mem::transmute(self.glGetBooleanv);
            (func)(name, result.as_mut_ptr());
        }

        #[inline]
        $( $opt )? unsafe fn get_float_v(&self, name: GLenum, result: &mut [GLfloat]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetFloatv"); }

            if self.glGetFloatv == ptr::null_mut() {
                _gl_impl_panic("glGetFloatv");
                return;
            }
            let func: extern "system" fn(GLenum, *mut GLfloat) = mem::transmute(self.glGetFloatv);
            (func)(name, result.as_mut_ptr());
        }

        $( $opt )? fn get_framebuffer_attachment_parameter_iv(
            &self,
            target: GLenum,
            attachment: GLenum,
            pname: GLenum,
        ) -> GLint {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetFramebufferAttachmentParameteriv"); }

            if self.glGetFramebufferAttachmentParameteriv == ptr::null_mut() {
                _gl_impl_panic("glGetFramebufferAttachmentParameteriv");
                return 0;
            }
            let mut result: GLint = 0;
            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLenum, *mut GLint) = mem::transmute(self.glGetFramebufferAttachmentParameteriv);
                (func)(
                    target,
                    attachment,
                    pname,
                    &mut result,
                );
            }
            result
        }

        $( $opt )? fn get_renderbuffer_parameter_iv(&self, target: GLenum, pname: GLenum) -> GLint {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetRenderbufferParameteriv"); }

            if self.glGetRenderbufferParameteriv == ptr::null_mut() {
                _gl_impl_panic("glGetRenderbufferParameteriv");
                return 0;
            }

            let mut result: GLint = 0;
            unsafe {
                let func: extern "system" fn(GLenum, GLenum, *mut GLint) = mem::transmute(self.glGetRenderbufferParameteriv);
                (func)(target, pname, &mut result);
            }
            result
        }

        $( $opt )? fn get_tex_parameter_iv(&self, target: GLenum, pname: GLenum) -> GLint {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetTexParameteriv"); }

            if self.glGetTexParameteriv == ptr::null_mut() {
                _gl_impl_panic("glGetTexParameteriv");
                return 0;
            }

            let mut result: GLint = 0;
            unsafe {
                let func: extern "system" fn(GLenum, GLenum, *mut GLint) = mem::transmute(self.glGetTexParameteriv);
                (func)(target, pname, &mut result);
            }
            result
        }

        $( $opt )? fn get_tex_parameter_fv(&self, target: GLenum, pname: GLenum) -> GLfloat {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetTexParameterfv"); }

            if self.glGetTexParameterfv == ptr::null_mut() {
                _gl_impl_panic("glGetTexParameterfv");
                return 0.0;
            }

            let mut result: GLfloat = 0.0;
            unsafe {
                let func: extern "system" fn(GLenum, GLenum, *mut GLfloat) = mem::transmute(self.glGetTexParameterfv);
                (func)(target, pname, &mut result);
            }
            result
        }

        $( $opt )? fn tex_parameter_i(&self, target: GLenum, pname: GLenum, param: GLint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glTexParameteri"); }

            if self.glTexParameteri == ptr::null_mut() {
                _gl_impl_panic("glTexParameteri");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLint) = mem::transmute(self.glTexParameteri);
                (func)(target, pname, param);
            }
        }

        $( $opt )? fn tex_parameter_f(&self, target: GLenum, pname: GLenum, param: GLfloat) {


            #[cfg(feature = "debug")] { _gl_print_debug("glTexParameterf"); }

            if self.glTexParameterf == ptr::null_mut() {
                _gl_impl_panic("glTexParameterf");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLfloat) = mem::transmute(self.glTexParameterf);
                (func)(target, pname, param);
            }
        }

        $( $opt )? fn framebuffer_texture_2d(
            &self,
            target: GLenum,
            attachment: GLenum,
            textarget: GLenum,
            texture: GLuint,
            level: GLint,
        ) {


            #[cfg(feature = "debug")] { _gl_print_debug("glFramebufferTexture2D"); }

            if self.glFramebufferTexture2D == ptr::null_mut() {
                _gl_impl_panic("glFramebufferTexture2D");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) = mem::transmute(self.glFramebufferTexture2D);
                (func)(target, attachment, textarget, texture, level);
            }
        }

        $( $opt )? fn framebuffer_texture_layer(
            &self,
            target: GLenum,
            attachment: GLenum,
            texture: GLuint,
            level: GLint,
            layer: GLint,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glFramebufferTextureLayer"); }

            if self.glFramebufferTextureLayer == ptr::null_mut() {
                _gl_impl_panic("glFramebufferTextureLayer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint) = mem::transmute(self.glFramebufferTextureLayer);
                (func)(target, attachment, texture, level, layer);
            }
        }

        $( $opt )? fn blit_framebuffer(
            &self,
            src_x0: GLint,
            src_y0: GLint,
            src_x1: GLint,
            src_y1: GLint,
            dst_x0: GLint,
            dst_y0: GLint,
            dst_x1: GLint,
            dst_y1: GLint,
            mask: GLbitfield,
            filter: GLenum,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glBlitFramebuffer"); }

            if self.glBlitFramebuffer == ptr::null_mut() {
                _gl_impl_panic("glBlitFramebuffer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) = mem::transmute(self.glBlitFramebuffer);
                (func)(
                    src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
                );
            }
        }

        $( $opt )? fn vertex_attrib_4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {

            #[cfg(feature = "debug")] { _gl_print_debug("glVertexAttrib4f"); }

            if self.glVertexAttrib4f == ptr::null_mut() {
                _gl_impl_panic("glVertexAttrib4f");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) = mem::transmute(self.glVertexAttrib4f);
                (func)(index, x, y, z, w)
            }
        }

        $( $opt )? fn vertex_attrib_pointer_f32(
            &self,
            index: GLuint,
            size: GLint,
            normalized: bool,
            stride: GLsizei,
            offset: GLuint,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glVertexAttribPointer"); }

            if self.glVertexAttribPointer == ptr::null_mut() {
                _gl_impl_panic("glVertexAttribPointer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const GLvoid) = mem::transmute(self.glVertexAttribPointer);
                (func)(
                    index,
                    size,
                    ffi::FLOAT,
                    normalized as GLboolean,
                    stride,
                    offset as *const GLvoid,
                )
            }
        }

        $( $opt )? fn vertex_attrib_pointer(
            &self,
            index: GLuint,
            size: GLint,
            type_: GLenum,
            normalized: bool,
            stride: GLsizei,
            offset: GLuint,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glVertexAttribPointer"); }

            if self.glVertexAttribPointer == ptr::null_mut() {
                _gl_impl_panic("glVertexAttribPointer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const GLvoid) = mem::transmute(self.glVertexAttribPointer);
                (func)(
                    index,
                    size,
                    type_,
                    normalized as GLboolean,
                    stride,
                    offset as *const GLvoid,
                )
            }
        }

        $( $opt )? fn vertex_attrib_i_pointer(
            &self,
            index: GLuint,
            size: GLint,
            type_: GLenum,
            stride: GLsizei,
            offset: GLuint,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glVertexAttribIPointer"); }

            if self.glVertexAttribIPointer == ptr::null_mut() {
                _gl_impl_panic("glVertexAttribIPointer");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const GLvoid) = mem::transmute(self.glVertexAttribIPointer);
                (func)(index, size, type_, stride, offset as *const GLvoid)
            }
        }

        $( $opt )? fn vertex_attrib_divisor(&self, index: GLuint, divisor: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glVertexAttribDivisor"); }

            if self.glVertexAttribDivisor == ptr::null_mut() {
                _gl_impl_panic("glVertexAttribDivisor");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLuint) = mem::transmute(self.glVertexAttribDivisor);
                (func)(index, divisor)
            }
        }

        $( $opt )? fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {


            #[cfg(feature = "debug")] { _gl_print_debug("glViewport"); }

            if self.glViewport == ptr::null_mut() {
                _gl_impl_panic("glViewport");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLint, GLsizei, GLsizei) = mem::transmute(self.glViewport);
                (func)(x, y, width, height);
            }
        }

        $( $opt )? fn scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {


            #[cfg(feature = "debug")] { _gl_print_debug("glScissor"); }

            if self.glScissor == ptr::null_mut() {
                _gl_impl_panic("glScissor");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLint, GLsizei, GLsizei) = mem::transmute(self.glScissor);
                (func)(x, y, width, height);
            }
        }

        $( $opt )? fn line_width(&self, width: GLfloat) {


            #[cfg(feature = "debug")] { _gl_print_debug("glLineWidth"); }

            if self.glLineWidth == ptr::null_mut() {
                _gl_impl_panic("glLineWidth");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLfloat) = mem::transmute(self.glLineWidth);
                (func)(width);
            }
        }

        $( $opt )? fn use_program(&self, program: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUseProgram"); }

            if self.glUseProgram == ptr::null_mut() {
                _gl_impl_panic("glUseProgram");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glUseProgram);
                (func)(program);
            }
        }

        $( $opt )? fn validate_program(&self, program: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glValidateProgram"); }

            if self.glValidateProgram == ptr::null_mut() {
                _gl_impl_panic("glValidateProgram");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glValidateProgram);
                (func)(program);
            }
        }

        $( $opt )? fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDrawArrays"); }

            if self.glDrawArrays == ptr::null_mut() {
                _gl_impl_panic("glDrawArrays");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLint, GLsizei) = mem::transmute(self.glDrawArrays);
                return (func)(mode, first, count);
            }
        }

        $( $opt )? fn draw_arrays_instanced(
            &self,
            mode: GLenum,
            first: GLint,
            count: GLsizei,
            primcount: GLsizei,
        ) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDrawArraysInstanced"); }

            if self.glDrawArraysInstanced == ptr::null_mut() {
                _gl_impl_panic("glDrawArraysInstanced");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum,GLint,GLsizei,GLsizei) = mem::transmute(self.glDrawArraysInstanced);
                return (func)(mode, first, count, primcount);
            }
        }

        $( $opt )? fn draw_elements(
            &self,
            mode: GLenum,
            count: GLsizei,
            element_type: GLenum,
            indices_offset: GLuint,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glDrawElements"); }

            if self.glDrawElements == ptr::null_mut() {
                _gl_impl_panic("glDrawElements");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void) = mem::transmute(self.glDrawElements);
                return (func)(
                    mode,
                    count,
                    element_type,
                    indices_offset as *const c_void,
                );
            }
        }

        $( $opt )? fn draw_elements_instanced(
            &self,
            mode: GLenum,
            count: GLsizei,
            element_type: GLenum,
            indices_offset: GLuint,
            primcount: GLsizei,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glDrawElementsInstanced"); }

            if self.glDrawElementsInstanced == ptr::null_mut() {
                _gl_impl_panic("glDrawElementsInstanced");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei) = mem::transmute(self.glDrawElementsInstanced);
                return (func)(
                    mode,
                    count,
                    element_type,
                    indices_offset as *const c_void,
                    primcount,
                );
            }
        }

        $( $opt )? fn blend_color(&self, r: f32, g: f32, b: f32, a: f32) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBlendColor"); }

            if self.glBlendColor == ptr::null_mut() {
                _gl_impl_panic("glBlendColor");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLclampf, GLclampf, GLclampf, GLclampf) = mem::transmute(self.glBlendColor);
                (func)(r, g, b, a);
            }
        }

        $( $opt )? fn blend_func(&self, sfactor: GLenum, dfactor: GLenum) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBlendFunc"); }

            if self.glBlendFunc == ptr::null_mut() {
                _gl_impl_panic("glBlendFunc");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum) = mem::transmute(self.glBlendFunc);
                (func)(sfactor, dfactor);
            }
        }

        $( $opt )? fn blend_func_separate(
            &self,
            src_rgb: GLenum,
            dest_rgb: GLenum,
            src_alpha: GLenum,
            dest_alpha: GLenum,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glBlendFuncSeparate"); }

            if self.glBlendFuncSeparate == ptr::null_mut() {
                _gl_impl_panic("glBlendFuncSeparate");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLenum, GLenum) = mem::transmute(self.glBlendFuncSeparate);
                (func)(src_rgb, dest_rgb, src_alpha, dest_alpha);
            }
        }

        $( $opt )? fn blend_equation(&self, mode: GLenum) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBlendEquation"); }

            if self.glBlendEquation == ptr::null_mut() {
                _gl_impl_panic("glBlendEquation");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum) = mem::transmute(self.glBlendEquation);
                (func)(mode);
            }
        }

        $( $opt )? fn blend_equation_separate(&self, mode_rgb: GLenum, mode_alpha: GLenum) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBlendEquationSeparate"); }

            if self.glBlendEquationSeparate == ptr::null_mut() {
                _gl_impl_panic("glBlendEquationSeparate");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum) = mem::transmute(self.glBlendEquationSeparate);
                (func)(mode_rgb, mode_alpha);
            }
        }

        $( $opt )? fn color_mask(&self, r: bool, g: bool, b: bool, a: bool) {


            #[cfg(feature = "debug")] { _gl_print_debug("glColorMask"); }

            if self.glColorMask == ptr::null_mut() {
                _gl_impl_panic("glColorMask");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) = mem::transmute(self.glColorMask);
                (func)(
                    r as GLboolean,
                    g as GLboolean,
                    b as GLboolean,
                    a as GLboolean,
                );
            }
        }

        $( $opt )? fn cull_face(&self, mode: GLenum) {


            #[cfg(feature = "debug")] { _gl_print_debug("glCullFace"); }

            if self.glCullFace == ptr::null_mut() {
                _gl_impl_panic("glCullFace");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum) = mem::transmute(self.glCullFace);
                (func)(mode);
            }
        }

        $( $opt )? fn front_face(&self, mode: GLenum) {


            #[cfg(feature = "debug")] { _gl_print_debug("glFrontFace"); }

            if self.glFrontFace == ptr::null_mut() {
                _gl_impl_panic("glFrontFace");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum) = mem::transmute(self.glFrontFace);
                (func)(mode);
            }
        }

        $( $opt )? fn enable(&self, cap: GLenum) {


            #[cfg(feature = "debug")] { _gl_print_debug("glEnable"); }

            if self.glEnable == ptr::null_mut() {
                _gl_impl_panic("glEnable");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum) = mem::transmute(self.glEnable);
                (func)(cap);
            }
        }

        $( $opt )? fn disable(&self, cap: GLenum) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDisable"); }

            if self.glDisable == ptr::null_mut() {
                _gl_impl_panic("glDisable");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum) = mem::transmute(self.glDisable);
                (func)(cap);
            }
        }

        $( $opt )? fn hint(&self, param_name: GLenum, param_val: GLenum) {


            #[cfg(feature = "debug")] { _gl_print_debug("glHint"); }

            if self.glHint == ptr::null_mut() {
                _gl_impl_panic("glHint");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum) = mem::transmute(self.glHint);
                (func)(param_name, param_val);
            }
        }

        $( $opt )? fn is_enabled(&self, cap: GLenum) -> GLboolean {


            #[cfg(feature = "debug")] { _gl_print_debug("glIsEnabled"); }

            if self.glIsEnabled == ptr::null_mut() {
                _gl_impl_panic("glIsEnabled");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLenum) -> GLboolean = mem::transmute(self.glIsEnabled);
                (func)(cap)
            }
        }

        $( $opt )? fn is_shader(&self, shader: GLuint) -> GLboolean {


            #[cfg(feature = "debug")] { _gl_print_debug("glIsShader"); }

            if self.glIsShader == ptr::null_mut() {
                _gl_impl_panic("glIsShader");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLuint) -> GLboolean = mem::transmute(self.glIsShader);
                (func)(shader)
            }
        }

        $( $opt )? fn is_texture(&self, texture: GLenum) -> GLboolean {


            #[cfg(feature = "debug")] { _gl_print_debug("glIsTexture"); }

            if self.glIsTexture == ptr::null_mut() {
                _gl_impl_panic("glIsTexture");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLenum) -> GLboolean = mem::transmute(self.glIsTexture);
                (func)(texture)
            }
        }

        $( $opt )? fn is_framebuffer(&self, framebuffer: GLenum) -> GLboolean {


            #[cfg(feature = "debug")] { _gl_print_debug("glIsFramebuffer"); }

            if self.glIsFramebuffer == ptr::null_mut() {
                _gl_impl_panic("glIsFramebuffer");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLenum) -> GLboolean = mem::transmute(self.glIsFramebuffer);
                (func)(framebuffer)
            }
        }

        $( $opt )? fn is_renderbuffer(&self, renderbuffer: GLenum) -> GLboolean {


            #[cfg(feature = "debug")] { _gl_print_debug("glIsRenderbuffer"); }

            if self.glIsRenderbuffer == ptr::null_mut() {
                _gl_impl_panic("glIsRenderbuffer");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLenum) -> GLboolean = mem::transmute(self.glIsRenderbuffer);
                (func)(renderbuffer)
            }
        }

        $( $opt )? fn check_frame_buffer_status(&self, target: GLenum) -> GLenum {


            #[cfg(feature = "debug")] { _gl_print_debug("glCheckFramebufferStatus"); }

            if self.glCheckFramebufferStatus == ptr::null_mut() {
                _gl_impl_panic("glCheckFramebufferStatus");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLenum) -> GLenum = mem::transmute(self.glCheckFramebufferStatus);
                (func)(target)
            }
        }

        $( $opt )? fn enable_vertex_attrib_array(&self, index: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glEnableVertexAttribArray"); }

            if self.glEnableVertexAttribArray == ptr::null_mut() {
                _gl_impl_panic("glEnableVertexAttribArray");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glEnableVertexAttribArray);
                (func)(index);
            }
        }

        $( $opt )? fn disable_vertex_attrib_array(&self, index: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDisableVertexAttribArray"); }

            if self.glDisableVertexAttribArray == ptr::null_mut() {
                _gl_impl_panic("glDisableVertexAttribArray");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glDisableVertexAttribArray);
                (func)(index);
            }
        }

        $( $opt )? fn uniform_1f(&self, location: GLint, v0: GLfloat) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform1f"); }

            if self.glUniform1f == ptr::null_mut() {
                _gl_impl_panic("glUniform1f");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLfloat) = mem::transmute(self.glUniform1f);
                (func)(location, v0);
            }
        }

        $( $opt )? fn uniform_1fv(&self, location: GLint, values: &[f32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform1fv"); }

            if self.glUniform1fv == ptr::null_mut() {
                _gl_impl_panic("glUniform1fv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, *const GLfloat) = mem::transmute(self.glUniform1fv);
                (func)(location, values.len() as GLsizei, values.as_ptr());
            }
        }

        $( $opt )? fn uniform_1i(&self, location: GLint, v0: GLint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform1i"); }

            if self.glUniform1i == ptr::null_mut() {
                _gl_impl_panic("glUniform1i");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLint) = mem::transmute(self.glUniform1i);
                (func)(location, v0);
            }
        }

        $( $opt )? fn uniform_1iv(&self, location: GLint, values: &[i32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform1iv"); }

            if self.glUniform1iv == ptr::null_mut() {
                _gl_impl_panic("glUniform1iv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, *const GLint) = mem::transmute(self.glUniform1iv);
                (func)(location, values.len() as GLsizei, values.as_ptr());
            }
        }

        $( $opt )? fn uniform_1ui(&self, location: GLint, v0: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform1ui"); }

            if self.glUniform1ui == ptr::null_mut() {
                _gl_impl_panic("glUniform1ui");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLuint) = mem::transmute(self.glUniform1ui);
                (func)(location, v0);
            }
        }

        $( $opt )? fn uniform_2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform2f"); }

            if self.glUniform2f == ptr::null_mut() {
                _gl_impl_panic("glUniform2f");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLfloat, GLfloat) = mem::transmute(self.glUniform2f);
                (func)(location, v0, v1);
            }
        }

        $( $opt )? fn uniform_2fv(&self, location: GLint, values: &[f32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform2fv"); }

            if self.glUniform2fv == ptr::null_mut() {
                _gl_impl_panic("glUniform2fv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, *const GLfloat) = mem::transmute(self.glUniform2fv);
                (func)(location, (values.len() / 2) as GLsizei, values.as_ptr());
            }
        }

        $( $opt )? fn uniform_2i(&self, location: GLint, v0: GLint, v1: GLint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform2i"); }

            if self.glUniform2i == ptr::null_mut() {
                _gl_impl_panic("glUniform2i");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLint, GLint) = mem::transmute(self.glUniform2i);
                (func)(location, v0, v1);
            }
        }

        $( $opt )? fn uniform_2iv(&self, location: GLint, values: &[i32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform2iv"); }

            if self.glUniform2iv == ptr::null_mut() {
                _gl_impl_panic("glUniform2iv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, *const GLint) = mem::transmute(self.glUniform2iv);
                (func)(location, (values.len() / 2) as GLsizei, values.as_ptr());
            }
        }

        $( $opt )? fn uniform_2ui(&self, location: GLint, v0: GLuint, v1: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform2ui"); }

            if self.glUniform2ui == ptr::null_mut() {
                _gl_impl_panic("glUniform2ui");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLuint, GLuint) = mem::transmute(self.glUniform2ui);
                (func)(location, v0, v1);
            }
        }

        $( $opt )? fn uniform_3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform3f"); }

            if self.glUniform3f == ptr::null_mut() {
                _gl_impl_panic("glUniform3f");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLfloat, GLfloat, GLfloat) = mem::transmute(self.glUniform3f);
                (func)(location, v0, v1, v2);
            }
        }

        $( $opt )? fn uniform_3fv(&self, location: GLint, values: &[f32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform3fv"); }

            if self.glUniform3fv == ptr::null_mut() {
                _gl_impl_panic("glUniform3fv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, *const GLfloat) = mem::transmute(self.glUniform3fv);
                (func)(location, (values.len() / 3) as GLsizei, values.as_ptr());
            }
        }

        $( $opt )? fn uniform_3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform3i"); }

            if self.glUniform3i == ptr::null_mut() {
                _gl_impl_panic("glUniform3i");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLint, GLint, GLint) = mem::transmute(self.glUniform3i);
                (func)(location, v0, v1, v2);
            }
        }

        $( $opt )? fn uniform_3iv(&self, location: GLint, values: &[i32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform3iv"); }

            if self.glUniform3iv == ptr::null_mut() {
                _gl_impl_panic("glUniform3iv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, *const GLint) = mem::transmute(self.glUniform3iv);
                (func)(location, (values.len() / 3) as GLsizei, values.as_ptr());
            }
        }

        $( $opt )? fn uniform_3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform3ui"); }

            if self.glUniform3ui == ptr::null_mut() {
                _gl_impl_panic("glUniform3ui");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLuint, GLuint, GLuint) = mem::transmute(self.glUniform3ui);
                (func)(location, v0, v1, v2);
            }
        }

        $( $opt )? fn uniform_4f(&self, location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform4f"); }

            if self.glUniform4f == ptr::null_mut() {
                _gl_impl_panic("glUniform4f");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) = mem::transmute(self.glUniform4f);
                (func)(location, x, y, z, w);
            }
        }

        $( $opt )? fn uniform_4i(&self, location: GLint, x: GLint, y: GLint, z: GLint, w: GLint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform4i"); }

            if self.glUniform4i == ptr::null_mut() {
                _gl_impl_panic("glUniform4i");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLint, GLint, GLint, GLint) = mem::transmute(self.glUniform4i);
                (func)(location, x, y, z, w);
            }
        }

        $( $opt )? fn uniform_4iv(&self, location: GLint, values: &[i32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform4iv"); }

            if self.glUniform4iv == ptr::null_mut() {
                _gl_impl_panic("glUniform4iv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, *const GLint) = mem::transmute(self.glUniform4iv);
                (func)(location, (values.len() / 4) as GLsizei, values.as_ptr());
            }
        }

        $( $opt )? fn uniform_4ui(&self, location: GLint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform4ui"); }

            if self.glUniform4ui == ptr::null_mut() {
                _gl_impl_panic("glUniform4ui");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint) = mem::transmute(self.glUniform4ui);
                (func)(location, x, y, z, w);
            }
        }

        $( $opt )? fn uniform_4fv(&self, location: GLint, values: &[f32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniform4fv"); }

            if self.glUniform4fv == ptr::null_mut() {
                _gl_impl_panic("glUniform4fv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, *const GLfloat) = mem::transmute(self.glUniform4fv);
                (func)(location, (values.len() / 4) as GLsizei, values.as_ptr());
            }
        }

        $( $opt )? fn uniform_matrix_2fv(&self, location: GLint, transpose: bool, value: &[f32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniformMatrix2fv"); }

            if self.glUniformMatrix2fv == ptr::null_mut() {
                _gl_impl_panic("glUniformMatrix2fv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = mem::transmute(self.glUniformMatrix2fv);
                (func)(
                    location,
                    (value.len() / 4) as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr(),
                );
            }
        }

        $( $opt )? fn uniform_matrix_3fv(&self, location: GLint, transpose: bool, value: &[f32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniformMatrix3fv"); }

            if self.glUniformMatrix3fv == ptr::null_mut() {
                _gl_impl_panic("glUniformMatrix3fv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = mem::transmute(self.glUniformMatrix3fv);
                (func)(
                    location,
                    (value.len() / 9) as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr(),
                );
            }
        }

        $( $opt )? fn uniform_matrix_4fv(&self, location: GLint, transpose: bool, value: &[f32]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glUniformMatrix4fv"); }

            if self.glUniformMatrix4fv == ptr::null_mut() {
                _gl_impl_panic("glUniformMatrix4fv");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = mem::transmute(self.glUniformMatrix4fv);
                (func)(
                    location,
                    (value.len() / 16) as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr(),
                );
            }
        }

        $( $opt )? fn depth_mask(&self, flag: bool) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDepthMask"); }

            if self.glDepthMask == ptr::null_mut() {
                _gl_impl_panic("glDepthMask");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLboolean) = mem::transmute(self.glDepthMask);
                (func)(flag as GLboolean);
            }
        }

        $( $opt )? fn depth_range(&self, near: f64, far: f64) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDepthRange"); }

            if self.glDepthRange == ptr::null_mut() {
                _gl_impl_panic("glDepthRange");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLclampd, GLclampd) = mem::transmute(self.glDepthRange);
                (func)(near as GLclampd, far as GLclampd);
            }
        }

        $( $opt )? fn get_active_attrib(&self, program: GLuint, index: GLuint) -> (i32, u32, String) {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetActiveAttrib"); }

            if self.glGetActiveAttrib == ptr::null_mut() {
                _gl_impl_panic("glGetActiveAttrib");
                return (0, 0, String::new());
            }

            let mut buf_size = [0];
            unsafe {
                self.get_program_iv(program, ffi::ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut buf_size);
            }
            let mut name = vec![0u8; buf_size[0] as usize];
            let mut length = 0 as GLsizei;
            let mut size = 0 as i32;
            let mut type_ = 0 as u32;
            unsafe {
                let func: extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) = mem::transmute(self.glGetActiveAttrib);
                (func)(
                    program,
                    index,
                    buf_size[0],
                    &mut length,
                    &mut size,
                    &mut type_,
                    name.as_mut_ptr() as *mut GLchar,
                );
            }
            name.truncate(if length > 0 { length as usize } else { 0 });
            (size, type_, String::from_utf8(name).unwrap())
        }

        $( $opt )? fn get_active_uniform(&self, program: GLuint, index: GLuint) -> (i32, u32, String) {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetActiveUniform"); }

            if self.glGetActiveUniform == ptr::null_mut() {
                _gl_impl_panic("glGetActiveUniform");
                return (0, 0, String::new());
            }

            let mut buf_size = [0];
            unsafe {
                self.get_program_iv(program, ffi::ACTIVE_UNIFORM_MAX_LENGTH, &mut buf_size);
            }
            let mut name = vec![0 as u8; buf_size[0] as usize];
            let mut length: GLsizei = 0;
            let mut size: i32 = 0;
            let mut type_: u32 = 0;

            unsafe {
                let func: extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) = mem::transmute(self.glGetActiveUniform);
                (func)(
                    program,
                    index,
                    buf_size[0],
                    &mut length,
                    &mut size,
                    &mut type_,
                    name.as_mut_ptr() as *mut GLchar,
                );
            }

            name.truncate(if length > 0 { length as usize } else { 0 });

            (size, type_, String::from_utf8(name).unwrap())
        }

        $( $opt )? fn get_active_uniforms_iv(
            &self,
            program: GLuint,
            indices: Vec<GLuint>,
            pname: GLenum,
        ) -> Vec<GLint> {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetActiveUniformsiv"); }

            if self.glGetActiveUniformsiv == ptr::null_mut() {
                _gl_impl_panic("glGetActiveUniformsiv");
                return Vec::new();
            }

            let mut result = Vec::with_capacity(indices.len());
            unsafe {
                result.set_len(indices.len());
                let func: extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint) = mem::transmute(self.glGetActiveUniformsiv);
                (func)(
                    program,
                    indices.len() as GLsizei,
                    indices.as_ptr(),
                    pname,
                    result.as_mut_ptr(),
                );
            }
            result
        }

        $( $opt )? fn get_active_uniform_block_i(&self, program: GLuint, index: GLuint, pname: GLenum) -> GLint {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetActiveUniformBlockiv"); }

            if self.glGetActiveUniformBlockiv == ptr::null_mut() {
                _gl_impl_panic("glGetActiveUniformBlockiv");
                return 0;
            }

            let mut result = 0;
            unsafe {
                let func: extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) = mem::transmute(self.glGetActiveUniformBlockiv);
                (func)(program, index, pname, &mut result);
            }
            result
        }

        $( $opt )? fn get_active_uniform_block_iv(
            &self,
            program: GLuint,
            index: GLuint,
            pname: GLenum,
        ) -> Vec<GLint> {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetActiveUniformBlockiv"); }

            if self.glGetActiveUniformBlockiv == ptr::null_mut() {
                _gl_impl_panic("glGetActiveUniformBlockiv");
                return Vec::new();
            }

            let count = self.get_active_uniform_block_i(program, index, ffi::UNIFORM_BLOCK_ACTIVE_UNIFORMS);
            let mut result = Vec::with_capacity(count as usize);
            unsafe {
                result.set_len(count as usize);
                let func: extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) = mem::transmute(self.glGetActiveUniformBlockiv);
                (func)(program, index, pname, result.as_mut_ptr());
            }
            result
        }

        $( $opt )? fn get_active_uniform_block_name(&self, program: GLuint, index: GLuint) -> String {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetActiveUniformBlockName"); }

            if self.glGetActiveUniformBlockName == ptr::null_mut() {
                _gl_impl_panic("glGetActiveUniformBlockName");
                return String::new();
            }

            let buf_size = self.get_active_uniform_block_i(program, index, ffi::UNIFORM_BLOCK_NAME_LENGTH);
            let mut name = vec![0 as u8; buf_size as usize];
            let mut length: GLsizei = 0;
            unsafe {
                let func: extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) = mem::transmute(self.glGetActiveUniformBlockName);
                (func)(
                    program,
                    index,
                    buf_size,
                    &mut length,
                    name.as_mut_ptr() as *mut GLchar,
                );
            }
            name.truncate(if length > 0 { length as usize } else { 0 });

            String::from_utf8(name).unwrap()
        }

        $( $opt )? fn get_attrib_location(&self, program: GLuint, name: &str) -> c_int {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetAttribLocation"); }

            if self.glGetAttribLocation == ptr::null_mut() {
                _gl_impl_panic("glGetAttribLocation");
                return 0;
            }

            let name = encode_ascii(name);
            unsafe {
                let func: extern "system" fn(GLuint, *const GLchar) -> c_int = mem::transmute(self.glGetAttribLocation);
                (func)(program, name.as_ptr())
            }
        }

        $( $opt )? fn get_frag_data_location(&self, program: GLuint, name: &str) -> c_int {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetFragDataLocation"); }

            if self.glGetFragDataLocation == ptr::null_mut() {
                _gl_impl_panic("glGetFragDataLocation");
                return 0;
            }

            let name = encode_ascii(name);
            unsafe {
                let func: extern "system" fn(GLuint, *const c_char)  -> c_int = mem::transmute(self.glGetFragDataLocation);
                (func)(program, name.as_ptr())
            }
        }

        $( $opt )? fn get_uniform_location(&self, program: GLuint, name: &str) -> c_int {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetUniformLocation"); }

            if self.glGetUniformLocation == ptr::null_mut() {
                _gl_impl_panic("glGetUniformLocation");
                return 0;
            }

            let name = encode_ascii(name);
            unsafe {
                let func: extern "system" fn(GLuint, *const GLchar)  -> c_int = mem::transmute(self.glGetUniformLocation);
                (func)(program, name.as_ptr())
            }
        }

        $( $opt )? fn get_program_info_log(&self, program: GLuint) -> String {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetProgramInfoLog"); }

            if self.glGetProgramInfoLog == ptr::null_mut() {
                _gl_impl_panic("glGetProgramInfoLog");
                return String::new();
            }

            let mut max_len = [0];
            unsafe {
                self.get_program_iv(program, ffi::INFO_LOG_LENGTH, &mut max_len);
            }
            if max_len[0] == 0 {
                return String::new();
            }
            let mut result = vec![0u8; max_len[0] as usize];
            let mut result_len = 0 as GLsizei;
            unsafe {
                let func: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) = mem::transmute(self.glGetProgramInfoLog);
                (func)(
                    program,
                    max_len[0] as GLsizei,
                    &mut result_len,
                    result.as_mut_ptr() as *mut GLchar,
                );
            }
            result.truncate(if result_len > 0 {
                result_len as usize
            } else {
                0
            });
            String::from_utf8(result).unwrap()
        }

        #[inline]
        $( $opt )? unsafe fn get_program_iv(&self, program: GLuint, pname: GLenum, result: &mut [GLint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetProgramiv"); }

            if self.glGetProgramiv == ptr::null_mut() {
                _gl_impl_panic("glGetProgramiv");
                return;
            }
            let func: extern "system" fn(GLuint, GLenum, *mut GLint) = mem::transmute(self.glGetProgramiv);
            (func)(program, pname, result.as_mut_ptr());
        }

        $( $opt )? fn get_program_binary(&self, program: GLuint) -> (Vec<u8>, GLenum) {

            const NONE: GLenum = 0;


            #[cfg(feature = "debug")] { _gl_print_debug("glGetProgramBinary"); }

            if self.glGetProgramBinary == ptr::null_mut() {
                _gl_impl_panic("glGetProgramBinary");
                return (Vec::new(), NONE);
            }

            let mut len = [0];
            unsafe {
                self.get_program_iv(program, ffi::PROGRAM_BINARY_LENGTH, &mut len);
            }
            if len[0] <= 0 {
                return (Vec::new(), NONE);
            }
            let mut binary: Vec<u8> = Vec::with_capacity(len[0] as usize);
            let mut format = NONE;
            let mut out_len = 0;
            unsafe {
                binary.set_len(len[0] as usize);
                let func: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut c_void) = mem::transmute(self.glGetProgramBinary);
                (func)(
                    program,
                    len[0],
                    &mut out_len as *mut GLsizei,
                    &mut format,
                    binary.as_mut_ptr() as *mut c_void,
                );
            }
            if len[0] != out_len {
                return (Vec::new(), NONE);
            }

            (binary, format)
        }

        $( $opt )? fn program_binary(&self, program: GLuint, format: GLenum, binary: &[u8]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glProgramBinary"); }

            if self.glProgramBinary == ptr::null_mut() {
                _gl_impl_panic("glProgramBinary");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLenum, *const c_void, GLsizei) = mem::transmute(self.glProgramBinary);
                (func)(
                    program,
                    format,
                    binary.as_ptr() as *const c_void,
                    binary.len() as GLsizei,
                );
            }
        }

        $( $opt )? fn program_parameter_i(&self, program: GLuint, pname: GLenum, value: GLint) {

            #[cfg(feature = "debug")] { _gl_print_debug("glProgramParameteri"); }

            if self.glProgramParameteri == ptr::null_mut() {
                _gl_impl_panic("glProgramParameteri");
                return;
            }
            unsafe {
                let func: extern "system" fn(GLuint, GLenum, GLint) = mem::transmute(self.glProgramParameteri);
                (func)(program, pname, value);
            }
        }

        #[inline]
        $( $opt )? unsafe fn get_vertex_attrib_iv(&self, index: GLuint, pname: GLenum, result: &mut [GLint]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetVertexAttribiv"); }

            if self.glGetVertexAttribiv == ptr::null_mut() {
                _gl_impl_panic("glGetVertexAttribiv");
                return;
            }
            let func: extern "system" fn(GLuint, GLenum, *mut GLint) = mem::transmute(self.glGetVertexAttribiv);
            (func)(index, pname, result.as_mut_ptr());
        }

        #[inline]
        $( $opt )? unsafe fn get_vertex_attrib_fv(&self, index: GLuint, pname: GLenum, result: &mut [GLfloat]) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGetVertexAttribfv"); }

            if self.glGetVertexAttribfv == ptr::null_mut() {
                _gl_impl_panic("glGetVertexAttribfv");
                return;
            }
            let func: extern "system" fn(GLuint, GLenum, *mut GLfloat) = mem::transmute(self.glGetVertexAttribfv);
            (func)(index, pname, result.as_mut_ptr());
        }

        $( $opt )? fn get_vertex_attrib_pointer_v(&self, index: GLuint, pname: GLenum) -> GLsizeiptr {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetVertexAttribPointerv"); }

            if self.glGetVertexAttribPointerv == ptr::null_mut() {
                _gl_impl_panic("glGetVertexAttribPointerv");
                return 0;
            }

            let mut result = 0 as *mut GLvoid;
            unsafe {
                let func: extern "system" fn(GLuint, GLenum, *mut *mut GLvoid) = mem::transmute(self.glGetVertexAttribPointerv);
                (func)(index, pname, &mut result)
            }
            result as GLsizeiptr
        }

        $( $opt )? fn get_buffer_parameter_iv(&self, target: GLuint, pname: GLenum) -> GLint {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetBufferParameteriv"); }

            if self.glGetBufferParameteriv == ptr::null_mut() {
                _gl_impl_panic("glGetBufferParameteriv");
                return 0;
            }

            let mut result = 0 as GLint;
            unsafe {
                let func: extern "system" fn(GLenum, GLenum, *mut GLint) = mem::transmute(self.glGetBufferParameteriv);
                (func)(target, pname, &mut result);
            }
            result
        }

        $( $opt )? fn get_shader_info_log(&self, shader: GLuint) -> String {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetShaderInfoLog"); }

            if self.glGetShaderInfoLog == ptr::null_mut() {
                _gl_impl_panic("glGetShaderInfoLog");
                return String::new();
            }

            let mut max_len = [0];
            unsafe {
                self.get_shader_iv(shader, ffi::INFO_LOG_LENGTH, &mut max_len);
            }
            if max_len[0] == 0 {
                return String::new();
            }
            let mut result = vec![0u8; max_len[0] as usize];
            let mut result_len = 0 as GLsizei;
            unsafe {
                let func: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) = mem::transmute(self.glGetShaderInfoLog);
                (func)(
                    shader,
                    max_len[0] as GLsizei,
                    &mut result_len,
                    result.as_mut_ptr() as *mut GLchar,
                );
            }
            result.truncate(if result_len > 0 {
                result_len as usize
            } else {
                0
            });
            String::from_utf8(result).unwrap()
        }

        $( $opt )? fn get_string(&self, which: GLenum) -> String {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetString"); }

            if self.glGetString == ptr::null_mut() {
                _gl_impl_panic("glGetString");
                return String::new();
            }

            unsafe {
                let func: extern "system" fn(GLenum) -> *mut GLubyte = mem::transmute(self.glGetString);
                let llstr = (func)(which);
                if !llstr.is_null() {
                    cstr_from_ptr(llstr as *const c_char).to_string()
                } else {
                    String::new()
                }
            }
        }

        $( $opt )? fn get_string_i(&self, which: GLenum, index: GLuint) -> String {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetStringi"); }

            if self.glGetStringi == ptr::null_mut() {
                _gl_impl_panic("glGetStringi");
                return String::new();
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint) -> *mut GLubyte = mem::transmute(self.glGetStringi);
                let llstr = (func)(which, index);
                if !llstr.is_null() {
                    cstr_from_ptr(llstr as *const c_char).to_string()
                } else {
                    String::new()
                }
            }
        }

        $( $opt )? unsafe fn get_shader_iv(&self, shader: GLuint, pname: GLenum, result: &mut [GLint]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetShaderiv"); }

            if self.glGetShaderiv == ptr::null_mut() {
                _gl_impl_panic("glGetShaderiv");
                return;
            }

            let func: extern "system" fn(GLuint, GLenum, *mut GLint) = mem::transmute(self.glGetShaderiv);
            (func)(shader, pname, result.as_mut_ptr());
        }

        $( $opt )? fn get_shader_precision_format(
            &self,
            _shader_type: GLuint,
            precision_type: GLuint,
        ) -> (GLint, GLint, GLint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetShaderPrecisionFormat"); }

            // if self.glGetShaderPrecisionFormat == ptr::null_mut() {
            //     _gl_impl_panic("glGetShaderPrecisionFormat");
            //     return (0, 0, 0);
            // }

            // gl.GetShaderPrecisionFormat is not available until OpenGL 4.1.
            // Fallback to OpenGL standard precissions that most desktop hardware support.
            match precision_type {
                ffi::LOW_FLOAT | ffi::MEDIUM_FLOAT | ffi::HIGH_FLOAT => {
                    // Fallback to IEEE 754 single precision
                    // Range: from -2^127 to 2^127
                    // Significand precision: 23 bits
                    (127, 127, 23)
                }
                ffi::LOW_INT | ffi::MEDIUM_INT | ffi::HIGH_INT => {
                    // Fallback to single precision integer
                    // Range: from -2^24 to 2^24
                    // Precision: For integer formats this value is always 0
                    (24, 24, 0)
                }
                _ => (0, 0, 0),
            }
        }

        $( $opt )? fn compile_shader(&self, shader: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glCompileShader"); }

            if self.glCompileShader == ptr::null_mut() {
                _gl_impl_panic("glCompileShader");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glCompileShader);
                (func)(shader);
            }
        }

        $( $opt )? fn create_program(&self) -> GLuint {


            #[cfg(feature = "debug")] { _gl_print_debug("glCreateProgram"); }

            if self.glCreateProgram == ptr::null_mut() {
                _gl_impl_panic("glCreateProgram");
                return 0;
            }

            unsafe {
                let func: extern "system" fn() -> GLuint = mem::transmute(self.glCreateProgram);
                return (func)();
            }
        }

        $( $opt )? fn delete_program(&self, program: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteProgram"); }

            if self.glDeleteProgram == ptr::null_mut() {
                _gl_impl_panic("glDeleteProgram");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glDeleteProgram);
                (func)(program);
            }
        }

        $( $opt )? fn create_shader(&self, shader_type: GLenum) -> GLuint {


            #[cfg(feature = "debug")] { _gl_print_debug("glCreateShader"); }

            if self.glCreateShader == ptr::null_mut() {
                _gl_impl_panic("glCreateShader");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLenum) -> GLuint = mem::transmute(self.glCreateShader);
                return (func)(shader_type);
            }
        }

        $( $opt )? fn delete_shader(&self, shader: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteShader"); }

            if self.glDeleteShader == ptr::null_mut() {
                _gl_impl_panic("glDeleteShader");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glDeleteShader);
                (func)(shader);
            }
        }

        $( $opt )? fn detach_shader(&self, program: GLuint, shader: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDetachShader"); }

            if self.glDetachShader == ptr::null_mut() {
                _gl_impl_panic("glDetachShader");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint, GLuint) = mem::transmute(self.glDetachShader);
                (func)(program, shader);
            }
        }

        $( $opt )? fn link_program(&self, program: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glLinkProgram"); }

            if self.glLinkProgram == ptr::null_mut() {
                _gl_impl_panic("glLinkProgram");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glLinkProgram);
                return (func)(program);
            }
        }

        $( $opt )? fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {


            #[cfg(feature = "debug")] { _gl_print_debug("glClearColor"); }

            if self.glClearColor == ptr::null_mut() {
                _gl_impl_panic("glClearColor");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLclampf, GLclampf, GLclampf, GLclampf) = mem::transmute(self.glClearColor);
                (func)(r, g, b, a);
            }
        }

        $( $opt )? fn clear(&self, buffer_mask: GLbitfield) {


            #[cfg(feature = "debug")] { _gl_print_debug("glClear"); }

            if self.glClear == ptr::null_mut() {
                _gl_impl_panic("glClear");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLbitfield) = mem::transmute(self.glClear);
                (func)(buffer_mask);
            }
        }

        $( $opt )? fn clear_depth(&self, depth: f64) {


            #[cfg(feature = "debug")] { _gl_print_debug("glClearDepth"); }

            if self.glClearDepth == ptr::null_mut() {
                _gl_impl_panic("glClearDepth");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLclampd) = mem::transmute(self.glClearDepth);
                (func)(depth as GLclampd);
            }
        }

        $( $opt )? fn clear_stencil(&self, s: GLint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glClearStencil"); }

            if self.glClearStencil == ptr::null_mut() {
                _gl_impl_panic("glClearStencil");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLint) = mem::transmute(self.glClearStencil);
                (func)(s);
            }
        }

        $( $opt )? fn flush(&self) {


            #[cfg(feature = "debug")] { _gl_print_debug("glFlush"); }

            if self.glFlush == ptr::null_mut() {
                _gl_impl_panic("glFlush");
                return;
            }

            unsafe {
                let func: extern "system" fn() = mem::transmute(self.glFlush);
                (func)();
            }
        }

        $( $opt )? fn finish(&self) {


            #[cfg(feature = "debug")] { _gl_print_debug("glFinish"); }

            if self.glFinish == ptr::null_mut() {
                _gl_impl_panic("glFinish");
                return;
            }

            unsafe {
                let func: extern "system" fn() = mem::transmute(self.glFinish);
                (func)();
            }
        }

        $( $opt )? fn get_error(&self) -> GLenum {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetError"); }

            if self.glGetError == ptr::null_mut() {
                _gl_impl_panic("glGetError");
                return 0;
            }

            unsafe {
                let func: extern "system" fn() -> GLenum = mem::transmute(self.glGetError);
                (func)()
            }
        }

        $( $opt )? fn stencil_mask(&self, mask: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glStencilMask"); }

            if self.glStencilMask == ptr::null_mut() {
                _gl_impl_panic("glStencilMask");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glStencilMask);
                (func)(mask)
            }
        }

        $( $opt )? fn stencil_mask_separate(&self, face: GLenum, mask: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glStencilMaskSeparate"); }

            if self.glStencilMaskSeparate == ptr::null_mut() {
                _gl_impl_panic("glStencilMaskSeparate");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint) = mem::transmute(self.glStencilMaskSeparate);
                (func)(face, mask)
            }
        }

        $( $opt )? fn stencil_func(&self, func: GLenum, ref_: GLint, mask: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glStencilFunc"); }

            if self.glStencilFunc == ptr::null_mut() {
                _gl_impl_panic("glStencilFunc");
                return;
            }

            unsafe {
                let glStencilFunc: extern "system" fn(GLenum, GLint, GLuint) = mem::transmute(self.glStencilFunc);
                (glStencilFunc)(func, ref_, mask)
            }
        }

        $( $opt )? fn stencil_func_separate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glStencilFuncSeparate"); }

            if self.glStencilFuncSeparate == ptr::null_mut() {
                _gl_impl_panic("glStencilFuncSeparate");
                return;
            }

            unsafe {
                let glStencilFuncSeparate: extern "system" fn(GLenum, GLenum, GLint, GLuint) = mem::transmute(self.glStencilFuncSeparate);
                (glStencilFuncSeparate)(face, func, ref_, mask)
            }
        }

        $( $opt )? fn stencil_op(&self, sfail: GLenum, dpfail: GLenum, dppass: GLenum) {


            #[cfg(feature = "debug")] { _gl_print_debug("glStencilOp"); }

            if self.glStencilOp == ptr::null_mut() {
                _gl_impl_panic("glStencilOp");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLenum) = mem::transmute(self.glStencilOp);
                (func)(sfail, dpfail, dppass)
            }
        }

        $( $opt )? fn stencil_op_separate(&self, face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) {


            #[cfg(feature = "debug")] { _gl_print_debug("glStencilOpSeparate"); }

            if self.glStencilOpSeparate == ptr::null_mut() {
                _gl_impl_panic("glStencilOpSeparate");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLenum, GLenum, GLenum) = mem::transmute(self.glStencilOpSeparate);
                (func)(face, sfail, dpfail, dppass)
            }
        }

        #[allow(unused_variables)]
        $( $opt )? fn egl_image_target_texture2d_oes(&self, target: GLenum, image: GLeglImageOES) {

            #[cfg(feature = "debug")] { _gl_print_debug("glEglImageTargetTexture2dOes"); }

            return; // not supported

            // if self.glEglImageTargetTexture2dOes == ptr::null_mut() {
            //     _gl_impl_panic("glEglImageTargetTexture2dOes");
            //     return;
            // }
        }

        #[allow(unused_variables)]
        $( $opt )? fn egl_image_target_renderbuffer_storage_oes(&self, target: GLenum, image: GLeglImageOES) {

            #[cfg(feature = "debug")] { _gl_print_debug("glEglImageTargetRenderbufferStorageOes"); }

            return; // not supported

            // if self.glEglImageTargetRenderbufferStorageOes == ptr::null_mut() {
            //     _gl_impl_panic("glEglImageTargetRenderbufferStorageOes");
            //     return;
            // }
        }

        $( $opt )? fn generate_mipmap(&self, target: GLenum) {

            #[cfg(feature = "debug")] { _gl_print_debug("glGenerateMipmap"); }

            if self.glGenerateMipmap == ptr::null_mut() {
                _gl_impl_panic("glGenerateMipmap");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum) = mem::transmute(self.glGenerateMipmap);
                (func)(target)
            }
        }

        $( $opt )? fn insert_event_marker_ext(&self, message: &str) {


            #[cfg(feature = "debug")] { _gl_print_debug("glInsertEventMarkerEXT"); }

            if self.glInsertEventMarkerEXT == ptr::null_mut() {
                _gl_impl_panic("glInsertEventMarkerEXT");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLchar) = mem::transmute(self.glInsertEventMarkerEXT);
                (func)(message.len() as GLsizei, message.as_ptr() as *const _);
            }
        }

        $( $opt )? fn push_group_marker_ext(&self, message: &str) {


            #[cfg(feature = "debug")] { _gl_print_debug("glPushGroupMarkerEXT"); }

            if self.glPushGroupMarkerEXT == ptr::null_mut() {
                _gl_impl_panic("glPushGroupMarkerEXT");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLchar) = mem::transmute(self.glPushGroupMarkerEXT);
                (func)(message.len() as GLsizei, message.as_ptr() as *const _);
            }
        }

        $( $opt )? fn pop_group_marker_ext(&self) {


            #[cfg(feature = "debug")] { _gl_print_debug("glPopGroupMarkerEXT"); }

            if self.glPopGroupMarkerEXT == ptr::null_mut() {
                _gl_impl_panic("glPopGroupMarkerEXT");
                return;
            }

            unsafe {
                let func: extern "system" fn() = mem::transmute(self.glPopGroupMarkerEXT);
                (func)();
            }
        }

        $( $opt )? fn debug_message_insert_khr(&self, source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, message: &str) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDebugMessageInsertKHR"); }

            if self.glDebugMessageInsertKHR == ptr::null_mut() {
                _gl_impl_panic("glDebugMessageInsertKHR");
                return;
            }

            unsafe {
                // TODO: correct?
                let func: extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const c_char) = mem::transmute(self.glDebugMessageInsertKHR);
                (func)(source, type_, id, severity, message.len() as GLsizei, message.as_ptr() as *const _);
            }
        }

        $( $opt )? fn push_debug_group_khr(&self, source: GLenum, id: GLuint, message: &str) {


            #[cfg(feature = "debug")] { _gl_print_debug("glPushDebugGroupKHR"); }

            if self.glPushDebugGroupKHR == ptr::null_mut() {
                _gl_impl_panic("glPushDebugGroupKHR");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint, GLsizei, *const c_char) = mem::transmute(self.glPushDebugGroupKHR);
                (func)(source, id, message.len() as GLsizei, message.as_ptr() as *const _);
            }
        }

        $( $opt )? fn pop_debug_group_khr(&self) {


            #[cfg(feature = "debug")] { _gl_print_debug("glPopDebugGroupKHR"); }

            if self.glPopDebugGroupKHR == ptr::null_mut() {
                _gl_impl_panic("glPopDebugGroupKHR");
                return;
            }

            unsafe {
                let func: extern "system" fn() = mem::transmute(self.glPopDebugGroupKHR);
                (func)();
            }
        }

        $( $opt )? fn fence_sync(&self, condition: GLenum, flags: GLbitfield) -> GLsync {


            #[cfg(feature = "debug")] { _gl_print_debug("glFenceSync"); }

            if self.glFenceSync == ptr::null_mut() {
                _gl_impl_panic("glFenceSync");
                return ptr::null_mut();
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLbitfield) -> GLsync = mem::transmute(self.glFenceSync);
                (func)(condition, flags) as *const _
            }
        }

        $( $opt )? fn client_wait_sync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {


            #[cfg(feature = "debug")] { _gl_print_debug("glClientWaitSync"); }

            if self.glClientWaitSync == ptr::null_mut() {
                _gl_impl_panic("glClientWaitSync");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum = mem::transmute(self.glClientWaitSync);
                (func)(sync as *const _, flags, timeout)
            }
        }

        $( $opt )? fn wait_sync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) {


            #[cfg(feature = "debug")] { _gl_print_debug("glWaitSync"); }

            if self.glWaitSync == ptr::null_mut() {
                _gl_impl_panic("glWaitSync");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsync, GLbitfield, GLuint64) = mem::transmute(self.glWaitSync);
                (func)(sync as *const _, flags, timeout);
            }
        }

        $( $opt )? fn texture_range_apple(&self, target: GLenum, data: &[u8]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glTextureRangeAPPLE"); }

            if self.glTextureRangeAPPLE == ptr::null_mut() {
                _gl_impl_panic("glTextureRangeAPPLE");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLsizei, *const c_void) = mem::transmute(self.glTextureRangeAPPLE);
                (func)(
                    target,
                    data.len() as GLsizei,
                    data.as_ptr() as *const c_void,
                );
            }
        }

        $( $opt )? fn delete_sync(&self, sync: GLsync) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteSync"); }

            if self.glDeleteSync == ptr::null_mut() {
                _gl_impl_panic("glDeleteSync");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsync) = mem::transmute(self.glDeleteSync);
                (func)(sync as *const _);
            }
        }

        $( $opt )? fn gen_fences_apple(&self, n: GLsizei) -> Vec<GLuint> {

            let mut result = vec![0 as GLuint; n as usize];


            #[cfg(feature = "debug")] { _gl_print_debug("glGenFencesAPPLE"); }

            if self.glGenFencesAPPLE == ptr::null_mut() {
                _gl_impl_panic("glGenFencesAPPLE");
                return result;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *mut GLuint) = mem::transmute(self.glGenFencesAPPLE);
                (func)(n, result.as_mut_ptr());
            }
            result
        }

        $( $opt )? fn delete_fences_apple(&self, fences: &[GLuint]) {


            #[cfg(feature = "debug")] { _gl_print_debug("glDeleteFencesAPPLE"); }

            if self.glDeleteFencesAPPLE == ptr::null_mut() {
                _gl_impl_panic("glDeleteFencesAPPLE");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLsizei, *const GLuint) = mem::transmute(self.glDeleteFencesAPPLE);
                (func)(fences.len() as GLsizei, fences.as_ptr());
            }
        }

        $( $opt )? fn set_fence_apple(&self, fence: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glSetFenceAPPLE"); }

            if self.glSetFenceAPPLE == ptr::null_mut() {
                _gl_impl_panic("glSetFenceAPPLE");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glSetFenceAPPLE);
                (func)(fence);
            }
        }

        $( $opt )? fn finish_fence_apple(&self, fence: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glFinishFenceAPPLE"); }

            if self.glFinishFenceAPPLE == ptr::null_mut() {
                _gl_impl_panic("glFinishFenceAPPLE");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glFinishFenceAPPLE);
                (func)(fence);
            }
        }

        $( $opt )? fn test_fence_apple(&self, fence: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glTestFenceAPPLE"); }

            if self.glTestFenceAPPLE == ptr::null_mut() {
                _gl_impl_panic("glTestFenceAPPLE");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLuint) = mem::transmute(self.glTestFenceAPPLE);
                (func)(fence);
            }
        }

        $( $opt )? fn test_object_apple(&self, object: GLenum, name: GLuint) -> GLboolean {


            #[cfg(feature = "debug")] { _gl_print_debug("glTestObjectAPPLE"); }

            if self.glTestObjectAPPLE == ptr::null_mut() {
                _gl_impl_panic("glTestObjectAPPLE");
                return 0;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLuint) -> GLboolean = mem::transmute(self.glTestObjectAPPLE);
                (func)(object, name)
            }
        }

        $( $opt )? fn finish_object_apple(&self, object: GLenum, name: GLuint) {


            #[cfg(feature = "debug")] { _gl_print_debug("glFinishObjectAPPLE"); }

            if self.glTestObjectAPPLE == ptr::null_mut() {
                _gl_impl_panic("glFinishObjectAPPLE");
                return;
            }

            unsafe {
                // the spec has a typo for name as GLint instead of GLuint
                let func: extern "system" fn(GLenum, GLint) = mem::transmute(self.glFinishObjectAPPLE);
                (func)(object, name as GLint);
            }
        }

        // GL_ARB_blend_func_extended
        $( $opt )? fn bind_frag_data_location_indexed(
            &self,
            program: GLuint,
            color_number: GLuint,
            index: GLuint,
            name: &str,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glBindFragDataLocationIndexed"); }

            if self.glBindFragDataLocationIndexed == ptr::null_mut() {
                _gl_impl_panic("glBindFragDataLocationIndexed");
                return;
            }

            let c_string = encode_ascii(name);

            unsafe {
                let func: extern "system" fn(GLuint, GLuint, GLuint, *const c_char) = mem::transmute(self.glBindFragDataLocationIndexed);
                (func)(
                    program,
                    color_number,
                    index,
                    c_string.as_ptr(),
                )
            }
        }

        $( $opt )? fn get_frag_data_index(&self, program: GLuint, name: &str) -> GLint {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetFragDataIndex"); }

            if self.glGetFragDataIndex == ptr::null_mut() {
                _gl_impl_panic("glGetFragDataIndex");
                return -1;
            }

            let c_string = encode_ascii(name);

            unsafe {
                let func: extern "system" fn(GLuint, *const c_char) -> GLint = mem::transmute(self.glGetFragDataIndex);
                (func)(program, c_string.as_ptr())
            }
        }

        // GL_KHR_debug
        $( $opt )? fn get_debug_messages(&self) -> Vec<DebugMessage> {


            #[cfg(feature = "debug")] { _gl_print_debug("glGetDebugMessageLog"); }

            if self.glGetDebugMessageLog == ptr::null_mut() {
                _gl_impl_panic("glGetDebugMessageLog");
                return Vec::new();
            }

            let mut max_message_len = [0];
            unsafe {
                self.get_integer_v(ffi::MAX_DEBUG_MESSAGE_LENGTH, &mut max_message_len[..])
            }

            let mut output = Vec::new();
            const CAPACITY: usize = 4;

            let mut msg_data = vec![0u8; CAPACITY * max_message_len[0] as usize];
            let mut sources = [0 as GLenum; CAPACITY];
            let mut types = [0 as GLenum; CAPACITY];
            let mut severities = [0 as GLenum; CAPACITY];
            let mut ids = [0 as GLuint; CAPACITY];
            let mut lengths = [0 as GLsizei; CAPACITY];

            loop {
                let count = unsafe {
                    let func: extern "system" fn(GLuint, GLsizei, *mut GLenum, *mut GLenum, *mut GLuint, *mut GLenum, *mut GLsizei, *mut GLchar) -> GLuint = mem::transmute(self.glGetDebugMessageLog);
                    (func)(
                        CAPACITY as _,
                        msg_data.len() as _,
                        sources.as_mut_ptr(),
                        types.as_mut_ptr(),
                        ids.as_mut_ptr(),
                        severities.as_mut_ptr(),
                        lengths.as_mut_ptr(),
                        msg_data.as_mut_ptr() as *mut _,
                    )
                };

                let mut offset = 0;
                output.extend((0..count as usize).map(|i| {
                    let len = lengths[i] as usize;
                    let slice = &msg_data[offset..offset + len];
                    offset += len;
                    DebugMessage {
                        message: String::from_utf8_lossy(slice).to_string(),
                        source: sources[i],
                        ty: types[i],
                        id: ids[i],
                        severity: severities[i],
                    }
                }));

                if (count as usize) < CAPACITY {
                    return output;
                }
            }
        }

        $( $opt )? fn provoking_vertex_angle(&self, _mode: GLenum) {

            #[cfg(feature = "debug")] { _gl_print_debug("glProvokingVertexAngle"); }

            _gl_impl_panic("glProvokingVertexAngle"); // GLES only
            return;
        }

        // GL_KHR_blend_equation_advanced
        $( $opt )? fn blend_barrier_khr(&self) {


            #[cfg(feature = "debug")] { _gl_print_debug("glBlendBarrierKHR"); }

            if self.glBlendBarrierKHR == ptr::null_mut() {
                _gl_impl_panic("glBlendBarrierKHR");
                return;
            }

            unsafe {
                let func: extern "system" fn() = mem::transmute(self.glBlendBarrierKHR);
                (func)();
            }
        }

        // GL_CHROMIUM_copy_texture
        $( $opt )? fn copy_texture_chromium(&self,
            _source_id: GLuint, _source_level: GLint,
            _dest_target: GLenum, _dest_id: GLuint, _dest_level: GLint,
            _internal_format: GLint, _dest_type: GLenum,
            _unpack_flip_y: GLboolean, _unpack_premultiply_alpha: GLboolean, _unpack_unmultiply_alpha: GLboolean)
        {
            #[cfg(feature = "debug")] { _gl_print_debug("glCopyTextureChromium"); }

            _gl_impl_panic("glCopyTextureChromium"); // GLES only
            return;
        }
        $( $opt )? fn copy_sub_texture_chromium(&self,
            _source_id: GLuint, _source_level: GLint,
            _dest_target: GLenum, _dest_id: GLuint, _dest_level: GLint,
            _x_offset: GLint, _y_offset: GLint, _x: GLint, _y: GLint, _width: GLsizei, _height: GLsizei,
            _unpack_flip_y: GLboolean, _unpack_premultiply_alpha: GLboolean, _unpack_unmultiply_alpha: GLboolean)
        {
            #[cfg(feature = "debug")] { _gl_print_debug("glCopySubTextureChromium"); }

            _gl_impl_panic("glCopySubTextureChromium"); // GLES only
            return;
        }

        // GL_ANGLE_copy_texture_3d
        $( $opt )? fn copy_texture_3d_angle(
            &self,
            _source_id: GLuint,
            _source_level: GLint,
            _dest_target: GLenum,
            _dest_id: GLuint,
            _dest_level: GLint,
            _internal_format: GLint,
            _dest_type: GLenum,
            _unpack_flip_y: GLboolean,
            _unpack_premultiply_alpha: GLboolean,
            _unpack_unmultiply_alpha: GLboolean,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glANGLECopyTexture3D"); }
            _gl_impl_panic("glANGLECopyTexture3D"); // ANGLE only
            return;
        }

        $( $opt )? fn copy_sub_texture_3d_angle(
            &self,
            _source_id: GLuint,
            _source_level: GLint,
            _dest_target: GLenum,
            _dest_id: GLuint,
            _dest_level: GLint,
            _x_offset: GLint,
            _y_offset: GLint,
            _z_offset: GLint,
            _x: GLint,
            _y: GLint,
            _z: GLint,
            _width: GLsizei,
            _height: GLsizei,
            _depth: GLsizei,
            _unpack_flip_y: GLboolean,
            _unpack_premultiply_alpha: GLboolean,
            _unpack_unmultiply_alpha: GLboolean,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glANGLECopySubTexture3D"); }

            _gl_impl_panic("glANGLECopySubTexture3D"); // ANGLE only
            // return;
        }

        $( $opt )? fn buffer_storage(
            &self,
            target: GLenum,
            size: GLsizeiptr,
            data: *const GLvoid,
            flags: GLbitfield,
        ) {

            #[cfg(feature = "debug")] { _gl_print_debug("glBufferStorage"); }

            if self.glBufferStorage == ptr::null_mut() {
                _gl_impl_panic("glBufferStorage");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLsizeiptr, *const GLvoid, GLbitfield) = mem::transmute(self.glBufferStorage);
                (func)(target, size, data, flags);
            }
        }

        $( $opt )? fn flush_mapped_buffer_range(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr) {


            #[cfg(feature = "debug")] { _gl_print_debug("glFlushMappedBufferRange"); }

            if self.glFlushMappedBufferRange == ptr::null_mut() {
                _gl_impl_panic("glFlushMappedBufferRange");
                return;
            }

            unsafe {
                let func: extern "system" fn(GLenum, GLintptr, GLsizeiptr) = mem::transmute(self.glFlushMappedBufferRange);
                (func)(target, offset, length);
            }
        }
    };
}

#[cfg(feature = "std")]
impl gleam::gl::Gl for GenericGlContext {
    impl_gl_context!();
}

#[cfg(not(feature = "std"))]
#[allow(dead_code)]
impl GenericGlContext {
    impl_gl_context!(pub);
}


#[cfg(feature = "std")]
fn _gl_impl_panic(s: &str) { println!("OpenGL function not loaded: {}", s); }

#[cfg(not(feature = "std"))]
#[inline(always)]
fn _gl_impl_panic(_s: &str) {  }

#[cfg(feature = "debug")]
fn _gl_print_debug(s: &str) { println!("Called OpenGL function: {}", s); }