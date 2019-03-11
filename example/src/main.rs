macro_rules! ipp_assert {
    ($result:expr) => {
        assert!(unsafe{$result}==ipp_sys::ippStsNoErr as i32);
    }
}

fn main() {
    // Get the version from IPP at runtime.
    let linked_version_major = unsafe{ (*ipp_sys::ippGetLibVersion()).major };
    let linked_version_minor = unsafe{ (*ipp_sys::ippGetLibVersion()).minor };

    // Compare the runtime major version with the compile-time major version.
    assert_eq!( linked_version_major as i32, ipp_sys::IPP_VERSION_MAJOR as i32);
    // And compare the minor version, too.
    assert_eq!( linked_version_minor as i32, ipp_sys::IPP_VERSION_MINOR as i32);

    ipp_assert!(ipp_sys::ippInit());

    const W: ::std::os::raw::c_int = 20;
    const H: ::std::os::raw::c_int = 20;
    let size = ipp_sys::IppiSize { width: W, height: H };

    // Allocate memory for an image. Note: aligned allocation is not done
    // in this example, but may be important for performance.
    let mut image = [0u8; (W*H) as usize];

    assert!(image[0]==0);
    ipp_assert!(ipp_sys::ippiSet_8u_C1R( 10, image.as_mut_ptr(), W, size));
    assert!(image[0]==10);
}
