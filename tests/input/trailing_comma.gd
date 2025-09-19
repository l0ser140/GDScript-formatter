var a = [
	1,
	2,
	3
]

var aa = [1, 2, 3,]

var b = [
	1,
	2,
	3 # comment
]

var c = {
	"a": 1,
	"b": 2,
	"c": 3 # comment
}

var d = {
	"a": 1,
	"b": 2,
	"c": 3
}

var dd = {"a": 1, "b": 2, "c": 3,}

enum Foo {
	A,
	B,
	C
}

enum Foo2 {
	A,
	B,
	C # comment
}

enum Foo3 {A, B, C,}

func foo(
	a,
	b
):
	pass


func bar(
	a,
	b # comment
):
	pass


func f():
	foo(
		1,
		2
	)


func test(a: int, b: int,):
	pass


func test():
	print("test", "test",)
