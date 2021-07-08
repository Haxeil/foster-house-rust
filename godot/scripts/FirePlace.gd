extends Node2D


func toggle_fire(toggle) -> void:
	$FirePlace/Fire.set_emitting(toggle)
	$FirePlace/Smoke.set_emitting(toggle)
	for candles in $Candles.get_children():
		for particles in candles.get_children():
			particles.set_emitting(toggle)



func _on_VisibilityNotifier2D_screen_entered():
	toggle_fire(true)


func _on_VisibilityNotifier2D_screen_exited():
	toggle_fire(false)
