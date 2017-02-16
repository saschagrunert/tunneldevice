extern crate tunneldevice;
use tunneldevice::Device;

#[test]
fn device() {
    let mut device = Device::new("test_device").unwrap();

    let test_string = b"Lorem Ipsum";
    device.write(test_string).unwrap();

    let mut buffer = vec![];
    device.read(&mut buffer).unwrap();

    assert_eq!(&buffer, &test_string);
}
