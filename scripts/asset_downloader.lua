print("--- STARTING ASSET MANAGER (LUA) ---")

-- List of dummy textures to prevent 404 errors
local textures = {
    { name = "dirt",  url = "https://dummyimage.com/256x256/8B4513/ffffff.png&text=Dirt" },
    { name = "grass", url = "https://dummyimage.com/256x256/008000/ffffff.png&text=Grass" },
    { name = "stone", url = "https://dummyimage.com/256x256/808080/ffffff.png&text=Stone" },
    { name = "sand",  url = "https://dummyimage.com/256x256/F4A460/ffffff.png&text=Sand" },
    { name = "wood",  url = "https://dummyimage.com/256x256/D2B48C/ffffff.png&text=Wood" }
}

-- Create directory if it doesn't exist (Rust handles this, but here for logical flow)
-- Iterate and download
for i, tex in ipairs(textures) do
    local filename = "assets/textures/" .. i .. ".png" 
    print("Downloading texture [" .. tex.name .. "] from: " .. tex.url)
    
    -- Call to the exposed Rust function
    local success = download_file(tex.url, filename)
    
    if success then
        print("  -> Saved to: " .. filename)
    else
        print("  -> ERROR downloading " .. tex.name)
    end
end

print("--- DOWNLOAD COMPLETE ---")