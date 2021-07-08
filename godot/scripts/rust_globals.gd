extends Node

onready var GameManager = preload("res://native/GameManager.gdns").new();

func _ready():
	#GameManager.connect("coinPicked", GameManager, "coin");
	
	add_child(GameManager)
	



