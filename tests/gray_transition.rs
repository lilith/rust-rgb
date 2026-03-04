//! Tests demonstrating forward-compatible usage of Gray and GrayAlpha types.
//!
//! The rgb crate is transitioning from v0.8 tuple-struct types to v0.9
//! named-field types. This file shows the recommended ("transitional")
//! approach alongside the legacy approach that will break in v0.9.
//!
//! # Summary
//!
//! | v0.8 (legacy)              | v0.9 (transitional)                 |
//! |----------------------------|--------------------------------------|
//! | `use rgb::Gray`            | `use rgb::Gray_v09 as Gray`          |
//! | `use rgb::GrayAlpha`       | `use rgb::GrayA`                     |
//! | `Gray(100)`                | `Gray { v: 100 }` or `Gray::new(100)` |
//! | `GrayAlpha(100, 255)`      | `GrayA { v: 100, a: 255 }` or `GrayA::new(100, 255)` |
//! | `pixel.0`                  | `pixel.v`                            |
//! | `pixel.1`                  | `pixel.a`                            |
//! | `*gray_pixel` (deref)      | `gray_pixel.v`                       |

// ============================================================
// TRANSITIONAL: How to write forward-compatible code today
// ============================================================

/// Import `Gray_v09 as Gray` instead of `rgb::Gray`.
/// Import `GrayA` instead of `rgb::GrayAlpha`.
mod transitional {
    use rgb::Gray_v09 as Gray;
    use rgb::GrayA;
    use rgb::{GainAlpha, HetPixel, Pixel};

    #[test]
    fn construct_gray_with_named_field() {
        let g: Gray<u8> = Gray { v: 128 };
        assert_eq!(g.v, 128);
    }

    #[test]
    fn construct_gray_with_new() {
        let g = Gray::new(200_u8);
        assert_eq!(g.v, 200);
    }

    #[test]
    fn construct_graya_with_named_fields() {
        let ga: GrayA<u8> = GrayA { v: 100, a: 255 };
        assert_eq!(ga.v, 100);
        assert_eq!(ga.a, 255);
    }

    #[test]
    fn construct_graya_with_new() {
        let ga = GrayA::new(100_u8, 255);
        assert_eq!(ga.v, 100);
        assert_eq!(ga.a, 255);
    }

    #[test]
    fn access_fields_by_name() {
        let mut g = Gray::new(50_u8);
        g.v = 60;
        assert_eq!(g.v, 60);

        let mut ga = GrayA::new(50_u8, 200);
        ga.v = 60;
        ga.a = 128;
        assert_eq!(ga.v, 60);
        assert_eq!(ga.a, 128);
    }

    #[test]
    fn use_value_getter() {
        // .value() works on both v08 and v09 types, useful in generic code
        let g = Gray::new(42_u8);
        assert_eq!(g.value(), 42);

        let ga = GrayA::new(42_u8, 100);
        assert_eq!(ga.value(), 42);
    }

    #[test]
    fn gray_with_alpha() {
        let g = Gray::new(128_u8);
        let ga: GrayA<u8> = g.with_alpha(255);
        assert_eq!(ga.v, 128);
        assert_eq!(ga.a, 255);
    }

    #[test]
    fn pixel_trait_map() {
        let g = Gray::new(100_u8);
        let doubled = g.map(|v| u16::from(v) * 2);
        assert_eq!(doubled.v, 200);

        let ga = GrayA::new(100_u8, 50);
        let doubled = ga.map(|c| u16::from(c) * 2);
        assert_eq!(doubled.v, 200);
        assert_eq!(doubled.a, 100);
    }

    #[test]
    fn pixel_trait_map_same() {
        let g = Gray::new(100_u8);
        let halved = g.map_same(|v| v / 2);
        assert_eq!(halved.v, 50);

        let ga = GrayA::new(100_u8, 50);
        let halved = ga.map_same(|c| c / 2);
        assert_eq!(halved.v, 50);
        assert_eq!(halved.a, 25);
    }

    #[test]
    fn pixel_trait_to_array() {
        let g = Gray::new(42_u8);
        assert_eq!(g.to_array(), [42]);
        assert_eq!(g.as_array(), &[42]);

        let ga = GrayA::new(42_u8, 100);
        assert_eq!(ga.to_array(), [42, 100]);
        assert_eq!(ga.as_array(), &[42, 100]);
    }

    #[test]
    fn het_pixel_color_array() {
        let ga = GrayA::new(42_u8, 100);
        assert_eq!(ga.to_color_array(), [42]);
    }

    #[test]
    fn as_ref_to_array() {
        let g = Gray::new(42_u8);
        let arr: &[u8; 1] = g.as_ref();
        assert_eq!(arr, &[42]);

        let ga = GrayA::new(42_u8, 100);
        let arr: &[u8; 2] = ga.as_ref();
        assert_eq!(arr, &[42, 100]);
    }

    #[test]
    fn tuple_conversions() {
        let g = Gray::new(42_u8);
        let t: (u8,) = g.into();
        assert_eq!(t, (42,));

        let g2: Gray<u8> = (42,).into();
        assert_eq!(g2, g);

        let ga = GrayA::new(42_u8, 100);
        let t: (u8, u8) = ga.into();
        assert_eq!(t, (42, 100));

        let ga2: GrayA<u8> = (42, 100).into();
        assert_eq!(ga2, ga);
    }

    #[test]
    fn graya_drop_alpha() {
        let ga = GrayA::new(42_u8, 100);
        let g: Gray<u8> = ga.into();
        assert_eq!(g.v, 42);
    }

    #[test]
    fn type_aliases_point_to_v09() {
        // GRAY8, GRAY16, GRAYA8, GRAYA16 are already the v0.9 types
        let g: rgb::GRAY8 = Gray::new(42);
        assert_eq!(g.v, 42);

        let ga: rgb::GRAYA8 = GrayA::new(42, 100);
        assert_eq!(ga.v, 42);
        assert_eq!(ga.a, 100);
    }

    #[test]
    fn equality_and_comparison() {
        let a = Gray::new(10_u8);
        let b = Gray::new(20_u8);
        assert!(a < b);
        assert_eq!(a, Gray::new(10));

        let ga1 = GrayA::new(10_u8, 255);
        let ga2 = GrayA::new(10_u8, 128);
        assert!(ga2 < ga1); // v equal, compare alpha
    }

    #[test]
    fn default_is_zero() {
        let g: Gray<u8> = Default::default();
        assert_eq!(g.v, 0);

        let ga: GrayA<u8> = Default::default();
        assert_eq!(ga.v, 0);
        assert_eq!(ga.a, 0);
    }
}

// ============================================================
// LEGACY: This code works on v0.8 but will break on v0.9
// ============================================================

/// These tests document patterns that will NOT survive the v0.9 transition.
/// Each test is annotated with what breaks and why.
#[allow(deprecated)]
mod legacy {
    use rgb::Gray;      // This import gives you Gray_v08 (tuple struct)
    use rgb::GrayAlpha; // This import gives you GrayAlpha_v08 (tuple struct)

    /// Tuple construction: `Gray(value)` will not work in v0.9.
    /// Use `Gray { v: value }` or `Gray::new(value)` instead.
    #[test]
    fn tuple_construction_gray() {
        let g = Gray(128_u8);
        assert_eq!(g.0, 128);
    }

    /// Tuple construction: `GrayAlpha(v, a)` will not work in v0.9.
    /// Use `GrayA { v, a }` or `GrayA::new(v, a)` instead.
    #[test]
    fn tuple_construction_gray_alpha() {
        let ga = GrayAlpha(100_u8, 255);
        assert_eq!(ga.0, 100);
        assert_eq!(ga.1, 255);
    }

    /// Accessing `.0` on Gray will not work in v0.9.
    /// Use `.v` instead (works today via deref on GrayAlpha, or natively on Gray_v09).
    #[test]
    fn field_access_by_index() {
        let g = Gray(42_u8);
        assert_eq!(g.0, 42);

        let ga = GrayAlpha(42_u8, 100);
        assert_eq!(ga.0, 42);
        assert_eq!(ga.1, 100);
    }

    /// Dereferencing Gray to get the inner value will not work in v0.9.
    /// Use `.v` or `.value()` instead.
    #[test]
    fn deref_gray() {
        let g = Gray(42_u8);
        let val: u8 = *g; // Gray_v08 implements Deref<Target=T>
        assert_eq!(val, 42);
    }

    /// Pattern matching on tuple fields will not work in v0.9.
    /// Use named field patterns instead.
    #[test]
    fn pattern_match_tuple() {
        let g = Gray(42_u8);
        let Gray(v) = g;
        assert_eq!(v, 42);

        let ga = GrayAlpha(42_u8, 100);
        let GrayAlpha(v, a) = ga;
        assert_eq!(v, 42);
        assert_eq!(a, 100);
    }

    // --------------------------------------------------------
    // BRIDGE: These patterns work on BOTH v0.8 and v0.9 types
    // because GrayAlpha_v08 derefs to GrayA, exposing .v and .a
    // --------------------------------------------------------

    /// .v and .a work on GrayAlpha_v08 today through the Deref bridge.
    /// This is the recommended way to access fields if you haven't
    /// switched your imports yet.
    #[test]
    fn deref_bridge_named_fields() {
        let ga = GrayAlpha(42_u8, 100);
        // .v and .a work via Deref<Target = GrayA>
        assert_eq!(ga.v, 42);
        assert_eq!(ga.a, 100);
    }

    /// .value() works on both Gray_v08 and Gray_v09.
    #[test]
    fn value_getter_bridge() {
        let g = Gray(42_u8);
        assert_eq!(g.value(), 42);

        let ga = GrayAlpha(42_u8, 100);
        assert_eq!(ga.value(), 42);
    }

    /// Mutating through .v and .a works on GrayAlpha_v08 via DerefMut.
    #[test]
    fn deref_bridge_mutation() {
        let mut ga = GrayAlpha(50_u8, 200);
        ga.v = 60;
        ga.a = 128;
        assert_eq!(ga.v, 60);
        assert_eq!(ga.a, 128);
    }
}
