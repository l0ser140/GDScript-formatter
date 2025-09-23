# Type inference should stay without space between : and = (issue #31)
func set_avatar_at(at := AvatarAt.LEFT) -> void:
	pass


func test_type_inference(a := 1, b := "string", c := true) -> void:
	pass
