extends Area2D


# don't forget to add it to the scene tree, otherwise memory must be managed manually 

func _on_Coin_body_entered(_body):
	
	$CollisionShape2D.set_deferred("disabled", true)
	$AnimationPlayer.play("Picked")
	print(RustGlobals.GameManager.coin_counter)
	RustGlobals.GameManager.coin_counter += 1;
	RustGlobals.GameManager.emit_signal("coinPicked");


func _on_AnimationPlayer_animation_finished(_anim_name):
	queue_free()
