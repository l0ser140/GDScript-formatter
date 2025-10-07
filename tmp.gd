# Example of buggy formatting of a comment.
static func some_func(amt):
	return amt
	#if amt <= 0:
	#return 0
	#
	#return Global.rng.randi_range(0,1)
