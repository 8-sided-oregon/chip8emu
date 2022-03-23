use chip8emu::memory::video::*;

#[test]
fn test_collision_detection() {
    let mut v = VideoRam::new();

    let (x, y) = (10, 12);
    let (width, height) = (10, 10);
    assert!(!v.draw(SpritePos {x, y}, SpriteSize {width, height}).unwrap());
    assert!(v.draw(SpritePos {x, y}, SpriteSize {width, height}).unwrap());

    let mut v = VideoRam::new();

    let (x, y) = (10, 10);
    let (width, height) = (50, 10);

    assert!(!v.draw(SpritePos {x, y}, SpriteSize {width, height}).unwrap());

    let (x, y) = (9, 9);
    let (width, height) = (10, 1);

    assert!(!v.draw(SpritePos {x, y}, SpriteSize {width, height}).unwrap());
}

#[test]
fn test_xor() {
    let mut v = VideoRam::new();

    let (x, y) = (10, 10);
    let (width, height) = (10, 10);

    v.draw(SpritePos {x, y}, SpriteSize {width, height}).unwrap();
    let matrix = v.get_matrx();

    assert!(matrix[10][10]);

    v.draw(SpritePos {x, y}, SpriteSize {width, height}).unwrap();
    let matrix = v.get_matrx();

    assert!(!matrix[10][10]);
}

#[test]
#[should_panic]
fn test_out_of_bounds_panic() {
    let mut v = VideoRam::new();

    let (x, y) = (64, 32);
    let (width, height) = (1, 1);
    v.draw(SpritePos {x, y}, SpriteSize {width, height}).unwrap();
}