print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
-- Fallback to dummyimage URLs to prevent 404 Not Found HTTP errors
local dummy_base = "https://dummyimage.com/16x16/"
local textures = {
    { name = "dirt",  url = dummy_base .. "8b5a2b/ffffff.png&text=D" },
    { name = "grass", url = dummy_base .. "3bba3b/ffffff.png&text=G" },
    { name = "stone", url = dummy_base .. "888888/ffffff.png&text=S" },
    { name = "sand",  url = dummy_base .. "eadd6f/000000.png&text=Sa" },
    { name = "wood",  url = dummy_base .. "5c4033/ffffff.png&text=W" }
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