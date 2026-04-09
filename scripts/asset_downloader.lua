print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
local textures = {
    { name = "dirt",  url = "https://dummyimage.com/256x256/8B4513/8B4513.png" },
    { name = "grass", url = "https://dummyimage.com/256x256/228B22/228B22.png" },
    { name = "stone", url = "https://dummyimage.com/256x256/808080/808080.png" },
    { name = "sand",  url = "https://dummyimage.com/256x256/F4A460/F4A460.png" },
    { name = "wood",  url = "https://dummyimage.com/256x256/A0522D/A0522D.png" }
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