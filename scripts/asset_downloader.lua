print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
-- Usamos dummyimage.com como fallback seguro porque las texturas de Craft dan 404
local base = "https://dummyimage.com/16x16/"
local textures = {
    { name = "dirt",  url = base .. "6e4c34/fff.png&text=dirt" },
    { name = "grass", url = base .. "3d8c40/fff.png&text=grass" },
    { name = "stone", url = base .. "7a7a7a/fff.png&text=stone" },
    { name = "sand",  url = base .. "d4c88a/fff.png&text=sand" },
    { name = "wood",  url = base .. "8f5a35/fff.png&text=wood" }
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