#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Things that are not exported by bindgen.

// These are `size_t`, and we enabled `bindgen::Builder::size_t_is_usize` in `build.rs`.
pub const PLUM_MODE_FILENAME: usize = usize::MAX;
pub const PLUM_MODE_BUFFER: usize = usize::MAX - 1;
pub const PLUM_MODE_CALLBACK: usize = usize::MAX - 2;
pub const PLUM_MAX_MEMORY_SIZE: usize = usize::MAX - 3;

#[cfg(test)]
mod tests {
    use std::ffi::{c_void, CString};

    use super::*;

    // One sanity check to see if the bindings work correctly.
    // This doesn't test all functionality, I'd expect that to come from either upstream or downstream.
    #[test]
    fn load_test_square() {
        // This entire test is a bit fast and loose with the lifetimes of refs/slices, but it's
        // straightforward enough that we never use anything dangling.

        let path = CString::new("./testsq.png").unwrap();
        let mut error = 0u32;
        // SAFETY: we conform to the API here, passing a pointer to a NUL-terminated string
        //         together with `PLUM_MODE_FILENAME`.
        let image = unsafe {
            plum_load_image_limited(
                path.as_ptr() as *const c_void,
                PLUM_MODE_FILENAME,
                PLUM_COLOR_32 | PLUM_PALETTE_NONE,
                isize::MAX as usize,
                &mut error as *mut _,
            )
        };
        // SAFETY: `plum_load_image`'s API is to return a pointer to a struct on success, and NULL on error.
        //         - The pointer comes from `malloc`, which ensures alignment.
        //         - The correct size is passed to `malloc`, which ensures it's dereferenceable.
        //         - `plum_new_image` does a struct-to-struct assignment, which inits the entire struct
        //           except padding, but AFAIK that's fine?
        //         - The original pointer is lost (name rebound), and no copy exists(*),
        //           which guarantees no aliasing.
        // (*) Actually, libplum stores a copy of the pointer in its "allocator", but does not
        //     make use of it outside of `plum_destroy_image`.
        let Some(image) = (unsafe { image.as_mut() }) else {
            panic!("Loading testsq.png failed, plum_load_image returned error {error}");
        };
        assert_eq!(plum_image_types::from(image.type_), PLUM_IMAGE_PNG);
        assert_eq!(plum_flags::from(image.color_format), PLUM_COLOR_32);
        assert_eq!(image.frames, 1);
        assert_eq!(image.height, 21);
        assert_eq!(image.width, 21);
        assert_eq!(image.palette, std::ptr::null_mut());

        let data = unsafe { image.data.as_ref() }.expect("image.data is NULL!?");
        let data = unsafe {
            std::slice::from_raw_parts(
                data as *const c_void as *const u32,
                (image.height * image.width) as usize, // The size is capped by `plum_image_load_limited`.
            )
        };

        const EXPECTED_COLORS: [u32; 21 * 21] = [
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff,
            0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff,
            0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7fff0000, 0x7fff0000, 0x7fff0000, 0x7fff0000,
            0x7fff0000, 0x7fff0000, 0x7fff0000, 0x00000000, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff,
            0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7fff0000, 0x7fff0000, 0x7fff0000, 0x7fff0000,
            0x7fff0000, 0x7fff0000, 0x7fff0000, 0x00000000, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff,
            0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7fff0000, 0x7fff0000, 0x7fff0000, 0x7fff0000,
            0x7fff0000, 0x7fff0000, 0x7fff0000, 0x00000000, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff,
            0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7fff0000, 0x7fff0000, 0x7fff0000, 0x7fff0000,
            0x7fff0000, 0x7fff0000, 0x7fff0000, 0x00000000, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff,
            0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7fff0000, 0x7fff0000, 0x7fff0000, 0x7fff0000,
            0x7fff0000, 0x7fff0000, 0x7fff0000, 0x00000000, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff,
            0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7fff0000, 0x7fff0000, 0x7fff0000, 0x7fff0000,
            0x7fff0000, 0x7fff0000, 0x7fff0000, 0x00000000, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff,
            0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7fff0000, 0x7fff0000, 0x7fff0000, 0x7fff0000,
            0x7fff0000, 0x7fff0000, 0x7fff0000, 0x00000000, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff,
            0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x7f0000ff, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff,
            0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x00000000, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00,
            0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff,
            0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x00000000, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00,
            0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff,
            0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x00000000, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00,
            0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff,
            0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x00000000, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00,
            0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff,
            0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x00000000, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00,
            0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff,
            0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x00000000, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00,
            0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x7f00ffff,
            0x7f00ffff, 0x7f00ffff, 0x7f00ffff, 0x00000000, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00,
            0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x7f00ff00, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff,
            0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff,
            0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00000000,
            //
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000,
            //
        ];
        let mut matched = true;
        for row_id in 0..21 {
            let start = row_id * 21;
            let range = start..=start + 20;
            let expected = &EXPECTED_COLORS[range.clone()];
            let got = &data[range];
            if got != expected {
                eprintln!("Row {row_id} mismatch!");
                eprintln!("\tExpected {expected:?},");
                eprintln!("\t     got {got:?}");
                matched = false;
            }
        }
        assert!(matched);
    }
}
