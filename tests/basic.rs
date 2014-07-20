extern crate tar = "tar";

#[test]
fn test_tar() {
    let (chan, port) = channel();
    /*
    let field_size = tar::FieldSize {
        path: 100,
        mode: 8,
        uid: 8,
        gid: 8,
        size: 12,
        mtime: 12,
        cksum: 8,
        typee: 1,
        linkpath: 100
    };
    */

    tar::read_tar(chan);

    let mut result = port.recv();

    assert_eq!(result.pop().unwrap(), 102);
    /*
    for block in port.iter() {
        println!("{}", block);
    }
    */
}
