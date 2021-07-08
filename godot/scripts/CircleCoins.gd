extends Node2D

var count : int = 0
var coins_count : int = 0

func _ready():
	for child in get_children():
		if child.name.begins_with("Coin"):
		
			coins_count += 1

func free_when_no_coin_in_tree() -> void:
	
	count += 1
	if count == coins_count:
		queue_free()


func _on_Coin_body_entered(_body):
	free_when_no_coin_in_tree()


func _on_Coin2_body_entered(_body):
	free_when_no_coin_in_tree()


func _on_Coin3_body_entered(_body):
	free_when_no_coin_in_tree()

func _on_Coin4_body_entered(_body):
	free_when_no_coin_in_tree()


func _on_Coin5_body_entered(_body):
	free_when_no_coin_in_tree()


func _on_Coin6_body_entered(_body):
	free_when_no_coin_in_tree()


func _on_Coin7_body_entered(_body):
	free_when_no_coin_in_tree()


func _on_Coin8_body_entered(_body):
	free_when_no_coin_in_tree()

func _on_Coin9_body_entered(_body):
	free_when_no_coin_in_tree()


func _on_Coin10_body_entered(_body):
	free_when_no_coin_in_tree()


func _on_Coin11_body_entered(_body):
	free_when_no_coin_in_tree()


func _on_Coin12_body_entered(_body):
	free_when_no_coin_in_tree()


func _on_VisibilityNotifier2D_screen_entered():
	$AnimationPlayer.play("Spin")
