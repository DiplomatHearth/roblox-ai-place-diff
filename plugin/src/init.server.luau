--!strict
-- Baseplate plugin. Studio's built-in MCP server already lets the agent read and
-- edit the place. This plugin adds the parts it doesn't have: one undo step per
-- turn, the current selection, place snapshots for the diff, test-copy
-- playtests and screenshots for the proof.

local ChangeHistoryService = game:GetService("ChangeHistoryService")
local LogService = game:GetService("LogService")
local Selection = game:GetService("Selection")
local StudioTestService = game:GetService("StudioTestService")

local Bridge = require(script.Bridge)

local toolbar = plugin:CreateToolbar("Baseplate")
local button = toolbar:CreateButton("Baseplate", "Show the Baseplate panel", "")

local info = DockWidgetPluginGuiInfo.new(Enum.InitialDockState.Right, false, false, 312, 560, 260, 360)
local panel = plugin:CreateDockWidgetPluginGui("BaseplatePanel", info)
panel.Title = "Baseplate"
button.Click:Connect(function()
	panel.Enabled = not panel.Enabled
end)

-- One recording per turn, so a single Ctrl+Z undoes the whole turn.
local recording: string? = nil

Bridge.on("begin_turn", function(args)
	if recording then
		ChangeHistoryService:FinishRecording(recording, Enum.FinishRecordingOperation.Commit)
	end
	recording = ChangeHistoryService:TryBeginRecording(`Baseplate turn {args.turn}`, args.summary)
	return { ok = recording ~= nil }
end)

Bridge.on("end_turn", function(args)
	if not recording then
		return { ok = false, reason = "no turn in progress" }
	end
	local op = if args.cancel then Enum.FinishRecordingOperation.Cancel else Enum.FinishRecordingOperation.Commit
	ChangeHistoryService:FinishRecording(recording, op)
	recording = nil
	return { ok = true }
end)

Bridge.on("rewind", function(args)
	for _ = 1, args.steps do
		ChangeHistoryService:Undo()
	end
	return { ok = true }
end)

Bridge.on("selection", function()
	local paths = {}
	for _, instance in Selection:Get() do
		table.insert(paths, instance:GetFullName())
	end
	return { paths = paths }
end)

-- A compact snapshot of a subtree: enough to compute the place diff.
local function snapshot(root: Instance, depth: number): { [string]: any }
	local node = { name = root.Name, class = root.ClassName, children = {} }
	if root:IsA("LuaSourceContainer") then
		node.lines = #string.split((root :: any).Source, "\n")
	end
	if depth > 0 then
		for _, child in root:GetChildren() do
			table.insert(node.children, snapshot(child, depth - 1))
		end
	end
	return node
end

Bridge.on("snapshot", function(args)
	local root = if args.path then game:FindFirstChild(args.path, true) else game
	return root and snapshot(root, args.depth or 6) or { missing = args.path }
end)

-- Playtests run in the local test copy that the terminal opens in a second
-- Studio window. Output is collected and sent back with the result.
Bridge.on("playtest", function(args)
	local lines = {}
	local conn = LogService.MessageOut:Connect(function(message, kind)
		if kind == Enum.MessageType.MessageError or kind == Enum.MessageType.MessageWarning then
			table.insert(lines, { kind = kind.Name, text = message })
		end
	end)
	local ok, result = pcall(function()
		return StudioTestService:ExecutePlayModeAsync({ seconds = args.seconds or 30, checks = args.checks })
	end)
	conn:Disconnect()
	return { ok = ok, result = result, output = lines }
end)

-- Inside the play session, the same plugin runs the checks and ends the test.
if StudioTestService:GetTestArgs() then
	Bridge.runChecks(StudioTestService:GetTestArgs(), function(report)
		StudioTestService:EndTest(report)
	end)
	return
end

Bridge.on("screenshot", function()
	local StudioCaptureService = game:GetService("StudioCaptureService")
	if not StudioCaptureService:RequestScreenshotPermissionAsync() then
		return { ok = false, reason = "screenshots are turned off in Studio" }
	end
	return Bridge.capture(StudioCaptureService)
end)

Bridge.start(plugin)
