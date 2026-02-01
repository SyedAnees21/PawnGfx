#[test]
pub fn point_inside_triangle() {
    use crate::geometry::edge_function;
    use crate::math::Vector2;

    let v0 = Vector2 { x: 0.0, y: 0.0 };
    let v1 = Vector2 { x: 1.0, y: 0.0 };
    let v2 = Vector2 { x: 0.0, y: 1.0 };
    let p_inside = Vector2 { x: 0.25, y: 0.25 };
    let p_outside = Vector2 { x: 1.5, y: 1.5 };

    let ef0_inside = edge_function(v0, v1, p_inside);
    let ef1_inside = edge_function(v1, v2, p_inside);
    let ef2_inside = edge_function(v2, v0, p_inside);

    let ef0_outside = edge_function(v0, v1, p_outside);
    let ef1_outside = edge_function(v1, v2, p_outside);
    let ef2_outside = edge_function(v2, v0, p_outside);

    assert!(
        ef0_inside <= 0.0 && ef1_inside <= 0.0 && ef2_inside <= 0.0,
        "Point should be inside the triangle"
    );
    assert!(
        ef0_outside < 0.0 || ef1_outside < 0.0 || ef2_outside < 0.0,
        "Point should be outside the triangle"
    );
}


#[test]
pub fn colors() {
    let red = hex::decode("ff0000").unwrap();
    assert_eq!(red, vec![0xff, 0x00, 0x00]);
}