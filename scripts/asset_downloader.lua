print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
-- Usamos un servicio dummy para evitar el 404 de los repositorios caídos
local base = "https://dummyimage.com/256x256/"
local textures = {
    { name = "dirt",  url = base .. "644632/ffffff.png&text=dirt" },
    { name = "grass", url = base .. "508c3c/ffffff.png&text=grass" },
    { name = "stone", url = base .. "787878/ffffff.png&text=stone" },
    { name = "sand",  url = base .. "e3d5a4/000000.png&text=sand" },
    { name = "wood",  url = base .. "8b5a2b/ffffff.png&text=wood" }
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