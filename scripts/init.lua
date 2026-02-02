print("Iniciando carga masiva de texturas...")
-- El motor leerá esto y cargará "assets/textures/block_1.png" en el array
local materials = {}
for i=1, 1000 do
    table.insert(materials, {
        id = i,
        path = "textures/block_" .. i .. ".png",
        properties = { reflectivity = 0.5 }
    })
end
print("Lua: Lista de materiales enviada al motor.")