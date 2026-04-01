print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
-- Lista de texturas CC0 para pruebas (rama master)
local base = "https://raw.githubusercontent.com/fogleman/Craft/master/textures/"
-- Como fogleman/Craft solo tiene texture.png, usaremos la misma para todas por ahora.
local textures = {
    { name = "dirt",  url = base .. "texture.png" },
    { name = "grass", url = base .. "texture.png" },
    { name = "stone", url = base .. "texture.png" },
    { name = "sand",  url = base .. "texture.png" },
    { name = "wood",  url = base .. "texture.png" }
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