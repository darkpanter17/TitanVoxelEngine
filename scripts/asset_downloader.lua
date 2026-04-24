print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
local textures = {
    { name = "dirt",  url = "https://dummyimage.com/16x16/7B3F00/ffffff.png&text=D" },
    { name = "grass", url = "https://dummyimage.com/16x16/228B22/ffffff.png&text=G" },
    { name = "stone", url = "https://dummyimage.com/16x16/808080/ffffff.png&text=S" },
    { name = "sand",  url = "https://dummyimage.com/16x16/C2B280/ffffff.png&text=Sa" },
    { name = "wood",  url = "https://dummyimage.com/16x16/8B5A2B/ffffff.png&text=W" }
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