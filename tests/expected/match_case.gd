func foo():
	match "1":
		"1":
			pass
		_:
			pass
	match "1":
		"1", "2", "3":
			pass
		_:
			pass
	match 0:
		0 when true:
			print("true")
