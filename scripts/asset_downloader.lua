print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Usando dummyimage.com para evitar errores 404
local textures = {
    { name = "dirt",  url = "https://dummyimage.com/16x16/8b4513/fff.png&text=Dirt" },
    { name = "grass", url = "https://dummyimage.com/16x16/228b22/fff.png&text=Grass" },
    { name = "stone", url = "https://dummyimage.com/16x16/808080/fff.png&text=Stone" },
    { name = "sand",  url = "https://dummyimage.com/16x16/f4a460/fff.png&text=Sand" },
    { name = "wood",  url = "https://dummyimage.com/16x16/deb887/fff.png&text=Wood" }
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
