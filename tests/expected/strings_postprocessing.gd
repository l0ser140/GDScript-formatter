func _ready() -> void:
	var dangling_comma = """
	first line
	,
	second line
	"""
	var new_line_after_extends = """
	extends Node

	something
	"""
	var dangling_semicolon = """
	asdasd;
	"""
	var trailing_comma_in_preload = """
	preload("",)
	"""
	var trailing_whitespaces_in_multiline_strings = """
	      
	"""
