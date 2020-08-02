use dr2::formats::gmo;

fn main() {
    let gmo_path = std::env::args().nth(1).unwrap();
    let gmo_data = std::fs::read(gmo_path).unwrap();
    let gmo = gmo::Gmo::from_bytes(&gmo_data).unwrap();

    let mut obj = obj::ObjData::default();
    let mut object = obj::Object::new("_".to_string());
    let mut group = obj::Group::new("_".to_string());

    for subfile in &gmo.file.subfiles {
        for chunk in &subfile.chunks {
            if let gmo::file::SubfileChunk::Model(model) = chunk {
                for chunk in &model.chunks {
                    match chunk {
                        gmo::model::ModelChunk::Mesh(mesh) => {
                            for face in &mesh.faces {
                                let a = face.0.into();
                                let b = face.1.into();
                                let c = face.2.into();
                                let d = face.3.into();
                                group.polys.push(obj::SimplePolygon(vec![
                                        obj::IndexTuple(a, Some(a), Some(a)),
                                        obj::IndexTuple(b, Some(b), Some(b)),
                                        obj::IndexTuple(c, Some(c), Some(c)),
                                        obj::IndexTuple(d, Some(d), Some(d)),
                                ]));
                            }
                        },

                        gmo::model::ModelChunk::VertexArray(vertex_array) => {
                            for vertex in &vertex_array.vertices {
                                obj.position.push([vertex.pos.0, vertex.pos.1, vertex.pos.2]);
                                obj.texture.push([vertex.uv.0, vertex.uv.1]);
                                obj.normal.push([vertex.normal.0, vertex.normal.1, vertex.normal.2]);
                            }
                        },

                        _ => (),
                    }
                }
            }
        }
    }

    object.groups.push(group);
    obj.objects.push(object);

    let mut buf = Vec::new();
    obj.write_to_buf(&mut buf).unwrap();
    let buf = String::from_utf8(buf).unwrap();
    print!("{}", buf);
}
