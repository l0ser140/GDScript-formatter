# allow one blank line between statements, in case user wants to logically separate blocks of code
func foo():
	print(123)

	print(123)

	# comment
	if true:
		while true:
			break

		for i in 10:
			continue

		if false:
			pass
		else:
			pass

		match "1":
			_:
				pass
