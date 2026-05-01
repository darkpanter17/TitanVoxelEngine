print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
-- Lista de texturas CC0 para pruebas
local base = "https://raw.githubusercontent.com/fogleman/Craft/master/textures/"
local textures = {
    { name = "texture",  url = base .. "texture.png" }
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