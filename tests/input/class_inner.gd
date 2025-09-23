class InnerClass extends Node:

	var a = 10

	var b = 20
	func _init() -> void:
		pass
	func foo():
		print(132)
	func bar():
		var c = 1
		print(a + b + c)

	class InnerInnerClass extends Node:

		var a = 10

		var b = 20
		func _init() -> void:
			pass
		func foo():
			print(132)
		func bar():
			var c = 1
			print(a + b + c)
class A:
	extends RefCounted
class B:
	extends A
	var test = 2

class C:func test(x):
	print(x)
