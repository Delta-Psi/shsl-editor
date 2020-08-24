bl_info = {
    "name": "GMO model importer and exporter",
    "author": "Delta-Psi",
    "version": (0, 0, 1),
    "location": "File > Import-Export",
    "description": "Allows editing of Danganronpa GMO files",
    "category": "Import-Export",
}

import bpy
from bpy.props import StringProperty
import bmesh
import bpy_extras.io_utils
import bpy_extras.image_utils
from bpy_extras.io_utils import (orientation_helper, axis_conversion)

from import_gmo import import_gmo

import os
import struct

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

@orientation_helper(axis_forward='-Z', axis_up='Y')
class ExportGMO(bpy.types.Operator, bpy_extras.io_utils.ImportHelper):
    """Save a GMO file."""
    bl_idname = "export_scene.gmo"
    bl_label = "GMO (.gmo)"
    bl_options = {'PRESET', 'UNDO'}

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
