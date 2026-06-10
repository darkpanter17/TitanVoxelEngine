def cross_product(v1, v2):
    return [
        v1[1]*v2[2] - v1[2]*v2[1],
        v1[2]*v2[0] - v1[0]*v2[2],
        v1[0]*v2[1] - v1[1]*v2[0]
    ]

def sub(p1, p2):
    return [p1[0]-p2[0], p1[1]-p2[1], p1[2]-p2[2]]

def check_normal(pts, name, expected):
    v1 = sub(pts[1], pts[0])
    v2 = sub(pts[2], pts[1])
    n = cross_product(v1, v2)
    print(f"{name}: Normal={n}, Expected={expected}")

check_normal([[0, 1, 1], [1, 1, 1], [1, 1, 0], [0, 1, 0]], "Top (+Y)", [0, 1, 0])
check_normal([[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]], "Bottom (-Y)", [0, -1, 0])
check_normal([[1, 0, 0], [1, 1, 0], [1, 1, 1], [1, 0, 1]], "Right (+X)", [1, 0, 0])
check_normal([[0, 0, 1], [0, 1, 1], [0, 1, 0], [0, 0, 0]], "Left (-X)", [-1, 0, 0])
check_normal([[1, 0, 1], [1, 1, 1], [0, 1, 1], [0, 0, 1]], "Front (+Z)", [0, 0, 1])
check_normal([[0, 0, 0], [0, 1, 0], [1, 1, 0], [1, 0, 0]], "Back (-Z)", [0, 0, -1])
