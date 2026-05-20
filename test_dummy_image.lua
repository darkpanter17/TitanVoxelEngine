local base = "https://dummyimage.com/16x16/"
local textures = {
    { name = "dirt",  url = base .. "8b5a2b/ffffff&text=dirt" },
    { name = "grass", url = base .. "3b7e3b/ffffff&text=grass" },
    { name = "stone", url = base .. "7a7a7a/ffffff&text=stone" },
    { name = "sand",  url = base .. "d2b48c/ffffff&text=sand" },
    { name = "wood",  url = base .. "8b4513/ffffff&text=wood" }
}
for i, tex in ipairs(textures) do
    print(tex.url)
end
