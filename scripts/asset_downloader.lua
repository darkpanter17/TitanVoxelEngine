print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
-- Usamos una imagen genérica para evitar fallos 404
local base = "https://dummyimage.com/256x256/"
local textures = {
    { name = "dirt",  url = base .. "5e4028/fff.png&text=Dirt" },
    { name = "grass", url = base .. "4a7a25/fff.png&text=Grass" },
    { name = "stone", url = base .. "888888/fff.png&text=Stone" },
    { name = "sand",  url = base .. "d4c679/fff.png&text=Sand" },
    { name = "wood",  url = base .. "6e5229/fff.png&text=Wood" }
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