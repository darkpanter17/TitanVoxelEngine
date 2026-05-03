print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
-- Lista de texturas CC0 para pruebas usando dummyimage como fallback
local base = "https://dummyimage.com/16x16/"
local textures = {
    { name = "dirt",  url = base .. "8B4513/fff.png&text=D" },
    { name = "grass", url = base .. "228B22/fff.png&text=G" },
    { name = "stone", url = base .. "808080/fff.png&text=S" },
    { name = "sand",  url = base .. "F4A460/fff.png&text=Sa" },
    { name = "wood",  url = base .. "A0522D/fff.png&text=W" }
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