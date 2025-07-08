class_name Player extends CharacterBody2D
var start_health := 100
func _ready():
	var area := Area2D.new()
	add_child(area)
	area.area_entered.connect(_on_area_entered)
func _on_area_entered(area: Area2D) -> void:
	if area is Player:
		area.die()
