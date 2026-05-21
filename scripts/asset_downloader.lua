print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
-- Fallback a dummyimage.com porque el repo original de Craft no existe/da 404
local textures = {
    { name = "dirt",  url = "https://dummyimage.com/16x16/79553a/fff.png&text=Dirt" },
    { name = "grass", url = "https://dummyimage.com/16x16/4b793a/fff.png&text=Grass" },
    { name = "stone", url = "https://dummyimage.com/16x16/808080/fff.png&text=Stone" },
    { name = "sand",  url = "https://dummyimage.com/16x16/d2b48c/fff.png&text=Sand" },
    { name = "wood",  url = "https://dummyimage.com/16x16/8b5a2b/fff.png&text=Wood" }
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