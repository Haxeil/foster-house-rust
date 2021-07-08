extends Control

onready var health_bar = $Bar
onready var update_tween = $UpdateTween
onready var damage_bar = $DamageBar
onready var pulse_tween = $PulseTween

export (Color) var healthy_color = Color.blue
export (Color) var caution_color = Color.darkorange
export (Color) var danger_color = Color.red
export (Color) var pulse_color = Color.darkred

export (float, 0, 1, 0.05) var caution_zone = 0.5
export (float, 0, 1, 0.05) var danger_zone = 0.2
export (bool) var will_paulse = false

func _on_Bloo_health_updated(health):
	health_bar.value = health
	update_tween.interpolate_property(damage_bar, "value", damage_bar.value, health, 0.4, Tween.TRANS_SINE, 0.4)
	update_tween.start()
	_assign_health_color(health)

func _assign_health_color(health):
	if health < health_bar.max_value * danger_zone:
		if will_paulse:
			pulse_tween.interpolate_property(health_bar, "tint_progress", pulse_color, danger_color, 2,Tween.TRANS_SINE, Tween.EASE_IN_OUT)
			pulse_tween.start()
		else:
			health_bar.tint_progress = danger_color
	elif health < health_bar.max_value * caution_zone:
		health_bar.tint_progress = caution_color
	else:
		health_bar.tint_progress = healthy_color



func _on_Bloo_max_health_updated(max_health):
	health_bar.max_value = max_health
	damage_bar.max_value = max_health
	health_bar.value = health_bar.max_value
	damage_bar.value = damage_bar.max_value













