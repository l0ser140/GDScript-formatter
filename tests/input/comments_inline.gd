@export_group("my_group") # annotation comment
var prop = 10: # var comment
	set(value): # set comment
		prop = value
	get: # get comment
		return prop

enum Foo {
	A, # Comment
	B,# Comment
	C
}

class InnerClass: # class comment
	pass

func _init(): # constructor comment
	var lua_dict = {
			# Comment
			a = 0, # Comment
			# Comment
			b = 1, # Comment
			# Comment
	}

	var arr = [
		1, # Comment
		2, # Comment
		# Comment
		2,
		# Comment
		# Comment 2
		2,
		# Comment
		3,# Comment
		]
	pass

func foo(): # func comment
	if true: # if comment
		pass
	elif false: # elif comment
		pass
	else: # else comment
		pass

	match 0: # match comment
		1: # case comment
			pass
		_: # default comment
			pass

	for i in 10: # for comment
		pass

	while false: # while comment
		pass

	var lam = func(): # lambda comment
		pass

	bar(
		a, # function call inline comment
		b,
	)

	return # function trailing comment at end
