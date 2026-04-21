print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Fallbacks to dummyimage since the original URLs 404
local textures = {
    { name = "dirt",  url = "https://dummyimage.com/16x16/644632/644632.png" },
    { name = "grass", url = "https://dummyimage.com/16x16/508c3c/508c3c.png" },
    { name = "stone", url = "https://dummyimage.com/16x16/787878/787878.png" },
    { name = "sand",  url = "https://dummyimage.com/16x16/d2b48c/d2b48c.png" },
    { name = "wood",  url = "https://dummyimage.com/16x16/8b5a2b/8b5a2b.png" }
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
