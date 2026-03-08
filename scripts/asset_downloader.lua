print("--- INICIANDO GESTOR DE ASSETS (LUA) ---")

-- Lista de texturas CC0 para pruebas (rama main; si falla, probar master)
local base = "https://raw.githubusercontent.com/fogleman/Craft/master/textures/"
-- Fogleman's repo doesn't have individual files for dirt/grass anymore, they are in a sprite sheet `texture.png`.
-- We'll use alternative CC0 block textures, or since this is a test, fallback to other URLs.
-- For this test we can use Kenney NL voxel textures or similar public domain assets if we had them.
-- Since we need 1.png, 2.png, 3.png directly, let's use some reliable URLs (e.g. dummyimage.com for placeholders as a fallback if needed, but we can also let Rust's placeholder take over if the download fails).
-- To try to actually download something valid as PNG for 1, 2, 3:
local fallback_url = "https://dummyimage.com/16x16/ff0000/fff.png"
local fallback_url2 = "https://dummyimage.com/16x16/00ff00/fff.png"
local fallback_url3 = "https://dummyimage.com/16x16/0000ff/fff.png"

local textures = {
    { name = "dirt",  url = fallback_url },
    { name = "grass", url = fallback_url2 },
    { name = "stone", url = fallback_url3 },
}
-- Si main no existe, el script puede probar master cambiando base arriba

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