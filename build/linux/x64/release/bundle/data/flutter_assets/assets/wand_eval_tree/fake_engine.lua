-- Set up the api we are using

---@class (exact) shot_ref
---@field state state
---@field num_of_cards_to_draw integer

---@diagnostic disable-next-line: unused-function
local function dbg_cards(pile)
	for _, v in ipairs(pile) do
		print(v.id)
	end
end
---@diagnostic disable-next-line: unused-function, unused-local
local function dbg_wand()
	print("discard")
	dbg_cards(discarded)
	print("hand")
	dbg_cards(hand)
	print("deck")
	dbg_cards(deck)
end

local function easy_add(id, charges, drained, unlimited_spells)
	for _, v in ipairs(actions) do
		if v.id == id then
			if v.max_uses == nil then
				charges = -1
			elseif unlimited_spells and not v.never_unlimited then
				charges = -1
			elseif charges ~= nil then
				charges = charges
			elseif drained then
				charges = 0
			else
				charges = v.max_uses
			end
			_add_card_to_deck(id, 0, charges, true)
			local card = deck[#deck]
			---@diagnostic disable-next-line: missing-parameter, assign-type-mismatch, param-type-mismatch
			card.action = card.action(card)
			return
		end
	end
end

---@class fake_engine
local M = {}

---@param options options
local function regenerate_translations(options)
	-- print(ModTextFileGetContent("data/translations/common.csv"))
	local actual_translations = {}
	local tcsv = require("extra.tcsv")
	local csv = tcsv.parse(ModTextFileGetContent("data/translations/common.csv"), "common.csv", false)
	local csv_lang_row = nil
	for k, v in ipairs(csv.langs) do
		if v == options.language then
			csv_lang_row = k + 1
		end
	end
	for _, v in ipairs(csv.rows) do
		actual_translations[v[1]] = v[csv_lang_row]
	end
	function GameTextGetTranslatedOrNot(text_or_key)
		if text_or_key:sub(1, 1) == "$" then
			return actual_translations[text_or_key:sub(2)] or text_or_key
		end
		return text_or_key
	end

	for _, v in ipairs(actions or {}) do
		if options.language then
			M.translations[v.id] = GameTextGetTranslatedOrNot(v.name)
			--print(v.id, v.name, GameTextGetTranslatedOrNot(v.name))
			--print(v.name:len())
		end
	end
end

M.mods_path = ""
M.data_path = ""

---@param options options
function M.make_fake_api(options)
	package.path = package.path .. ";" .. M.data_path .. "?.lua;" .. M.mods_path .. "?.lua"
	M.vfs = {
		["data/translations/common.csv"] = assert(
			assert(io.open(M.data_path .. "data/translations/common.csv", "r")):read("*a")
		),
	}
	local _print = print
	require("meta.out")
	print = _print

	regenerate_translations(options)

	local frame = math.floor(os.time() * 1000) % 2 ^ 16
	function Random(a, b)
		if not a and not b then
			return math.random()
		end
		if not b then
			b = a
			a = 0
		end
		return math.floor(math.random() * (b - a + 1)) + a
	end

	local globals = {}
	local append_map = {}

	function GlobalsSetValue(key, value)
		globals[key] = tostring(value)
	end

	function ModTextFileGetContent(filename)
		local success, res = pcall(function()
			if M.vfs[filename] then
				return M.vfs[filename]
			end
			if filename:sub(1, 4) == "mods" then
				return assert(assert(io.open(M.mods_path .. filename)):read("*a"))
			end
			return assert(assert(io.open(M.data_path .. filename)):read("*a"))
		end)
		if not success then
			return ""
		end
		return res
	end

	function ModTextFileSetContent(filename, new_content)
		M.vfs[filename] = new_content
		if filename == "data/translations/common.csv" then
			regenerate_translations(options)
		end
	end

	function GlobalsGetValue(key, value)
		return tostring(globals[key] or value)
	end

	function SetRandomSeed(x, y)
		math.randomseed(x * 591.321 + y * 8541.123 + 124.545)
	end

	function GameGetFrameNum()
		return frame
	end

	function ModLuaFileAppend(to, from)
		append_map[to] = append_map[to] or {}
		table.insert(append_map[to], from)
	end

	function dofile(file)
		local res = { require(file:sub(1, file:len() - 4)) }
		for _, v in ipairs(append_map[file] or {}) do
			dofile(v)
		end
		return unpack(res)
	end
	dofile_once = dofile

	dofile("data/scripts/gun/gun_enums.lua")

	--[[function BeginProjectile(p)
		print(p)
	end]]
end

---@param text_formatter text_formatter
---@param options options
function M.initialise_engine(text_formatter, options)
	dofile("data/scripts/gun/gun.lua")
	local _create_shot = create_shot
	function create_shot(...)
		local uv = { _create_shot(...) }
		local v = uv[1]
		M.nodes_to_shot_ref[M.cur_parent] = v
		M.shot_refs_to_nums[v] = { disp = M.cur_shot_num, real = M.cur_shot_num }
		M.cur_shot_num = M.cur_shot_num + 1
		-- v.state.wand_tree_initial_mana = mana
		-- TODO: find a way to do this in a garunteed safe way
		return unpack(uv)
	end

	function StartReload(reload_time)
		M.reload_time = reload_time
	end

	--[[local _draw_shot = draw_shot
	function draw_shot(...)
		local v = { _draw_shot(...) }
		local args = { ... }
		local shot = args[1]
		shot.state.wand_tree_mana = mana - shot.state.wand_tree_initial_mana
		shot.state.wand_tree_initial_mana = nil
		return unpack(v)
	end]]

	M.translations = {}
	for _, v in ipairs(actions) do
		text_formatter.ty_map[v.id] = v.type
		local _a = v.action
		v.action = function(clone, ...)
			local new = function(...)
				---@cast clone action
				local old_node = M.cur_node
				local new_node = { name = v.id, children = {}, index = clone.deck_index }
				M.counts[v.id] = (M.counts[v.id] or 0) + 1
				M.cur_node = new_node.children
				M.cur_parent = new_node
				table.insert(old_node, new_node)
				local res = { _a(...) }
				M.cur_node = old_node
				return unpack(res)
			end
			if type(clone) == "table" then -- this is awful
				---@diagnostic disable-next-line: return-type-mismatch
				return new
			end
			clone = { deck_index = -1 }
			return unpack({ new(...) })
		end
	end
	regenerate_translations(options)
end

---@param options options
function M.evaluate(options)
	---@type node
	M.calls = { name = "Wand", children = {} }
	M.nodes_to_shot_ref = {}
	M.shot_refs_to_nums = {}
	M.lines_to_shot_nums = {}
	M.cur_shot_num = 1
	M.counts = {}

	_clear_deck(false)
	for _, v in ipairs(options.spells) do
		if type(v) == "table" then
			easy_add(v.name, v.count, options.drained)
		else
			easy_add(v, nil, options.drained, options.unlimited_spells)
		end
	end

	ConfigGun_ReadToLua(options.spells_per_cast, false, options.reload_time, 66)
	_set_gun()
	local data = require("data")
	local arg_list = require("arg_list")
	data.fire_rate_wait = options.cast_delay
	local value = {}
	for _, v in ipairs(arg_list) do
		table.insert(value, data[v])
	end

	--[[local _handle = _handle_reload
	_handle_reload = function()
		print("reloaded")
		print(reloading)
		_handle()
	end]]
	mana = options.mana
	GlobalsSetValue("GUN_ACTION_IF_HALF_STATUS", options.every_other and 1 or 0)
	for i = 1, options.number_of_casts do
		table.insert(M.calls.children, { name = "Cast #" .. i, children = {} })
		ConfigGunActionInfo_ReadToLua(unpack(value))
		_set_gun2()
		M.cur_parent = M.calls.children[#M.calls.children]
		local cur_root = M.cur_parent
		M.cur_node = M.cur_parent.children

		local old_mana = mana
		_start_shot(mana)
		for _, v in ipairs(options.always_casts) do
			if type(v) == "table" then
				v = v.name
			end
			---@cast v string
			--[[local s = "set_current_action"
			local _c = _G[s]
			_G[s] = function(...)
				for _, v2 in ipairs({ ... }) do
					print_table(v2)
				end
				_c(...)
			end]]
			_play_permanent_card(v)
			--_G[s] = _c
		end
		_draw_actions_for_shot(true)
		--dbg_wand()
		local delay = root_shot.state.fire_rate_wait

		-- cursed nolla design.
		_handle_reload()
		if M.reload_time then
			delay = math.max(delay, M.reload_time)
			M.reload_time = nil
		end
		delay = math.max(delay, 1)
		cur_root.extra = "Delay: " .. delay .. "f, ΔMana: " .. (old_mana - mana)
		mana = mana + (delay) * options.mana_charge / 60
	end
end

return M
