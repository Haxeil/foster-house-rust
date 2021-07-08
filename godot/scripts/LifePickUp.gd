extends Area2D

export var health = 50






func _on_LifePickUp_body_entered(body):
	$CollisionPolygon2D.set_deferred("disabled",true)
	$AnimationPlayer.play("Picked")
	body.Health(health)

