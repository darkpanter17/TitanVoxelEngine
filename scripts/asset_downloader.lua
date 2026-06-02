print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
local textures = {
    { name = "dirt",  url = "https://dummyimage.com/16x16/7b5d3f/fff.png?text=dirt" },
    { name = "grass", url = "https://dummyimage.com/16x16/5d7b3f/fff.png?text=grass" },
    { name = "stone", url = "https://dummyimage.com/16x16/7b7b7b/fff.png?text=stone" },
    { name = "sand",  url = "https://dummyimage.com/16x16/d2b48c/fff.png?text=sand" },
    { name = "wood",  url = "https://dummyimage.com/16x16/8b5a2b/fff.png?text=wood" }
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