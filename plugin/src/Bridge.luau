--!strict
-- Talks to the baseplate terminal over localhost. Studio asks once for HTTP
-- permission to localhost; nothing leaves the machine.

local EncodingService = game:GetService("EncodingService")
local HttpService = game:GetService("HttpService")
local RunService = game:GetService("RunService")

local PORT = 44781
local BASE = `http://localhost:{PORT}`

type Handler = (args: any) -> any

local Bridge = {}
local handlers: { [string]: Handler } = {}

function Bridge.on(name: string, handler: Handler)
	handlers[name] = handler
end

local function request(method: string, path: string, body: any?): any?
	local ok, res = pcall(function()
		return HttpService:RequestAsync({
			Url = BASE .. path,
			Method = method,
			Headers = { ["Content-Type"] = "application/json" },
			Body = if body then HttpService:JSONEncode(body) else nil,
		})
	end)
	if not ok or not res.Success then
		return nil
	end
	return if res.Body ~= "" then HttpService:JSONDecode(res.Body) else nil
end

-- Long-poll for commands. Each command gets exactly one reply.
function Bridge.start(plugin: Plugin)
	if RunService:IsRunning() then
		return
	end
	task.spawn(function()
		request("POST", "/hello", { plugin = "1.5.6", place = game.Name, placeId = game.PlaceId })
		while plugin.Parent do
			local cmd = request("GET", "/poll")
			if cmd and handlers[cmd.name] then
				local ok, result = pcall(handlers[cmd.name], cmd.args or {})
				request("POST", "/reply", { id = cmd.id, ok = ok, result = result })
			elseif not cmd then
				task.wait(1)
			end
		end
	end)
end

-- Screenshot of the viewport, sent back as base64 PNG.
function Bridge.capture(capture: any): any
	local shot = capture:CaptureScreenshot({
		CaptureSize = Vector2.new(1280, 720),
		OutputSize = Vector2.new(1280, 720),
		Format = Enum.ImageFormat.PNG,
		UICaptureMode = Enum.UICaptureMode.All,
	})
	while shot.BufferStatus ~= Enum.BufferStatus.Ready do
		task.wait(0.1)
	end
	return { ok = true, png = EncodingService:Base64Encode(shot:GetBuffer()) }
end

-- Runs inside the play session: each check is a small Luau snippet written
-- by the agent, for example "leaderstats.Coins exists after touching a coin".
function Bridge.runChecks(args: any, done: (report: any) -> ())
	task.delay(args.seconds or 30, function()
		local report = {}
		for _, check in args.checks or {} do
			local fn = loadstring(check.source)
			local ok, value = pcall(fn :: any)
			table.insert(report, { name = check.name, ok = ok and value ~= false, value = value })
		end
		done(report)
	end)
end

return Bridge
