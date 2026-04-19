print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
local base = "https://dummyimage.com/16x16/"
local textures = {
    { name = "dirt",  url = base .. "7a5230/fff.png&text=D" },
    { name = "grass", url = base .. "468c3c/fff.png&text=G" },
    { name = "stone", url = base .. "787878/fff.png&text=S" },
    { name = "sand",  url = base .. "e6d3a8/000.png&text=S" },
    { name = "wood",  url = base .. "9c6a3c/fff.png&text=W" }
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