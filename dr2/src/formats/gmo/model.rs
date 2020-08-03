use super::*;

#[derive(Debug)]
pub struct Model {
    pub header: Vec<u8>,
    pub chunks: Vec<ModelChunk>,
}

impl Chunk for Model {
    fn new(chunk: ChunkRef) -> Self {
        assert_eq!(chunk.type_, 0x0005, "chunk type doesn't match");

        Self {
            header: chunk.header.to_owned(),
            chunks: chunk.children()
                .map(ModelChunk::new)
                .collect(),
        }
    }

    fn type_(&self) -> u16 {
        0x0005
    }

    fn encode_impl(&self) -> (Vec<u8>, Vec<u8>) {
        let mut buf = Vec::new();
        for chunk in &self.chunks {
            buf.write_all(&chunk.encode()).unwrap();
        }

        (self.header.clone(), buf)
    }
}

#[derive(Debug)]
pub enum ModelChunk {
    Mesh(Mesh),
    VertexArray(VertexArray),

    Generic(Generic),
}

impl Chunk for ModelChunk {
    fn new(chunk: ChunkRef) -> Self {
        match chunk.type_ {
            0x0006 => ModelChunk::Mesh(Mesh::new(chunk)),
            0x0007 => ModelChunk::VertexArray(VertexArray::new(chunk)),

            _ => ModelChunk::Generic(Generic::new(chunk)),
        }
    }

    fn type_(&self) -> u16 {
        match self {
            ModelChunk::Mesh(inner) => inner.type_(),
            ModelChunk::VertexArray(inner) => inner.type_(),
            ModelChunk::Generic(inner) => inner.type_(),
        }
    }

    fn encode_impl(&self) -> (Vec<u8>, Vec<u8>) {
        match self {
            ModelChunk::Mesh(inner) => inner.encode_impl(),
            ModelChunk::VertexArray(inner) => inner.encode_impl(),
            ModelChunk::Generic(inner) => inner.encode_impl(),
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub header: Vec<u8>,

    pub material_id: Option<u16>,
    pub faces: Vec<(u16, u16, u16, u16)>,
}

impl Chunk for Mesh {
    fn new(chunk: ChunkRef) -> Self {
        assert_eq!(chunk.type_, 0x0006);

        let mut material_id = None;
        let mut faces = Vec::new();

        for chunk in chunk.children() {
            match chunk.type_ {
                0x8061 => {
                    // material id
                    assert_eq!(chunk.header.len(), 0);
                    let id = LE::read_u16(&chunk.data[0..2]) - 0x2000;
                    assert_eq!(&chunk.data[2..], &[8, 0]);
                    material_id = Some(id);
                },

                0x8066 => {
                    // face data
                    assert_eq!(chunk.header.len(), 0);

                    let data = &chunk.data;
                    let val0 = &data[0..4];
                    assert_eq!(val0, &[0,16,7,0]);

                    let primitive_type = LE::read_u32(&data[4..8]);
                    assert_eq!(primitive_type, 4);

                    let stripe_size = LE::read_u32(&data[8..12]);
                    let stripe_count = LE::read_u32(&data[12..16]);

                    let mut data = &data[16..];
                    for _ in 0..stripe_count {
                        for _ in 0..stripe_size/2 - 1 {
                            let a = LE::read_u16(&data[0..]);
                            let b = LE::read_u16(&data[2..]);
                            let d = LE::read_u16(&data[4..]);
                            let c = LE::read_u16(&data[6..]);

                            faces.push((a, b, c, d));
                            data = &data[4..];
                        }
                        data = &data[4..];
                    }
                    assert_eq!(data.len(), 0);
                },

                _ => panic!("unknown mesh chunk"),
            }
        }

        Mesh {
            header: chunk.header.to_owned(),
            material_id,
            faces,
        }
    }

    fn type_(&self) -> u16 {
        0x0006
    }
    fn encode_impl(&self) -> (Vec<u8>, Vec<u8>) {
        let mut buf = Vec::new();

        if let Some(material_id) = self.material_id {
            let mut cdata = Vec::new();
            cdata.write_u16::<LE>(material_id + 0x2000).unwrap();
            cdata.write_all(&[8, 0]).unwrap();

            let chunk = Generic {
                type_: 0x8061,
                header: Vec::new(),
                data: cdata,
            };
            buf.write_all(&chunk.encode()).unwrap();
        }

        let mut cdata = Vec::new();
        cdata.write_all(&[0,16,7,0]).unwrap();
        cdata.write_u32::<LE>(4).unwrap();

        cdata.write_u32::<LE>(4).unwrap();
        cdata.write_u32::<LE>(self.faces.len() as u32).unwrap();

        for face in &self.faces {
            let (a, b, c, d) = *face;

            cdata.write_u16::<LE>(a).unwrap();
            cdata.write_u16::<LE>(b).unwrap();
            cdata.write_u16::<LE>(d).unwrap();
            cdata.write_u16::<LE>(c).unwrap();
        }

        let chunk = Generic {
            type_: 0x8066,
            header: Vec::new(),
            data: cdata,
        };
        buf.write_all(&chunk.encode()).unwrap();

        (self.header.clone(), buf)
    }
}

#[derive(Debug)]
pub struct Vertex {
    pub uv: (f32, f32),
    pub normal: (f32, f32, f32),
    pub pos: (f32, f32, f32),
}

#[derive(Debug)]
pub struct VertexArray {
    pub header: Vec<u8>,

    pub vertices: Vec<Vertex>,
}

impl Chunk for VertexArray {
    fn new(chunk: ChunkRef) -> Self {
        assert_eq!(chunk.type_, 0x0007);

        let data = chunk.data;
        let format = LE::read_u32(&data[0..4]);
        assert_eq!(format, 0x200011e3, "unknown vertex format");
        let count = LE::read_u32(&data[4..8]);

        // ???
        let value = &data[8..16];
        assert_eq!(value, &[1,0,0,0,0,0,0,0]);

        let mut vertices = Vec::new();

        let mut data = &data[16..];
        for _ in 0..count {
            let u = LE::read_f32(&data[0..]);
            let v = LE::read_f32(&data[4..]);
            let nx = LE::read_f32(&data[8..]);
            let ny = LE::read_f32(&data[12..]);
            let nz = LE::read_f32(&data[16..]);
            let x = LE::read_f32(&data[20..]);
            let y = LE::read_f32(&data[24..]);
            let z = LE::read_f32(&data[28..]);
            vertices.push(Vertex {
                uv: (u, v),
                normal: (nx, ny, nz),
                pos: (x, y, z),
            });

            data = &data[32..];
        }
        assert_eq!(data.len(), 0);

        VertexArray {
            header: chunk.header.to_owned(),
            vertices,
        }
    }

    fn type_(&self) -> u16 {
        0x0007
    }

    fn encode_impl(&self) -> (Vec<u8>, Vec<u8>) {
        let mut buf = Vec::new();
        buf.write_u32::<LE>(0x200011e3).unwrap();
        buf.write_u32::<LE>(self.vertices.len() as u32).unwrap();
        buf.write_all(&[1,0,0,0,0,0,0,0]).unwrap();

        for vertex in &self.vertices {
            buf.write_f32::<LE>(vertex.uv.0).unwrap();
            buf.write_f32::<LE>(vertex.uv.1).unwrap();
            buf.write_f32::<LE>(vertex.normal.0).unwrap();
            buf.write_f32::<LE>(vertex.normal.1).unwrap();
            buf.write_f32::<LE>(vertex.normal.2).unwrap();
            buf.write_f32::<LE>(vertex.pos.0).unwrap();
            buf.write_f32::<LE>(vertex.pos.1).unwrap();
            buf.write_f32::<LE>(vertex.pos.2).unwrap();
        }

        (self.header.to_owned(), buf)
    }
}
