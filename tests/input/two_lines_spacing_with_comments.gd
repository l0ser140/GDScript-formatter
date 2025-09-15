var a

# case 1
func test2() -> void:
	pass

var a # case 2

func test2() -> void:
	pass

var a

func test2() -> void:
	pass

const a = 10

func test2() -> void:
	pass

class CheckSameCasesInsideNestedClass:
	var a

	# case 1
	func test2() -> void:
		pass

	var a # case 2

	func test2() -> void:
		pass

	var a

	func test2() -> void:
		pass

	const a = 10

	func test2() -> void:
		pass
	
	class DoubleNestedCaseWithInlineComments:
		var a # case 2

		func test2() -> void:
			pass
