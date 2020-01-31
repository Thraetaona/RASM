/* WORK IN PROGRESS

#![feature(stdsimd)]
#![cfg(target_arch = "wasm32")]

use std::arch::wasm32::*;

macro_rules! _ps_const_ty {
    ($name:ident, $field:ident, $x:expr) => {
        const $name: UnionCast = UnionCast {
            $field: [$x, $x, $x, $x],
        };
    };
}

pub union F32x4 {
    pub simd: v128,
    pub array: [f32; 4],
}

pub union I32x4 {
    pub simd: v128,
    pub array: [i32; 4],
}

pub union F32x8 {
    pub simd: __m256,
    pub array: [f32; 8],
}

pub union I32x8 {
    pub simd: __m256i,
    pub array: [i32; 8],
}

pub union UnionCast {
    pub m128: v128,
    pub m128i: v128,
    pub f32x4: [f32; 4],
    pub i32x4: [i32; 4],
    pub u32x4: [u32; 4],
}

_ps_const_ty!(PS_INV_SIGN_MASK, u32x4, !0x8000_0000);
_ps_const_ty!(PS_SIGN_MASK, u32x4, 0x8000_0000);

_ps_const_ty!(PS_1_0, f32x4, 1.0);
_ps_const_ty!(PS_0_5, f32x4, 0.5);

_ps_const_ty!(PI32_1, i32x4, 1);
_ps_const_ty!(PI32_INV_1, i32x4, !1);
_ps_const_ty!(PI32_2, i32x4, 2);
_ps_const_ty!(PI32_4, i32x4, 4);

_ps_const_ty!(PS_MINUS_CEPHES_DP1, f32x4, -0.78515625);
_ps_const_ty!(PS_MINUS_CEPHES_DP2, f32x4, -2.4187564849853515625e-4);
_ps_const_ty!(PS_MINUS_CEPHES_DP3, f32x4, -3.77489497744594108e-8);
_ps_const_ty!(PS_SINCOF_P0, f32x4, -1.9515295891E-4);
_ps_const_ty!(PS_SINCOF_P1, f32x4, 8.3321608736E-3);
_ps_const_ty!(PS_SINCOF_P2, f32x4, -1.6666654611E-1);
_ps_const_ty!(PS_COSCOF_P0, f32x4, 2.443315711809948E-005);
_ps_const_ty!(PS_COSCOF_P1, f32x4, -1.388731625493765E-003);
_ps_const_ty!(PS_COSCOF_P2, f32x4, 4.166664568298827E-002);
_ps_const_ty!(PS_CEPHES_FOPI, f32x4, 1.27323954473516); // 4 / M_PI

pub fn sinf_cosf(x: f32) -> (f32, f32) {
    // expect sse2 to be available on all x86 builds
    unsafe {
        let (sinx, cosx) = sinf_cosf_sse2(_mm_set1_ps(x));
        (_mm_cvtss_f32(sinx), _mm_cvtss_f32(cosx))
    }
    x.sin_cos()
}

// Based on http://gruntthepeon.free.fr/ssemath/sse_mathfun.h
pub unsafe fn sinf_cosf_sse2(x: v128) -> (v128, v128) {
    let mut sign_bit_sin = x;
    // take the absolute value
    let mut x = _mm_and_ps(x, PS_INV_SIGN_MASK.m128);
    // extract the sign bit (upper one)
    sign_bit_sin = _mm_and_ps(sign_bit_sin, PS_SIGN_MASK.m128);

    // scale by 4/Pi
    let mut y = _mm_mul_ps(x, PS_CEPHES_FOPI.m128);

    // store the integer part of y in emm2
    let mut emm2 = _mm_cvttps_epi32(y);

    // j=(j+1) & (~1) (see the cephes sources)
    emm2 = _mm_add_epi32(emm2, PI32_1.m128i);
    emm2 = _mm_and_si128(emm2, PI32_INV_1.m128i);
    y = _mm_cvtepi32_ps(emm2);

    let mut emm4 = emm2;

    /* get the swap sign flag for the sine */
    let mut emm0 = _mm_and_si128(emm2, PI32_4.m128i);
    emm0 = _mm_slli_epi32(emm0, 29);
    let swap_sign_bit_sin = _mm_castsi128_ps(emm0);

    /* get the polynom selection mask for the sine*/
    emm2 = _mm_and_si128(emm2, PI32_2.m128i);
    emm2 = _mm_cmpeq_epi32(emm2, _mm_setzero_si128());
    let poly_mask = _mm_castsi128_ps(emm2);

    /* The magic pass: "Extended precision modular arithmetic" 
     x = ((x - y * DP1) - y * DP2) - y * DP3; */
    let mut xmm1 = PS_MINUS_CEPHES_DP1.m128;
    let mut xmm2 = PS_MINUS_CEPHES_DP2.m128;
    let mut xmm3 = PS_MINUS_CEPHES_DP3.m128;
    xmm1 = _mm_mul_ps(y, xmm1);
    xmm2 = _mm_mul_ps(y, xmm2);
    xmm3 = _mm_mul_ps(y, xmm3);
    x = _mm_add_ps(x, xmm1);
    x = _mm_add_ps(x, xmm2);
    x = _mm_add_ps(x, xmm3);

    emm4 = _mm_sub_epi32(emm4, PI32_2.m128i);
    emm4 = _mm_andnot_si128(emm4, PI32_4.m128i);
    emm4 = _mm_slli_epi32(emm4, 29);
    let sign_bit_cos = _mm_castsi128_ps(emm4);

    sign_bit_sin = _mm_xor_ps(sign_bit_sin, swap_sign_bit_sin);

    // Evaluate the first polynom  (0 <= x <= Pi/4)
    let z = _mm_mul_ps(x, x);
    y = PS_COSCOF_P0.m128;

    y = _mm_mul_ps(y, z);
    y = _mm_add_ps(y, PS_COSCOF_P1.m128);
    y = _mm_mul_ps(y, z);
    y = _mm_add_ps(y, PS_COSCOF_P2.m128);
    y = _mm_mul_ps(y, z);
    y = _mm_mul_ps(y, z);
    let tmp = _mm_mul_ps(z, PS_0_5.m128);
    y = _mm_sub_ps(y, tmp);
    y = _mm_add_ps(y, PS_1_0.m128);

    // Evaluate the second polynom  (Pi/4 <= x <= 0)
    let mut y2 = PS_SINCOF_P0.m128;
    y2 = _mm_mul_ps(y2, z);
    y2 = _mm_add_ps(y2, PS_SINCOF_P1.m128);
    y2 = _mm_mul_ps(y2, z);
    y2 = _mm_add_ps(y2, PS_SINCOF_P2.m128);
    y2 = _mm_mul_ps(y2, z);
    y2 = _mm_mul_ps(y2, x);
    y2 = _mm_add_ps(y2, x);

    // select the correct result from the two polynoms
    xmm3 = poly_mask;
    let ysin2 = _mm_and_ps(xmm3, y2);
    let ysin1 = _mm_andnot_ps(xmm3, y);
    y2 = _mm_sub_ps(y2, ysin2);
    y = _mm_sub_ps(y, ysin1);

    xmm1 = _mm_add_ps(ysin1, ysin2);
    xmm2 = _mm_add_ps(y, y2);

    // update the sign
    (
        _mm_xor_ps(xmm1, sign_bit_sin),
        _mm_xor_ps(xmm2, sign_bit_cos),
    )
}

pub fn simd_bits() -> usize {
    {
            return unsafe { simd_bits_sse2() };
    }
    32
}

pub unsafe fn simd_bits_sse2() -> usize {
    128
}

pub unsafe fn blend_i32_sse2(lhs: v128, rhs: v128, cond: v128) -> v128 {
    let d = _mm_srai_epi32(_mm_castps_si128(cond), 31);
    _mm_or_si128(_mm_and_si128(d, rhs), _mm_andnot_si128(d, lhs))
}

pub unsafe fn hmin_sse2(v: v128) -> f32 {
    let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
    let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
    _mm_cvtss_f32(v)
}


pub unsafe fn blend_f32_sse2(lhs: v128, rhs: v128, cond: v128) -> v128 {
    let d = _mm_castsi128_ps(_mm_srai_epi32(_mm_castps_si128(cond), 31));
    _mm_or_ps(_mm_and_ps(d, rhs), _mm_andnot_ps(d, lhs))
}

pub unsafe fn dot3_sse2(
    x0: v128,
    x1: v128,
    y0: v128,
    y1: v128,
    z0: v128,
    z1: v128,
) -> v128 {
    let mut dot = _mm_mul_ps(x0, x1);
    dot = _mm_add_ps(dot, _mm_mul_ps(y0, y1));
    dot = _mm_add_ps(dot, _mm_mul_ps(z0, z1));
    dot
}

*/