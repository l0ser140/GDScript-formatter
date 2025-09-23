func foo():
	match   "1"  :
		"1"  :
			pass
		_   :
			pass
	match   "1"  :
		"1",    "2","3"  :
			pass
		_   :
			pass
	match 0:
		@warning_ignore("unreachable_pattern")
		0 when true:
			print("true")
