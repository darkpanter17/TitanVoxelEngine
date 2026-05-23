print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

local textures = {
    { name = "dirt",  url = "https://dummyimage.com/16x16/8b4513/ffffff&text=Dirt" },
    { name = "grass", url = "https://dummyimage.com/16x16/228b22/ffffff&text=Grass" },
    { name = "stone", url = "https://dummyimage.com/16x16/808080/ffffff&text=Stone" }
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
