print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Fallback to dummyimage.com since original fogleman/Craft textures return 404
local base = "https://dummyimage.com/64x64/"
local textures = {
    { name = "dirt",  url = base .. "8b5a2b/ffffff.png&text=dirt" },
    { name = "grass", url = base .. "3b7e3b/ffffff.png&text=grass" },
    { name = "stone", url = base .. "7a7a7a/ffffff.png&text=stone" },
    { name = "sand",  url = base .. "d2b48c/ffffff.png&text=sand" },
    { name = "wood",  url = base .. "8b4513/ffffff.png&text=wood" }
}

for i, tex in ipairs(textures) do
    local filename = "assets/textures/" .. i .. ".png" 
    print("Descargando textura [" .. tex.name .. "] desde: " .. tex.url)
    
    local success = download_file(tex.url, filename)
    
    if success then
        print("  -> Guardado en: " .. filename)
    else
        print("  -> ERROR al descargar " .. tex.name)
    end
end

print("--- DESCARGA COMPLETA ---")
