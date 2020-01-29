use std::fs::File;
use std::io::{BufRead, BufReader};

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

pub fn load(
    path: &str,
) -> (
    Vec<glm::Vector3<f32>>,
    Vec<glm::Vector2<f32>>,
    Vec<glm::Vector3<f32>>,
) {
    // println!("{:?}", sun_mvp.c0);
    // println!("{:?}", sun_mvp.c1);
    // println!("{:?}", sun_mvp.c2);
    // println!("{:?}", sun_mvp.c3);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let (mut vertices, mut uvs, mut normals) = (vec![], vec![], vec![]);

    let (mut vertex_indices, mut uv_indices, mut normal_indices): (
        Vec<usize>,
        Vec<usize>,
        Vec<usize>,
    ) = (vec![], vec![], vec![]);

    let (mut temp_vertices, mut temp_uvs, mut temp_normals) = (vec![], vec![], vec![]);

    for line in reader.lines().map(Result::unwrap) {
        // just straight up bad code e.e
        let line_clone = line.clone();
        let line_type = line_clone.split_whitespace().next().unwrap();
        let line: String = line
            .split_whitespace()
            .skip(1)
            .map(|s| format!("{}\n", s))
            .collect();

        if line_type == "v" {
            let (x, y, z) = scan!(line, char::is_whitespace, f32, f32, f32);

            let x = x.unwrap();
            let y = y.unwrap();
            let z = z.unwrap();

            temp_vertices.push(glm::vec3(x, y, z));
        } else if line_type == "vt" {
            let (x, y) = scan!(line, char::is_whitespace, f32, f32);

            let x = x.unwrap();
            let y = y.unwrap();

            temp_uvs.push(glm::vec2(x, y));
        } else if line_type == "vn" {
            let (x, y, z) = scan!(line, char::is_whitespace, f32, f32, f32);

            let x = x.unwrap();
            let y = y.unwrap();
            let z = z.unwrap();

            temp_normals.push(glm::vec3(x, y, z));
        } else if line_type == "f" {
            let (mut vertex_index, mut uv_index, mut normal_index) =
                ([0, 0, 0], [0, 0, 0], [0, 0, 0]);

            let (group1, group2, group3) = scan!(line, char::is_whitespace, String, String, String);

            let group1 = group1.unwrap();
            let group1_str: Vec<&str> = group1.split('/').collect();
            let n0 = group1_str[0].parse::<usize>().unwrap();
            let n1 = group1_str[1].parse::<usize>().unwrap();
            let n2 = group1_str[2].parse::<usize>().unwrap();
            vertex_index[0] = n0;
            uv_index[0] = n1;
            normal_index[0] = n2;

            let group2 = group2.unwrap();
            let group2_str: Vec<&str> = group2.split('/').collect();
            let n0 = group2_str[0].parse::<usize>().unwrap();
            let n1 = group2_str[1].parse::<usize>().unwrap();
            let n2 = group2_str[2].parse::<usize>().unwrap();
            vertex_index[1] = n0;
            uv_index[1] = n1;
            normal_index[1] = n2;

            let group3 = group3.unwrap();
            let group3_str: Vec<&str> = group3.split('/').collect();
            let n0 = group3_str[0].parse::<usize>().unwrap();
            let n1 = group3_str[1].parse::<usize>().unwrap();
            let n2 = group3_str[2].parse::<usize>().unwrap();
            vertex_index[2] = n0;
            uv_index[2] = n1;
            normal_index[2] = n2;

            vertex_indices.push(vertex_index[0]);
            vertex_indices.push(vertex_index[1]);
            vertex_indices.push(vertex_index[2]);
            uv_indices.push(uv_index[0]);
            uv_indices.push(uv_index[1]);
            uv_indices.push(uv_index[2]);
            normal_indices.push(normal_index[0]);
            normal_indices.push(normal_index[1]);
            normal_indices.push(normal_index[2]);
        }
    }

    for i in 0..vertex_indices.len() {
        let vertex_index = vertex_indices[i];
        let uv_index = uv_indices[i];
        let normal_index = normal_indices[i];

        vertices.push(temp_vertices[vertex_index - 1]);
        uvs.push(temp_uvs[uv_index - 1]);
        normals.push(temp_normals[normal_index - 1]);
    }

    (vertices, uvs, normals)
}
