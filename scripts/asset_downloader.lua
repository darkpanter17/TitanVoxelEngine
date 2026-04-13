print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
local textures = {
    { name = "dirt",  url = "https://dummyimage.com/16x16/7B3F00/7B3F00.png" },
    { name = "grass", url = "https://dummyimage.com/16x16/4C9A2A/4C9A2A.png" },
    { name = "stone", url = "https://dummyimage.com/16x16/808080/808080.png" },
    { name = "sand",  url = "https://dummyimage.com/16x16/C2B280/C2B280.png" },
    { name = "wood",  url = "https://dummyimage.com/16x16/6B4423/6B4423.png" }
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