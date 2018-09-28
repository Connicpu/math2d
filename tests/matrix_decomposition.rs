extern crate math2d;
extern crate rand;

use std::f32::consts::PI;

use math2d::matrix3x2f::IDENTITY;
use math2d::point2f::ORIGIN;
use math2d::vector2f::{ONE, ZERO};
use math2d::Matrix3x2f;

const EPSILON: f32 = 1e-5;
const SEED: [u8; 16] = [
    0x68, 0x16, 0x78, 0x24, 0x6a, 0xc0, 0x74, 0x5f, 0xf0, 0x60, 0xf8, 0xe9, 0x8f, 0x66, 0xcc, 0x12,
];

#[test]
fn identity() {
    let decomp = IDENTITY.decompose();

    assert!(decomp.translation.is_approx_eq(ZERO, EPSILON));
    assert!(decomp.scaling.is_approx_eq(ONE, EPSILON));
    assert!(decomp.rotation.abs() <= EPSILON);
}

#[test]
fn scaling_only_2() {
    let scale = Matrix3x2f::scaling(2.0, ORIGIN);
    let decomp = scale.decompose();

    assert!(decomp.translation.is_approx_eq(ZERO, EPSILON));
    assert!(decomp.scaling.is_approx_eq(2.0, EPSILON));
    assert_angle_approx(decomp.rotation, 0.0);

    let composed: Matrix3x2f = decomp.into();
    assert!(composed.is_approx_eq(&scale, EPSILON));
}

#[test]
fn scaling_only_various() {
    for i in 0..=20 {
        let s = i as f32 / 10.0;
        let scale = Matrix3x2f::scaling(s, ORIGIN);
        let decomp = scale.decompose();

        assert!(decomp.translation.is_approx_eq(ZERO, EPSILON));
        assert!(decomp.scaling.is_approx_eq(s, EPSILON));
        assert_angle_approx(decomp.rotation, 0.0);

        let composed: Matrix3x2f = decomp.into();
        assert!(composed.is_approx_eq(&scale, EPSILON));
    }
}

#[test]
fn rotation_only_180() {
    let rot = Matrix3x2f::rotation(PI, ORIGIN);
    let decomp = rot.decompose();

    assert!(decomp.translation.is_approx_eq(ZERO, EPSILON));
    assert!(decomp.scaling.is_approx_eq(ONE, EPSILON));
    assert_angle_approx(decomp.rotation, PI);

    let composed: Matrix3x2f = decomp.into();
    assert!(composed.is_approx_eq(&rot, EPSILON));
}

#[test]
fn rotation_only_various() {
    for i in 1..=360 {
        let angle = 2.0 * PI * i as f32 / 360.0;
        let rot = Matrix3x2f::rotation(angle, ORIGIN);
        let decomp = rot.decompose();

        assert!(decomp.translation.is_approx_eq(ZERO, EPSILON));
        assert!(decomp.scaling.is_approx_eq(ONE, EPSILON));
        assert_angle_approx(decomp.rotation, angle);

        let composed: Matrix3x2f = decomp.into();
        assert!(composed.is_approx_eq(&rot, EPSILON));
    }
}

#[test]
fn scaling_2_rotation_180() {
    let mat = Matrix3x2f::scaling(2.0, ORIGIN) * Matrix3x2f::rotation(PI, ORIGIN);
    let decomp = mat.decompose();

    assert!(decomp.translation.is_approx_eq(ZERO, EPSILON));
    assert!(decomp.scaling.is_approx_eq(2.0, EPSILON));
    assert_angle_approx(decomp.rotation, PI);

    let composed: Matrix3x2f = decomp.into();
    assert!(composed.is_approx_eq(&mat, EPSILON));
}

#[test]
fn scaling_rotation_various() {
    for i in 1..=20 {
        for j in 0..=36 {
            let s = i as f32 / 10.0;
            let angle = 2.0 * PI * j as f32 / 36.0;
            let mat = Matrix3x2f::scaling(s, ORIGIN) * Matrix3x2f::rotation(angle, ORIGIN);
            let decomp = mat.decompose();

            assert!(decomp.translation.is_approx_eq(ZERO, EPSILON));
            assert!(decomp.scaling.is_approx_eq(s, EPSILON));
            assert_angle_approx(decomp.rotation, angle);

            let composed: Matrix3x2f = decomp.into();
            assert!(composed.is_approx_eq(&mat, EPSILON));
        }
    }
}

#[test]
fn random_compositions() {
    use rand::{Rng, SeedableRng, XorShiftRng};
    let mut rng = XorShiftRng::from_seed(SEED);

    for _ in 0..1_000_000 {
        let angle = rng.gen::<f32>() * 2.0 * PI - PI;
        let sx = rng.gen::<f32>() * 5.0 + EPSILON;
        let sy = rng.gen::<f32>() * 5.0 + EPSILON;
        let tx = rng.gen::<f32>() * 50.0 - 25.0;
        let ty = rng.gen::<f32>() * 50.0 - 25.0;
        println!("s({}, {}) * r({}) * t({}, {})", sx, sy, angle, tx, ty);

        let mat = Matrix3x2f::compose((sx, sy), angle, (tx, ty));
        let decomp = mat.decompose();

        assert!(decomp.translation.is_approx_eq((tx, ty), EPSILON));
        assert!(decomp.scaling.is_approx_eq((sx, sy), EPSILON));
        assert_angle_approx(decomp.rotation, angle);

        let composed: Matrix3x2f = decomp.into();
        assert!(composed.is_approx_eq(&mat, EPSILON));
    }
}

fn assert_angle_approx(a1: f32, a2: f32) {
    let diff = (a1 - a2).abs();
    assert!(
        diff <= EPSILON || (diff - 2.0 * PI).abs() <= EPSILON,
        "Angles which should have been equivalent were different: {} and {}",
        a1,
        a2
    );
}
