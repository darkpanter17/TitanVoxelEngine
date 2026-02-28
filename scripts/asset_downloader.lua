print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

local io = require("io")
local os = require("os")

-- Crear la carpeta si no existe (Lua puro es limitado, pero os.execute funciona en *nix)
os.execute("mkdir -p assets/textures")

-- PNG de 1x1 píxel para las texturas de fallback, generadas localmente
local dirt_png  = "\137\80\78\71\13\10\26\10\0\0\0\13\73\72\68\82\0\0\0\1\0\0\0\1\8\6\0\0\0\31\21\196\137\0\0\0\13\73\68\65\84\120\156\99\98\170\43\0\0\3\222\1\112\176\220\218\104\0\0\0\0\73\69\78\68\174\66\96\130"
local grass_png = "\137\80\78\71\13\10\26\10\0\0\0\13\73\72\68\82\0\0\0\1\0\0\0\1\8\6\0\0\0\31\21\196\137\0\0\0\13\73\68\65\84\120\156\99\100\232\47\0\0\4\11\1\121\161\240\8\212\0\0\0\0\73\69\78\68\174\66\96\130"
local stone_png = "\137\80\78\71\13\10\26\10\0\0\0\13\73\72\68\82\0\0\0\1\0\0\0\1\8\6\0\0\0\31\21\196\137\0\0\0\13\73\68\65\84\120\156\99\204\204\204\31\0\0\4\144\1\223\213\151\108\212\0\0\0\0\73\69\78\68\174\66\96\130"

local function save_file(path, data)
    local f = io.open(path, "wb")
    if f then
        f:write(data)
        f:close()
        print("  -> Generado fallback: " .. path)
    else
        print("  -> ERROR al escribir: " .. path)
    end
end

print("URLs caídas. Generando texturas de fallback locales (1x1 PNG).")
save_file("assets/textures/1.png", dirt_png)
save_file("assets/textures/2.png", grass_png)
save_file("assets/textures/3.png", stone_png)

print("--- DESCARGA COMPLETA ---")
