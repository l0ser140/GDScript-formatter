# Language configuration for GDScript
# This file defines the language configuration for Topiary

{
  languages = {
    gdscript = {
      extensions = ["gd"],
      grammar = {
        source = {
          git = {
            git = "https://github.com/PrestonKnopp/tree-sitter-gdscript.git",
            rev = "master",
          },
        },
      },
      indent = "\t",
    },
  },
}
