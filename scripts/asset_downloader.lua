print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
-- Lista de texturas CC0 para pruebas
local base = "https://raw.githubusercontent.com/fogleman/Craft/master/textures/"
local textures = {
    { name = "texture_atlas",  url = base .. "texture.png" }
}

-- Iterar y descargar
for _, tex in ipairs(textures) do
    local filename = "assets/textures/texture.png"
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