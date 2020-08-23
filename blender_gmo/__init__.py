import bpy
from bpy.props import StringProperty
import bmesh
import bpy_extras.io_utils
import bpy_extras.image_utils
from bpy_extras.io_utils import (orientation_helper, axis_conversion)

import os
import struct

bl_info = {
    "name": "GMO model importer and exporter",
    "author": "Delta-Psi",
    "version": (0, 0, 1),
    "location": "File > Import-Export",
    "description": "Allows editing of Danganronpa GMO files",
    "category": "Import-Export",
}

@orientation_helper(axis_forward='-Z', axis_up='Y')
class ImportGMO(bpy.types.Operator, bpy_extras.io_utils.ImportHelper):
    """Load a GMO file."""
    bl_idname = "import_scene.gmo"
    bl_label = "GMO (.gmo)"
    bl_options = {'PRESET', 'UNDO'}

    filename_ext = ".gmo"
    filter_glob: StringProperty(
                default="*.gmo",
                options={'HIDDEN'},
                )

    def execute(self, context):
        global_matrix = axis_conversion(
            from_forward=self.axis_forward,
            from_up=self.axis_up,
        ).to_4x4();

        import_gmo(context, self.filepath, global_matrix)

        return {'FINISHED'}

if __name__ == "__main__":
    bpy.utils.register_class(ImportGMO)
    #bpy.types.TOPBAR_MT_file_import.append(lambda self, context: self.layout.operator(ImportGMO.bl_idname, text="GMO (.gmo)"))

#############################################

def import_gmo(context, filepath, global_matrix):
    # create new collection from filename
    filename = os.path.basename(filepath)
    collection = bpy.data.collections.new(filename)
    bpy.context.collection.children.link(collection)

    with open(filepath, 'rb') as file:
        data = file.read()
    
    # check magic numbers
    assert data[0:16] == b"OMG.00.1PSP\0\0\0\0\0"
    data = data[16:]
    
    # read file chunk!
    chunk_type, header_size, chunk_size = read_chunk_metadata(data)
    
    # checks that it's a file-type chunk
    assert chunk_type == 0x0002
    
    chunk_header = data[8:header_size]
    chunk_data = data[header_size:chunk_size]
    import_file(collection, chunk_header, chunk_data, global_matrix)
    
    data = data[chunk_size:]
    
    # we don't want any extra data
    assert len(data) == 0

def read_chunk_metadata(data):
     chunk_type, header_size, chunk_size = struct.unpack_from('<HHI', data)
     return chunk_type, max(header_size, 8), chunk_size

# per-chunk importing routines

def import_file(collection, header, data, global_matrix):
    index = 0
    
    # read subfiles
    while len(data) > 0:
        chunk_type, header_size, chunk_size = read_chunk_metadata(data)
        
        # checks that it's a subfile-type chunk
        assert chunk_type == 0x0003
        
        subfile_collection = bpy.data.collections.new("{}.{:02}".format(collection.name, index))
        collection.children.link(subfile_collection)
        
        chunk_header = data[8:header_size]
        chunk_data = data[header_size:chunk_size]
        import_subfile(subfile_collection, chunk_header, chunk_data, global_matrix)

        data = data[chunk_size:]
        index += 1

def import_subfile(collection, header, data, global_matrix):
    # import order: texture -> material -> model surface -> bone info
    textures = []
    materials = []
    model_surfaces = []
    bones = []
    
    while len(data) > 0:
        chunk_type, header_size, chunk_size = read_chunk_metadata(data)
        chunk_header = data[8:header_size]
        chunk_data = data[header_size:chunk_size]
        
        if chunk_type == 0x8014:
            # ignore
            pass
        elif chunk_type == 0x0004:
            bones.append((chunk_header, chunk_data))
        elif chunk_type == 0x0005:
            model_surfaces.append((chunk_header, chunk_data))
        elif chunk_type == 0x0008:
            materials.append((chunk_header, chunk_data))
        elif chunk_type == 0x000a:
            textures.append((chunk_header, chunk_data))
        else:
            raise ValueError("unknown subfile chunk {:04x}".format(chunk_type))

        data = data[chunk_size:]
        
    for index, (chunk_header, chunk_data) in enumerate(textures):
        textures[index] = import_texture(collection, index, chunk_header, chunk_data)
        
    for index, (chunk_header, chunk_data) in enumerate(materials):
        materials[index] = import_material(collection, textures, index, chunk_header, chunk_data)
        
    for index, (chunk_header, chunk_data) in enumerate(model_surfaces):
        model_surfaces[index] = import_model_surface(collection, materials, index, chunk_header, chunk_data, global_matrix)
        
    for index, (chunk_header, chunk_data) in enumerate(bones):
        bones[index] = import_bone_info(chunk_header, chunk_data)

def import_texture(collection, index, header, data):
    # find texture path subchunk
    chunk_type, header_size, chunk_size = read_chunk_metadata(data)
    assert chunk_type == 0x8012
    chunk_header = data[8:header_size]
    chunk_data = data[header_size:chunk_size]
    assert len(data) == chunk_size
    
    zero_index = 0
    while chunk_data[zero_index] != 0:
        zero_index += 1
    texture_path = chunk_data[:zero_index]
    # TODO: extract actual path data
    
    # store as image
    image = bpy_extras.image_utils.load_image(texture_path, place_holder=True)
    image.name = "{}_image{:02}".format(collection.name, index)
    
    return image

def import_material(collection, textures, index, header, data):
    material = bpy.data.materials.new("{}_material{:02}".format(collection.name, index))
    material.blend_method = 'BLEND' # transparency support

    # set up nodes
    material.use_nodes = True
    bsdf = material.node_tree.nodes["Principled BSDF"]
    
    def read_texture_reference(data):
        chunk_type, header_size, chunk_size = read_chunk_metadata(data)
        assert chunk_type == 0x8091
        chunk_header = data[8:header_size]
        chunk_data = data[header_size:chunk_size]
        assert len(data) == chunk_size
        
        return struct.unpack_from('<H', chunk_data)[0] - 0x2000
    
    while len(data) > 0:
        chunk_type, header_size, chunk_size = read_chunk_metadata(data)
        chunk_header = data[8:header_size]
        chunk_data = data[header_size:chunk_size]
        
        if chunk_type == 0x0009:
            # texture reference
            texture_index = read_texture_reference(chunk_data)
            image = textures[texture_index]
            
            # link texture to BSDF
            texture_node = material.node_tree.nodes.new('ShaderNodeTexImage')
            texture_node.image = image
            material.node_tree.links.new(bsdf.inputs['Base Color'], texture_node.outputs['Color'])
            material.node_tree.links.new(bsdf.inputs['Alpha'], texture_node.outputs['Alpha'])
        elif chunk_type == 0x8082:
            # RGBA
            r, g, b, a = struct.unpack_from('<ffff', chunk_data)
            
            # link color to BSDF
            bsdf.inputs['Base Color'].default_value = (r, g, b, a)
            bsdf.inputs['Alpha'].default_value = a
        else:
            raise ValueError("unknown material chunk {:04x}".format(chunk_type))

        data = data[chunk_size:]
    
    return material

def import_model_surface(collection, materials, index, header, data, global_matrix):
    # make a mesh
    mesh = bpy.data.meshes.new("{}_mesh{:02}".format(collection.name, index))
    bm = bmesh.new()
    
    # add UV layer
    uv_layer = bm.loops.layers.uv.new("GMO UVs")
    uvs = {}
    
    # import order: vertex array -> mesh
    vertex_arrays = []
    meshes = []
    
    while len(data) > 0:
        chunk_type, header_size, chunk_size = read_chunk_metadata(data)
        chunk_header = data[8:header_size]
        chunk_data = data[header_size:chunk_size]
        
        if chunk_type == 0x8014:
            # ignore
            pass
        elif chunk_type == 0x0006:
            meshes.append((chunk_header, chunk_data))
        elif chunk_type == 0x0007:
            vertex_arrays.append((chunk_header, chunk_data))
        else:
            raise ValueError("unknown model surface chunk {:04x}".format(chunk_type))

        data = data[chunk_size:]
    
    arrays = []
    for index, (header, data) in enumerate(vertex_arrays):
        format, count = struct.unpack_from('<HxxI', data)
        array = []

        if format == 0x11e3:
            data = data[16:]
            for _ in range(count):
                format = '<ffffffff'
                u, v, nx, ny, nz, x, y, z = struct.unpack_from(format, data)
                
                # flip v value
                v = -v
                
                vert = bm.verts.new((x, y, z))
                vert.normal = (nx, ny, nz)
                uvs[vert] = (u, v)
                
                array.append(vert)

                data = data[struct.calcsize(format):]
        else:
            raise ValueError("unknown vertex array format {:04x}".format(chunk_type))
        
        arrays.append(array)
    
    for index, (header, data) in enumerate(meshes):
        read_mesh_data(mesh, bm, arrays, materials, index, header, data)
        
    # actually apply uvs
    for face in bm.faces:
        for loop in face.loops:
            loop[uv_layer].uv = uvs[loop.vert]

    bm.to_mesh(mesh)
    # fix orientation
    mesh.transform(global_matrix)
    object = bpy.data.objects.new("{}_object{:02}".format(collection.name, index), mesh)
    collection.objects.link(object)
    
    return object

def read_mesh_data(mesh, bm, arrays, materials, index, header, data):
    while len(data) > 0:
        chunk_type, header_size, chunk_size = read_chunk_metadata(data)
        chunk_header = data[8:header_size]
        chunk_data = data[header_size:chunk_size]
        
        if chunk_type == 0x8061:
            # material info
            material_index = struct.unpack_from('<H', chunk_data)[0] - 0x2000
            material = materials[material_index]
            print(material)
            mesh.materials.append(material)
        elif chunk_type == 0x8066:
            vertex_array, primitive_type, stripe_size, stripe_count = struct.unpack_from("<HxxIII", chunk_data)
            vertex_array -= 0x1000
            stripe_data = chunk_data[16:]
            
            if primitive_type == 4:
                stripe_faces = stripe_size//2 - 1
                for _ in range(stripe_count):
                    for _ in range(stripe_faces):
                        a, b, d, c = struct.unpack_from("<HHHH", stripe_data)
                        verts = (
                            arrays[vertex_array][a],
                            arrays[vertex_array][b],
                            arrays[vertex_array][c],
                            arrays[vertex_array][d],
                        )
                        
                        face = bm.faces.new(verts)
                            
                        stripe_data = stripe_data[4:]
                    stripe_data = stripe_data[4:]
        else:
            raise ValueError("unknown mesh data chunk {:04x}".format(chunk_type))
    
        data = data[chunk_size:]
        

def import_bone_info(header, data):
    return None

#import_gmo(bpy.context, "/run/media/delta/work/steam/steamapps/common/Danganronpa 2 Goodbye Despair/dr2_data/Dr2/data/all/model/stand_00_00.gmo", axis_conversion(
#            from_forward='-Z',
#            from_up='Y',
#        ).to_4x4());