local filepath = "data/scripts/gun/gun_actions.lua"

dofile_once = dofile

ACTION_TYPE_PROJECTILE = 1
ACTION_TYPE_STATIC_PROJECTILE = 2
ACTION_TYPE_MODIFIER = 0
ACTION_TYPE_MATERIAL = 3
ACTION_TYPE_UTILITY = 4
ACTION_TYPE_PASSIVE = 5
ACTION_TYPE_DRAW_MANY = 6
ACTION_TYPE_OTHER = 7


dofile(filepath)

for i, v in ipairs(actions) do
	print(v.id .. "," .. v.type .. "," .. v.sprite)
end