print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
local base = "https://dummyimage.com/256x256/fff/000.png&text="
local textures = {
    { name = "dirt",  url = base .. "dirt" },
    { name = "grass", url = base .. "grass" },
    { name = "stone", url = base .. "stone" },
    { name = "sand",  url = base .. "sand" },
    { name = "wood",  url = base .. "wood" }
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