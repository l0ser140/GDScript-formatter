# There should be two empty lines between top-level functions or classes.
func _ready() -> void:
	var health_bar := ProgressBar.new()
	health_bar.max_value = 100
	add_child(health_bar)


func _physics_process(delta: float) -> void:
	var move_direction := Input.get_vector("move_left", "move_right", "move_up", "move_down")


class Test:
	pass
