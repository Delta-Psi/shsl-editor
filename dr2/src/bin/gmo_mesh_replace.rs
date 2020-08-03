use dr2::formats::gmo;

fn main() {
    let gmo_path = std::env::args().nth(1).unwrap();
    let gmo_data = std::fs::read(gmo_path).unwrap();
    let mut gmo = gmo::Gmo::from_bytes(&gmo_data).unwrap();

    let obj_path = std::env::args().nth(2).unwrap();
    let obj = obj::Obj::load(obj_path).unwrap().data;
    let object = &obj.objects[0];
    let group = &object.groups[0];

    let out_path = std::env::args().nth(3).unwrap();

    let mut faces = Vec::new();
    let mut vertices = Vec::new();
    for poly in &group.polys {
        let poly = &poly.0;
        assert_eq!(poly.len(), 4);
        let mut ii = [0,0,0,0];
        for i in 0..4 {
            let posi = poly[i].0;
            let texi = poly[i].1.unwrap();
            let normi = poly[i].2.unwrap();

            ii[i] = vertices.len() as u16;
            vertices.push(gmo::model::Vertex {
                pos: (obj.position[posi][0], obj.position[posi][1], obj.position[posi][2]),
                uv: (obj.texture[texi][0], 1.0 - obj.texture[texi][1]),
                normal: (obj.normal[normi][0], obj.normal[normi][1], obj.normal[normi][2]),
            });
        }
        faces.push((ii[0], ii[1], ii[2], ii[3]));
    }

    for subfile in &mut gmo.file.subfiles {
        for chunk in &mut subfile.chunks {
            if let gmo::file::SubfileChunk::Model(model) = chunk {
                for chunk in &mut model.chunks {
                    match chunk {
                        gmo::model::ModelChunk::Mesh(mesh) => {
                            std::mem::swap(&mut mesh.faces, &mut faces);
                        },

                        gmo::model::ModelChunk::VertexArray(vertex_array) => {
                            std::mem::swap(&mut vertex_array.vertices, &mut vertices);
                        },

                        _ => (),
                    }
                }
            }
        }
    }

    let gmo = gmo.encode();
    std::fs::write(out_path, &gmo).unwrap();
}
