print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
-- Fallback urls usando dummyimage.com (las originales de GitHub dan 404)
local base = "https://dummyimage.com/16x16/"
local textures = {
    { name = "dirt",  url = base .. "7B3F00/fff.png&text=dirt" },
    { name = "grass", url = base .. "4CBB17/fff.png&text=grass" },
    { name = "stone", url = base .. "808080/fff.png&text=stone" },
    { name = "sand",  url = base .. "C2B280/fff.png&text=sand" },
    { name = "wood",  url = base .. "8B5A2B/fff.png&text=wood" }
}

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