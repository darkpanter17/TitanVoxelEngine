print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (usando dummyimage.com como fallback)
local textures = {
    { name = "dirt",  url = "https://dummyimage.com/256x256/8b4513/ffffff.png&text=dirt" },
    { name = "grass", url = "https://dummyimage.com/256x256/228b22/ffffff.png&text=grass" },
    { name = "stone", url = "https://dummyimage.com/256x256/808080/ffffff.png&text=stone" },
    { name = "sand",  url = "https://dummyimage.com/256x256/f4a460/ffffff.png&text=sand" },
    { name = "wood",  url = "https://dummyimage.com/256x256/a0522d/ffffff.png&text=wood" }
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