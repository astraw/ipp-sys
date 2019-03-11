extern crate ipp_sys as ipp;

macro_rules! ipp_assert {
    ($result:expr) => {
        assert!(unsafe{$result}==ipp::ippStsNoErr as i32);
    }
}

#[test]
fn test_link_ippcore() {
    // Get the version from IPP at runtime.
    let linked_version_major = unsafe{ (*ipp::ippGetLibVersion()).major };
    let linked_version_minor = unsafe{ (*ipp::ippGetLibVersion()).minor };

    // Compare the runtime major version with the compile-time major version.
    assert_eq!( linked_version_major as i32, ipp::IPP_VERSION_MAJOR as i32);
    // And compare the minor version, too.
    assert_eq!( linked_version_minor as i32, ipp::IPP_VERSION_MINOR as i32);

    ipp_assert!(ipp::ippInit());
    println!("ippcore OK");
}

#[test]
fn test_link_ippi() {
    // Not calling ippInit() because allegedly this is not necessary
    // and this test partly checks if linking is OK without explicitly
    // pulling in ippcore. ippInit() is defined in ippcore.

    const W: ::std::os::raw::c_int = 20;
    const H: ::std::os::raw::c_int = 20;
    let size = ipp::IppiSize { width: W, height: H };

    // Allocate memory for an image. Note: aligned allocation is not done
    // in this example, but may be important for performance.
    let mut image = [0u8; (W*H) as usize];

    assert!(image[0]==0);
    ipp_assert!(ipp::ippiSet_8u_C1R( 10, image.as_mut_ptr(), W, size));
    assert!(image[0]==10);
    println!("ippi OK");
}

#[test]
fn test_link_ippcv() {
    // Not calling ippInit() because allegedly this is not necessary
    // and this test partly checks if linking is OK without explicitly
    // pulling in ippcore. ippInit() is defined in ippcore.
    const W: ::std::os::raw::c_int = 20;
    const H: ::std::os::raw::c_int = 20;
    let size = ipp::IppiSize { width: W, height: H };

    let src = [10u8; (W*H) as usize];
    let mut dest = [0u8; (W*H) as usize];

    assert!(dest[0]==0);
    ipp_assert!(ipp::ippiAbsDiffC_8u_C1R( src.as_ptr(), W, dest.as_mut_ptr(), W, size, 9));
    assert!(dest[0]==1);
    println!("ippcv OK");
}

#[test]
fn test_link_ipps() {
    // Not calling ippInit() because allegedly this is not necessary
    // and this test partly checks if linking is OK without explicitly
    // pulling in ippcore. ippInit() is defined in ippcore.
    use ipp::Ipp32f;

    const W: ::std::os::raw::c_int = 20;
    let src = [ -1.23 as Ipp32f; W as usize];
    let mut dest = [ 0 as Ipp32f; W as usize];

    assert!(dest[0]==0.0);
    ipp_assert!(ipp::ippsAbs_32f( src.as_ptr(), dest.as_mut_ptr(), W));
    assert!(dest[0]==1.23);
    println!("ipps OK");
}
