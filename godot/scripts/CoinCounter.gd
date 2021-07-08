extends Control





func _ready():
	$Texture/Count.text = String(RustGlobals.GameManager.coin_counter)
	RustGlobals.GameManager.connect("coinPicked", self, "on_CoinPicked")
	


func on_CoinPicked():
	$Texture/Count.text = String(RustGlobals.GameManager.coin_counter)


