print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
local base = "https://dummyimage.com/16x16/"
local textures = {
    { name = "dirt",  url = base .. "8b4513/fff.png&text=D" },
    { name = "grass", url = base .. "2d5a27/fff.png&text=G" },
    { name = "stone", url = base .. "808080/fff.png&text=S" },
    { name = "sand",  url = base .. "eebd79/fff.png&text=A" },
    { name = "wood",  url = base .. "6b4e31/fff.png&text=W" }
}
-- Si main no existe, el script puede probar master cambiando base arriba

-- Crear directorio si no existe (Rust lo maneja, pero por orden lógico)
-- Iterar y descargar
for i, tex in ipairs(textures) do
    local filename = "assets/textures/" .. i .. ".png" 
    print("Descargando textura [" .. tex.name .. "] desde: " .. tex.url)
    
    -- Llamada a la función Rust expuesta
    local success = download_file(tex.url, filename)
    
    if success then
        print("  -> Guardado en: " .. filename)
    else
        print("  -> ERROR al descargar " .. tex.name)
    end
end

print("--- DESCARGA COMPLETA ---")