extern crate ipp_sys as ipp;

macro_rules! ipp_assert {
    ($result:expr) => {
        assert!(unsafe{$result}==ipp::ippStsNoErr as i32);
    }
}

fn main() {
    ipp_assert!(ipp::ippInit());

    const W: ::std::os::raw::c_int = 20;
    const H: ::std::os::raw::c_int = 20;
    let size = ipp::IppiSize { width: W, height: H };

    // Allocate memory for an image. Note: aligned allocation is not done
    // in this example, but may be important for performance.
    let mut image = [0u8; (W*H) as usize];

    assert!(image[0]==0);
    ipp_assert!(ipp::ippiSet_8u_C1R( 10, image.as_mut_ptr(), W, size));
    assert!(image[0]==10);
}
