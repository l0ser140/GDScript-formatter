func test() -> void:
	for x in range(10):
		if x != 5:
			continue
		# This comment should stay here
		print(x)

func test2():
	pass # This comment should stay here too
